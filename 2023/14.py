import itertools
import os.path
import re
import sys
from typing import AnyStr
from functools import reduce
from math import lcm


import requests

SESSIONID = ''

if not os.path.exists('data/14'):
    response = requests.get(
        "https://adventofcode.com/2023/day/14/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/14', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/14', 'r') as file:
    inputFile = file.read()

test = """
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"""

def load(lines):
    if not lines:
        return 0, 0
    l, d = load(lines[1:])
    line = 0
    for i in range(len(lines[0])):
        if lines[0][i] == '#':
            x = 0
            for j in range(1, len(lines)):
                if lines[j][i] == 'O':
                    line = line + d - x
                    x = x + 1
                if lines[j][i] == '#':
                    break
    return line + l, d + 1

print(load(["#"*len(test.split('\n')[1]), *list(filter(lambda x: not x == "", test.split('\n')))]))
print(load(["#"*len(inputFile.split('\n')[1]), *list(filter(lambda x: not x == "", inputFile.split('\n')))]))

def shift(lines):
    if not lines:
        return []
    grid = shift(lines[1:])
    line = list(lines[0].replace('O', '.'))
    for i in range(len(lines[0])):
        if lines[0][i] == '#':
            x = 0
            for j in range(1, len(lines)):
                if lines[j][i] == 'O':
                    grid[x][i] = 'O'
                    x = x + 1
                if lines[j][i] == '#':
                    break
    return [line, *grid]

start = [
    "#"*(len(test.split('\n')[1])+2),
    *list(map(lambda x: '#' + x + '#', filter(lambda x: not x == "", test.split('\n')))),
    "#"*(len(test.split('\n')[1])+2),
    ]


def transpose(grid):
    return [''.join(s) for s in zip(*grid)]
def north(grid):
    return shift(grid)
def west(grid):
    return transpose(shift(transpose(grid)))
def south(grid):
    return shift(grid[::-1])[::-1]
def east(grid):
    return transpose(shift(transpose(grid)[::-1])[::-1])

# n = north(start)
# list(map(lambda x: print(x), n))
# w = west(n)
# list(map(lambda x: print(x), w))
# s = south(w)
# list(map(lambda x: print(x), s))
# e = east(s)
# list(map(lambda x: print(x), e))

def cycle(grid):
    return east(south(west(north(grid))))
list(map(lambda x: print(x), cycle(start)))
list(map(lambda x: print(x), cycle(cycle(start))))
list(map(lambda x: print(x), cycle(cycle(cycle(start)))))

def weight(grid):
    weight = 0
    for i, line in enumerate(grid):
        for x in line:
            if x == 'O':
                weight += len(start) - 1 - i
    return weight

print(weight(cycle(cycle(cycle(cycle(cycle(start)))))))


# print(shift(["#"*len(inputFile.split('\n')[1]), *list(filter(lambda x: not x == "", inputFile.split('\n')))]))

start = [
    "#"*(len(inputFile.split('\n')[1])+2),
    *list(map(lambda x: '#' + x + '#', filter(lambda x: not x == "", inputFile.split('\n')))),
    "#"*(len(inputFile.split('\n')[1])+2),
    ]
for i in range(300):
    start = cycle(start)
    print(i+1, weight(start))

print( (1000000000 - 139) % 14 ) # = 7

# 104662 is too low
# 104671 is the answer, 7th in the cycle