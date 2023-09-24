class Inspectable:
    def show_vars(self):
        for field_name, field_type in self.__annotations__.items():
            print(f"{field_name!r} : {field_type.__name__}")

class Foo(Inspectable):
    bar: int
    baz: str

foo = Foo()
foo.show_vars()
