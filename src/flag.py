def foo(*, something_is_on):
    if something_is_on:
        print("Something is ON")
    else:
        print("Something is OFF")

foo(something_is_on=True)
foo(something_is_on=False)
