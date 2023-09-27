require "syscall"

module OurSysCalls
  Syscall.def_syscall getpid, Int32
end

pid = OurSysCalls.getpid
puts(pid)
