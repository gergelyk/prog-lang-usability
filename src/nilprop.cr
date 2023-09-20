def div(a : Float32, b : Float32) : Float32?
  return nil if b == 0.0
  a / b
end

def div_and_add(a : Float32, b : Float32) : Float32?
  div(a, b).try do |n|
    div(b, a).try do |m|
      if (s = m + n) < 10
        s
      end
    end
  end
end

def show(a : Float32, b : Float32)
  puts div_and_add(a, b) || "None"
end

show(2.0, 4.0)   # 2.5
show(0.0, 4.0)   # None
show(2.0, 0.0)   # None
show(100.0, 1.0) # None
