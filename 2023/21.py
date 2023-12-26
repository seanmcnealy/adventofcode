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

if not os.path.exists('data/21'):
    response = requests.get(
        "https://adventofcode.com/2023/day/21/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/21', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/21', 'r') as file:
    inputFile = file.read()

test = """
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
"""

grid = list(filter(lambda x: not x == "", test.split('\n')))

print(grid)

search = []
found = set()
last_found = set()

for i in range(len(grid)):
    for j in range(len(grid[0])):
        if grid[i][j] == 'S':
            last_found.add((i, j))

DIRS = {
    'N': [-1, 0],
    'E': [0, 1],
    'S': [1, 0],
    'W': [0, -1],
}

def step():
    global search
    global found
    global last_found
    for i,j in last_found:
        search.append((i,j,2))
    last_found = set()
    while search:
        s = search[0]
        search = search[1:]

        for i, j in DIRS.values():
            ni = s[0] + i
            nj = s[1] + j
            # if 0 <= ni < len(grid) and 0 <= nj < len(grid[0]):
            if grid[ni % len(grid)][nj % len(grid[0])] in ".S" and (ni, nj) not in found and s[2] > 0:
                found.add((ni, nj))
                search.append((ni, nj, s[2] - 1))
                last_found.add((ni, nj))

# print(found)
# print(sum(map(lambda x: 1 if (x[0] + x[1]) % 2 == 1 else 0, found)))
# print(sum(map(lambda x: 1 if (x[0] + x[1]) % 2 == 0 else 0, found)) + 1)
for i in range(3):
    step()
    if (i+1)*2 in [6,10,50,100,500,1000,5000]:
        print((i+1)*2, sum(map(lambda x: 1 if (x[0] + x[1]) % 2 == 0 else 0, found)), sum(map(lambda x: 1 if (x[0] + x[1]) % 2 == 1 else 0, found)))

grid = list(filter(lambda x: not x == "", inputFile.split('\n')))
search = []
found = set()
last_found = set()
for i in range(len(grid)):
    for j in range(len(grid[0])):
        if grid[i][j] == 'S':
            last_found.add((i, j))
            print("Start", i, j, "in", len(grid), len(grid[0]))
def step():
    global search
    global found
    global last_found
    for i,j in last_found:
        search.append((i,j,2))
    last_found = set()
    while search:
        s = search[0]
        search = search[1:]

        for i, j in DIRS.values():
            ni = s[0] + i
            nj = s[1] + j
            # if 0 <= ni < len(grid) and 0 <= nj < len(grid[0]):
            if grid[ni % len(grid)][nj % len(grid[0])] in ".S" and (ni, nj) not in found and s[2] > 0:
                    found.add((ni, nj))
                    search.append((ni, nj, s[2] - 1))
                    last_found.add((ni,nj))

search = []
found = set()
for i in range(500):
    step()
    if ((i+1)*2 - 65)% 131 in [0,1,130]:
        print((i+1)*2, sum(map(lambda x: 1 if (x[0] + x[1]) % 2 == 0 else 0, found)), sum(map(lambda x: 1 if (x[0] + x[1]) % 2 == 1 else 0, found)))

# expand to edges, then repeat 2023 times
# 64 3600
# 196 33150
# 326 91286
# 458 179940
# 588 296212
# 720 443970
# 850 618378
# 982 825240
# 1112 1057784
# 1244 1323750
# 1374 1614430
import matplotlib.pyplot as plt
fig, ax = plt.subplots()
ax.plot([1, 2, 3, 4, 5, 6, 7, 8], [3600, 33150, 91890, 179940, 297300, 297300, 619950, 825240])
plt.show()
# 60082553344 is too low
# 59887346850 also too low
# 60005906040 again too low

# 599757184523100 too low
def q(x):
    return 14655 * x * x - 14535 * x + 3600
print(q(202301))
