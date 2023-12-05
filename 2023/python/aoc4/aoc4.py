#!/usr/bin/env python3
from dataclasses import dataclass


def points(winning_numbers):
    if winning_numbers == 0:
        return 0
    return 2 ** (winning_numbers - 1)


@dataclass
class Card:
    name: str
    winning_numbers: list[int]
    my_numbers: list[int]

    def my_winning_numbers(self):
        count = 0
        for n in self.my_numbers:
            if n in self.winning_numbers:
                count += 1
        return count

    def points(self):
        n = self.my_winning_numbers()
        if n == 0:
            return 0
        return 2 ** (n - 1)


def parse_input(input: str):
    cards = []
    with open(input) as f:
        for line in f.readlines():
            [card, numbers] = line.split(":")
            [winning_numbers, my_numbers] = numbers.split("|")
            cards.append(
                Card(
                    card,
                    [int(x) for x in winning_numbers.strip().split(" ") if len(x) > 0],
                    [int(x) for x in my_numbers.strip().split(" ") if len(x) > 0],
                )
            )
    return cards

def part1(input):
    sum = 0
    for card in parse_input(input):
        sum += card.points()
    return sum

part1("/home/humberto/projects/aoc/2023/04.input")
