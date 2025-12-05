def fight(x, y):
    mapping = {'X': 'A', 'Y' : 'B', 'Z' : 'C' }
    y = mapping[y]
    if x == y:
        return 'Y'
    if y == 'A':
        if x == 'C':
            return 'Z'
        else:
            return 'X'
    if y == 'B':
        if x == 'A':
            return 'Z'
        else:
            return 'X'
    if y == 'C':
        if x == 'B':
            return 'Z'
        else:
            return 'X'

def shape_score(x,y):
    s = ['X', 'Y', 'Z']
    mapping = {'X': 1, 'Y': 2, 'Z': 3}
    for step in s:
        if fight(x,step) == y:
            return mapping[step]


def result_score(y):
    mapping = {'X': 0, 'Y': 3, 'Z': 6}
    return mapping[y]

f = open('input')
inp = f.read().strip().split('\n')
inp = [x.split(' ') for x in inp]
s = 0
for x in inp:
    s = s + result_score(x[1]) + shape_score(x[0],x[1])
    print(result_score(x[1]), shape_score(x[0],x[1]))
print(s)




