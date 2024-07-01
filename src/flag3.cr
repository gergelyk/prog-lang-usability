enum Something
  SomethingOn
  SomethingOff
end

def foo(something : Something)
  # fully qualified name is needed here
  if something == Something::SomethingOn
    puts "Something is ON"
  else
    puts "Something is OFF"
  end
end

foo(:SomethingOn)
foo(:SomethingOff)
