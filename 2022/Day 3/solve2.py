import string

def solve(inp):
    priority = [ord(x)-96 if x.islower() else ord(x) - 38 for x in string.ascii_letters]
    mapping = {}
    for x in range(len(priority)):
        mapping[string.ascii_letters[x]] = priority[x]
    list_badge = []
    for i in range(0,len(inp),3):
        for c in string.ascii_letters:
            if c in inp[i] and c in inp[i+1] and c in inp[i+2]:
                list_badge.append(c)

    print(sum([mapping[x] for x in list_badge]))


with open('input') as f:
    inp = f.read().strip().split('\n')
    solve(inp)
