import itertools
import os.path
import re
import sys
from typing import AnyStr
from functools import reduce

import requests

SESSIONID = ''

if not os.path.exists('data/5'):
    response = requests.get(
        "https://adventofcode.com/2023/day/5/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/5', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/5', 'r') as file:
    inputFile = file.read()

test = """seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"""

def load(input):
    lines = input.split('\n')
    seeds = map(lambda x: int(x.group()), re.finditer(r'\d+', lines[0]))

    layers = []
    for line in filter(lambda x: not x == "", lines[1:]):
        if line[-1] == ':':
            layers = [*layers, []]
        else:
            layers[len(layers)-1] = [
                *layers[len(layers)-1],
                [*list(map(lambda x: int(x.group()), re.finditer(r'\d+', line)))]]
    layers = list(map(lambda x: [*x, [0, 0, sys.maxsize - 1]], layers))
    return seeds, layers

# print(list(seeds))
# print(layers)


seeds, layers = load(test)
for s in seeds:
    for l in layers:
        for c in l:
            if c[1] <= s and ((c[1] + c[2]) > s):
                # print("s", s, "->", c[0] + s - c[1])
                s = c[0] + s - c[1]
                break
    print(s)


print("")
seeds, layers = load(inputFile)
out = []
for s in seeds:
    for l in layers:
        for c in l:
            if c[1] <= s and ((c[1] + c[2]) > s):
                s = c[0] + s - c[1]
                break
    out = [*out, s]
# print(out)
print(min(out))

def intersect(x, y, a, b):
    if x > a:
        return intersect(a,b,x,y)
    start = max(x, a)
    end = min(x + y - 1, a + b - 1)
    if end >= start:
        return start, end - start + 1
    return None

# print("i1", intersect(1,1,2,1))
# print("i2", intersect(1,5,3,1))
# print("i3", intersect(1,25,15,5))
# print("i4", intersect(1,5,4,2))
# print("i5", intersect(1,5,1,2))

def difference(x, y, a, b):
    if x >= a and x + y <= a + b:
        return []
    intersection = intersect(x,y,a,b)
    if intersection is None:
        return [(x,y)]
    if x == a:
        start = x + min(y, b)
        end = x + max(y, b)
        return [(start, end - start)]
    if x < a and x + y > a + b:
        return [(x, a - x), (a + b, x + y - a - b)]
    if x > a:
        return [(a + b, x + y - a - b)]
    return [(x, a - x)]

# print("s1", difference(1,1,2,1))
# print("s2", difference(1,5,3,1))
# print("s3", difference(1,25,15,5))
# print("s4", difference(1,5,4,2))
# print("s5", difference(1,5,1,2))
# print("ss", difference(90, 9, 56, 37))


seeds, layers = load(test)
seeds = list(seeds)
out = []
for (ss, sr) in zip(seeds[0::2], seeds[1::2]):
    next = [(ss, sr)]
    for l in layers:
        current = next
        next = []
        for c in l:
            build = []
            for s, range in current:
                intersection = intersect(s, range, c[1], c[2])
                if intersection is not None:
                    next = [*next, (c[0] + intersection[0] - c[1], intersection[1])]
                    if s < c[1] or s + range > c[1] + c[2]:
                        dif = difference(s, range, c[1], c[2])
                        build = [*build, *dif]
                else:
                    build = [*build, (s, range)]
            current = build
        # print(next)
        if sum(map(lambda x: x[1], next)) != sr:
            print("help, should not lose quantity")
    out = [*out, *next]
# print(out)
print(min(map(lambda x: x[0], out)))

seeds, layers = load(inputFile)
seeds = list(seeds)
out = []
for (ss, sr) in zip(seeds[0::2], seeds[1::2]):
    next = [(ss, sr)]
    for l in layers:
        current = next
        next = []
        for c in l:
            build = []
            for s, range in current:
                intersection = intersect(s, range, c[1], c[2])
                if intersection is not None:
                    next = [*next, (c[0] + intersection[0] - c[1], intersection[1])]
                    if s < c[1] or s + range > c[1] + c[2]:
                        dif = difference(s, range, c[1], c[2])
                        build = [*build, *dif]
                else:
                    build = [*build, (s, range)]
            current = build
        # print(next)
        if sum(map(lambda x: x[1], next)) != sr:
            print("help, should not lose quantity")
    out = [*out, *next]
# print(out)
print(min(map(lambda x: x[0], out)))
# 47238414 too high
