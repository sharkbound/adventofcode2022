from enum import Enum, auto

from icecream import ic

from day import Day
import re
import numpy as np
import utils

"""
addx V takes two cycles to complete. After two cycles, the X register is increased by the value V. (V can be negative.)
noop takes one cycle to complete. It has no other effect.

For now, consider the signal strength (the cycle number multiplied by the value of the X register) 
    during the 20th cycle and every 40 cycles after that
    (that is, during the 20th, 60th, 100th, 140th, 180th, and 220th cycles).

Find the signal strength during the 20th, 60th, 100th, 140th, 180th, and 220th cycles. 
What is the sum of these six signal strengths?
"""


def cast_int_args(args):
    out = []
    for arg in args:
        try:
            out.append(int(arg))
        except ValueError:
            out.append(arg)
    return tuple(out)


class TickResult(Enum):
    PROCESSING = auto()
    DONE = auto()


def process_addx(value, memory, local_elapsed_ticks):
    match local_elapsed_ticks:
        case 1:
            return TickResult.PROCESSING
        case 2:
            memory['x'] += value
            return TickResult.DONE


class Day10Part2(Day):
    day = 10
    part = 2

    def get_sample_input(self):
        return ''

    def parse_input(self):
        return tuple(
            map(
                cast_int_args,
                map(
                    str.split,
                    self.input_text_lines
                    # self.read_sample_file_lines(1)
                )
            )
        )

    def exec(self, instruction: tuple[str | int, ...], memory: dict, elapsed_ticks: int, local_elapsed_ticks: int):
        match instruction:
            case ['noop']:
                return TickResult.DONE
            case ['addx', value]:
                return process_addx(value, memory, local_elapsed_ticks)

    SUM_TICKS = {20, 60, 100, 140, 180, 220}

    def solve(self):
        data = self.parse_input()

        sum_ = 0
        memory = {'x': 1}
        elapsed_ticks = 1

        for instr in data:
            local_elapsed_ticks = 0

            while True:
                elapsed_ticks += 1
                local_elapsed_ticks += 1

                exec_result = self.exec(instr, memory, elapsed_ticks, local_elapsed_ticks)

                if elapsed_ticks in self.__class__.SUM_TICKS:
                    sum_ += elapsed_ticks * memory["x"]
                    # print(f'{elapsed_ticks=}, {memory=}, {elapsed_ticks * memory["x"]=}')

                if exec_result == TickResult.DONE:
                    break

        # print(f'{elapsed_ticks=}, {memory=}, {sum_=}')
        # https://adventofcode.com/2022/day/10#part2
        self.print_answer(sum_)
