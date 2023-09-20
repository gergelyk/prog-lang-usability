# by-val version

def inc(x : Int32)
  return x + 1
end

def inc(x : Bool)
  return !x
end

puts inc 3
puts inc false

# by-ref version

def inc!(x : Pointer(Int32))
  x.value += 1
end

def inc!(x : Pointer(Bool))
  x.value != 1
end

x = 3
inc!(pointerof(x))
puts x

y = false
inc!(pointerof(y))
puts y
