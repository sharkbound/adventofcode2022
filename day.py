from functools import cached_property
from inspect import isclass
from pathlib import Path
from time import perf_counter_ns
from typing import Optional
from importlib import import_module
from typing import NamedTuple
from collections import namedtuple

__all__ = [
    'run_day',
    'DayNotFoundError',
    'DayInputFileNotFoundError',
    'MissingDayFileError',
    'Day',
    'Path',
    'NamedTuple',
    'namedtuple'
]

import utils


class DayNotFoundError(Exception):
    def __init__(self, day, part, search_path) -> None:
        super().__init__(f'failed to load day file\n\t[day: {day}, part: {part}, path: {search_path}]')


class DayInputFileNotFoundError(Exception):
    def __init__(self, day, part, search_path):
        super().__init__(f'could not find input file for day {day}\n\t[day: {day}, part: {part}, path: {search_path}]')


class DaySampleFileNotFoundError(Exception):
    def __init__(self, day, part, sample_id):
        super().__init__(f'could not find sample file for day {day}\n\t[day: {day}, part: {part}, sample_id: {sample_id}]')


class MissingDayFileError(Exception):
    def __init__(self, day, part, search_path) -> None:
        super().__init__(f'day python does not exist\n\t[day: {day}, part: {part}, path: {search_path}]')


class Day:
    day = 0
    part = 1

    @cached_property
    def input_path(self):
        return Path(f'inputs/day_{self.day}.txt')

    @cached_property
    def input_text(self):
        if not self.input_path.exists():
            raise DayInputFileNotFoundError(self.day, self.part, self.input_path)

        return self.input_path.read_text()

    @property
    def input_text_lines(self):
        return self.input_text.splitlines(keepends=False)

    def get_sample_input(self):
        return ''

    @property
    def input_sample(self) -> str:
        return self.get_sample_input()

    def get_sample_file_path(self, sample_id) -> Optional[Path]:
        for file in utils.get_sample_files_for_day(self.day):
            filename = str(file.stem)
            sample_file_id = filename[filename.index('_') + 1:]
            if sample_file_id == str(sample_id):
                return file.absolute()
        return None

    def read_sample_file_text(self, sample_id) -> str:
        if (file := self.get_sample_file_path(sample_id)) is None:
            raise DaySampleFileNotFoundError(self.day, self.part, sample_id)
        return file.read_text()

    def read_sample_file_lines(self, sample_id: int, keepends=False) -> list[str]:
        if (file := self.get_sample_file_path(sample_id)) is None:
            raise DaySampleFileNotFoundError(self.day, self.part, sample_id)
        return file.read_text().splitlines(keepends=keepends)

    @property
    def input_sample_lines(self):
        return self.input_sample.splitlines(keepends=False)

    def parse_input(self):
        return self.input_text

    def solve(self):
        raise NotImplementedError(f'solve() is not implemented for [day: {self.day}, part: {self.part}]')

    def solve_with_timer(self):
        start = perf_counter_ns()
        self.solve()
        diff = perf_counter_ns() - start
        print(f'\nday {self.day} part {self.part} completed in \n\t{diff} NS\n\t{diff * 0.000001} MS\n\t{diff * 0.000000001} SECONDS\n')

    def print_answer(self, value):
        print(f'day {self.day} part {self.part} answer is >>> {value}')


def run_day(day, part, *, timed=False):
    days_folder = Path('./days/')
    day_py_file = days_folder / f'day_{day}' / f'day_{day}_part_{part}.py'
    day_to_run: Optional[Day] = None

    if not day_py_file.exists():
        raise MissingDayFileError(day, part, day_py_file)

    module = import_module(f'days.day_{day}.{day_py_file.name.replace(".py", "")}')
    for value in module.__dict__.values():
        if value is not Day and isclass(value) and issubclass(value, Day) and value.day == day and value.part == part:
            day_to_run = value()
            break

    if day_to_run is None:
        raise DayNotFoundError(day, part, day_py_file)

    if timed:
        day_to_run.solve_with_timer()
    else:
        day_to_run.solve()
