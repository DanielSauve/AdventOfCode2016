f = open('day4.txt', 'r').read().split('\n')
total = 0

for i in f:
    contains = True
    count = dict()
    code = i[-7:].strip('[]')
    room = int(i[-10:-7])
    encrypt = i[:-10]
    for c in code:
        if c not in encrypt:
            contains = False
    if not contains:
        continue
    unencrypt = []
    for c in encrypt:
        if c in count:
            count[c] = count[c] + 1
        else:
            count[c] = 1
        if chr((ord(c) + (room % 26))) < 'z':
            unencrypt.append(chr(ord(c) + (room % 26)))
        else:
            unencrypt.append(chr(ord(c) - (26 - (room % 26))))
    output = ''.join(unencrypt)
    if output.find('northpole') != -1:
        print(output)
        print(room)
    for c in code:
        index = code.index(c)
        index = -index - 1
        if not count[c] == sorted(count.values())[index]:
            contains = False
    if not contains:
        continue
    total += room
print(total)
