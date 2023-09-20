struct Point
  property x : Int32
  property y : Int32

  def initialize(@x, @y)
  end
end

def flip(a : Bool)
  !a
end

# implementation for all integer types
def flip(a : Int)
  -a
end

def flip(a : Point)
  Point.new(a.y, a.x)
end

# implementation for all remaining types
def flip(a)
  puts "Warning: Type #{a.class} not flippable"
  a
end

# types known in advance
p! flip false
p! flip 3_i32
p! flip 33_i64
p! flip Point.new(5, 7)
p! flip 33.3

# types determined in runtime
items = [false, 3_i32, 3_i64, Point.new(5, 7), 33.3]
items.each do |item|
  flipped = flip item
  puts "#{item} flipped is #{flipped}"
end
