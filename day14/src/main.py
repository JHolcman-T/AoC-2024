data_path = "../data/data.txt"

with open(data_path, "r") as file:
    data = file.readlines()

# data = """p=0,4 v=3,-3
# p=6,3 v=-1,-3
# p=10,3 v=-1,2
# p=2,0 v=2,-1
# p=0,0 v=1,3
# p=3,0 v=-2,-2
# p=7,6 v=-1,-3
# p=3,0 v=-1,-2
# p=9,3 v=2,3
# p=7,3 v=-1,2
# p=2,4 v=2,-3
# p=9,5 v=-3,-3""".splitlines()

WORLD_WIDTH = 101
WORLD_HEIGHT = 103
# WORLD_WIDTH = 11
# WORLD_HEIGHT = 7

robots = []

for line in data:
    robot_dict = {"S": dict(), "V": dict()}
    split = line.split(" ")
    # get start position
    sp = split[0].split("=")[1].split(",")
    robot_dict["S"]["X"] = int(sp[0])
    robot_dict["S"]["Y"] = int(sp[1])
    # get velocity
    v = split[1].split("=")[1].split(",")
    robot_dict["V"]["X"] = int(v[0])
    robot_dict["V"]["Y"] = int(v[1])
    # add to list
    robots.append(robot_dict)

cache = dict()


def simulate_movement(robot, time, save_last=False):
    # returns postion after given time
    pos_x, pos_y = robot["S"]["X"], robot["S"]["Y"]
    v_x, v_y = robot["V"]["X"], robot["V"]["Y"]

    # caching for part 2
    key = (pos_x, pos_y, v_x, v_y)
    cached = cache.get(key, False)
    if save_last is True:
        if cached:
            pos_x, pos_y = cached
            time = 1

    # compute position
    for _ in range(time):
        pos_x += v_x
        pos_y += v_y
        if pos_x >= WORLD_WIDTH:
            pos_x = pos_x - WORLD_WIDTH
        elif pos_x < 0:
            pos_x = WORLD_WIDTH + pos_x
        if pos_y >= WORLD_HEIGHT:
            pos_y = pos_y - WORLD_HEIGHT
        elif pos_y < 0:
            pos_y = WORLD_HEIGHT + pos_y

    # caching for part 2
    if save_last is True:
        cache[key] = (pos_x, pos_y)
    else:
        if cached:
            cache.pop(key)

    return pos_x, pos_y


positions = [simulate_movement(robot, 100) for robot in robots]

quadrants = [[], [], [], []]
middle_w = (WORLD_WIDTH - 1) // 2
middle_h = (WORLD_HEIGHT - 1) // 2
for pos_x, pos_y in positions:
    if pos_x < middle_w and pos_y < middle_h:
        quadrants[0].append((pos_x, pos_y))
    elif pos_x < middle_w and pos_y > middle_h:
        quadrants[2].append((pos_x, pos_y))
    elif pos_x > middle_w and pos_y < middle_h:
        quadrants[1].append((pos_x, pos_y))
    elif pos_x > middle_w and pos_y > middle_h:
        quadrants[3].append((pos_x, pos_y))


# PART 1
part_1 = 1
for q in quadrants:
    part_1 *= len(q)
print("Part 1:", part_1)


for seconds in range(10000):
    image = [list("." * WORLD_WIDTH) for _ in range(WORLD_HEIGHT)]
    positions = [simulate_movement(robot, seconds, True) for robot in robots]
    for pos_x, pos_y in positions:
        image[pos_y][pos_x] = "#"
    rendered_image = "\n".join(["".join(row) for row in image])
    if "########" in rendered_image:
        print(rendered_image)
        print("Possible part 2:", seconds)
