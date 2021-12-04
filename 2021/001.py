#!env python3

last = None
inc = 0
with open('001.input.txt', 'r') as f:
    for line in f:
        d = int(line)
        if last is not None and d > last:
            inc += 1
        last = d
print(inc)
