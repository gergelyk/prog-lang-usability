text = "Hello John!"

/(\w+)\!$/.match(text).try do |x|
  name = x[1]
  puts "Name: #{name}"
end
