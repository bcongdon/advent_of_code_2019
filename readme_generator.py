#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
import itertools

day_labels = [
    ":santa:",
    ":star2:",
    ":snowflake:",
    ":snowman:",
    ":sparkles:",
    ":fire:",
    ":christmas_tree:",
    ":gift:",
    ":bell:",
    ":tada:",
]

folders = [(10, "Day1-9"), (20, "Day10-19"), (26, "Day20-25")]

extensions = {
    "py": "Python",
    "s": "MIPS",
    "hs": "Haskell",
    "cpp": "C++",
    "c": "C",
    "rb": "Ruby",
    "swift": "Swift",
    "java": "Java",
    "js": "Javascript",
    "go": "Go",
    "rs": "Rust",
    "fs": "F#",
}

TEMPLATE = """# advent_of_code_2019
🎅 My solutions to the 2019 "Advent of Code"


## Solutions

{}"""


def files_for_day(day):
    fdr = next(x[1] for x in folders if x[0] > i)
    if not os.path.isdir(fdr):
        return

    for fn in os.listdir(fdr):
        try:
            name, ext = os.path.splitext(fn)
            day = next(
                int(name[i:]) for i in range(len(name)) if name[i:].isdigit()
            )
        except StopIteration:
            pass
        # print("Coundn't parse: %s" % fn)
        except ValueError:
            pass
        else:
            if ext[1:] in extensions and day == i:
                yield os.path.join(fdr, fn)


def pretty_extension_name(path):
    _, ext = os.path.splitext(path)
    return extensions[ext[1:]]


if __name__ == "__main__":
    out = ""
    soln_temp = "    * [{}]({})\n"
    symbols = itertools.cycle(list(day_labels))
    for i in range(1, 26):
        header = "* Day {}:  {}\n".format(i, next(symbols))
        solution_files = files_for_day(i)

        tmp = ''
        for s in solution_files:
            sol_type = pretty_extension_name(s)
            tmp += soln_temp.format(sol_type, s)

        if tmp != '':
            out += header
            out += tmp
    print(TEMPLATE.format(out))
