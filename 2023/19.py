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

if not os.path.exists('data/19'):
    response = requests.get(
        "https://adventofcode.com/2023/day/19/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/19', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/19', 'r') as file:
    inputFile = file.read()

test = """px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"""


def parse_rule(line):
    name = re.search(r'^([a-z]+){', line).group(1)
    tests = re.search(r'\{(.*)}', line).group(1).split(',')
    return name, tests

def parse_event(line):
    r = {}
    for k,v in map(lambda x: x.group(1).split('='), re.finditer(r'([xmas]=\d+)[,}]', line)):
        r[k] = int(v)
    return r

def run_rule(apply, event):
    for rule in apply:
        if rule[0] in "AR":
            return rule[0] == "A"
        elif rule[1] in "<>":
            v = event[rule[0]]
            t = int(re.search('[<>](\d+):', rule).group(1))
            d = rule[rule.index(':')+1:]
            match = False
            if rule[1] == '<':
                if v < t:
                    match = True
            else:
                if v > t:
                    match = True
            if match:
                if d in "AR":
                    return d == "A"
                return run_rule(rules[d], event)
        else:
            return run_rule(rules[rule], event)

first, second = test.split("\n\n")
events = list(filter(lambda x: not x == "", second.split('\n')))

rules = {}
for k,v in map(lambda line: parse_rule(line), first.split('\n')):
    rules[k] = v
print(rules)

total = 0
for event in map(lambda line: (parse_event(line)), events):
    if run_rule(rules["in"], event):
        total += sum(event.values())
print(total)

first, second = inputFile.split("\n\n")
events = list(filter(lambda x: not x == "", second.split('\n')))

rules = {}
for k,v in map(lambda line: parse_rule(line), first.split('\n')):
    rules[k] = v
# print(rules)

total = 0
for event in map(lambda line: (parse_event(line)), events):
    if run_rule(rules["in"], event):
        total += sum(event.values())
print(total)

ACCEPT = {
    'x': [1,4000],
    'm': [1,4000],
    'a': [1,4000],
    's': [1,4000]
}

def range_from_check(check):
    a = copy.deepcopy(ACCEPT)
    if check[1] == '<':
        a[check[0]] = [1, int(check[2:]) - 1]
    else:
        a[check[0]] = [int(check[2:]) + 1, 4000]
    return a
def inverse_from_check(check):
    a = copy.deepcopy(ACCEPT)
    if check[1] == '<':
        a[check[0]] = [int(check[2:]), 4000]
    else:
        a[check[0]] = [1, int(check[2:])]
    return a
def combine_and(a, b):
    r = {}
    for k in a.keys():
        r[k] = max(a[k][0], b[k][0]), min(a[k][1], b[k][1])
    return r
def possible(a):
    for s, e in a.values():
        if e < s:
            return False
    return True
def count(a):
    return reduce(lambda x,y: x*y, map(lambda x: x[1] - x[0] + 1, a.values()))

def analyze_rule(name, condition):
    answer = []
    for rule in rules[name]:
        if rule[0] in "AR":
            if rule[0] == 'A' and possible(condition):
                answer.append(condition)
            return answer
        elif len(rule) < 4:
            if possible(condition):
                answer = [*answer, *analyze_rule(rule, condition)]
            return answer
        else:
            r, d = rule.split(":")
            combined = combine_and(condition, range_from_check(r))
            if possible(combined):
                if d in "AR":
                    if d == "A":
                        answer.append(combined)
                else:
                    answer = [*answer, *analyze_rule(d, combined)]
            condition = combine_and(condition, inverse_from_check(r))
    return answer


first, second = test.split("\n\n")
events = list(filter(lambda x: not x == "", second.split('\n')))

rules = {}
for k,v in map(lambda line: parse_rule(line), first.split('\n')):
    rules[k] = v
print(analyze_rule('lnx', ACCEPT))
print(analyze_rule('in', ACCEPT))
print(sum(map(count, analyze_rule('lnx', ACCEPT))))
print(167409079868000)
print(sum(map(count, analyze_rule('in', ACCEPT))))


first, second = inputFile.split("\n\n")
events = list(filter(lambda x: not x == "", second.split('\n')))

rules = {}
for k,v in map(lambda line: parse_rule(line), first.split('\n')):
    rules[k] = v
print(analyze_rule('in', ACCEPT))
print(sum(map(count, analyze_rule('in', ACCEPT))))
