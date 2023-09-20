enum Something
  On
  Off
end

def foo(something : Something)
  if something == Something::On
    puts "Something is ON"
  else
    puts "Something is OFF"
  end
end

# fully qualified names are ok
foo(Something::On)
foo(Something::Off)

# but symbols also do the job
foo(:On)
foo(:Off)

# use kwargs for readability
foo(something: :On)
foo(something: :Off)
