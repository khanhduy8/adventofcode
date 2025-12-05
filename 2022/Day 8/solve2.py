q = []
score = []
direct = {'U':(0,1),'D':(0,-1),'L':(-1,0),'R':(1,0)}

def solve(inp):
    n = len(inp)
    m = len(inp[0])
    for i in range(n):
        for j in range(m):
            inp[i][j]
            s = 1
            for d in direct:
                x = i
                y = j
                count = 0
                while True:
                    x = x + direct[d][0]
                    y = y + direct[d][1]
                    if x < 0 or x >= n or y < 0 or y >= m:
                        break
                    if inp[x][y] < inp[i][j]:
                        count += 1
                    else:
                        count += 1
                        break
                s *= count
            score[i][j] = s

with open('input') as f:
    inp = f.read().strip().split('\n')
    for i in range(len(inp)):
        inp[i] = list(inp[i])
    score = [[0 for y in range(len(inp[0]))] for x in range(len(inp))]
    solve(inp)
    max = 0
    for x in score:
        for y in x:
            if y > max:
                max = y
    print(max)