def foo(bar=True, baz=123, qux="abc"):
    print(f"bar={bar}, baz={baz}, qux={qux}")
    
foo()
foo(qux="hello")
foo(qux="hello", bar=True, baz=987)
foo(False, qux="hello", baz=987)
