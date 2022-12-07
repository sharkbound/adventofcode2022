from day import Day

"""
To be able to communicate with the Elves, the device needs to lock on to their signal. 
The signal is a series of seemingly-random characters that the device receives one at a time.

add a subroutine to the device that detects a start-of-packet marker in the datastream
the start of a packet is indicated by a sequence of four characters that are all different.

your subroutine needs to identify the first position where the four most recently received characters were all different. 
Specifically, it needs to report the number of characters from the beginning of the buffer to the end of the first such four-character marker.
"""


class Day6Part1(Day):
    day = 6
    part = 1

    def get_sample_input(self):
        return ''

    def parse_input(self):
        # return self.read_sample_file_text(2)
        return self.input_text

    def solve(self):
        data = self.parse_input()
        for i, _ in enumerate(data):
            if len(set(data[i: i + 4])) != 4:
                continue

            self.print_answer(i + 4)
            return
