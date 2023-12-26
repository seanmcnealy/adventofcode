import bisect
import itertools
import os.path
import re
import sys
from typing import AnyStr
from functools import reduce
from math import lcm


import requests

SESSIONID = ''

if not os.path.exists('data/18'):
    response = requests.get(
        "https://adventofcode.com/2023/day/18/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/18', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/18', 'r') as file:
    inputFile = file.read()

test = """
U 2 (#7a21e3)
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
"""

x = y = 1
len_grid = 12
width_grid = 9

def move(x, y, dir, l):
    if dir == 'U': return x-l, y
    if dir == 'R': return x, y+l
    if dir == 'D': return x+l, y
    if dir == 'L': return x, y-l
    return x,y

def step(x, y, dir, length):
    next_x, next_y = move(x, y, dir, length)
    return next_x, next_y

def parse(line):
    dir = re.search(r'[URDL]', line).group()
    length = re.search(r' (\d+) ', line).group(1)
    color = re.search(r'\(#([\da-f]+)\)', line).group(1)
    return dir, int(length), color

# step(0, 0, 'R', 'U', 6)

total = 1
for line in filter(lambda x: not x == "", test.split('\n')):
    dir, length, _ = parse(line)
    if dir == 'D':
        total += (y) * length
    if dir == 'U':
        total -= (y-1) * length
    if dir == 'R':
        total += length

    x, y = step(x, y, dir, length)

print(x,y,total)

total = 1
x = y = 1
for line in filter(lambda x: not x == "", inputFile.split('\n')):
    dir, length, _ = parse(line)
    if dir == 'D':
        total += y * length
    if dir == 'U':
        total -= (y-1) * length
    if dir == 'R':
        total += length

    x, y = step(x, y, dir, length)

print(x,y,total)
# 41019 is correct

dir_map = {
    '0': 'R',
    '1': 'D',
    '2': 'L',
    '3': 'U'
}

total = 1
for line in filter(lambda x: not x == "", test.split('\n')):
    _, _, color = parse(line)
    dir = dir_map[color[5]]
    length = int(color[0:5], 16)
    if dir == 'D':
        total += (y) * length
    if dir == 'U':
        total -= (y-1) * length
    if dir == 'R':
        total += length

    x, y = step(x, y, dir, length)

print(x,y,total)

total = 1
for line in filter(lambda x: not x == "", inputFile.split('\n')):
    _, _, color = parse(line)
    dir = dir_map[color[5]]
    length = int(color[0:5], 16)
    if dir == 'D':
        total += (y) * length
    if dir == 'U':
        total -= (y-1) * length
    if dir == 'R':
        total += length

    x, y = step(x, y, dir, length)

print(x,y,total)