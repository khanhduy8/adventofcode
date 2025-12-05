from bigtree import Node, print_tree
node = []
root = Node('/')
dir_size = []
def solve(inp,current_node,root):
    for line in inp:
        line = line.split(' ')
        if line[0] == '$':
            if line[1] == 'cd':
                if line[2] == '..':
                    if current_node.parent is not None:
                        current_node = current_node.parent
                elif line[2] == '/':
                    current_node = root
                else:
                    for child in current_node.children:
                        if child.name == line[2]:
                            current_node = child
            elif line[1] == 'ls':
                continue
        elif line[0] == 'dir':
            Node(line[1], size=0, parent=current_node)
        else:
            Node(line[1], size=int(line[0]),file=True, parent=current_node)

def update_size(parent_node):
    for child in parent_node.children:
        if parent_node.get_attr('size') is None:
            parent_node.__setattr__('size',0)
        if child.get_attr('size') is not None:
            parent_node.__setattr__('size',parent_node.get_attr('size') + update_size(child))
    return parent_node.get_attr('size')

def travel_child(parent_node):
    if parent_node.get_attr('file') is None:
        dir_size.append(parent_node.get_attr('size'))
    for child in parent_node.children:
        travel_child(child)

with open('input') as f:
    inp = f.read().strip().split('\n')
    node.append(root)
    solve(inp,current_node=root,root=root)
    update_size(root)
    travel_child(root)
    unusespace = 70_000_000 - root.get_attr('size')
    needspace = 30_000_000
    dir_size.sort()
    for size in dir_size:
        if size + unusespace >= needspace:
            print(size)
            break




