import bisect
import copy
import itertools
import os.path
import re
import sys
from typing import AnyStr
from functools import reduce
from math import lcm


import requests

SESSIONID = ''

if not os.path.exists('data/24'):
    response = requests.get(
        "https://adventofcode.com/2023/day/24/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/24', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/24', 'r') as file:
    inputFile = file.read()

test = """19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
"""

def parse(line):
    return list(map(lambda x: int(x.group()), re.finditer(r'-?\d+', line)))

lines = list(map(parse, filter(lambda x: not x == "", test.split('\n'))))
print(lines)

def intersect(x1, y1, vx1, vy1, x2, y2, vx2, vy2):
    m1 = vy1 / vx1
    m2 = vy2 / vx2
    b1 = y1 - m1 * x1
    b2 = y2 - m2 * x2

    if m1 - m2 == 0:
        return None

    x = (b2 - b1) / (m1 - m2)
    y = m1 * x + b1

    if (x <= x1 and vx1 > 0) or (x >= x1 and vx1 < 0):
        return None
    if (x <= x2 and vx2 > 0) or (x >= x2 and vx2 < 0):
        return None

    return x, y


total = 0
for i in range(len(lines)):
    for j in range(i+1, len(lines)):
        x1, y1, z1, vx1, vy1, vz1 = lines[i]
        x2, y2, z2, vx2, vy2, vz2 = lines[j]
        print(lines[i][0],lines[j][0], intersect(x1, y1, vx1, vy1, x2, y2, vx2, vy2))
        xy = intersect(x1, y1, vx1, vy1, x2, y2, vx2, vy2)
        if xy and 7 <= xy[0] <= 27 and 7 <= xy[1] <= 27:
            total += 1
print(total)

lines = list(map(parse, filter(lambda x: not x == "", inputFile.split('\n'))))
print(lines)
total = 0
for i in range(len(lines)):
    for j in range(i+1, len(lines)):
        x1, y1, z1, vx1, vy1, vz1 = lines[i]
        x2, y2, z2, vx2, vy2, vz2 = lines[j]
        # print(lines[i][0],lines[j][0], intersect(x1, y1, vx1, vy1, x2, y2, vx2, vy2))
        xy = intersect(x1, y1, vx1, vy1, x2, y2, vx2, vy2)
        if xy and 200000000000000 <= xy[0] <= 400000000000000 and 200000000000000 <= xy[1] <= 400000000000000:
            total += 1
print(total)

# X + VX * t = x1 + vx1 * t
# t = (x1 - X) / (VX - vx1)
# (x1 - X) / (VX - vx1) = (y1 - Y) / (VY - vy1)
# (x1 - X) * (VY - vy1) = (y1 - Y) * (VX - vx1)
# x1 * VY - x1 * vy1 - X * VY + X * vy1 = y1 * VX - y1 * vx1 - Y * VX + Y * vx1
# - X * VY + Y * VX = - x1 * VY + x1 * vy1 - X * vy1 + y1 * VX - y1 * vx1 + Y * vx1
#  - x1 * VY + x1 * vy1 - X * vy1 + y1 * VX - y1 * vx1 + Y * vx1
# X * (vy2-vy1) + Y * (vx1-vx2) + VX * (y1-y2) + VY * (x2-x1) = (x2 * vy2) - (x1 - vy1) - (y2 * vx2) + (y1 * vx1)
def equation(x1, y1, vx1, vy1, x2, y2, vx2, vy2):
    return vy2 - vy1, vx1 - vx2, y1 - y2, x2 - x1
def constant(x1, y1, vx1, vy1, x2, y2, vx2, vy2):
    return (x2 * vy2) - (x1 * vy1) - (y2 * vx2) + (y1 * vx1)
equations = []
constants = []
for i in range(16, 24, 2):
    x1, y1, z1, vx1, vy1, vz1 = lines[i]
    x2, y2, z2, vx2, vy2, vz2 = lines[i+1]
    equations.append(equation(x1, y1, vx1, vy1, x2, y2, vx2, vy2))
    constants.append(constant(x1, y1, vx1, vy1, x2, y2, vx2, vy2))
import numpy as np
X, Y, _, _ = np.linalg.solve(np.array(equations), np.array(constants))

equations = []
constants = []
for i in range(8, 16, 2):
    x1, y1, z1, vx1, vy1, vz1 = lines[i]
    x2, y2, z2, vx2, vy2, vz2 = lines[i+1]
    equations.append(equation(x1, z1, vx1, vz1, x2, z2, vx2, vz2))
    constants.append(constant(x1, z1, vx1, vz1, x2, z2, vx2, vz2))
t, Z, _, _ = np.linalg.solve(np.array(equations), np.array(constants))
print(X + Y + Z)
