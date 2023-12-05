#!/usr/bin/env python3
from dataclasses import dataclass


@dataclass
class Card:
    name: str
    winning_numbers: list[int]
    my_numbers: list[int]

    def match_numbers(self):
        count = 0
        for n in self.my_numbers:
            if n in self.winning_numbers:
                count += 1
        return count

    def points(self):
        n = self.match_numbers()
        if n == 0:
            return 0
        return 2 ** (n - 1)


def parse_input(input: str) -> list[Card]:
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
    return sum(card.points() for card in parse_input(input))


def part2(input):
    cards = [{"matches": n.match_numbers(), "count": 1} for n in parse_input(input)]
    for i, m in enumerate(cards):
        matches = m["matches"]
        count = m["count"]
        # para as matches cards seguintes ganha uma cópia de cada
        # ex: se a card 1 tem 4 matches, adiciona uma cópia para a card 2 a 5
        for j in range(matches):
            cards[i + j + 1]["count"] += count
    return sum(card["count"] for card in cards)
