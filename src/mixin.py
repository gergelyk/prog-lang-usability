class Greeting:
    def hello(self):
        print(f"Hello, my name is {self.name}. How can I help you?")

class Farewell:
    def bye(self):
        print("Thank you. Bye!")

class ChatBot(Greeting, Farewell):
    def __init__(self, name):
        self.name = name

bot = ChatBot("R2D2")
bot.hello()
bot.bye()
