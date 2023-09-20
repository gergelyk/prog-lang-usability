import re

text = "Hello John!"

if m := re.search("(\w+)!$", text):
    name = m.groups()[0]
    print(f"Name: {name}")
