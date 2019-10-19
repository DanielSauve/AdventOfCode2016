f = open("day3.txt").read().split("\n")
n = 0
for t in f:
    x = [int(num) for num in t.split()]
    m = max(x)
    maxIndex = x.index(m)
    otherIndex1, otherIndex2 = (maxIndex + 1) % 3, (maxIndex + 2) % 3
    if (x[otherIndex1] + x[otherIndex2]) > m:
        n += 1

print(n)
