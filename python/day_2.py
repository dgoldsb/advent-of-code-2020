import re
from typing import List, Tuple

from aocd.models import Puzzle

from util import *


DatabaseEntry = Tuple[int, int, str, str]


def parse_line(line) -> DatabaseEntry:
    match = re.match(r"(\d+)-(\d+) (.): (.+)", line)
    if match:
        groups = match.groups()
        return int(groups[0]), int(groups[1]), groups[2], groups[3]
    else:
        raise RuntimeError(f"Could not parse {line}!")


def part_a(inputs: List[DatabaseEntry]) -> int:
    return sum([e[0] <= e[3].count(e[2]) <= e[1] for e in inputs])


def part_b(inputs: List[DatabaseEntry]) -> int:
    return sum([
        (e[3][e[0] - 1] == e[2]) is not (e[3][e[1] - 1] == e[2]) for e in inputs
    ])


puzzle = Puzzle(year=2020, day=2)
puzzle_inputs = [parse_line(l) for l in puzzle.input_data.split("\n")]
puzzle.answer_a = part_a(puzzle_inputs)
puzzle.answer_b = part_b(puzzle_inputs)
