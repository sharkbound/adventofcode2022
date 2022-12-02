from day import Day
import numpy as np


class Day1Part2(Day):
    day = 1
    part = 2

    def get_sample_input(self):
        return ''

    def parse_input(self):
        return [np.array(x.splitlines(), dtype=int) for x in self.input_text.split('\n\n')]

    def solve(self):
        data = self.parse_input()
        data.sort(key=lambda x: x.sum(), reverse=True)
        self.print_answer(sum(x.sum() for x in data[:3]))
