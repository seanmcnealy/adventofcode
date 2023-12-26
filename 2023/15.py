import itertools
import os.path
import re
import sys
from typing import AnyStr
from functools import reduce
from math import lcm


import requests

SESSIONID = ''

if not os.path.exists('data/15'):
    response = requests.get(
        "https://adventofcode.com/2023/day/15/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/15', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/15', 'r') as file:
    inputFile = file.read()

test = """HASH"""

def hash(s):
    current = 0
    for c in s:
        current += ord(c)
        current *= 17
        current %= 256
    return current

print(hash(test))

test = """rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"""

print(sum(map(lambda line: hash(line), test.split('\n')[0].split(','))))
print(sum(map(lambda line: hash(line), inputFile.split('\n')[0].split(','))))

def process(lines):
    box = {}
    for line in lines:
        if '=' in line:
            x,y = line.split('=')
            print(x,y, hash(x))
            if hash(x) in box:
                existing = box[hash(x)]
                replaced = False
                for i in range(len(existing)):
                    e = existing[i]
                    if e[0] == x:
                        existing[i] = (x,y)
                        replaced = True
                if replaced:
                    box[hash(x)] = existing
                else:
                    box[hash(x)] = [*existing, (x,y)]
            else:
                box[hash(x)] = [(x,y)]
        elif '-' in line:
            x = line.split('-')[0]
            print(x, hash(x))
            box[hash(x)] = [*filter(lambda l: l[0] != x, box[hash(x)])] if hash(x) in box else []
    return box
box = process(test.split('\n')[0].split(','))

# print(box)
def total(box):
    total = 0
    for i, v in box.items():
        for j, y in enumerate(v):
            total += (int(i)+1)*int(y[1])*(j+1)
    return total

print(total(box))
print(total(process(inputFile.split('\n')[0].split(','))))
