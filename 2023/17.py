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

if not os.path.exists('data/17'):
    response = requests.get(
        "https://adventofcode.com/2023/day/17/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/17', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/17', 'r') as file:
    inputFile = file.read()

test = """
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
"""

LEFT = {
    'N': 'W',
    'E': 'N',
    'S': 'E',
    'W': 'S'
}

RIGHT = {
    'N': 'E',
    'E': 'S',
    'S': 'W',
    'W': 'N'
}
EXP_COST = 2

def move(x, y, dir):
    if dir == 'N': return x-1, y
    if dir == 'E': return x, y+1
    if dir == 'S': return x+1, y
    if dir == 'W': return x, y-1
    return x,y

grid = list(filter(lambda x: not x == "", test.split('\n')))
len_grid = len(grid)
width_grid = len(grid[0])
# print(grid)

search = [(0,0,'E',-1,0,0)]
found = {}

def expand(x, y, dir, dist, cost):
    if dist < 2:
        nextx, nexty = move(x,y,dir)
        if 0 <= nextx < len_grid and 0 <= nexty < width_grid:
            # print(nextx, nexty, dir, dist + 1, cost + int(grid[nextx][nexty]))
            search.insert(bisect.bisect_left(search, cost + int(grid[nextx][nexty]) - nextx * EXP_COST - nexty * EXP_COST, key=lambda x: x[5]),
                          (nextx, nexty, dir, dist + 1, cost + int(grid[nextx][nexty]), cost + int(grid[nextx][nexty]) - nextx * EXP_COST - nexty * EXP_COST))
    leftx, lefty = move(x,y,LEFT[dir])
    if 0 <= leftx < len_grid and 0 <= lefty < width_grid:
        # print(leftx, lefty, LEFT[dir], 0, cost + int(grid[leftx][lefty]))
        search.insert(bisect.bisect_left(search, cost + int(grid[leftx][lefty]) - leftx * EXP_COST - lefty * EXP_COST, key=lambda x: x[5]),
        (leftx, lefty, LEFT[dir], 0, cost + int(grid[leftx][lefty]), cost + int(grid[leftx][lefty]) - leftx * EXP_COST - lefty * EXP_COST))
    rightx, righty = move(x,y,RIGHT[dir])
    if 0 <= rightx < len_grid and 0 <= righty < width_grid:
        # print(rightx, righty, RIGHT[dir], 0, cost + int(grid[rightx][righty]))
        search.insert(bisect.bisect_left(search, cost + int(grid[rightx][righty]) - rightx * EXP_COST - righty * EXP_COST, key=lambda x: x[5]),
        (rightx, righty, RIGHT[dir], 0, cost + int(grid[rightx][righty]), cost + int(grid[rightx][righty]) - rightx * EXP_COST - righty * EXP_COST))

# expand(*search.pop())
# print(search)

m = sys.maxsize
while True:
    if not search:
        break
    x, y, dir, dist, cost, _ = search[0]
    search = search[1:]
    if x == len_grid - 1 and y == width_grid - 1:
        # print("end", cost)
        m = min(m, cost)
        continue
    key = "" + str(x) + "," + str(y) + "," + dir + str(dist)
    if key in found:
        c = found[key]
        if c <= cost:
            continue
    found[key] = cost
    if cost + len_grid - 1 - x + width_grid - 1 - y > m:
        continue
    expand(x, y, dir, dist, cost)
print(m)


grid = list(filter(lambda x: not x == "", inputFile.split('\n')))
len_grid = len(grid)
width_grid = len(grid[0])
search = [(0,0,'E',-1,0,0)]
found = {}

m = 1100
i = 0
while True:
    i += 1
    if i % 10000 == 0:
        print(m, len(search), len(found))
    if not search:
        break
    x, y, dir, dist, cost, _ = search[0]
    search = search[1:]
    if x == len_grid - 1 and y == width_grid - 1:
        print("end", cost)
        m = min(m, cost)
        continue
    if cost + len_grid - 1 - x + width_grid - 1 - y > m:
        continue
    key = "" + str(x) + "," + str(y) + "," + dir + str(dist)
    if key in found:
        c = found[key]
        if c <= cost:
            continue
    found[key] = cost
    expand(x, y, dir, dist, cost)
print(m)

# 3692 too high
# 3183 too high
# 3009 too high
# 2282 too high
# 1523 too high
# 1302 incorrect
# 1099

def expand(x, y, dir, dist, cost):
    if dist < 9:
        nextx, nexty = move(x,y,dir)
        if 0 <= nextx < len_grid and 0 <= nexty < width_grid:
            # print(nextx, nexty, dir, dist + 1, cost + int(grid[nextx][nexty]))
            search.insert(bisect.bisect_left(search, cost + int(grid[nextx][nexty]) - nextx * EXP_COST - nexty * EXP_COST, key=lambda x: x[5]),
                          (nextx, nexty, dir, dist + 1, cost + int(grid[nextx][nexty]), cost + int(grid[nextx][nexty]) - nextx * EXP_COST - nexty * EXP_COST))
    leftx, lefty = move(x,y,LEFT[dir])
    if dist >= 3 and 0 <= leftx < len_grid and 0 <= lefty < width_grid:
        # print(leftx, lefty, LEFT[dir], 0, cost + int(grid[leftx][lefty]))
        search.insert(bisect.bisect_left(search, cost + int(grid[leftx][lefty]) - leftx * EXP_COST - lefty * EXP_COST, key=lambda x: x[5]),
                      (leftx, lefty, LEFT[dir], 0, cost + int(grid[leftx][lefty]), cost + int(grid[leftx][lefty]) - leftx * EXP_COST - lefty * EXP_COST))
    rightx, righty = move(x,y,RIGHT[dir])
    if dist >= 3 and 0 <= rightx < len_grid and 0 <= righty < width_grid:
        # print(rightx, righty, RIGHT[dir], 0, cost + int(grid[rightx][righty]))
        search.insert(bisect.bisect_left(search, cost + int(grid[rightx][righty]) - rightx * EXP_COST - righty * EXP_COST, key=lambda x: x[5]),
                      (rightx, righty, RIGHT[dir], 0, cost + int(grid[rightx][righty]), cost + int(grid[rightx][righty]) - rightx * EXP_COST - righty * EXP_COST))

grid = list(filter(lambda x: not x == "", inputFile.split('\n')))
len_grid = len(grid)
width_grid = len(grid[0])
search = [(0,0,'E',-1,0,0), (0,0,'S',-1,0,0)]
found = {}

m = 1300
i = 0
while True:
    i += 1
    if i % 10000 == 0:
        print(m, len(search), len(found))
    if not search:
        break
    x, y, dir, dist, cost, _ = search[0]
    search = search[1:]
    if x == len_grid - 1 and y == width_grid - 1:
        print("end", cost)
        m = min(m, cost)
        continue
    if cost + len_grid - 1 - x + width_grid - 1 - y > m:
        continue
    key = "" + str(x) + "," + str(y) + "," + dir + str(dist)
    if key in found:
        c = found[key]
        if c <= cost:
            continue
    found[key] = cost
    expand(x, y, dir, dist, cost)
print(m)
