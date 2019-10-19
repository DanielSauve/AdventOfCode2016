f = open("day3.txt").read().split("\n")
array = []
j = 0
while j < f.__len__():
    first = f[j].split()
    second = f[j+1].split()
    third = f[j+2].split()
    array.append([int(first[0]), int(second[0]), int(third[0])])
    array.append([int(first[1]), int(second[1]), int(third[1])])
    array.append([int(first[2]), int(second[2]), int(third[2])])
    j += 3
n = 0
for t in array:
    m = max(t)
    maxIndex = t.index(m)
    otherIndex1, otherIndex2 = (maxIndex+1)%3, (maxIndex+2)%3
    if (t[otherIndex1] + t[otherIndex2]) > m:
        n += 1

print n
