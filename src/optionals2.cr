def bar(x : Bool?)
  # x = x || true # not valid, it always gives true
  x = x.nil? ? true : x
  puts x
end

bar false # false
bar true  # true
bar nil   # true
