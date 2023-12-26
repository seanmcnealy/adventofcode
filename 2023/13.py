import itertools
import os.path
import re
import sys
from typing import AnyStr
from functools import reduce
from math import lcm


import requests

SESSIONID = ''

if not os.path.exists('data/13'):
    response = requests.get(
        "https://adventofcode.com/2023/day/13/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/13', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/13', 'r') as file:
    inputFile = file.read()

test = """
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"""

# print(list(map(lambda x: list(filter(lambda y: not y == "", x.split('\n'))), test.split("\n\n"))))

test1 = list(map(lambda x: list(filter(lambda y: not y == "", x.split('\n'))), test.split("\n\n")))[0]
test2 = list(map(lambda x: list(filter(lambda y: not y == "", x.split('\n'))), test.split("\n\n")))[1]
print(test1)

def h_mirrors(line, available):
    r = []
    for i in range(len(line))[1:]:
        if i not in available:
            continue
        # left = line[max(0, 2*i - len(line)):i]
        # right = line[min(len(line), i*2-1):i-1:-1]
        if line[max(0, 2*i - len(line)):i] == line[min(len(line), i*2-1):i-1:-1]:
            r = [*r, i]
    return set(r)

def transpose(grid):
    return [''.join(s) for s in zip(*grid)]

print(list(map(lambda line: h_mirrors(line, range(len(line))), test1)))
print(list(map(lambda line: h_mirrors(line, range(len(line))), transpose(test1))))
print("test1 horizontal", reduce(lambda x,y: x&y, map(lambda line: h_mirrors(line, range(len(line))), test1)))
print("test2 horizontal", reduce(lambda x,y: x&y, map(lambda line: h_mirrors(line, range(len(line))), test2)))
print("test1 vertical", reduce(lambda x,y: x&y, map(lambda line: h_mirrors(line, range(len(line))), transpose(test1))))
print("test1 vertical", reduce(lambda x,y: x&y, map(lambda line: h_mirrors(line, range(len(line))), transpose(test2))))

def part1(grid):
    h = reduce(lambda x,y: x&y, map(lambda line: h_mirrors(line, range(len(line))), grid))
    v = reduce(lambda x,y: x&y, map(lambda line: h_mirrors(line, range(len(line))), transpose(grid)))
    return (h.pop() if len(h) > 0 else 0) + ((v.pop() * 100) if len(v) > 0 else 0)

print(sum(map(part1, list(map(lambda x: list(filter(lambda y: not y == "", x.split('\n'))), test.split("\n\n"))))))
print(sum(map(part1, list(map(lambda x: list(filter(lambda y: not y == "", x.split('\n'))), inputFile.split("\n\n"))))))

def part2_mirrors(line):
    r = []
    for i in range(len(line))[1:]:
        left = line[max(0, 2*i - len(line)):i]
        right = line[min(len(line), i*2-1):i-1:-1]
        fix = -1
        for j in range(len(left)):
            if fix == -1 and left[j] != right[j]:
                fix = j
            elif left[j] != right[j]:
                fix = -2
        if fix >= 0:
            r = [*r, i]
    return set(r)

print(list(map(lambda line: part2_mirrors(line), test1)))
print(list(map(lambda line: part2_mirrors(line), transpose(test1))))

def combine(p1, p2):
    r = []
    for i in range(len(p1)):
        r = [*r, [*p2[0:i], p1[i], *p2[i+1:len(p2)]]]
    return r

print("test1 horizontal", list(map(lambda l: reduce(lambda x,y: x&y, l),
    combine( list(map(lambda line: part2_mirrors(line), test1)),
             list(map(lambda line: h_mirrors(line, range(len(line))), test1)))
)))
# print("test2 horizontal", reduce(lambda x,y: x&y, map(lambda line: h_mirrors(line, range(len(line))), test2)))
print("test1 vertical", list(map(lambda l: reduce(lambda x,y: x&y, l),
       combine( list(map(lambda line: part2_mirrors(line), test1)),
                list(map(lambda line: h_mirrors(line, range(len(line))), transpose(test1))))
       )))
# print("test1 vertical", reduce(lambda x,y: x&y, map(lambda line: h_mirrors(line, range(len(line))), transpose(test2))))

def part2(grid):
    h = reduce(lambda x,y: x|y, map(lambda l: reduce(lambda x,y: x&y, l),
            combine(
                list(map(lambda line: part2_mirrors(line), grid)),
                list(map(lambda line: h_mirrors(line, range(len(line))), grid))
            )))
    v = reduce(lambda x,y: x|y, map(lambda l: reduce(lambda x,y: x&y, l),
                                    combine(
                                        list(map(lambda line: part2_mirrors(line), transpose(grid))),
                                        list(map(lambda line: h_mirrors(line, range(len(line))), transpose(grid)))
                                    )))
    return (h.pop() if len(h) > 0 else 0) + ((v.pop() * 100) if len(v) > 0 else 0)

print(sum(map(part2, list(map(lambda x: list(filter(lambda y: not y == "", x.split('\n'))), test.split("\n\n"))))))
print(sum(map(part2, list(map(lambda x: list(filter(lambda y: not y == "", x.split('\n'))), inputFile.split("\n\n"))))))
