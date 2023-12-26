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

if not os.path.exists('data/22'):
    response = requests.get(
        "https://adventofcode.com/2023/day/22/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/22', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/22', 'r') as file:
    inputFile = file.read()

test = """1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
"""

def parse(line):
    return list(map(lambda x: int(x.group()), re.finditer(r'(\d+)', line)))

bricks = list(map(parse, filter(lambda x: not x == "", test.split('\n'))))

def under(x1,y1,z1,x2,y2,z2):
    global bricks
    return list(filter(lambda x:
           min(z1, z2) - 1 == max(x[1][2],x[1][5]) and
           min(x1,x2) <= max(x[1][0],x[1][3]) and
           max(x1,x2) >= min(x[1][0],x[1][3]) and
           min(y1,y2) <= max(x[1][1],x[1][4]) and
           max(y1,y2) >= min(x[1][1],x[1][4]),
           enumerate(bricks)))

print(list(map(lambda x: under(*x), bricks)))
moved = True
while moved:
    lower_bricks = []
    moved = False
    for x1,y1,z1,x2,y2,z2 in bricks:
        while min(z1,z2) > 1 and not under(x1, y1, z1, x2, y2, z2):
            z1 -= 1
            z2 -= 1
            moved = True
        lower_bricks.append((x1,y1,z1,x2,y2,z2))
    bricks = lower_bricks

print(bricks)
print(set(itertools.chain.from_iterable(filter(lambda x: len(x) == 1, map(lambda b: list(map(lambda x: x[0], under(*b))), bricks)))))
print(len(bricks) - len(set(itertools.chain.from_iterable(filter(lambda x: len(x) == 1, map(lambda b: list(map(lambda x: x[0], under(*b))), bricks))))))

graph = list(map(lambda b: list(map(lambda x: x[0], under(*b))), bricks))
total = 0
for i in range(len(bricks)):
    gone = {i}
    more = True
    while more:
        more = False
        for j, g in enumerate(graph):
            if j not in gone and g and all(map(lambda u: u in gone, g)):
                gone.add(j)
                more = True
    total += len(gone) - 1
print(total)

bricks = list(map(parse, filter(lambda x: not x == "", inputFile.split('\n'))))
bricks.sort(key = lambda x: min(x[2], x[5]))
moved = 1
while moved > 0:
    lower_bricks = []
    moved = 0
    for x1,y1,z1,x2,y2,z2 in bricks:
        while min(z1,z2) > 1 and not under(x1, y1, z1, x2, y2, z2):
            z1 -= 1
            z2 -= 1
            moved += 1
        lower_bricks.append((x1,y1,z1,x2,y2,z2))
    bricks = lower_bricks
# bricks.sort(key = lambda x: min(x[2], x[5]))
print(list(filter(lambda x: len(x) == 1, map(lambda b: list(map(lambda x: x[0], under(*b))), bricks))))
print(len(bricks) - len(set(itertools.chain.from_iterable(filter(lambda x: len(x) == 1, map(lambda b: list(map(lambda x: x[0], under(*b))), bricks))))))

graph = list(map(lambda b: list(map(lambda x: x[0], under(*b))), bricks))
total = 0
for i in range(len(bricks)):
    gone = {i}
    more = True
    while more:
        more = False
        for j, g in enumerate(graph):
            if j not in gone and g and all(map(lambda u: u in gone, g)):
                gone.add(j)
                more = True
    total += len(gone) - 1
print(total)
