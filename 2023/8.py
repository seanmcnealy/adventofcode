import itertools
import os.path
import re
import sys
from typing import AnyStr
from functools import reduce
from math import lcm


import requests

SESSIONID = ''

if not os.path.exists('data/8'):
    response = requests.get(
        "https://adventofcode.com/2023/day/8/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/8', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/8', 'r') as file:
    inputFile = file.read()

test = """RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"""


def parse(input):
    return input[0], {l[0]: (l[1], l[2]) for l in
                      map(lambda line: list(map(lambda x: x.group(), re.finditer(r'[0-9A-Z]+', line))), input[1:])}


directions = parse(list(filter(lambda x: not x == "", test.split('\n'))))
current = 'AAA'
steps = 0
for d in itertools.cycle(directions[0]):
    if current == 'ZZZ':
        break
    steps += 1
    current = directions[1][current][0 if d == 'L' else 1]
print(steps)

directions = parse(list(filter(lambda x: not x == "", inputFile.split('\n'))))
current = 'AAA'
steps = 0
for d in itertools.cycle(directions[0]):
    if current == 'ZZZ':
        break
    steps += 1
    current = directions[1][current][0 if d == 'L' else 1]
print(steps)

test = """LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"""

directions = parse(list(filter(lambda x: not x == "", test.split('\n'))))
current = list(filter(lambda x: x[2] == 'A', directions[1].keys()))
steps = 0
for d in itertools.cycle(directions[0]):
    if all(map(lambda x: x[2] == 'Z', current)):
        break
    steps += 1
    current = list(map(lambda x: directions[1][x][0 if d == 'L' else 1], current))
print(steps)

print(lcm(lcm(lcm(lcm(lcm(12599, 17287), 17873, 20803, 21389, 19631)))))

directions = parse(list(filter(lambda x: not x == "", inputFile.split('\n'))))
current = list(filter(lambda x: x[2] == 'A', directions[1].keys()))
steps = 0
for d in itertools.cycle(directions[0]):
    for i, c in enumerate(current):
        if c[2] == 'Z':
            print(i, steps)
    if all(map(lambda x: x[2] == 'Z', current)):
        break
    steps += 1
    current = list(map(lambda x: directions[1][x][0 if d == 'L' else 1], current))
print(steps)
# 3 12599
# 2 17287
# 0 17873
# 1 19631
# 5 20803
# 4 21389
