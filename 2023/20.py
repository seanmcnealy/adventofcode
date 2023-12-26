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

if not os.path.exists('data/20'):
    response = requests.get(
        "https://adventofcode.com/2023/day/20/input",
        cookies={'session': SESSIONID}
    )

    data = response.text

    with open('data/20', 'w') as file:
        file.write(data)

inputFile: AnyStr = ""
with open('data/20', 'r') as file:
    inputFile = file.read()

test1 = """broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
"""

test2 = """broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
"""

class Element:
    name = ""
    outputs = []

    def __init__(self, name, outputs):
        self.name = name
        self.outputs = outputs
    def update(self, input, level):
        pass
class Broadcast(Element):
    pass
class FF(Element):

    def __init__(self, name, outputs):
        super().__init__(name, outputs)
        self.state = False

    def update(self, input, level):
        if not level:
            self.state = not self.state
            return self.state, True
        return self.state, False


class CON(Element):


    def __init__(self, name, outputs):
        super().__init__(name, outputs)
        self.state = {}

    def update(self, input, level):
        self.state[input] = level
        if all(self.state.values()):
            return False, True
        return True, True


def parse(line):
    name = re.search(r'^[%&]([a-z]+) ', line).group(1)
    output = list(map(lambda x: x.group(1), re.finditer(r' ([a-z]+)', line)))
    if line[0] == '%':
        return FF(name, output)
    else:
        return CON(name, output)

def parse_file(test):
    elements = {}
    for line in list(filter(lambda x: not x == "", test.split('\n')))[1:]:
        e = parse(line)
        elements[e.name] = e
    for line in list(filter(lambda x: not x == "", test.split('\n')))[1:]:
        e = parse(line)
        for o in e.outputs:
            if o in elements and isinstance(elements[o], CON):
                elements[o].update(e.name, False)
    elements['broadcast'] = Broadcast('broadcast', list(map(lambda x: x.group(1), re.finditer(r' ([a-z]+)', test.split('\n')[0]))))
    return elements
elements = parse_file(test1)
print(elements)

def pulse(input, level, pulses, i):
    for output in elements[input].outputs:
        if output in elements:
            next_level, send_pulse = elements[output].update(input, level)
            if send_pulse:
                # for n in elements[output].outputs:
                #     print(output + (" -high-> " if next_level else " -low-> ") + n)
                if next_level and output in ['bm', 'cl', 'tn', 'dr', 'rx']:
                    print(i+1, output)
                pulses.append((output, next_level))
    return len(elements[input].outputs)

def broadcast(i):
    pulses = [('broadcast', False)]
    high = 0
    low = 1
    while pulses:
        name, level = pulses[0]
        pulses = pulses[1:]
        c = pulse(name, level, pulses, i)
        high += c if level else 0
        low += c if not level else 0
    return high, low

high = low = 0
for i in range(1000):
    h, l = broadcast(i)
    high += h
    low += l
print(high * low, high, low)

elements = parse_file(test2)
high = low = 0
for i in range(1000):
    # print("broadcast")
    h, l = broadcast(i)
    high += h
    low += l
print(high * low, high, low)

elements = parse_file(inputFile)
high = low = 0
# for i in range(1000):
#     # print("broadcast")
#     h, l = broadcast(i)
#     high += h
#     low += l
# print(high * low, high, low)

elements = parse_file(inputFile)
for i in range(38431):
    broadcast(i)


# 2760 tn
# 2820 dr
# 2888 bm
# 2942 cl
# tn 2760, 6521, 10282 -> 3761
# dr 2820, 6641, 10462 -> 3821
# bm 2888, 6777, 10666 -> 3889
# cl 2942, 6885, 10828 -> 3943
print(lcm(2760, 2820, 2888, 2942))
# 68885341320 too low

print(lcm(3761, 3821, 3889, 3943))


def extended_gcd(a, b):
    """Extended Greatest Common Divisor Algorithm

    Returns:
        gcd: The greatest common divisor of a and b.
        s, t: Coefficients such that s*a + t*b = gcd

    Reference:
        https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Pseudocode
    """
    old_r, r = a, b
    old_s, s = 1, 0
    old_t, t = 0, 1
    while r:
        quotient, remainder = divmod(old_r, r)
        old_r, r = r, remainder
        old_s, s = s, old_s - quotient * s
        old_t, t = t, old_t - quotient * t

    return old_r, old_s, old_t
def combine_phased_rotations(a_period, a_phase, b_period, b_phase):
    """Combine two phased rotations into a single phased rotation

    Returns: combined_period, combined_phase

    The combined rotation is at its reference point if and only if both a and b
    are at their reference points.
    """
    gcd, s, t = extended_gcd(a_period, b_period)
    phase_difference = a_phase - b_phase
    pd_mult, pd_remainder = divmod(phase_difference, gcd)
    if pd_remainder:
        raise ValueError("Rotation reference points never synchronize.")

    combined_period = a_period // gcd * b_period
    combined_phase = (a_phase - s * pd_mult * a_period) % combined_period
    return combined_period, combined_phase
# tn 2760, 6521, 10282 -> 3761
# dr 2820, 6641, 10462 -> 3821
# bm 2888, 6777, 10666 -> 3889
# cl 2942, 6885, 10828 -> 3943
p = combine_phased_rotations(3761, 2761, 3821, 2821)
print(p)
p = combine_phased_rotations(p[0], p[1], 3889, 2889)
print(p)
p = combine_phased_rotations(p[0], p[1], 3943, 2943)
print(p)
print(p[0] + p[1])
# 440732510197773 too high
# 220366255099387 lcm
# 220366255098386 too low
# is it just one more? an off by one?
# 220366255098387 isn't right

print("tn", list(map(lambda x: 3761*x + 2761, range(10))))
print("dr", list(map(lambda x: 3821*x + 2821, range(10))))
print("bm", list(map(lambda x: 3889*x + 2889, range(10))))
print("cl", list(map(lambda x: 3943*x + 2943, range(10))))
