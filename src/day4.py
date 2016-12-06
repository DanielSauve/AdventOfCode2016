f = open('../resources/day4.txt', 'r').read().split('\n')
total = 0

for i in f:
    contains = True
    count = dict()
    code = i[-7:].translate(None, '[]')
    room = int(i[-10:-7])
    encrypt = i[:-10].translate(None, '-')
    for c in code:
        if c not in encrypt:
            contains = False
    if not contains:
        continue
    for c in encrypt:
        if count.has_key(c):
            count[c] = count[c]+1
        else:
            count[c] = 1
    for c in code:
        index = code.index(c)
        index = -index -1
        if not count[c] == sorted(count.values())[index]:
            contains = False
    if not contains:
        continue
    total += room
print total
