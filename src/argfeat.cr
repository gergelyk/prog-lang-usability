def foo(bar = true, baz = 123, qux = "abc")
  puts "bar=#{bar}, baz=#{baz}, qux=#{qux}"
end

foo()
foo(qux: "hello")
foo(qux: "hello", bar: true, baz: 987)
foo(false, qux: "hello", baz: 987)
