import re

f = open('day7.txt', 'r').read().split("\n")
count = 0
ssl = 0
for i in f:
    b = str.join('', re.findall('\[.*?\]', i))
    i = re.sub('\[.*?\]', ' ', i)
    index = 0
    tls = True
    while index < len(b) - 3:
        if b[index] == b[index + 3] and b[index + 1] == b[index + 2]:
            tls = False
            break
        index += 1
    index = 0
    counted = False
    ssled = False
    if i[0] == i[2] and str(i[1:3] + i[1]) in b:
        ssl += 1
        ssled = True
    while index < len(i) - 3:
        if tls and not counted and i[index] == i[index + 3] and i[index + 1] == i[index + 2] and \
                i[index] != i[index + 1]:
            count += 1
            counted = True
        elif not ssled and i[index + 1] == i[index + 3] and str(i[index + 2:index + 4] + i[index + 2]) in b:
            ssl += 1
            ssled = True
        index += 1
print(count)
print(ssl)
