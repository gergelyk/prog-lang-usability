enum Something
  On
  Off
end

def foo(*, something : Something)
  if something == Something::On
    puts "Something is ON"
  else
    puts "Something is OFF"
  end
end

foo(something: :On)
foo(something: :Off)
