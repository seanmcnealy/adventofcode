import bisect
import copy
import itertools
import os.path
import re
import sys
from typing import AnyStr
from functools import reduce
from math import lcm


import requests

SESSIONID = ''

if not os.path.exists('data/25'):
    response = requests.get(
        "https://adventofcode.com/2023/day/25/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/25', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/25', 'r') as file:
    inputFile = file.read()

test = """jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
"""

graph = {}
for line in list(map(lambda line: list(map(lambda g: g.group(), re.finditer(r'[a-z]+', line))), filter(lambda x: not x == "", test.split('\n')))):
    x = line[0]
    for y in line[1:]:
        if x in graph:
            graph[x].append(y)
        else:
            graph[x] = [y]
        if y in graph:
            graph[y].append(x)
        else:
            graph[y] = [x]
print(graph)

bfs = {}
for k in graph.keys():
    search = [k]
    found = set()
    i = 0
    while search:
        next_search = set()
        while search:
            s = search.pop()
            for g in graph[s]:
                if g not in found and g not in next_search:
                    next_search.add(g)
        i += 1
        for s in next_search:
            found.add(s)
            search.append(s)
    bfs[k] = i
print(bfs)
m = min(bfs.values())
print(list(filter(lambda x: x[1] == m, bfs.items())))


# hfx/pzl, the wire between bvb/cmg, and the wire between nvd/jqt
graph['hfx'].remove('pzl')
graph['pzl'].remove('hfx')
graph['bvb'].remove('cmg')
graph['cmg'].remove('bvb')
graph['nvd'].remove('jqt')
graph['jqt'].remove('nvd')
bfs = {}
for k in graph.keys():
    search = [k]
    found = set()
    i = 0
    while search:
        next_search = set()
        while search:
            s = search.pop()
            for g in graph[s]:
                if g not in found and g not in next_search:
                    next_search.add(g)
                    i += 1
        for s in next_search:
            found.add(s)
            search.append(s)
    bfs[k] = i
print(bfs)

graph = {}
for line in list(map(lambda line: list(map(lambda g: g.group(), re.finditer(r'[a-z]+', line))), filter(lambda x: not x == "", inputFile.split('\n')))):
    x = line[0]
    for y in line[1:]:
        if x in graph:
            graph[x].append(y)
        else:
            graph[x] = [y]
        if y in graph:
            graph[y].append(x)
        else:
            graph[y] = [x]
print(graph)


# bfs = {}
# for k in graph.keys():
#     search = [k]
#     found = set()
#     i = 0
#     while search:
#         next_search = set()
#         while search:
#             s = search.pop()
#             for g in graph[s]:
#                 if g not in found and g not in next_search:
#                     next_search.add(g)
#         i += 1
#         for s in next_search:
#             found.add(s)
#             search.append(s)
#     bfs[k] = i
# print(bfs)
# m = min(bfs.values())
# print(list(filter(lambda x: x[1] == m, bfs.items())))
# [('gqr', 9), ('scr', 9), ('vbk', 9), ('klj', 9), ('mxv', 9), ('sdv', 9)]
# gqr - vbk
# scr - klj
# mxv - sdv
graph['gqr'].remove('vbk')
graph['vbk'].remove('gqr')
graph['scr'].remove('klj')
graph['klj'].remove('scr')
graph['mxv'].remove('sdv')
graph['sdv'].remove('mxv')

bfs = {}
for k in graph.keys():
    search = [k]
    found = set()
    i = 0
    while search:
        next_search = set()
        while search:
            s = search.pop()
            for g in graph[s]:
                if g not in found and g not in next_search:
                    next_search.add(g)
                    i += 1
        for s in next_search:
            found.add(s)
            search.append(s)
    bfs[k] = i
print(bfs)
print(len(graph.keys()))
# 784 and 1493
# 1,172,790 is too high
# 1,170,512 is too high