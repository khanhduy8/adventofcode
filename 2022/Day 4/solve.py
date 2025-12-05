def solve(inp):
    items = []
    for pair in inp:
        pairs = pair.split(',')
        item = []
        item.append([int(x) for x in pairs[0].split('-')])
        item.append([int(x) for x in pairs[1].split('-')])
        items.append(item)
    count = 0
    for item in items:
        section1 = item[0]
        section2 = item[1]
        if section2[0] <= section1[0]:
            section1, section2 = section2, section1

        if section2[0] >= section1[0] and section2[1] <= section1[1]:
            count += 1
        elif section1[0] >= section2[0] and section1[1] <= section2[1]:
            count += 1
            print(section1, section2)


    print(count)

with open('input') as f:
    inp = f.read().strip().split('\n')
    solve(inp)
