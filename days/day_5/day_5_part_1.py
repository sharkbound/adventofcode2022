import utils
from day import Day

"""
Supplies are stored in stacks of marked crates, 
    but because the needed supplies are buried under many other crates, 
    the crates need to be rearranged.

The ship has a giant cargo crane capable of moving crates between stacks.
The crane operator will rearrange them in a series of carefully-planned steps

The Elves don't want to interrupt the crane operator during this delicate procedure, 
    but they forgot to ask her which crate will end up where
    
***
After the rearrangement procedure completes, what crate ends up on top of each stack? 
"""


class Day5Part1(Day):
    day = 5
    part = 1

    def get_sample_input(self):
        return ''

    def parse_input(self):
        raw_text = self.input_text
        rows, instructions = (_parts := raw_text.split('\n\n'))[0].splitlines(keepends=False), _parts[1].splitlines(keepends=False)
        cargo_stack_id_line = rows[-1]

        del rows[-1]

        stacks = {}
        for i, char in enumerate(cargo_stack_id_line):
            if char.isspace():
                continue

            char = int(char)
            stacks[char] = [row[i] for row in reversed(rows) if i < len(row) and row[i].strip()]

        parsed_instructions = [utils.get_all_ints(line, transform=tuple) for line in reversed(instructions)]
        return stacks, parsed_instructions

    def solve(self):
        stacks, instructions = self.parse_input()
        for times, from_, to_ in reversed(instructions):
            for _ in range(int(times)):
                stacks[to_].append(stacks[from_].pop())

        self.print_answer(''.join(s.pop() for s in stacks.values()))
