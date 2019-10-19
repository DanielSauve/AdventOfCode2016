def check():
    if {X, Y} in P:
        print abs(X)+abs(Y)
    P.append({X, Y})

f = open('day1.txt', 'r').read().split(", ")
X, Y, D, P = 0, 0, 1, []
for s in f:
    z = int(s[1:])
    if s[0] == "R":
        D = (D+1)%4
    else:
        D = (D-1)%4
    if D == 1:
        for p in range(0, z):
            Y += 1
            check()
    elif D == 2:
        for p in range(0, z):
            X += 1
            check()
    elif D == 3:
        for p in range(0, z):
            Y -= 1
            check()
    else:
        for p in range(0, z):
            X -= 1
            check()
print abs(X)+abs(Y)
