#!env python3

w = [ ]
l = None
n = 0
with open('001.txt', 'r') as f:
    for line in f:
        d = int(line)
        w.append(d)
        if len(w) == 3:
            t = w[0] + w[1] + w[2]
            print(f'{d}, {w[0]}+{w[1]}+{w[2]} = {t}')
            if l is not None and t > l:
                n += 1
            l = t
            w.pop(0)

print(n)
