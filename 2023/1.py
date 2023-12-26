import os.path
import re
from typing import AnyStr
from functools import reduce

import requests

SESSIONID = ''

if not os.path.exists('data/1'):
    response = requests.get(
        "https://adventofcode.com/2023/day/1/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/1', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/1', 'r') as file:
    inputFile = file.read()

forward = {
    "one": "1",
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9",
    "1": "1",
    "2": "2",
    "3": "3",
    "4": "4",
    "5": "5",
    "6": "6",
    "7": "7",
    "8": "8",
    "9": "9",
}

reverse = {
    "one"[::-1]: "1",
    "two"[::-1]: "2",
    "three"[::-1]: "3",
    "four"[::-1]: "4",
    "five"[::-1]: "5",
    "six"[::-1]: "6",
    "seven"[::-1]: "7",
    "eight"[::-1]: "8",
    "nine"[::-1]: "9",
    "1": "1",
    "2": "2",
    "3": "3",
    "4": "4",
    "5": "5",
    "6": "6",
    "7": "7",
    "8": "8",
    "9": "9",
}

print(
    reduce(lambda x, y: x + y,
           map(lambda x: int(
               re.search(r'\d', x).group() +
               re.search(r'\d', x[::-1]).group()),
               filter(lambda x: not x == "", inputFile.split('\n')))))

print(
    reduce(lambda x, y: x + y,
           map(lambda x: int(
               forward[re.search(r'one|two|three|four|five|six|seven|eight|nine|\d', x).group()] +
               reverse[re.search(r'eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d', x[::-1]).group()]),
               filter(lambda x: not x == "", inputFile.split('\n')))))
