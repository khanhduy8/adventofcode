def solve(status, steps):
    steps = steps.strip().split('\n')
    status = status.split('\n')
    #create carge
    carge = []
    num = len(status[-1].strip().split('  '))
    for i in range(num):
        carge.append([])
    for i in range(len(status)-2,-1,-1):
        line = status[i]
        p = 0
        for j in range(0,num*3+num-1,4):
            try:
                crate = line[j:j+3]
                if len(crate.strip()) > 0:
                    carge[p].append(crate)
                p = p + 1
            except:
                continue
    for step in steps:
        s = step.split(' ')
        temp = []
        for move in range(int(s[1])):
            c = carge[int(s[3])-1].pop()
            temp.append(c)
        for c in temp[::-1]:
            carge[int(s[5])-1].append(c)

    result = ''
    for c in carge:
        try:
            result += c[-1].strip('[]')
        except:
            continue
    print(result)

with open('input') as f:
    inp = f.read().split('\n\n')
    status = inp[0]
    step = inp[1]
    solve(status, step)