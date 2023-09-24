class Inspectable:
    def show_vars(self):
        for field_name in dir(self):
            if not (field_name.startswith('__') and field_name.endswith('__')):
                field_value = getattr(self, field_name)
                if not callable(field_value):
                    print(f"{field_name!r} : {field_value.__class__.__name__}")

class Foo(Inspectable):
    def __init__(self):
        self.bar = 123
        self.baz = "qwx"

foo = Foo()
foo.show_vars()
