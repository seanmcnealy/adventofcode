import itertools
import os.path
import re
import sys
from typing import AnyStr
from functools import reduce

import requests

SESSIONID = ''

if not os.path.exists('data/7'):
    response = requests.get(
        "https://adventofcode.com/2023/day/7/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/7', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/7', 'r') as file:
    inputFile = file.read()

test = """32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"""

order1 = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2']
order2 = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J']


def histogram(hand):
    return map(lambda x: (x[0], len(list(x[1]))),
               itertools.groupby(sorted(hand)))


def fiveOfKind(hand, histogram):
    return hand[0] == hand[1] == hand[2] == hand[3] == hand[4]


def fourOfKind(hand, histogram):
    return any(map(lambda h: h[1] == 4, histogram))


def fullHouse(hand, histogram):
    values = sorted(map(lambda x: x[1], histogram))
    return len(values) == 2 and values[0] == 2 and values[1] == 3


def threeOfKind(hand, histogram):
    return any(map(lambda h: h[1] == 3, histogram))


def twoPair(hand, histogram):
    return sum(map(lambda h: 1 if h[1] == 2 else 0, histogram)) == 2


def onePair(hand, histogram):
    return any(map(lambda h: h[1] == 2, histogram))


def parseLine(line):
    return line[0:5], int(line[6:])


def rankHand(hand):
    h = list(histogram(hand))
    r = ''.join(map(lambda h: chr(ord('a') + order1.index(h)), hand))
    if fiveOfKind(hand, h):
        return '1' + r
    if fourOfKind(hand, h):
        return '2' + r
    if fullHouse(hand, h):
        return '3' + r
    if threeOfKind(hand, h):
        return '4' + r
    if twoPair(hand, h):
        return '5' + r
    if onePair(hand, h):
        return '6' + r
    return '7' + r


print(sum(
    map(lambda x: (x[0] + 1) * x[1][1],
        enumerate(reversed(sorted(
            map(parseLine, filter(lambda x: not x == "", test.split('\n'))),
            key=lambda x: rankHand(x[0])))))
))

print(sum(
    map(lambda x: (x[0] + 1) * x[1][1],
        enumerate(reversed(sorted(
            map(parseLine, filter(lambda x: not x == "", inputFile.split('\n'))),
            key=lambda x: rankHand(x[0])))))
))


def fiveOfKind2(hand, histogram, wild):
    return (hand[0] == hand[1] == hand[2] == hand[3] == hand[4]
            or (wild == 1 and any(map(lambda h: h[1] == 4, histogram)))
            or (wild == 2 and any(map(lambda h: h[1] == 3, histogram)))
            or (wild == 3 and any(map(lambda h: h[1] == 2, histogram)))
            or (wild == 4))


def fourOfKind2(hand, histogram, wild):
    return (any(map(lambda h: h[1] == 4, histogram))
            or (wild == 1 and any(map(lambda h: h[1] == 3, histogram)))
            or (wild == 2 and sum(map(lambda h: 1 if h[1] == 2 else 0, histogram)) == 2)
            or (wild == 3))


def fullHouse2(hand, histogram, wild):
    values = sorted(map(lambda x: x[1], histogram))
    return (len(values) == 2 and values[0] == 2 and values[1] == 3
            or (wild == 1 and sum(map(lambda h: 1 if h[1] == 2 else 0, histogram)) == 2))


def threeOfKind2(hand, histogram, wild):
    return (any(map(lambda h: h[1] == 3, histogram))
            or (wild == 1 and any(map(lambda h: h[1] == 2, histogram)))
            or (wild == 2))


def twoPair2(hand, histogram, wild):
    return (sum(map(lambda h: 1 if h[1] == 2 else 0, histogram)) == 2
            or (wild == 1 and any(map(lambda h: h[1] == 2, histogram))))


def onePair2(hand, histogram, wild):
    return (any(map(lambda h: h[1] == 2, histogram))
            or (wild == 1))


def rankHand2(hand):
    h = list(histogram(hand))
    wild = sum(map(lambda x: 1 if x == 'J' else 0, hand))
    r = ''.join(map(lambda h: chr(ord('a') + order2.index(h)), hand))
    if fiveOfKind2(hand, h, wild):
        return '1' + r
    if fourOfKind2(hand, h, wild):
        return '2' + r
    if fullHouse2(hand, h, wild):
        return '3' + r
    if threeOfKind2(hand, h, wild):
        return '4' + r
    if twoPair2(hand, h, wild):
        return '5' + r
    if onePair2(hand, h, wild):
        return '6' + r
    return '7' + r


print(sum(
    map(lambda x: (x[0] + 1) * x[1][1],
        enumerate(reversed(sorted(
            map(parseLine, filter(lambda x: not x == "", test.split('\n'))),
            key=lambda x: rankHand2(x[0])))))
))

print(sum(
    map(lambda x: (x[0] + 1) * x[1][1],
        enumerate(reversed(sorted(
            map(parseLine, filter(lambda x: not x == "", inputFile.split('\n'))),
            key=lambda x: rankHand2(x[0])))))
))
