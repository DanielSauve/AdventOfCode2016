f = open('../resources/day2.txt').read().split("\n")
pad = [[None, None, 1, None, None], [None, 2, 3, 4, None], [5, 6, 7, 8, 9], [None, "A", "B", "C", None], [None, None, "D", None, None]]
x, y = 0, 2
code = []

for row in f:
    for instruction in row:
        if instruction == "U":
            if not y == 0 and not pad[x][y-1] == None:
                y -= 1
        elif instruction == "D":
            if not y == 4 and not pad[x][y+1] == None:
                y += 1
        elif instruction == "L":
            if not x == 0 and not pad[x-1][y] == None:
                x -= 1
        elif instruction == "R":
            if not x == 4 and not pad[x+1][y] == None:
                x += 1
        if x < 0:
            x = 0
        if y < 0:
            y = 0
        if y > 4:
            y = 4
        if x > 4:
            x = 4
    code.append(pad[y][x])
print code
