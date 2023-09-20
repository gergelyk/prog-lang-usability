class Cat
  def make_sound
    puts "miao"
  end
end

class Cow
  def make_sound
    puts "mooo"
  end
end

alias Animal = Cat | Cow
animals = [] of Animal
animals << Cat.new
animals << Cow.new

animals.each &.make_sound()
