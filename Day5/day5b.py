import hashlib

PUZZLE = 'wtnhxymk'
output = [None, None, None, None, None, None, None, None]
count = 0
index = 0
while count < 8:
    test = hashlib.md5(PUZZLE + str(index)).hexdigest()
    if test[0:5] == '00000':
        try:
            num = int(test[5])
        except ValueError:
            index += 1
            continue
        if num < 8 and output[num] is None:
            output[num] = test[6]
            count += 1
    index += 1
print(output)
