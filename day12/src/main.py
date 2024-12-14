# data = """AAAA
# BBCD
# BBCC
# EEEC"""

data = """RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"""

from collections import defaultdict, deque

split_data = data.splitlines()
D_H = len(split_data)
D_W = len(split_data[0])

used_fields = set()
areas = []


def get_area(element_pos: tuple[int, int], area: list):
    if element_pos in used_fields:
        return
    element_value = split_data[element_pos[0]][element_pos[1]]
    used_fields.add(element_pos)
    area.append(element_pos)
    directions = [
        (-1, 0),
        (0, 1),
        (1, 0),
        (0, -1),
    ]
    for direction in directions:
        new_row = element_pos[0] + direction[0]
        new_col = element_pos[1] + direction[1]
        if new_row < 0 or new_col < 0 or new_row >= D_H or new_col >= D_W:
            continue
        if element_value != split_data[new_row][new_col]:
            continue
        get_area((new_row, new_col), area)


for row in range(D_H):
    for col in range(D_W):
        if (row, col) in used_fields:
            continue
        area = []
        get_area((row, col), area)
        areas.append((split_data[row][col], area))


part_1 = 0
all_edges = list()
for area_id, area in areas:
    u = 0
    edges = defaultdict(set)
    for plot in area:
        directions = [
            (-1, 0),
            (0, 1),
            (1, 0),
            (0, -1),
        ]
        for direction in directions:
            new_row = plot[0] + direction[0]
            new_col = plot[1] + direction[1]
            if new_row < -1 or new_col < -1 or new_row > D_H or new_col > D_W:
                continue
            if (new_row, new_col) not in area:
                edges[direction].add((new_row, new_col))
                u += 1
    all_edges.append((area, edges))

    part_1 += u * len(area)
print(part_1)


part_2 = 0
for area, edges in all_edges:
    s = 0
    directions = [
        (-1, 0),
        (0, 1),
        (1, 0),
        (0, -1),
    ]
    for edge_dir, edge_set in edges.items():
        seen = set()
        for e_row, e_col in edge_set:
            if (e_row, e_col) not in seen:
                s += 1
                queue = deque([(e_row, e_col)])
                while queue:
                    r_row, r_col = queue.popleft()
                    if (r_row, r_col) in seen:
                        continue
                    seen.add((r_row, r_col))
                    for direction in directions:
                        new_row = r_row + direction[0]
                        new_col = r_col + direction[1]
                        if (new_row, new_col) in edge_set:
                            queue.append((new_row, new_col))
    part_2 += s * len(area)
print(part_2)
