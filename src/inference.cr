# generic function
def foo(x, y)
  x + y
end

class C
  # type needs to be specified
  def initialize(@x : Int32)
  end

  # type is inferred
  def get_x
    @x
  end
end

puts foo(2, 3)             # calls foo<Int32, Int32>:Int32
puts foo("hello", "world") # calls foo<String, String>:String

c = C.new(123)
puts c.get_x
