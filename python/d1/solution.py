import os

def part_one(path):
    file = open(path)
    data = file.read()
    elf_calories = data.split("\n\n")
    largest = 0
    for elf in elf_calories:
        calories = elf.split()
        res = sum(map(lambda s: int(s), calories))
        if res > largest:
            largest = res
    return largest

def part_two(path):
    file = open(path)
    data = file.read()
    elf_calories = data.split("\n\n")
    return sum(sorted(list(map(lambda calories: sum(map(lambda calorie: int(calorie) ,calories.split())), elf_calories)), reverse=True)[:3])


if __name__ == "__main__":
    path = os.getcwd() + "/input.txt"
    print(part_one(path))
    print(part_two(path))