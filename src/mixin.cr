module Greeting
  def hello
    puts "Hello, my name is #{@name}. How can I help you?"
  end
end

module Farewell
  def bye
    puts "Thank you. Bye!"
  end
end

class ChatBot
  include Greeting
  include Farewell

  def initialize(@name : String)
  end
end

trx = ChatBot.new "R2D2"
trx.hello
trx.bye
