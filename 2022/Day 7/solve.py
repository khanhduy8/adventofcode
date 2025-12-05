tree_dir = {'/': {'dir': [], 'file': []}}
parent = {'/': '/'}
dir_size = {'/': 0}
current_dir = '/'
def solve(line, current_dir):
    line = line.split(' ')
    if line[0] == '$':
        if line[1] == 'cd':
            if line[2] != '..':
                current_dir = line[2]
            else:
                current_dir = parent[current_dir]
    else:
        if line[0] == 'dir':
            try:
                tree_dir[current_dir].keys()
            except:
                tree_dir[current_dir] = {'dir': [], 'file': []}
                dir_size[current_dir] = 0
            tree_dir[current_dir]['dir'].append(line[1])
            parent[line[1]] = current_dir
        else:
            try:
                tree_dir[current_dir].keys()
            except:
                tree_dir[current_dir] = {'dir': [], 'file': []}
                dir_size[current_dir] = 0
            tree_dir[current_dir]['file'].append((int(line[0]),line[1]))
            dir_size[current_dir] += int(line[0])
    return current_dir

def update_parent_dir(parent_dir):
    for child_dir in tree_dir[parent_dir]['dir']:
        dir_size[parent_dir] += update_parent_dir(child_dir)
    return dir_size[parent_dir]


with open('input') as f:
    inp = f.read().strip().split('\n')
    result = 0
    for line in inp:
        current_dir = solve(line,current_dir)
    # print(tree_dir)
    # update_parent_dir('/')
    # for x in dir_size:
    #     if dir_size[x] <= 100000:
    #         result += dir_size[x]
    # print(dir_size)
    print(tree_dir['btjwzz'])
