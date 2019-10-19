f = open('day2.txt').read().split("\n")
pad = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
x, y = 1, 1
code = []

for row in f:
    for instruction in row:
        if instruction == "U":
            y -= 1
        elif instruction == "D":
            y += 1
        elif instruction == "L":
            x -= 1
        elif instruction == "R":
            x += 1
        if x < 0:
            x = 0
        if y < 0:
            y = 0
        if y > 2:
            y = 2
        if x > 2:
            x = 2
    code.append(pad[y][x])
print code
