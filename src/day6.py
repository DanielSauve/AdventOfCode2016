f = open('../resources/day6.txt').read().split()

out = []
length = len(f[0])
for i in range(length):
    dic = dict()
    for c in f:
        if dic.has_key(c[i]):
            dic[c[i]] = dic[c[i]] + 1
        else:
            dic[c[i]] = 1
    for char, num in dic.iteritems():
        if num == max(sorted(dic.values())):
            out.append(char)
print out
