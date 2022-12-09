from enum import IntFlag, auto
from pathlib import Path

from icecream import ic

from day import Day
import utils


class Type:
    COMMAND_CD = object()
    COMMAND_LS = object()
    PENDING_ADDITION = object()


DISK_SPACE = 70000000
REQUIRED_UPDATE_DISKS_SPACE = 30000000


class Day7Part1(Day):
    day = 7
    part = 1

    def get_sample_input(self):
        return ''

    def parse_input(self):
        return [
            [(int(x) if x.isnumeric() else x) for x in line.split()]
            for line in self.input_text_lines
        ]

    def process_cmd_cd(self, target: str, current_path: Path) -> Path:
        match target:
            case '/':
                return Path('/')
            case '..':
                if current_path.name != '/':
                    current_path = current_path.parent
                return current_path
            case child_dir:
                return current_path / child_dir

    def process_cmd_ls(self, data: list[list], i: int, current_path: Path):
        """
        :return: (index to resume at, files listed, sum of file sizes)
        """
        files = set()
        total_size = 0
        while i < len(data):
            match data[i]:
                case ['$', *_]:
                    return i, files, total_size
                case [int() as size, str() as filename]:
                    total_size += size
                    files.add((size, current_path / filename))
                case ['dir', foldername]:
                    files.add((Type.PENDING_ADDITION, current_path, current_path / foldername))
            i += 1

        return i, files, total_size

    def process_generic_command(self, data: list[list], i: int, command: list[str], current_path: Path):
        match command[1]:
            case 'cd':
                return self.process_cmd_cd(command[2], current_path), Type.COMMAND_CD
            case 'ls':
                return current_path, Type.COMMAND_LS, *self.process_cmd_ls(data, i + 1, current_path)
        return current_path,

    def _append_pending_additions(self, ls_results):
        # reverse the output so innermost folders are calculated first
        for folder_data in reversed(ls_results.values()):
            for file in tuple(folder_data['files']):
                if file[0] is Type.PENDING_ADDITION:
                    # file[1]: the directory that will be extended
                    # file[2]: the directory whose files will be added to file[1]
                    folder_data['files'].update(ls_results[file[2]]['files'])

            folder_data['files'] = {x for x in folder_data['files'] if x[0] is not Type.PENDING_ADDITION}
            folder_data['size'] = sum(x[0] for x in folder_data['files'])

    def size_of(self, ls_results, path: Path) -> int:
        return ls_results[path]['size']

    def solve(self):
        data = self.parse_input()
        current_path = Path('/')
        ls_results = {}
        i = 0
        while i < len(data):
            line = data[i]
            rest = []
            if line[0] == '$':
                current_path, *rest = self.process_generic_command(data, i, line, current_path)

            if not rest:
                i += 1

            match rest:
                case [Type.COMMAND_LS, index, files, total_size]:
                    i = index

                    ls_results[current_path] = {'files': files, 'size': total_size, 'dir': current_path}
                    continue

                case [Type.COMMAND_CD]:
                    pass

            i += 1

        self._append_pending_additions(ls_results)
        self.print_answer(sum(folder['size'] for folder in ls_results.values() if folder['size'] <= 100000))
