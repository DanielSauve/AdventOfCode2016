f = open('../resources/day6.txt').read().split()

out1 = []
out2 = []
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
            out1.append(char)
        elif num == min(sorted(dic.values())):
            out2.append(char)
print out1
print out2
