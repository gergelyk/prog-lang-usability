class ZeroDivisionError < Exception
end

class Overflow < Exception
end

def div(a : Float, b : Float) : Float
  if b == 0.0
    raise ZeroDivisionError.new("Division by zero is illegal")
  end
  a / b
end

def div_and_add(a : Float, b : Float) : Float
  n = div(a, b)
  m = div(b, a)
  s = m + n
  if s >= 10
    raise Overflow.new("Sum is too large")
  end

  return s
end

def show(a, b)
  begin
    res = div_and_add(a, b)
  rescue exc : ZeroDivisionError
    puts "Zero division error: #{exc}"
  rescue exc : Overflow
    puts "Overflow error: #{exc}"
  rescue exc
    puts "Error: #{exc}"
  else
    puts "Result: #{res}"
  end
end

show 2.0, 4.0   # Result: 2.5
show 0.0, 4.0   # Zero division error: Division by zero is illegal
show 2.0, 0.0   # Zero division error: Division by zero is illegal
show 100.0, 1.0 # Overflow error: Sum is too large
