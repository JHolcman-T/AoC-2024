data = """Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"""

data = data.splitlines()
data.append("")
data_cols = len(data)
machines = []
# parse data
for index, line in enumerate(data):
    if line == "":
        machine = data[index - 3 : index]
        machine_dict = {"A": dict(), "B": dict(), "P": dict()}
        for m_index, button in enumerate(machine[:-1]):
            button_data = button.split(":")
            button_id = button_data[0].split(" ")[-1].strip()
            for direction in button_data[-1].split(","):
                letter, number = direction.split("+")
                machine_dict[button_id][letter.strip()] = int(number.strip())
        for prize_axis in machine[-1].split(":")[-1].strip().split(","):
            letter, number = prize_axis.split("=")
            machine_dict["P"][letter.strip()] = int(number.strip())
        machines.append(machine_dict)

# pprint.pprint(machines)

CONSTANT_INCREASE = 10000000000000


def get_press_count(machine: dict["A" : dict(), "B" : dict(), "P" : dict()]):
    a = machine["A"]
    b = machine["B"]
    p = machine["P"]

    # Equations
    #
    # x*(AX) + y*(BX) = (PX)
    # x*(AY) + y*(BY) = (PY)
    #
    # x := press count for button A
    # y := press count for button B
    #
    # (AX) and (AY) := button A; X and Y values from the input
    # (BX) and (BY) := button B; X and Y values from the input
    # (PX) and (PY) := prize; X and Y values from the input

    # solve for x
    #
    # x * some factor
    xf = a["Y"] - ((a["X"] * b["Y"]) / b["X"])
    # value for "x" times some factor
    xpf = p["Y"] - ((p["X"] * b["Y"]) / b["X"])
    # represents press count for button A
    value_for_x = round(xpf / xf) #0.0112

    # solve for y
    #
    # to compute press count for button B we simply need to put x value into one of the equations from beginning
    # x*(AX) + y*(BX) = (PX)
    # y*(BX) = (PX) - value_for_x*(AX)
    # y = ((PX) - value_for_x*(AX)) / (BX)
    value_for_y = round((p["X"] - (value_for_x * a["X"])) / b["X"])

    a_btn_press_count = value_for_x
    b_btn_press_count = value_for_y

    # moves for A
    ax = a_btn_press_count * a["X"]
    ay = a_btn_press_count * a["Y"]
    # moves for B
    bx = b_btn_press_count * b["X"]
    by = b_btn_press_count * b["Y"]

    # check if the rounded values fit the prize values if not it is not possible to get any combination of the buttons
    if ax + bx == p["X"] and ay + by == p["Y"]:
        return (a_btn_press_count, b_btn_press_count)
    return (None, None)


def get_cost(a_press, b_press):
    if None in [a_press, b_press]:
        return -1
    return (3 * a_press) + b_press


# part 1
# compute cost per machine
cost_per_machine = [get_cost(*get_press_count(machine)) for machine in machines]
# filter out invalid values
cost_per_machine = list(filter(lambda x: x > -1, cost_per_machine))
# sum all costs
print("Part 1:", sum(cost_per_machine))

# part 2
# increase values for prize
for machine in machines:
    machine["P"]["X"] += CONSTANT_INCREASE
    machine["P"]["Y"] += CONSTANT_INCREASE
cost_per_machine = [get_cost(*get_press_count(machine)) for machine in machines]
# filter out invalid values
cost_per_machine = list(filter(lambda x: x > -1, cost_per_machine))
# sum all costs
print("Part 2:", sum(cost_per_machine))
