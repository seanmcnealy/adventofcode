import itertools
import os.path
import re
import sys
from typing import AnyStr
from functools import reduce
from math import lcm


import requests

SESSIONID = ''

if not os.path.exists('data/9'):
    response = requests.get(
        "https://adventofcode.com/2023/day/9/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/9', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/9', 'r') as file:
    inputFile = file.read()

test = """0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"""

def predict(line):
    x = line[0]
    if all(map(lambda y: x == y, line)):
        return line[0]
    diff = []
    for z in line[1:]:
        diff = [*diff, z - x]
        x = z
    else:
        return line[-1] + predict(diff)

print(sum(
    map(lambda line: predict(list(map(lambda x: int(x.group()), re.finditer('-?\d+', line)))), test.split('\n'))
))

print(sum(
    map(lambda line: predict(list(map(lambda x: int(x.group()), re.finditer('-?\d+', line)))), filter(lambda x: not x == "", inputFile.split('\n')))
))

def predict2(line):
    x = line[0]
    if all(map(lambda y: x == y, line)):
        return line[0]
    diff = []
    for z in line[1:]:
        diff = [*diff, z - x]
        x = z
    else:
        return line[0] - predict2(diff)

print(sum(
    map(lambda line: predict2(list(map(lambda x: int(x.group()), re.finditer('-?\d+', line)))), test.split('\n'))
))

print(sum(
    map(lambda line: predict2(list(map(lambda x: int(x.group()), re.finditer('-?\d+', line)))), filter(lambda x: not x == "", inputFile.split('\n')))
))
