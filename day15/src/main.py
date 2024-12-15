from collections import deque

data_path = "../data/data.txt"

with open(data_path, "r") as file:
    data_raw = file.read()

data = data_raw.split("\n\n")
grid_raw = data[0]
get_grid = lambda: [list(line) for line in grid_raw.splitlines()]
grid = get_grid()
grid_height = len(grid)
grid_width = len(grid[0])
moves = data[1].replace("\n", "")
move_count = len(moves)


def get_start_pos(grid_height, grid_width, grid):
    for row in range(grid_height):
        for col in range(grid_width):
            if grid[row][col] == "@":
                grid[row][col] = "."
                return row, col
    return None, None


def do_move(grid, pos, move):
    new_row, new_col = pos
    if move == "^":
        new_row -= 1
    elif move == ">":
        new_col += 1
    elif move == "v":
        new_row += 1
    elif move == "<":
        new_col -= 1
    else:
        raise Exception(f"Unknown move={move}")

    current_tile = grid[pos[0]][pos[1]]
    next_tile = grid[new_row][new_col]

    if next_tile == ".":
        grid[pos[0]][pos[1]] = "."
        grid[new_row][new_col] = current_tile
        return new_row, new_col
    if next_tile == "#":
        return pos
    if next_tile == "O":
        out_pos = do_move(grid, (new_row, new_col), move)
        if out_pos != (new_row, new_col):
            grid[pos[0]][pos[1]] = "."
            grid[new_row][new_col] = current_tile
            return (new_row, new_col)
    return pos


current_row, current_col = get_start_pos(grid_height, grid_width, grid)
for id, move in enumerate(moves):
    current_row, current_col = do_move(grid, (current_row, current_col), move)

part_1 = 0
for row in range(1, grid_height - 1):
    for col in range(1, grid_width - 1):
        element = grid[row][col]
        if element == "O":
            part_1 += 100 * row + col
print("Part 1:", part_1)

grid_raw = (
    grid_raw.replace("#", "##").replace("O", "[]").replace(".", "..").replace("@", "@.")
)
grid = get_grid()
grid_height = len(grid)
grid_width = len(grid[0])
current_row, current_col = get_start_pos(grid_height, grid_width, grid)


def do_move_2(grid, pos, move):
    d_r, d_c = {"^": (-1, 0), ">": (0, 1), "v": (1, 0), "<": (0, -1)}[move]
    new_row, new_col = pos[0] + d_r, pos[1] + d_c
    new_pos = (new_row, new_col)

    next_tile = grid[new_row][new_col]
    if next_tile == ".":
        return new_pos
    if next_tile in ["[", "]"]:
        q = deque([pos])
        visited = set()
        hit_wall = False
        while q:
            root_row, root_col = q.popleft()
            if (root_row, root_col) in visited:
                continue
            new_root_row, new_root_col = root_row + d_r, root_col + d_c
            elem = grid[new_root_row][new_root_col]
            visited.add((root_row, root_col))
            if elem == "#":
                hit_wall = True
                break
            if elem == "[":
                q.append((new_root_row, new_root_col))
                q.append((new_root_row, new_root_col + 1))
            if elem == "]":
                q.append((new_root_row, new_root_col))
                q.append((new_root_row, new_root_col - 1))
        if hit_wall is True:
            return pos
        while len(visited) > 0:
            for root_row, root_col in list(visited):
                new_root_row, new_root_col = root_row + d_r, root_col + d_c
                if (new_root_row, new_root_col) not in visited:
                    grid[new_root_row][new_root_col] = grid[root_row][root_col]
                    grid[root_row][root_col] = "."
                    visited.remove((root_row, root_col))
        return new_pos
    return pos


for id, move in enumerate(moves):
    current_row, current_col = do_move_2(grid, (current_row, current_col), move)

part_2 = 0
for row in range(1, grid_height - 1):
    for col in range(1, grid_width - 1):
        element = grid[row][col]
        if element == "[":
            part_2 += 100 * row + col
print("Part 2:", part_2)
