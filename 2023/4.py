import itertools
import os.path
import re
from typing import AnyStr
from functools import reduce

import requests

SESSIONID = ''

if not os.path.exists('data/4'):
    response = requests.get(
        "https://adventofcode.com/2023/day/4/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/4', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/4', 'r') as file:
    inputFile = file.read()

test = """
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"""
def f(line):
    card, numbers = line.split(':')
    winning, picked = numbers.split('|')
    w = list(map(lambda x: x.group(), re.finditer(r'\d+', winning)))
    p = map(lambda x: x.group(), re.finditer(r'\d+', picked))
    e = sum(map(lambda x: 1 if x in w else 0, p))
    return 1 << e-1 if e > 0 else 0

print(sum(
    map(lambda line: f(line), filter(lambda line: not line == "", test.split('\n')))
))
print(sum(
    map(lambda line: f(line), filter(lambda line: not line == "", inputFile.split('\n')))
))

def f2(line):
    card, numbers = line.split(':')
    winning, picked = numbers.split('|')
    w = list(map(lambda x: x.group(), re.finditer(r'\d+', winning)))
    p = map(lambda x: x.group(), re.finditer(r'\d+', picked))
    e = sum(map(lambda x: 1 if x in w else 0, p))
    return e

def g(total, line, wins):
    x = f2(line)
    y = list(filter(lambda x: x > 0, map(lambda x: x - 1, wins)))
    z = [x] * (len(wins)+1) if x > 0 else []
    # print(x, total + len(wins) + 1, y + z)
    return total + len(wins) + 1, "", y + z

print(reduce(
    lambda acc, line: g(acc[0], line[1], acc[2]), map(
        lambda x: (0, x, []), [(0, "", [])] + list(filter(
            lambda line: not line == "", test.split('\n'))))
)[0])

print(reduce(
    lambda acc, line: g(acc[0], line[1], acc[2]), map(
        lambda x: (0, x, []), [(0, "", [])] + list(filter(
            lambda line: not line == "", inputFile.split('\n'))))
)[0])
