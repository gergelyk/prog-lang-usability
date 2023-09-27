lib Libc
    alias PidT = Int32
    fun getpid : PidT
end

pid = Libc.getpid
puts(pid)
