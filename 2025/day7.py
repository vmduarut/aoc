# /// script
# requires-python = ">=3.14"
# dependencies = []
# ///
from functools import cache
from pathlib import Path


def part2():
    input_path = Path("inputs/day7_test.txt")
    result = 0
    with input_path.open() as file:
        grid = [line.rstrip("\n") for line in file]

    found = False
    nrows = len(grid)
    ncols = len(grid[0])
    print(grid)
    for row in range(nrows):
        for col in range(ncols):
            if grid[row][col] == "S":
                initial_pos = (row, col)
                found = True
                break
        if found:
            break

    @cache
    def paths(row: int, col: int) -> int:
        if row == len(grid):
            return 1
        if grid[row][col] == "^":
            return paths(row + 1, col - 1) + paths(row + 1, col + 1)
        else:
            return paths(row + 1, col)

    print(initial_pos)
    result = paths(initial_pos[0], initial_pos[1])

    print(f"Result is {result}")


def part1():
    input_path = Path("inputs/day7.txt")
    result = 0
    with input_path.open() as file:
        grid = [line.rstrip("\n") for line in file]

    found = False
    nrows = len(grid)
    ncols = len(grid[0])
    print(grid)
    for row in range(nrows):
        for col in range(ncols):
            if grid[row][col] == "S":
                initial_pos = col
                found = True
                break
        if found:
            break
    current_pos = {initial_pos}
    print(current_pos)
    for row in grid:
        next_beam = set()
        for col in current_pos:
            if row[col] == "^":
                result += 1
                next_beam.add(col + 1)
                next_beam.add(col - 1)
            else:
                next_beam.add(col)
        current_pos = next_beam

    print(f"Result is {result}")


def main() -> None:
    print("Hello from day7.py!")
    part2()


if __name__ == "__main__":
    main()
