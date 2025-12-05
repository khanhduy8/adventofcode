with open('input') as f:
    inp = f.read()
    inp = inp.split('\n\n')

    for i in range(len(inp)):
        inp[i] = inp[i].split('\n')
        inp[i] = [int(x) if len(x) > 0 else 0 for x in inp[i] ]
        inp[i] = sum(inp[i])

    inp.sort(reverse=True)
    print(sum(inp[:3]))
