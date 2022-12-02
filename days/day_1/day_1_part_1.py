from day import Day
import numpy as np


class Day1Part1(Day):
    day = 1
    part = 1

    def get_sample_input(self):
        return ''

    def parse_input(self):
        return [np.array(x.splitlines(), dtype=int) for x in self.input_text.split('\n\n')]

    def solve(self):
        data = self.parse_input()
        self.print_answer(max(data, key=lambda x: x.sum()).sum())
