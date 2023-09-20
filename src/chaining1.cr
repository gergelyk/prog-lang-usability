data = [2, 3, 7, 4, 1]
puts data.map { |x| x**2 }
  .select { |x| x < 20 }
  .sum
