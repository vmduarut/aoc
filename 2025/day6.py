# /// script
# requires-python = ">=3.14"
# dependencies = []
# ///
from math import prod
from pathlib import Path


def part2():
    input_path = Path("inputs/day6.txt")
    grid = []
    result = 0
    with input_path.open() as file:
        lines = [line.rstrip("\n") for line in file]
        width = max(map(len, lines))
        lines = [line.ljust(width) for line in lines]
    rows = lines[:-1]
    operation = lines[-1].split()
    problem = []
    numbers = []
    for i in range(len(rows[0]) - 1, -1, -1):
        number = ""
        for x in range(len(rows)):
            if rows[x][i] != "":
                number += rows[x][i]
        if number.strip():
            numbers.append(int(number))
            if i == 0:
                problem.append(numbers)

        else:
            problem.append(numbers)
            numbers = []

    result = 0
    operation.reverse()
    for i in range(len(problem)):
        current_op = operation[i]
        result += prod(problem[i]) if current_op == "*" else sum(problem[i])

    print(f"Result is: {result}")


def part1():
    input_path = Path("inputs/day6.txt")
    grid = []
    result = 0
    with input_path.open() as file:
        for line in file:
            line = line.rstrip()
            print(line)
            grid.append(line.split())
    print(grid)
    for i in range(len(grid[0])):
        operation = grid[-1][i]
        problem = [int(x[i]) for x in grid[:-1]]
        print(problem)
        result += prod(problem) if operation == "*" else sum(problem)
    print(f"Result is: {result}")


def main() -> None:
    print("Hello from day6.py!")
    part2()


if __name__ == "__main__":
    main()
