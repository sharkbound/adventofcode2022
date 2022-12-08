from enum import IntFlag, auto
from pathlib import Path

from icecream import ic

from day import Day
import utils


class Type:
    COMMAND_CD = object()
    COMMAND_LS = object()
    PENDING_ADDITION = object()


class Day7Part1(Day):
    day = 7
    part = 1

    def get_sample_input(self):
        return ''

    def parse_input(self):
        return [
            [(int(x) if x.isnumeric() else x) for x in line.split()]
            for line in self.read_sample_file_lines(0)
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
        files = []
        total_size = 0
        while i < len(data):
            match data[i]:
                case ['$', *_]:
                    return i, files, total_size
                case [int() as size, str() as filename]:
                    total_size += size
                    files.append((size, current_path / filename))
                case ['dir', foldername]:
                    files.append((Type.PENDING_ADDITION, current_path, current_path / foldername))
            i += 1

        return i, files, total_size

    def process_generic_command(self, data: list[list], i: int, command: list[str], current_path: Path):
        match command[1]:
            case 'cd':
                return self.process_cmd_cd(command[2], current_path), Type.COMMAND_CD
            case 'ls':
                return current_path, Type.COMMAND_LS, *self.process_cmd_ls(data, i + 1, current_path)
        return current_path,

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
                    print(f'[LS ({current_path})]\n\tindex: {index}\n\t{files}\n\tsize ({current_path}): {total_size}')
                    i = index
                    ls_results[current_path] = {'files': files, 'size': total_size, 'dir': current_path}
                    continue

                case [Type.COMMAND_CD]:
                    print(f'[CD] {current_path}')

            i += 1
            ic(ls_results)
            for key, extra in ls_results.items():
                for file in extra['files']:
                    ic(file)
                    if file[0] is Type.PENDING_ADDITION:
                        print(f'[ADDITION (to {file[1]})] {file[2]}')
                        extra['files'].extend(ls_results[file[2]])
                        extra['size'] += ls_results[file[2]]['size']


"""
The total sizes of the directories above can be found as follows:

    The total size of directory e is 584 because it contains a single file i of size 584 and no other directories.
    The directory a has total size 94853 because it contains files f (size 29116), g (size 2557), and h.lst (size 62596), plus file i indirectly (a contains e which contains i).
    Directory d has total size 24933642.
    As the outermost directory, / contains every file. Its total size is 48381165, the sum of the size of every file.
"""
