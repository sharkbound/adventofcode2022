from pathlib import Path
from utils import ask_int, create_day_input_file_path, get_day_folder_path

DAY_PART_PY_CODE = '''
from day import Day
import re
import numpy as np
import utils


class Day{day}Part{part}(Day):
    day = {day}
    part = {part}

    def get_sample_input(self):
        return ''

    def parse_input(self):
        return ''

    def solve(self):
        data = self.parse_input()
'''


class CreateMode:
    FOLDER = 'FOLDER'
    FILE = 'FILE'


class Result:
    FILE_CREATED = 'FILE_CREATED'
    FILE_ALREADY_EXISTS = 'FILE_ALREADY_EXISTS'
    FOLDER_CREATED = 'FOLDER_CREATED'
    FOLDER_ALREADY_EXISTS = 'FOLDER_ALREADY_EXISTS'


def create_if_not_exists(path: Path, type: str):
    if type == CreateMode.FOLDER:
        if path.exists():
            return Result.FOLDER_ALREADY_EXISTS

        path.mkdir(parents=True, exist_ok=True)
        return Result.FOLDER_CREATED

    if type == CreateMode.FILE:
        if path.exists():
            return Result.FILE_ALREADY_EXISTS

        path.parent.mkdir(parents=True, exist_ok=True)
        path.touch(exist_ok=True)
        return Result.FILE_CREATED


day = ask_int('enter day number >>> ')
day_input_file_path = create_day_input_file_path(day)
day_folder_path = get_day_folder_path(day)

create_if_not_exists(day_input_file_path, CreateMode.FILE)

day_part_1_file_path = day_folder_path / f'day_{day}_part_1.py'
result_part_1_py = create_if_not_exists(day_part_1_file_path, CreateMode.FILE)

day_part_2_file_path = day_folder_path / f'day_{day}_part_2.py'
result_part_2_py = create_if_not_exists(day_part_2_file_path, CreateMode.FILE)

if result_part_1_py == Result.FILE_CREATED:
    day_part_1_file_path.write_text(DAY_PART_PY_CODE.format(day=day, part=1))

if result_part_2_py == Result.FILE_CREATED:
    day_part_2_file_path.write_text(DAY_PART_PY_CODE.format(day=day, part=2))

print(f'done! created input and folder for day {day}!\n{day_input_file_path!s}\n{day_folder_path!s}')
