import itertools
import os.path
import re
from typing import AnyStr
from functools import reduce

import requests

SESSIONID = ''

if not os.path.exists('data/3'):
    response = requests.get(
        "https://adventofcode.com/2023/day/3/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/3', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/3', 'r') as file:
    inputFile = file.read()

test = """
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"""
def flatmap(func, *iterable):
    return itertools.chain.from_iterable(map(func, *iterable))

test_grid = list(filter(lambda x: not x == "", test.split('\n')))
def gear(row, col, test_grid):
    if(row >= 0 and row < len(test_grid) and col >= 0 and col < len(test_grid[0])):
        return ".1234567890".find(test_grid[row][col]) == -1
    return False
test_a = list(flatmap(
    lambda line: map(lambda x: (line[0], x), re.finditer(r'(\d+)', line[1])), enumerate(filter(lambda x: not x == "", test.split('\n')))
))
accum = 0
for row, match in test_a:
    hasgear = (any(map(lambda col: gear(row-1, col, test_grid), range(match.span()[0]-1, match.span()[1]+1)))
               or any(map(lambda col: gear(row, col, test_grid), range(match.span()[0]-1, match.span()[1]+1)))
               or any(map(lambda col: gear(row+1, col, test_grid), range(match.span()[0]-1, match.span()[1]+1))))
    print(hasgear)
    if hasgear:
        accum += int(match.group())
print(accum)


input_grid = list(filter(lambda x: not x == "", inputFile.split('\n')))
input_a = list(flatmap(
    lambda line: map(lambda x: (line[0], x), re.finditer(r'(\d+)', line[1])), enumerate(filter(lambda x: not x == "", inputFile.split('\n')))
))
accum = 0
for row, match in input_a:
    hasgear = (any(map(lambda col: gear(row-1, col, input_grid), range(match.span()[0]-1, match.span()[1]+1)))
               or any(map(lambda col: gear(row, col, input_grid), range(match.span()[0]-1, match.span()[1]+1)))
               or any(map(lambda col: gear(row+1, col, input_grid), range(match.span()[0]-1, match.span()[1]+1))))
    print(match.group(), hasgear)
    if hasgear:
        accum += int(match.group())
print(accum)


def numbers(row, col, matches):
    return map(lambda x: x[1].group(),
               filter(lambda t: row - 1 <= t[0] <= row + 1 and t[1].span()[0]-1 <= col <= t[1].span()[1], matches))

accum = 0
for i, line in enumerate(test_grid):
    for j, c in enumerate(line):
        if test_grid[i][j] == '*':
            n = list(numbers(i, j, test_a))
            if len(n) == 2:
                accum += int(n[0]) * int(n[1])
print(accum)


accum = 0
for i, line in enumerate(input_grid):
    for j, c in enumerate(line):
        if input_grid[i][j] == '*':
            n = list(numbers(i, j, input_a))
            if len(n) == 2:
                accum += int(n[0]) * int(n[1])
print(accum)
