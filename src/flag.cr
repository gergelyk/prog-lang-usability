def foo(something_is_on : Bool)
  if something_is_on
    puts "Something is ON"
  else
    puts "Something is OFF"
  end
end

foo(something_is_on: true)
foo(something_is_on: false)
