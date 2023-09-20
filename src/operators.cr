class Hundred
  def +(other)
    other + 100
  end
end

struct Number
  def +(other : Hundred)
    other + self
  end
end

puts 2 + Hundred.new
puts Hundred.new + 4
puts Hundred.new + 4.5
