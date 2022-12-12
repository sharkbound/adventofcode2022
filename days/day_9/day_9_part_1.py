from dataclasses import dataclass

from day import Day
import numpy as np

"""
Consider a rope with a knot at each end; these knots mark the head and the tail of the rope. 
If the head moves far enough away from the tail, the tail is pulled toward the head.

the head (H) and tail (T) must always be touching (diagonally adjacent and even overlapping both count as touching)

If the head is ever two steps directly up, down, left, or right from the tail, 
    the tail must also move one step in that direction so it remains close enough:

.....    .....    .....
.TH.. -> .T.H. -> ..TH.
.....    .....    .....

...    ...    ...
.T.    .T.    ...
.H. -> ... -> .T.
...    .H.    .H.
...    ...    ...


Otherwise, if the head and tail aren't touching and aren't in the same row or column, the tail always moves one step diagonally to keep up:

.....    .....    .....
.....    ..H..    ..H..
..H.. -> ..... -> ..T..
.T...    .T...    .....
.....    .....    .....

.....    .....    .....
.....    .....    .....
..H.. -> ...H. -> ..TH.
.T...    .T...    .....
.....    .....    .....


"""


@dataclass
class Move:
    dir: str
    steps: int


X = 0
Y = 1

DIR_TO_OFFSET = {
    'U': (0, -1),
    'D': (0, 1),
    'L': (-1, 0),
    'R': (1, 0),
}


class Day9Part1(Day):
    day = 9
    part = 1

    def get_sample_input(self):
        return ''

    def parse_input(self):
        data = self.input_text_lines
        # data = self.read_sample_file_lines(0)
        ret = []
        for line in data:
            parts = line.split()
            ret.append(Move(parts[0], int(parts[1])))
        return ret

    def solve(self):
        # def render_head_and_tail():
        #     diff = (head - tail)
        #     head_symbol = 'H' if not np.array_equal(head, tail) else 'S'
        #     render = [
        #         ['_', '_', '_'],
        #         ['_', 'H', '_'],
        #         ['_', '_', '_']
        #     ]
        #     if head_symbol != 'S':
        #         render[diff[1]][diff[0]] = 'T'
        #     print('\n'.join(' '.join(row) for row in render))
        #     print()

        data = self.parse_input()
        head, tail = np.array([0, 0]), np.array([0, 0])
        tail_positions = set()

        def is_tail_away_from_head():
            return abs(head[X] - tail[X]) > 1 or abs(head[Y] - tail[Y]) > 1

        def add_tail_pos():
            tail_positions.add((tail[X], tail[Y]))

        add_tail_pos()

        for move in data:
            for _ in range(move.steps):
                offset = DIR_TO_OFFSET[move.dir]
                head += offset

                if not is_tail_away_from_head():
                    continue

                if head[Y] == tail[Y]:
                    tail[X] += offset[X]
                    add_tail_pos()
                    # render_head_and_tail()
                    continue

                elif head[X] == tail[X]:
                    tail[Y] += offset[Y]
                    add_tail_pos()
                    # render_head_and_tail()
                    continue

                y_change = -1 if head[Y] < tail[Y] else 1
                x_change = -1 if head[X] < tail[X] else 1
                tail += (x_change, y_change)
                add_tail_pos()

                # render_head_and_tail()
                # ic(head, tail, np.abs(head - tail))

        self.print_answer(len(tail_positions))
