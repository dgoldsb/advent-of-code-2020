from typing import List

from aocd.models import Puzzle

from util import *


def part_a(inputs: List[int]):
    for x in inputs:
        for y in inputs:
            if x + y == 2020:
                answer = x * y
                print(f"Answer is {answer}!")
                return answer


def part_b(inputs: List[int]):
    for x in inputs:
        for y in inputs:
            for z in inputs:
                if x + y + z == 2020:
                    answer = x * y * z
                    print(f"Answer is {answer}!")
                    return answer


puzzle = Puzzle(year=2020, day=1)
puzzle_inputs = [int(i) for i in puzzle.input_data.split("\n")]
puzzle.answer_a = part_a(puzzle_inputs)
puzzle.answer_b = part_b(puzzle_inputs)
