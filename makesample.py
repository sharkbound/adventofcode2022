from itertools import count

from utils import ask_int, get_day_folder_path


def main():
    day = ask_int('enter day to create sample file for >>> ')
    day_folder_path = get_day_folder_path(day)

    if not day_folder_path.exists():
        print(f'path ({day_folder_path}) does not exist! terminating...')
        return

    for i in count(0):
        if not (file := day_folder_path / f'sample_{i}.txt').exists():
            file.write_text('')
            print(f'created sample file "{file}"')
            return


main()
