import re
from pathlib import Path

INPUT = Path.cwd() / '2023' / '01.input'

def part1():
    print(Path.cwd())
    with open(INPUT) as f:
        total = 0
        for line in f.readlines():
            first = None
            last = None
            for c in line:
                if c.isdigit():
                    last = c
                    if not first:
                        first = c
            total += int(first+last)
        return total

def translate(digits):
    letters_to_number = {
       "one" : "1", "two" : "2", "three" : "3", "four": "4", "five": "5", "six": "6", "seven" : "7", "eight": "8", "nine" : "9"
    }
    return map(lambda n: n if n.isdigit() else letters_to_number[n], digits)

def part2():
    # overlapping regex matches with look ahead
    digits = re.compile(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))")
    with open(INPUT) as f:
        total = 0
        for line in f.readlines():
            matches = re.finditer(digits, line)
            [first, *rest] = translate([match.group(1) for match in matches])
            if len(rest) == 0:
                last = first
            else:
                last = rest[-1]
            total += int(first+last)
        return total
        
