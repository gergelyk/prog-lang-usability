class Int(int):
    
    @property
    def dots(self):
        return '.' * self

x = Int(9)

# Method used as a property, so parentheses not needed.
print(x.dots)
