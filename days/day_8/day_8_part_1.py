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
        return np.array([list(line) for line in self.read_sample_file_lines(0)])

    def check_array(self, array: np.ndarray):
        pass

    def solve(self):
        data = self.parse_input()
        # edges
        total = data.shape[0] * 2 + (data.shape[1] * 2) - 4
        ic(total)
