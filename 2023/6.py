import itertools
import os.path
import re
import sys
from typing import AnyStr
from functools import reduce

import requests

SESSIONID = ''

if not os.path.exists('data/6'):
    response = requests.get(
        "https://adventofcode.com/2023/day/6/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/6', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/6', 'r') as file:
    inputFile = file.read()

test = """Time:      7  15   30
Distance:  9  40  200
"""

def parse(input):
    lines = input.split('\n')
    times = re.finditer('\d+', lines[0])
    distances = re.finditer('\d+', lines[1])
    return zip(map(lambda x: int(x.group()), times), map(lambda x: int(x.group()), distances))

def distance(charge, time):
    return (time - charge) * charge

print(list(parse(test)))

print(reduce(lambda x,y: x*y,
             map(lambda r: sum(map(
                 lambda x: 1 if distance(x, r[0]) > r[1] else 0, range(r[0]))), parse(test))))
print(reduce(lambda x,y: x*y,
             map(lambda r: sum(map(
                 lambda x: 1 if distance(x, r[0]) > r[1] else 0, range(r[0]))), parse(inputFile))))

time = 71530
record = 940200

def printResult(charge):
    print(charge, distance(charge, time) > record, distance(charge, time))

# printResult(10)
# printResult(13)
# printResult(14)
# printResult(15)
# printResult(25)
# printResult(50)
# printResult(100)
# printResult(71500)
# printResult(71510)
# printResult(71515)
# printResult(71516)
# printResult(71517)
# printResult(71518)
# printResult(71520)

def binarySearch1(start, end):
    mid = int((start + end) / 2)
    if start == end or mid == start or mid == end:
        return mid
    if(distance(mid, time) > record):
        return binarySearch1(start, mid)
    return binarySearch1(mid, end)

def binarySearch2(start, end):
    mid = int((start + end) / 2)
    if start == end or mid == start or mid == end:
        return mid
    if(distance(mid, time) < record):
        return binarySearch2(start, mid)
    return binarySearch2(mid, end)

print(binarySearch1(1, 100))
print(binarySearch2(100, time))

print(71516 - 13)

time = 42686985
record = 284100511221341

def binarySearch1(start, end):
    mid = int((start + end) / 2)
    if start == end or mid == start or mid == end:
        return mid
    if(distance(mid, time) > record):
        return binarySearch1(start, mid)
    return binarySearch1(mid, end)

def binarySearch2(start, end):
    mid = int((start + end) / 2)
    if start == end or mid == start or mid == end:
        return mid
    if(distance(mid, time) < record):
        return binarySearch2(start, mid)
    return binarySearch2(mid, end)

print(binarySearch1(1, 42686985))
print(binarySearch2(4268698, time))
print(binarySearch2(4268698, time) - binarySearch1(1, 42686985))