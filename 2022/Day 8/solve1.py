q = []
visible = []
direct = {'U':(0,1),'D':(0,-1),'L':(-1,0),'R':(1,0)}

def solve(inp):
    n = len(inp)
    m = len(inp[0])
    for i in range(m):
        tree = [0,i]
        temp = inp[tree[0]][tree[1]]
        visible[tree[0]][tree[1]] = True
        for j in range(1,n):
            if inp[j][i] > temp:
                visible[j][i] = True
                temp = inp[j][i]

    for i in range(m):
        tree = [n-1, i]
        temp = inp[tree[0]][tree[1]]
        visible[tree[0]][tree[1]] = True
        for j in range(n-2,0,-1):
            if inp[j][i] > temp:
                visible[j][i] = True
                temp = inp[j][i]

    for j in range(1,n-1):
        tree = [j, 0]
        temp = inp[tree[0]][tree[1]]
        visible[tree[0]][tree[1]] = True
        for i in range(m):
            if inp[j][i] > temp:
                visible[j][i] = True
                temp = inp[j][i]

    for j in range(1, n-1):
        tree = [j, m-1]
        temp = inp[tree[0]][tree[1]]
        visible[tree[0]][tree[1]] = True
        for i in range(m-2,0,-1):
            if inp[j][i] > temp:
                visible[j][i] = True
                temp = inp[j][i]

with open('input') as f:
    inp = f.read().strip().split('\n')
    for i in range(len(inp)):
        inp[i] = list(inp[i])
    visible = [[False for y in range(len(inp[0]))] for x in range(len(inp))]
    solve(inp)
    count = 0
    for x in visible:
        print(x)
        for y in x:
            if y:
                count += 1
    print(count)
