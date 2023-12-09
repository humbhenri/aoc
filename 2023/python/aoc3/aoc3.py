#!/usr/bin/env python3
import re
from dataclasses import dataclass
from typing import List, Set
from pprint import pprint
from itertools import product

@dataclass()
class Number:
    value: int
    row: int
    start_col: int
    end_col: int
    adjacents: Set[str]

def find_numbers(schema: List[str]):
    for row_index, row in enumerate(schema):
        for match in list(re.finditer(r'\d+', row)):
            start = match.start()
            end = match.end()
            adjacents = set()
            for i, j, in product([-1, 0, 1], [-1, 0, 1]):
                if row_index+i < 0 or row_index+i > len(schema)-1:
                    continue
                for col in range(start, end):
                    if col+j < 0 or col+j > len(row)-1:
                        continue
                    adjacents.add((row_index+i, col+j))
            yield Number(int(match.group()), row_index, start, end, adjacents)

def is_symbol(s: str) -> bool:
    return s != '.' and not s.isdigit()

def number_is_part(number: Number, schema: List[str]) -> bool:
    """ number is part if is adjacent to a symbol """
    return any(map(lambda xy: is_symbol(schema[xy[0]][xy[1]]), number.adjacents))

def part1(schema: List[str]):
    numbers = find_numbers(schema)
    return sum(n.value for n in numbers if number_is_part(n, schema))

file = "/home/humberto/projects/aoc/2023/03.input"
ex = "/home/humberto/projects/aoc/2023/03.example"
with open(file) as f:
    schema = f.readlines()
    pprint(part1(schema))
