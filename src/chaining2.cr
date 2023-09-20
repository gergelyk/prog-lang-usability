data = [2, 3, 7, 4, 1]
puts data.map(&.** 2).select(&.< 20).sum
