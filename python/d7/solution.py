import os
from collections import defaultdict
from typing import Optional

class Node:
    def __init__(self, name, is_dir=False, size=0):
        self.name = name
        self.size = size
        self.is_dir = is_dir
        self.parent: Optional[Node] = None
        self.children = []

    def add_child(self, child):
        self.children.append(child)
        child.parent = self

    def find_child_by_name(self, name):
        child = list(filter(lambda node: node.name == name, self.children))[0]
        return child

    def to_string(self):
        return "Node {}, size: {}, dir: {} ".format(self.name, self.size, self.is_dir)

    def print_tree(self):
        print(self.to_string())
        for child in self.children:
            child.print_tree()


def build_tree(path):
    file = open(path)
    data = file.read()
    lines = iter(data.split("\n"))
    sentinel = Node("")
    cur_node = sentinel
    root_node = Node("/", is_dir=True)
    sentinel.add_child(root_node)
    root_node.parent = sentinel
    for line in lines:
        match line.split(" "):
            case ["$", *args]:  # this is a command
                match args:
                    case [cd, target]:
                        match target:
                            case "..":
                                cur_node = cur_node.parent
                            case _:
                                try:
                                    node: Node = cur_node.find_child_by_name(target)
                                    node.parent = cur_node
                                    cur_node = node
                                except Exception as e:
                                    print("File does not exist or not known as a child")
                    case [ls]:
                        continue
            #this is where we're listing files
            case [str, file_name]:
                match str:
                    case "dir":
                        node = Node(file_name, is_dir=True)
                        node.parent = cur_node
                        cur_node.add_child(node)
                    case size:
                        size = int(size)
                        node = Node(file_name, is_dir=False, size=size)
                        node.parent = cur_node
                        cur_node.add_child(node)
    return sentinel

node_sizes = {}
res = 0


def calculate_sizes(node):
    global res
    if node in node_sizes:
        return node_sizes[node]
    else:
        size = node.size
        for child in node.children:
            size += calculate_sizes(child)
        node_sizes[node] = size
        node.size = size
        if size <= 100000 and node.is_dir == True:
            res += size
        return size

smallest_to_delete = 0
smallest_difference = 0

def get_node_to_delete(node, target):
    global smallest_to_delete
    if node.size >= target and node.is_dir:
        if node.size - target < smallest_to_delete - target:
            smallest_to_delete = node.size
        for child in node.children:
            get_node_to_delete(child, target)


def part_one(path):
    global res
    root = build_tree(path).children[0]
    root_size = calculate_sizes(root)
    root.print_tree()
    return res

def part_two(path):
    global smallest_to_delete
    root = build_tree(path).children[0]
    calculate_sizes(root)
    total_space = 70000000
    smallest_to_delete = total_space
    used_space = root.size
    unused_space = total_space-used_space
    print(f"unused space: {unused_space}")

    get_node_to_delete(root, target=30000000-unused_space)
    return smallest_to_delete


if __name__ == "__main__":
    path = os.getcwd() + "/input.txt"
    res1 = part_one(path)
    print(res1)
    res2 = part_two(path)
    print(res2)
