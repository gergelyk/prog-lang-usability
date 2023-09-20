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

# calling with positional agrs
foo(:SomethingOn)
foo(:SomethingOff)

# calling with keyword agrs
foo(something: :SomethingOn)
foo(something: :SomethingOff)
