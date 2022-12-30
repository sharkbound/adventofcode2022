from enum import Enum, auto

from day import Day

"""
the X register controls the horizontal position of a sprite
the sprite is 3 pixels wide,
    and the X register sets the horizontal position of the middle of that sprite

You count the pixels on the CRT: 40 wide and 6 high
This CRT screen draws the top row of pixels left-to-right
     then the row below that, and so on

The left-most pixel in each row is in position 0, 
    and the right-most pixel in each row is in position 39
    
the CRT draws a single pixel during each cycle
    Representing each pixel of the screen as a #
    
If the sprite is positioned such that one of its three pixels is the pixel currently being drawn, 
    the screen produces a lit pixel (#)
    otherwise, the screen leaves the pixel dark (.). 
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
        elapsed_ticks = 0
        drawn = [[]]

        for instr in data:
            local_elapsed_ticks = 0

            while True:
                line = drawn[-1]
                beam_x = elapsed_ticks % 40
                line.append('#' if abs(beam_x - memory['x']) <= 1 else ' ')

                elapsed_ticks += 1
                local_elapsed_ticks += 1

                if len(line) >= 40:
                    drawn.append([])

                exec_result = self.exec(instr, memory, elapsed_ticks, local_elapsed_ticks)

                if exec_result == TickResult.DONE:
                    break

        # print(f'{elapsed_ticks=}, {memory=}, {sum_=}')
        # https://adventofcode.com/2022/day/10#part2
        self.print_answer('\n' + '\n'.join(''.join(line) for line in drawn))
