data = [2, 3, 7, 4, 1]
sqr = (x**2 for x in data)
lim = (x for x in sqr if x < 20)
res = sum(lim)
print(res)
