fun getpid() : Int32
    pid = uninitialized Int32
    
    asm("
    movl $$20, %eax
    int $$0x80
    movl %eax, $0" : "=r"(pid))
    
    return pid
end

puts getpid
