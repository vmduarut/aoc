# /// script
# requires-python = ">=3.14"
# dependencies = []
# ///
from pathlib import Path

def part2() -> None:
    current_pos = 50
    result = 0
    print("Hello from day1.py!")
    input = Path("inputs/day1.txt")
    with input.open() as file:
        for line in file:
            instruction = line.strip()
            direction = instruction[0]
            number = int(instruction[1:])
            if direction not in ("L", "R"):
                print("wrong")
            if direction == "R":
                current_pos = current_pos + number
                if current_pos > 99:
                    result += current_pos // 100
                    current_pos = current_pos % 100
            elif direction == "L":
                new_pos = current_pos - number
                if new_pos == 0:
                    result += 1
                if new_pos < 0:
                    result += (abs(new_pos) // 100)
                    if current_pos != 0:
                        result += 1
                current_pos = new_pos % 100
    
    print(f"Result is: {result}")

def part1() -> None:
    current_pos = 50
    result = 0
    print("Hello from day1.py!")
    input = Path("inputs/day1.txt")
    with input.open() as file:
        for line in file:
            instruction = line.strip()
            direction = instruction[0]
            number = int(instruction[1:])
            if direction not in ("L", "R"):
                print("wrong")
            if direction == "R":
                current_pos = current_pos + number
                if current_pos > 99:
                    current_pos = current_pos % 100
            if direction == "L":
                current_pos = current_pos - number
                if current_pos < 0:
                    current_pos = current_pos % 100

            print(f"Current position is {current_pos}")
            result += 1 if current_pos == 0 else 0

    print(f"Result is: {result}")

def main() -> None:
            
    # part1()
    part2()


if __name__ == "__main__":
    main()
