import itertools
import os.path
import re
import sys
from typing import AnyStr
from functools import reduce
from math import lcm


import requests

SESSIONID = ''

if not os.path.exists('data/11'):
    response = requests.get(
        "https://adventofcode.com/2023/day/11/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/11', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/11', 'r') as file:
    inputFile = file.read()

test = """
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"""

grid = list(filter(lambda x: not x == "", test.split('\n')))
expandx = list(map(lambda x: x[0], (filter(lambda x: all(map(lambda y: y == '.', x[1])), enumerate(grid)))))
expandy = list(filter(lambda y: all(map(lambda x: grid[x][y] == '.', range(len(grid)))), range(len(grid[0]))))

def expanded(x,y,i,j):
    return 9 * (
            len(list(filter(lambda a: min(x,i) < a < max(x,i), expandx))) +
            len(list(filter(lambda a: min(y,j) < a < max(y,j), expandy))))

def distance(x,y,i,j):
    return abs(i - x) + abs(j - y) + expanded(x,y,i,j)

nodes = list(itertools.chain(
    *map(lambda x: list(map(lambda y: (x[0],y), x[1])),
         filter(lambda x: x[1], enumerate(
             map(lambda line: list(
                 map(lambda x: x[0],
                     filter(lambda x: x[1] == '#', enumerate(line)))),
                 grid))))))
d = 0
for i, node in enumerate(nodes):
    for other in nodes[i+1:]:
        d += distance(node[0], node[1], other[0], other[1])
print(d)



grid = list(filter(lambda x: not x == "", inputFile.split('\n')))
expandx = list(map(lambda x: x[0], (filter(lambda x: all(map(lambda y: y == '.', x[1])), enumerate(grid)))))
expandy = list(filter(lambda y: all(map(lambda x: grid[x][y] == '.', range(len(grid)))), range(len(grid[0]))))

def expanded(x,y,i,j):
    return 999999 *(len(list(filter(lambda a: min(x,i) < a < max(x,i), expandx))) + len(list(filter(lambda a: min(y,j) < a < max(y,j), expandy))))

def distance(x,y,i,j):
    return abs(i - x) + abs(j - y) + expanded(x,y,i,j)

nodes = list(itertools.chain(
    *map(lambda x: list(map(lambda y: (x[0],y), x[1])),
         filter(lambda x: x[1], enumerate(
             map(lambda line: list(
                 map(lambda x: x[0],
                     filter(lambda x: x[1] == '#', enumerate(line)))),
                 grid))))))
d = 0
for i, node in enumerate(nodes):
    for other in nodes[i+1:]:
        d += distance(node[0], node[1], other[0], other[1])
print(d)
