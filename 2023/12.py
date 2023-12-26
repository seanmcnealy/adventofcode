import itertools
import os.path
import re
import sys
from typing import AnyStr
from functools import reduce
from math import lcm


import requests

SESSIONID = ''

if not os.path.exists('data/12'):
    response = requests.get(
        "https://adventofcode.com/2023/day/12/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/12', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/12', 'r') as file:
    inputFile = file.read()

test = """
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"""

def parse(line):
    return ((next(re.finditer(r'[.?#]+', line)).group() + '?') * 5)[:-1], list(map(lambda x: int(x.group()), re.finditer(r'\d+', line))) * 5

def count_sizes(line):
    x = []
    j = 0
    for i in line:
        if i == '#':
            j += 1
        elif j > 0:
            x = [*x, j]
            j = 0
        else:
            j = 0
    if j > 0:
        x = [*x, j]
    return x

memos = {}
def possible(line, sizes, answer = ""):
    if (line + str(sizes)) in memos:
        return memos[line + str(sizes)]
    elif '#' not in line and not sizes:
        # print(answer)
        return 1
    elif line == "":
        return 0
    elif len(re.findall("#", line)) > sum(sizes):
        return 0
    elif len(re.findall("[#?]", line)) < sum(sizes):
        return 0
    elif sizes:
        if line[0] == '?' and not ('.' in line[0:sizes[0]]) and (len(line) == sizes[0] or not(line[sizes[0]] == '#')):
            a = possible(line[1:], sizes, answer + '.') + possible(line[(sizes[0]+1):], sizes[1:], answer + ('#' * sizes[0]) + '.')
            memos[line + str(sizes)] = a
            return a
        elif line[0] == '#' and not ('.' in line[0:sizes[0]]) and (len(line) == sizes[0] or not(line[sizes[0]] == '#')):
            a = possible(line[(sizes[0]+1):], sizes[1:], answer + ('#' * sizes[0]) + '.')
            memos[line + str(sizes)] = a
            return a
        elif line[0] == '.' or line[0] == '?':
            a = possible(line[1:], sizes, answer + '.')
            memos[line + str(sizes)] = a
            return a
        else:
            return 0
    else:
        return 0

first_line = parse(test.split('\n')[6])
print(first_line)
print(possible(first_line[0], first_line[1], ""))

print(list(map(lambda x: possible(parse(x)[0], parse(x)[1], ""), filter(lambda x: not x == "", test.split('\n') ))))
print(sum(map(lambda x: possible(*parse(x)), filter(lambda x: not x == "", test.split('\n') ))))

# first_line = parse(inputFile.split('\n')[3])
# print(first_line)
# print(possible(first_line[0], first_line[1]))

# print(list(map(lambda x: possible(*parse(x)), filter(lambda x: not x == "", inputFile.split('\n') ))))
print(sum(map(lambda x: possible(*parse(x)), filter(lambda x: not x == "", inputFile.split('\n') ))))
