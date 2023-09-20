require "syscall"

WEXITED = 4
P_PID   = 1

module OurSysCalls
  Syscall.def_syscall fork, Int32
  Syscall.def_syscall waitid, Int32, idtype : Int32, id : Int32, infop : Int32*, options : Int32
  Syscall.def_syscall exit, Int32, status : Int32
end

struct Fork
  @pid : Int32?

  def initialize(&@target)
  end

  def start
    pid = OurSysCalls.fork
    if pid == 0
      # Child process
      @target.call
      OurSysCalls.exit 0
    else
      # Main process
      p! pid
      @pid = pid
    end
  end

  def join
    raise "Process not started" if !@pid
    OurSysCalls.waitid(P_PID, @pid.not_nil!, Pointer(Int32).new(0), WEXITED)
  end
end

puts "Main process started"
producer_proc = Fork.new do
  puts "Producer process started"
  sleep(0.001 * rand(5))
  puts "Producer process finished"
  txt = STDIN.read_line
  STDOUT.puts txt
end

# consumer_proc = Fork.new do
# puts "Consumer process started"
# sleep(0.001 * rand(5))
# puts "Consumer process finished"
# end

# consumer_proc.start
producer_proc.start
producer_proc.join
# consumer_proc.join
puts "Main process finished"
