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


class Day8Part1(Day):
    day = 8
    part = 1

    def get_sample_input(self):
        return ''

    def parse_input(self):
        return np.array([list(line) for line in self.input_text_lines])
        # return np.array([list(line) for line in self.read_sample_file_lines(0)])

    def check_visibility(self, data: np.ndarray, index: tuple, value: int):
        row = data[index[0], :]
        column = data[:, index[1]]
        left, right, up, down = row[:index[1]], row[index[1] + 1:], column[:index[0]], column[index[0] + 1:]
        return (left < value).all() or (right < value).all() or (up < value).all() or (down < value).all()

    def solve(self):
        data = self.parse_input()
        total = 0
        for i, v in np.ndenumerate(data):
            if i[0] == 0 or i[1] == 0 or i[0] == data.shape[0] - 1 or i[1] == data.shape[1] - 1:
                total += 1
                continue
                
            if self.check_visibility(data, i, v):
                total += 1
        self.print_answer(total)
