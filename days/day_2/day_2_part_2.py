from day import Day

"""
The first column is what your opponent is going to play
The second column, X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win

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


class Result:
    LOSE = 'X'
    DRAW = 'Y'
    WIN = 'Z'


SHAPE_SCORE = {
    ROCK: 1,
    PAPER: 2,
    SCISSORS: 3
}

SHAPE_MATCHUPS = {
    ROCK: {'lose': PAPER, 'win': SCISSORS},
    PAPER: {'lose': SCISSORS, 'win': ROCK},
    SCISSORS: {'lose': ROCK, 'win': PAPER}
}


class Day2Part2(Day):
    day = 2
    part = 2

    def get_sample_input(self):
        return ''

    def parse_input(self):
        return self.input_text_lines
        # return self.read_sample_file_lines(0)

    def solve(self):
        data = self.parse_input()
        score = 0
        for played, result in map(str.split, data):
            matchups_for_played = SHAPE_MATCHUPS[played]
            match result:
                case Result.WIN:
                    score += SCORE_WIN + SHAPE_SCORE[matchups_for_played['lose']]
                case Result.DRAW:
                    score += SCORE_DRAW + SHAPE_SCORE[played]
                case Result.LOSE:
                    score += SCORE_LOST + SHAPE_SCORE[matchups_for_played['win']]
        self.print_answer(score)
