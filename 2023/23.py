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

if not os.path.exists('data/23'):
    response = requests.get(
        "https://adventofcode.com/2023/day/23/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/23', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/23', 'r') as file:
    inputFile = file.read()

test = """#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
"""

DIRS = {
    'N': [-1,0],
    'E': [0,1],
    'S': [1,0],
    'W': [0,-1]
}

grid = list(filter(lambda x: not x == "", test.split('\n')))

search = [(0,1,[],0.)]
m = 0
def step():
    global search
    global m
    s = search.pop()
    for i, d in DIRS.items():
        nx = s[0] + d[0]
        ny = s[1] + d[1]
        if (0 <= nx < len(grid) and 0 <= ny < len(grid[0]) and
                grid[nx][ny] in '.><v^' and (nx, ny) not in s[2]):
            # if grid[s[0]][s[1]] == '>' and i != 'E':
            #     continue
            # if grid[s[0]][s[1]] == '<' and i != 'W':
            #     continue
            # if grid[s[0]][s[1]] == 'v' and i != 'S':
            #     continue
            # if grid[s[0]][s[1]] == '^' and i != 'N':
            #     continue
            if nx == len(grid) -1 and ny == len(grid[0]) - 2:
                m = max(m, len(s[2])+1)
            else:
                search.insert(bisect.bisect_left(search, len(s[2]) - nx - ny, key=lambda x: x[3]),
                              (nx, ny, [*s[2], (nx,ny)], len(s[2]) - nx - ny))

while search:
    step()
print(m)

grid = list(filter(lambda x: not x == "", inputFile.split('\n')))

def print_grid(p):
    for i in range(len(grid)):
        s = ""
        for j in range(len(grid[0])):
            if((i,j) in p):
                s += '*'
            else:
                s += grid[i][j]
        print(s)

graph = []
search = [(0,1,[],0,1)]
m = 0
p = []
def step():
    global search
    global m
    global p
    # s = search[0]
    # search = search[1:]
    s = search.pop()
    for i, d in DIRS.items():
        nx = s[0] + d[0]
        ny = s[1] + d[1]
        if (0 <= nx < len(grid) and 0 <= ny < len(grid[0]) and
                grid[nx][ny] in '.><v^' and (nx, ny) not in s[2]):
            if ((grid[nx][ny-1] == '>' and grid[nx-1][ny] == 'v') or
                    (grid[nx][ny+1] == '>' and grid[nx-1][ny] == 'v') or
                    (grid[nx][ny-1] == '>' and grid[nx][ny+1] == '>')):
                print("found", nx, ny, "from", s[3], s[4], "costing", len(s[2]))
                if not any(map(lambda x: x[0] == nx and x[1] == ny, graph)):
                    search.append((nx, ny, [(nx,ny)], nx, ny))
                graph.append((nx, ny, len(s[2]), s[3], s[4]))
            elif nx == len(grid) -1 and ny == len(grid[0]) - 2:
                print("FOUND", nx, ny, "from", s[3], s[4], "costing", len(s[2]))
                graph.append((nx, ny, len(s[2]), s[3], s[4]))
            else:
                search.append((nx, ny, [*s[2], (nx,ny)], s[3], s[4]))

while search:
    step()
print(graph)

search = [(0,1,0,[])]
m = 0
def graph_step():
    global search
    global m
    s = search.pop()
    for nx, ny, c, _, _ in filter(lambda g: g[3] == s[0] and g[4] == s[1], graph):
        if nx == len(grid) -1 and ny == len(grid[0]) - 2:
            m = max(m, c + s[2])
            print(m)
        elif (nx, ny) not in s[3]:
            search.append((nx, ny, c + s[2], [*s[3], (nx,ny)]))
while search:
    graph_step()
print(m+1)
