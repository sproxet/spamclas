use std::mem::transmute;
use pyo3::prelude::*;
use telegram_bot::*;
use std::fs::File;
use std::io::Read;
use futures::StreamExt;
use crossbeam_channel::{Sender, Receiver, unbounded};

fn python_runner(sender: Sender<bool>, receiver: Receiver<Message>, code: String) {
    std::thread::spawn(move || {
        Python::with_gil(|py| {
            let code = PyModule::from_code(py, &code, "detect_spam.py", "detect_spam").unwrap();
            let check = code.getattr("detect_spam").unwrap();

            for m in receiver.iter() {
                sender.send(match m.kind {
                    MessageKind::Text { data, entities: _} => {
                        let chat_id: i64 = unsafe { transmute(m.chat.id()) };
                        let user_id: i64 = unsafe { transmute(m.from.id) };
                        check.call1((chat_id, user_id, m.from.username, data)).unwrap().extract().unwrap()
                    },
                    _ => false
                }).unwrap();
            }
        })
    });
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let (msg_sender, msg_receiver) = unbounded();
    let (result_sender, result_receiver) = unbounded();

    let mut code = String::new();
    File::open(&args[2]).unwrap().read_to_string(&mut code).unwrap();
    python_runner(result_sender, msg_receiver, code.to_owned());

    let api = Api::new(&args[1]);
    let mut telegram_stream = api.stream();

    while let Some(update) = telegram_stream.next().await {
        if let Ok(update) = update {
            match update.kind {
                UpdateKind::Message(msg) => {
                    msg_sender.send(msg.clone()).unwrap();
                    let res = result_receiver.recv().unwrap();
                    if res {
                        api.send(msg.text_reply("Don't be naughty!")).await.unwrap();
                    }
                },
                _ => {}
            }
        }
    }
}