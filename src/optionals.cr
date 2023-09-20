def foo(x : Int32?)
  # note that here only nil evaluates to false
  x = x || 123
  puts x
end

foo 321 # 321
foo 0   # 0
foo nil # 123
