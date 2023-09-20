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
alias Any = Pointer(Void)

animals = [] of Any

animals << Box(Animal).box(Cat.new)
animals << Box(Animal).box(Cow.new)

animals.each do |animal|
  unboxed = Box(Animal).unbox(animal)
  if cat = unboxed.as?(Cat)
    cat.make_sound
  elsif cow = unboxed.as?(Cow)
    cow.make_sound
  end
end
