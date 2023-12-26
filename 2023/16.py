import itertools
import os.path
import re
import sys
from typing import AnyStr
from functools import reduce
from math import lcm


import requests

SESSIONID = ''

if not os.path.exists('data/16'):
    response = requests.get(
        "https://adventofcode.com/2023/day/16/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/16', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/16', 'r') as file:
    inputFile = file.read()

test = """
.|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....
"""

grid = list(filter(lambda x: not x == "", test.split('\n')))
print(grid)

visited = {}
states = {}

# /
reflect_right = {
    'N': 'E',
    'E': 'N',
    'S': 'W',
    'W': 'S'
}

# \
reflect_left = {
    'N': 'W',
    'E': 'S',
    'S': 'E',
    'W': 'N'
}

def step(grid, x, y, dir):
    def stop(x, y, dir):
        return ("" + str(x) + "," + str(y) + "," + dir in states) or (not(0 <= x < len(grid)) or not(0 <= y < len(grid[0])))
    if dir == 'N':
        x -= 1
    elif dir == 'E':
        y += 1
    elif dir == 'S':
        x += 1
    else:
        y -= 1
    if stop(x, y, dir):
        return
    visited["" + str(x) + "," + str(y)] = True
    states["" + str(x) + "," + str(y) + "," + dir] = True
    if grid[x][y] == '|' and dir in "EW":
        step(grid, x, y, 'N')
        step(grid, x, y, 'S')
    elif grid[x][y] == '-' and dir in 'NS':
        step(grid, x, y, 'E')
        step(grid, x, y, 'W')
    elif grid[x][y] == '/':
        step(grid, x, y, reflect_right[dir])
    elif grid[x][y] == '\\':
        step(grid, x, y, reflect_left[dir])
    else:
        step(grid, x, y, dir)

step(grid, 0, -1, 'E')
list(map(print, grid))
print(len(visited))

def runTest(input_string, x, y, dir):
    visited.clear()
    states.clear()
    grid = list(filter(lambda x: not x == "", input_string.split('\n')))
    sys.setrecursionlimit(3000)
    step(grid, x, y, dir)
    print(x, y, dir, len(visited))
    return len(visited)

# runTest(inputFile, 0, -1, 'E')

# for i in range(len(test.split('\n')) - 2):
#     runTest(test, i, -1, 'E')
# for i in range(len(test.split('\n')[1])):
#     runTest(test, -1, i, 'S')
m = 0
for i in range(len(inputFile.split('\n'))):
    m = max(m, runTest(inputFile, i, -1, 'E'))
for i in range(len(inputFile.split('\n')[1])):
    m = max(m, runTest(inputFile, -1, i, 'S'))
for i in range(len(inputFile.split('\n'))):
    m = max(m, runTest(inputFile, i, len(inputFile.split('\n')[1]), 'W'))
for i in range(len(inputFile.split('\n')[1])):
    m = max(m, runTest(inputFile, len(inputFile.split('\n')) - 1, i, 'N'))
print(m)
