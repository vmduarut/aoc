# /// script
# requires-python = ">=3.14"
# dependencies = []
# ///
from pathlib import Path


# def is_rollpaper(pos: str) -> bool:
#     if pos == "@"
def check_rollpaper(grid: list[str], row: int, col: int) -> bool:
    count = 0
    for i in [-1, 0, 1]:
        for y in [-1, 0, 1]:
            new_row = row + i
            new_col = col + y
            if (
                (0 <= new_row < len(grid))
                and (0 <= new_col < len(grid[new_row]))
                and ((i, y) != (0, 0))
            ) and grid[new_row][new_col] == "@":
                count += 1
            if count >= 4:
                return False
    return True


def get_grid(input_path: Path) -> list[list[str]]:
    with input_path.open() as file:
        return [list(line.rstrip()) for line in file]


def part1():
    input_path = Path("inputs/day4.txt")
    grid = get_grid(input_path)
    result = 0
    for row, line in enumerate(grid):
        for col, pos in enumerate(line):
            if pos == "@" and check_rollpaper(grid, row, col):
                result += 1
    print(f"Result is {result}")


def part2():
    input_path = Path("inputs/day4.txt")
    grid = get_grid(input_path)
    print(f"Grid: {grid}")
    result = 0
    while True:
        #     miss_rollpaper = True
        to_remove = []
        for row, line in enumerate(grid):
            for col, pos in enumerate(line):
                if pos == "@" and check_rollpaper(grid, row, col):
                    to_remove.append((row, col))
                    # miss_rollpaper = False
                    # result += 1
                    # grid[row][col] = "."
        if not to_remove:
            break
        for row, col in to_remove:
            grid[row][col] = "."
        result += len(to_remove)
        to_remove = []
    print(f"Grid: {grid}")
    print(f"Result is {result}")


def main() -> None:
    print("Hello from day4.py!")
    part2()


if __name__ == "__main__":
    main()
