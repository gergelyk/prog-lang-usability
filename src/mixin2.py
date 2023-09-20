def mixin(*sources):
    def decorator(target):
        for src in sources:
            for key in dir(src):
                if not (key.startswith('__') and key.endswith('__')):
                    setattr(target, key, getattr(src, key))
        return target
    return decorator

class Greeting:
    def hello(self):
        print(f"Hello, my name is {self.name}. How can I help you?")

class Farewell:
    def bye(self):
        print("Thank you. Bye!")

@mixin(Greeting, Farewell)
class ChatBot:
    def __init__(self, name):
        self.name = name

bot = ChatBot("R2D2")
bot.hello()
bot.bye()
