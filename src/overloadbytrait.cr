abstract struct Colorful
  abstract def get_color
end

struct Apple < Colorful
  def get_color
    "green"
  end
end

struct Sky < Colorful
  def get_color
    "blue"
  end
end

def get_color(x : Colorful)
  return x.get_color
end

def get_color(x)
  return "transparent"
end

# types known in advance
p! get_color Apple.new
p! get_color Sky.new
p! get_color 123
p! get_color "example"

# types determined in runtime
items = [Apple.new, Sky.new, 123, "example"]
items.each do |item|
  color = get_color(item)
  puts "#{item} is #{color}"
end
