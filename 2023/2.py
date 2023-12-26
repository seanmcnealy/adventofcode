import os.path
import re
from typing import AnyStr
from functools import reduce

import requests

SESSIONID = ''

if not os.path.exists('data/2'):
    response = requests.get(
        "https://adventofcode.com/2023/day/2/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/2', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/2', 'r') as file:
    inputFile = file.read()

maximum = {
    "blue": 14,
    "green": 13,
    "red": 12
}

test = """
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"""

def game(line):
    g = dict(map(lambda x: (re.search(r' ([a-z]+)$', x).group(1), int(re.search('\d+', x).group())), line.split(", ")))
    # print(g)
    for k, v in g.items():
        if(maximum[k] < v):
            return False
    return True


def f(line):
    game_number = int(re.search(r'Game (\d+):', line).group(1))
    games = re.search(r': (.*)$', line).group(1).split("; ")
    possible = all(map(game, games))
    if possible:
        return game_number
    else:
        return 0

print("test 1", sum(map(f, filter(lambda x: not x == "", test.split('\n')))))
print("part 1", sum(map(f, filter(lambda x: not x == "", inputFile.split('\n')))))


def game2(line, minimum):
    g = dict(map(lambda x: (re.search(r' ([a-z]+)$', x).group(1), int(re.search('\d+', x).group())), line.split(", ")))
    for k, v in g.items():
        if k not in minimum or minimum[k] < v:
            minimum[k] = v
    return minimum


def f2(line):
    minimum = {}
    games = re.search(r': (.*)$', line).group(1).split("; ")
    for g in games:
        minimum = game2(g, minimum)
    # print(minimum)
    return reduce(lambda x,y: x*y, minimum.values())

print("test 2", sum(map(f2, filter(lambda x: not x == "", test.split('\n')))))
print("part 2", sum(map(f2, filter(lambda x: not x == "", inputFile.split('\n')))))
