from icecream import ic

from day import Day
import re
import numpy as np
import utils

"""
First, determine whether there is enough tree cover here to keep a tree house hidden.

To do this, you need to count the number of trees that are visible from outside 
    the grid when looking directly along a row or column

A tree is visible if all of the other trees between it and an edge of the grid are shorter than it,
    only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.

All of the trees around the edge of the grid are visible
"""


class Day8Part2(Day):
    day = 8
    part = 2

    def get_sample_input(self):
        return ''

    def parse_input(self):
        return np.array([list(line) for line in self.input_text_lines])
        # return np.array([list(line) for line in self.read_sample_file_lines(0)])

    def get_score(self, array, index):
        row = array[index[0], :]
        column = array[:, index[1]]

        left = row[:index[1]][::-1]
        right = row[index[1] + 1:]
        up = column[:index[0]][::-1]
        down = column[index[0] + 1:]

        center_value = array[index]
        mul_total = 1

        for arr in (up, left, down, right):
            total = 0
            for val in arr:
                if val > center_value:
                    total += 1
                    break

                if val == center_value:
                    total += 1
                    break

                if val < center_value:
                    total += 1

            mul_total *= total

        return mul_total

    def solve(self):
        data = self.parse_input()
        best_score = 0
        for i, v in np.ndenumerate(data):
            if i[0] == 0 or i[1] == 0 or i[0] == data.shape[0] - 1 or i[1] == data.shape[1] - 1:
                continue
            best_score = max(best_score, self.get_score(data, i))
        self.print_answer(best_score)
