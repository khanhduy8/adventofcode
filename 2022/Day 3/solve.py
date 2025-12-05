import string

def solve(inp):
    priority = [ord(x)-96 if x.islower() else ord(x) - 38 for x in string.ascii_letters]
    mapping = {}
    for x in range(len(priority)):
        mapping[string.ascii_letters[x]] = priority[x]

    list_items = []

    for step in inp:
        mid = int(len(step)/2)

        for i in range(mid):
            if step[i] in step[mid:]:
                list_items.append(step[i])
                break
    print(list_items)
    print(sum([mapping[x] for x in list_items]))

with open('input') as f:
    inp = f.read().strip().split('\n')
    solve(inp)
