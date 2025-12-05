def fight(x, y):
    mapping = {'X': 'A', 'Y' : 'B', 'Z' : 'C' }
    y = mapping[y]
    if x == y:
        return 3
    if y == 'A':
        if x == 'C':
            return 6
        else:
            return 0
    if y == 'B':
        if x == 'A':
            return 6
        else:
            return 0
    if y == 'C':
        if x == 'B':
            return 6
        else:
            return 0

def shape_score(y):
    mapping = {'X': 1, 'Y': 2, 'Z': 3}
    return mapping[y]

f = open('input')
inp = f.read().strip().split('\n')
inp = [x.split(' ') for x in inp]
s = 0
for x in inp:
    s = s + fight(x[0], x[1]) + shape_score(x[1])
    print(fight(x[0], x[1]), shape_score(x[1]))
print(s)


