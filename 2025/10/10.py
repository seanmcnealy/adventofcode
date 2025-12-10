import numpy as np
from scipy.optimize import linprog

exampledata = """[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"""

with open("data/10") as file: filedata = file.read()

def parse(data):
    INPUT = [r.split(" ") for r in data.split("\n")]

    def make_button(sch, size):
        new_button = size*[0]
        for inst in sch:
            new_button[int(inst)] = 1
        return new_button

    indicator_lights = [list(map(lambda x: 1 if x == '#' else 0, r[0][1:-1])) for r in INPUT[:-1]]
    button_schematics = [[make_button(button[1:-1].split(","), len(indicator_lights[i])) for button in schem[1:-1]] for i, schem in enumerate(INPUT[:-1])]
    joltage_requirements = [list(map(int, req[-1][1:-1].split(","))) for req in INPUT[:-1]]

    machines = list(zip(indicator_lights, button_schematics, joltage_requirements))
    return machines

def solve2(machines):
    total = 0
    for i, machine in enumerate(machines):
        buttons = -np.array(machine[1]).T
        jolt = -np.array(machine[2])
        A_eq = buttons
        b_eq = jolt
        c = np.ones(A_eq.shape[1])

        result = linprog(c, A_eq=A_eq, b_eq=b_eq, integrality=1)

        print(f"Machine {i}: {sum(result.x)} {result.x}")
        total += sum(result.x)
    return total

print(f"Example: {solve2(parse(exampledata))}")
print(f"Solution to Part 2: {solve2(parse(filedata))}")
