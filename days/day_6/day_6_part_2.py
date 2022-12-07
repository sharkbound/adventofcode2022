from day import Day

"""
To be able to communicate with the Elves, the device needs to lock on to their signal. 
The signal is a series of seemingly-random characters that the device receives one at a time.

add a subroutine to the device that detects a start-of-packet marker in the datastream
the start of a packet is indicated by a sequence of four characters that are all different.

your subroutine needs to identify the first position where the four most recently received characters were all different. 
Specifically, it needs to report the number of characters from the beginning of the buffer to the end of the first such four-character marker.

# PART 2

Your device's communication system is correctly detecting packets, but still isn't working. It looks like it also needs to look for messages.

A start-of-message marker is just like a start-of-packet marker, except it consists of *14* distinct characters rather than 4.

How many characters need to be processed before the first start-of-message marker is detected?
"""


class Day6Part2(Day):
    day = 6
    part = 2

    def get_sample_input(self):
        return ''

    def parse_input(self):
        # return 'nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg'
        return self.input_text

    def solve(self):
        data = self.parse_input()
        for i, _ in enumerate(data):
            if len(set(data[i: i + 14])) != 14:
                continue

            self.print_answer(i + 14)
            return
