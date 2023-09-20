N = 10

def shift(x)
  sleep(0.001 * rand(10))
  puts "Done with #{x}"
  100 * x
end

alias ResultChannel = Channel(Int32)
result_channels = [] of ResultChannel

N.times do |x|
  result_channel = ResultChannel.new
  result_channels << result_channel
  spawn { result_channel.send(shift(x)) }
end

puts "All spawned"
results = result_channels.map &.receive
puts "Results: #{results}"
