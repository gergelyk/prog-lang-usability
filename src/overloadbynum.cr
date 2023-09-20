def say_hello(firstname : String)
  puts "Hi #{firstname}!"
end

def say_hello(firstname : String, surname : String)
  puts "Hello #{firstname} #{surname}!"
end

say_hello "John"
say_hello("Tom", "Brown")
