from icecream import ic

from day import Day
import numpy as np
import utils

"""
Each rucksack has two large compartments. 
All items of a given type are meant to go into exactly one of the two compartments. 
The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.

The Elves have made a list of all of the items currently in each rucksack,
    but they need your help finding the errors.

Every item type is identified by a single `LOWERCASE` or `UPPERCASE LETTER` (that is, `a` and `A` refer to different types of items).

The list of items for each rucksack is given as characters all on a single line

A given rucksack always has the same number of items in each of its two compartments, 
    so the first half of the characters represent items in the first compartment, 
    while the second half of the characters represent items in the second compartment.
    
To help prioritize item rearrangement, every item type can be converted to a priority:
    Lowercase item types a through z have priorities 1 through 26.
    Uppercase item types A through Z have priorities 27 through 52.

Find the item type that appears in both compartments of each rucksack. 
What is the sum of the priorities of those item types?
"""
_LETTERS = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'
PRIORITIES = {v: i + 1 for i, v in enumerate(_LETTERS)}


class Day3Part1(Day):
    day = 3
    part = 1

    def get_sample_input(self):
        return ''

    def parse_input(self):
        return [
            (set(line[:(_mid := len(line) // 2)]), set(line[_mid:]))
            for line in self.input_text_lines
        ]

    def solve(self):
        data = self.parse_input()
        self.print_answer(
            sum(
                PRIORITIES[next(iter(x))]
                for x in [inv[0] & inv[1] for inv in data]
            )
        )
