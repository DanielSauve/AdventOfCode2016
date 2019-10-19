import hashlib

PUZZLE = 'wtnhxymk'
output = []

index = 0
while len(output) < 8:
    test = hashlib.md5(PUZZLE+str(index)).hexdigest()
    if test[0:5] == '00000':
        output.append(test[5])
    index += 1
print output
