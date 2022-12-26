from __future__ import annotations

from dataclasses import dataclass
from typing import Optional

from day import Day
import numpy as np


def new_pos(x, y) -> np.ndarray:
    return np.array([x, y])


@dataclass
class Move:
    dir: str
    steps: int


@dataclass
class Node:
    id: int
    pos: np.ndarray
    parent: Optional[Node] = None
    child: Optional[Node] = None

    @classmethod
    def create_chain(cls, length):
        id_ = 0
        head = Node(id_, new_pos(0, 0))
        tail = head
        for i in range(length - 1):
            id_ += 1
            child = Node(id_, new_pos(0, 0))
            tail.child = child
            child.parent = tail
            tail = child
        return head, tail

    def __iter__(self):
        node = self
        while node is not None:
            yield node
            node = node.child


X = 0
Y = 1

DIR_TO_OFFSET = {
    'U': (0, -1),
    'D': (0, 1),
    'L': (-1, 0),
    'R': (1, 0),
}


class Day9Part2(Day):
    day = 9
    part = 2

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

    def get_movement_for_nodes(self, head, tail):
        if head[Y] == tail[Y]:
            return -1 if head[X] < tail[X] else 1, 0

        if head[X] == tail[X]:
            return 0, -1 if head[Y] < tail[Y] else 1

        y_change = -1 if head[Y] < tail[Y] else 1
        x_change = -1 if head[X] < tail[X] else 1

        return x_change, y_change

    def display_nodes(self, nodes: Node):
        positions = {}
        for node in nodes:
            positions.setdefault(tuple(node.pos), []).append(node.id)
        print(positions)

    def is_child_out_of_range_of_parent(self, child, parent):
        return abs(child[X] - parent[X]) > 1 or abs(child[Y] - parent[Y]) > 1

    def process_movement(self, head: Node):
        node = head.child
        while node:
            if not self.is_child_out_of_range_of_parent(node.pos, node.parent.pos):
                break

            move = self.get_movement_for_nodes(node.parent.pos, node.pos)
            node.pos += move
            node = node.child

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
        head, tail = Node.create_chain(10)
        tail_positions = set()

        def add_tail_pos():
            tail_positions.add((tail.pos[X], tail.pos[Y]))

        add_tail_pos()

        for move in data:
            for _ in range(move.steps):
                offset = DIR_TO_OFFSET[move.dir]
                head.pos[X] += offset[X]
                head.pos[Y] += offset[Y]
                self.process_movement(head)
                add_tail_pos()
                # self.display_nodes(head)

        self.print_answer(len(tail_positions))
