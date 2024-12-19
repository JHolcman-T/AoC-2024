data_path = "../data/data.txt"

with open(data_path, "r") as file:
    data_raw = file.read()

data = data_raw.split("\n\n")
towels = data[0].replace(" ", "").split(",")
designs = data[1].splitlines()

import re

towels.sort(key=len)
towels.reverse()
or_pattern = "(" + "|".join(towels) + ")"
find_design_regex = r"" + or_pattern + r"+"


m1 = 0
for design in designs:
    o = re.fullmatch(find_design_regex, design)
    if o is not None:
        m1 += 1

m2 = 0
cache = dict()


def recursive_search(towels, design: str):
    if design in cache:
        return cache[design]
    count = 0
    if not design:
        count = 1
    for towel in towels:
        if design.startswith(towel):
            count += recursive_search(towels, design[len(towel) :])
    cache[design] = count
    return count


for design in designs:
    m2 += recursive_search(towels, design)

print(m1)
print(m2)
