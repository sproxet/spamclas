import openai

examples = [
    ["Buy free penis enlargement pills!", "Spam"],
    ["Hello, world!", "Not Spam"],
    ["Join our crypto chat room for the latest signals from these huge WHALES!", "Spam"],
    ["Join hot singles online!", "Spam"],
    ["OpenAI is a really cool project.", "Not Spam"],
    ["Fast $$$$", "Spam"],
    ["Sign up for MILFs here", "Spam"],
    ["Mules are a cross between donkeys and horses.", "Not Spam"],
    ["Loki was a Norse god.", "Not Spam"],
    ["Best Forex Signals Channel", "Spam"],
    ["Home prices in the US have fallen for the third month in a row.", "Not Spam"]
]

def detect_spam(channel_id, user_id, username, message):
    print(message)
    message = message.replace("\n", " ")
    prompt = "\n\n---\n\n".join([f"Message: {example[0]}\nClassification: {example[1]}" for example in examples]) + f"Message: {message}\nClassification:"
    completion = openai.Completion.create(stop="---", prompt=prompt, engine="ada")
    print(completion)
    return completion.choices[0].text.strip() == "Spam"
