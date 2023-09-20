struct Apple
  def get_color
    "green"
  end
end

struct Sky
  def get_color
    "blue"
  end
end

def get_color(x)
  if x.responds_to?(:get_color)
    return x.get_color
  end
  return "transparent"
end
