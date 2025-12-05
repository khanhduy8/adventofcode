q = []
visible = []
direct = {'U':(0,1),'D':(0,-1),'L':(-1,0),'R':(1,0)}

def bfs(n,m):
    while len(q) > 0:
        tree = q.pop(0)
        visible[tree[0]][tree[1]] = True
        tree_tall = inp[tree[0]][tree[1]]
        for d in direct:
            new_cord = [tree[0] + direct[d][0], tree[1] + direct[d][1]]
            if 0 <= new_cord[0] < n and 0 <= new_cord[1] < m and not visible[new_cord[0]][new_cord[1]] and inp[new_cord[0]][new_cord[1]] > tree_tall:
                q.append(new_cord)
                visible[new_cord[0]][new_cord[1]] = True


def solve(inp):
    n = len(inp)
    m = len(inp[0])
    for i in range(m):
        q.append([0,i])
        q.append([n-1,i])
    for j in range(1,n-1):
        q.append([j,0])
        q.append([j,m-1])
    bfs(n,m)

with open('input') as f:
    inp = f.read().strip().split('\n')
    for i in range(len(inp)):
        inp[i] = list(inp[i])
    visible = [[False for y in range(len(inp[0]))] for x in range(len(inp))]
    solve(inp)
    count = 0
    for i in visible:
        print(i)
        for j in i:
            if j:
                count += 1
    print(count)