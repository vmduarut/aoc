# /// script
# requires-python = ">=3.14"
# dependencies = []
# ///
from dataclasses import dataclass
from pathlib import Path
from typing import Self


@dataclass(frozen=True)
class Interval:
    start: int
    end: int

    def overlaps(self, other: Self) -> bool:
        return other.start <= self.end

    def merge(self, other: Self) -> Self:
        return Interval(
            start=min(self.start, other.start), end=max(self.end, other.end)
        )

    def contains(self, id: int) -> bool:
        return self.start <= id <= self.end


def part2():
    input_path = Path("inputs/day5.txt")
    intervals: list[Interval] = []
    change = False
    result = 0
    with input_path.open() as file:
        for line in file:
            line = line.rstrip()
            print(line)
            if not line:
                break
            start, end = line.split("-")
            intervals.append(Interval(start=int(start), end=int(end)))

    sorted_intervals = sorted(intervals, key=lambda x: x.start)
    merged = [sorted_intervals[0]]
    for current in sorted_intervals[1:]:
        previous = merged[-1]
        if previous.overlaps(current):
            merged[-1] = previous.merge(current)
        else:
            merged.append(current)

    print(merged)
    result = sum([x.end - x.start + 1 for x in merged])
    print(f"Result is: {result}")


def part1():
    input_path = Path("inputs/day5.txt")
    intervals: list[Interval] = []
    possible_ids: list[int] = []
    change = False
    result = 0
    with input_path.open() as file:
        for line in file:
            line = line.rstrip()
            print(line)
            if not line:
                change = True
                continue
            if not change:
                start, end = line.split("-")
                intervals.append(Interval(start=int(start), end=int(end)))
            if change:
                possible_ids.append(int(line))

    sorted_intervals = sorted(intervals, key=lambda x: x.start)
    merged = [sorted_intervals[0]]
    for current in sorted_intervals[1:]:
        previous = merged[-1]
        if previous.overlaps(current):
            merged[-1] = previous.merge(current)
        else:
            merged.append(current)

    for id in possible_ids:
        for interval in merged:
            if interval.contains(id):
                result += 1
                break
    print(f"Result is: {result}")


def main() -> None:
    print("Hello from day5.py!")
    part2()


if __name__ == "__main__":
    main()
