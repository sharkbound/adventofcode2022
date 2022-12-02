from day import Day
import numpy as np
import utils

"""
The first column is what your opponent is going to play
The second column, you reason, must be what you should play in response

Your total score is the sum of your scores for each round. 

The score for a single round is the score for the shape you selected:
    (1 for Rock, 2 for Paper, and 3 for Scissors) 
    plus the score for the outcome of the round 
    (0 if you lost, 3 if the round was a draw, and 6 if you won).
"""

ROCK = 'A'
PAPER = 'B'
SCISSORS = 'C'

SCORE_LOST = 0
SCORE_DRAW = 3
SCORE_WIN = 6

SHAPE_SCORE = {
    ROCK: 1,
    PAPER: 2,
    SCISSORS: 3
}

SHAPE_WIN_TO_LOSS = {
    ROCK: SCISSORS,
    PAPER: ROCK,
    SCISSORS: PAPER
}


class Day2Part1(Day):
    day = 2
    part = 1

    def get_sample_input(self):
        return ''

    def encrypted_to_normal(self, char):
        match char:
            case 'X':
                return ROCK
            case 'Y':
                return PAPER
            case 'Z':
                return SCISSORS

    def parse_input(self):
        return self.input_text_lines
        # return self.read_sample_file_lines(0)

    def solve(self):
        data = self.parse_input()
        score = 0
        for line in data:
            played, recommended = line.split(' ')
            recommended = self.encrypted_to_normal(recommended)

            if played == recommended:
                score += SCORE_DRAW
            elif SHAPE_WIN_TO_LOSS[recommended] == played:
                score += SCORE_WIN
            else:
                score += SCORE_LOST

            score += SHAPE_SCORE[recommended]
        self.print_answer(score)
