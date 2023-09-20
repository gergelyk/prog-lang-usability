def connection(address)
  puts "Connection open to #{address}"
  begin
    yield 123
  ensure
    puts "Connection closed"
  end
end

connection("target") do |conn|
  puts "Doing something well"
end

connection("target") do |conn|
  raise "Doing something wrong"
end
