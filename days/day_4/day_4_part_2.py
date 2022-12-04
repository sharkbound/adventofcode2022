from day import Day
import re
import numpy as np
import utils

"""
Every section has a unique ID number, and each Elf is assigned a range of section IDs.
They've noticed that many of the assignments overlap

To try to quickly find *overlaps* and reduce duplicated effort, 
    the Elves *pair up and make a big list of the section assignments for each pair*  

Some of the pairs have noticed that one of their assignments *fully contains the other*

In how many assignment pairs does one range fully contain the other?
"""


class Day4Part2(Day):
    day = 4
    part = 2

    def get_sample_input(self):
        return ''

    def parse_input(self):
        return [
            (set(range(item[0], item[1] + 1)), set(range(item[2], item[3] + 1)))
            for item in
            (utils.get_all_ints(line, transform=tuple) for line in self.input_text_lines)
        ]

    def solve(self):
        data = self.parse_input()
        self.print_answer(
            sum(
                1 for pair in data
                if len(pair[0] | pair[1]) != len(pair[0]) + len(pair[1])
            )
        )
