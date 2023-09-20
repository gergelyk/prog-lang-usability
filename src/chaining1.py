data = [2, 3, 7, 4, 1]
sqr = map(lambda x: x**2, data)
lim = filter(lambda x: x < 20, sqr)
res = sum(lim)
print(res)
