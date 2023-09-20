def say_hello(firstname : String, surname : String? = nil)
  if surname
    puts "Hello #{firstname} #{surname}!"
  else
    puts "Hi #{firstname}!"
  end
end

say_hello "John"
say_hello("Tom", "Brown")
