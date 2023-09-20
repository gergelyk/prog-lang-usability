class Hundred:
    def __radd__(self, other):
        return other + 100

    def __add__(self, other):
        return other + self

print(2 + Hundred())   # 102
print(Hundred() + 4)   # 104
print(Hundred() + 4.5) # 104.5
