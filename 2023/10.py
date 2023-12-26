import itertools
import os.path
import re
import sys
from typing import AnyStr
from functools import reduce
from math import lcm


import requests

SESSIONID = ''

if not os.path.exists('data/10'):
    response = requests.get(
        "https://adventofcode.com/2023/day/10/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/10', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/10', 'r') as file:
    inputFile = file.read()

test = """
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"""

test2 = """
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"""

def set(outgrid, i, j, side):
    if i >= 0 and i < len(outgrid) and j >= 0 and j < len(outgrid[0]) and not outgrid[i][j] == 'X':
        outgrid[i][j] = side
    return outgrid

def findNext(grid, outgrid, i,j,x,y,steps):
    # i,j current location
    # x,y last location, filter this out
    if grid[i][j] in "S-J7" and j - 1 >= 0 and not (i == x and j-1 == y) and grid[i][j-1] in "-FL":
        if grid[i][j-1] == '-':
            outgrid = set(outgrid, i-1, j-1, 'R')
            outgrid = set(outgrid, i+1, j-1, 'L')
        if grid[i][j-1] == 'F':
            outgrid = set(outgrid, i, j-2, 'R')
            outgrid = set(outgrid, i-1, j-1, 'R')
        if grid[i][j-1] == 'L':
            outgrid = set(outgrid, i, j-2, 'L')
            outgrid = set(outgrid, i+1, j-1, 'L')
        outgrid[i][j-1] = 'X'
        return outgrid, i, j-1, i, j, steps + 1
    if grid[i][j] in "S|JL" and i - 1 >= 0 and not (i-1 == x and j == y) and grid[i-1][j] in "|F7":
        if grid[i-1][j] == '|':
            outgrid = set(outgrid, i-1, j-1, 'L')
            outgrid = set(outgrid, i-1, j+1, 'R')
        if grid[i-1][j] == 'F':
            outgrid = set(outgrid, i-1, j-1, 'L')
            outgrid = set(outgrid, i-2, j, 'L')
        if grid[i-1][j] == '7':
            outgrid = set(outgrid, i-1, j+1, 'R')
            outgrid = set(outgrid, i-2, j, 'R')
        outgrid[i-1][j] = 'X'
        return outgrid, i-1, j, i, j, steps + 1
    if grid[i][j] in "S-FL" and j + 1 < len(grid[0]) and not (i == x and j+1 == y) and grid[i][j+1] in "-J7":
        if grid[i][j+1] == '-':
            outgrid = set(outgrid, i-1, j+1, 'L')
            outgrid = set(outgrid, i+1, j+1, 'R')
        if grid[i][j+1] == 'J':
            outgrid = set(outgrid, i, j+2, 'R')
            outgrid = set(outgrid, i+1, j+1, 'R')
        if grid[i][j+1] == '7':
            outgrid = set(outgrid, i, j+2, 'L')
            outgrid = set(outgrid, i+1, j-1, 'L')
        outgrid[i][j+1] = 'X'
        return outgrid, i, j+1, i, j, steps + 1
    if grid[i][j] in "S|F7" and i + 1 < len(grid) and not (i+1 == x and j== y) and grid[i+1][j] in "|JL":
        if grid[i+1][j] == '|':
            outgrid = set(outgrid, i+1, j-1, 'R')
            outgrid = set(outgrid, i+1, j+1, 'L')
        if grid[i+1][j] == 'J':
            outgrid = set(outgrid, i+1, j+1, 'L')
            outgrid = set(outgrid, i+2, j, 'L')
        if grid[i+1][j] == 'L':
            outgrid = set(outgrid, i+1, j+1, 'R')
            outgrid = set(outgrid, i+2, j, 'R')
        outgrid[i+1][j] = 'X'
        return outgrid, i+1, j, i, j, steps + 1
    return outgrid, -1, -1, i, j, steps + 1

import copy
grid = list(map(lambda x: list(x), filter(lambda x: not x == "", test2.split('\n'))))
outgrid = copy.deepcopy(grid)
i,j = next(filter(lambda x: x[1] is not None, enumerate(map(lambda line: line.index('S') if 'S' in line else None, grid))))
x = y = -1
steps = 0
while i >= 0:
 outgrid, i, j, x, y, steps = findNext(grid, outgrid, i, j, x, y, steps)
print(steps / 2)
print("\n".join(map(lambda x: "".join(x), outgrid)))

# grid = list(filter(lambda x: not x == "", inputFile.split('\n')))
# i,j = next(filter(lambda x: x[1] is not None, enumerate(map(lambda line: line.index('S') if 'S' in line else None, grid))))
# x = y = -1
# steps = 0
# while i >= 0:
#     i, j, x, y, steps = findNext(grid, i, j, x, y, steps)
# print(steps / 2)

grid = list(map(lambda x: list(x), filter(lambda x: not x == "", inputFile.split('\n'))))
outgrid = copy.deepcopy(grid)
i,j = next(filter(lambda x: x[1] is not None, enumerate(map(lambda line: line.index('S') if 'S' in line else None, grid))))
x = y = -1
steps = 0
while i >= 0:
    outgrid, i, j, x, y, steps = findNext(grid, outgrid, i, j, x, y, steps)
print(steps / 2)
# print("\n".join(map(lambda x: "".join(x), outgrid)))
print(sum(map(lambda x: sum(map(lambda x: 1 if x == 'R' else 0, x)), outgrid)))
# 120 Low
def expand(outgrid):
    for i in range(0, len(grid)):
        for j in range(0, len(grid[0])):
            if outgrid[i][j] in "FJL7|-." and (
               (i - 1 >= 0 and outgrid[i-1][j] == 'R')
            or (j - 1 >= 0 and outgrid[i][j-1] == 'R')
            or (i + 1 < len(grid) and outgrid[i+1][j] == 'R')
            or (j + 1 < len(grid[0]) and outgrid[i][j+1] == 'R')):
                outgrid[i][j] = 'R'
    return outgrid
outgrid = expand(outgrid)
# print("\n".join(map(lambda x: "".join(x), outgrid)))
print(sum(map(lambda x: sum(map(lambda x: 1 if x == 'R' else 0, x)), outgrid)))

outgrid = expand(outgrid)
# print("\n".join(map(lambda x: "".join(x), outgrid)))
print(sum(map(lambda x: sum(map(lambda x: 1 if x == 'R' else 0, x)), outgrid)))
