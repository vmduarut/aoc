# /// script
# requires-python = ">=3.14"
# dependencies = []
# ///
from pathlib import Path

def check_number(id: int) ->  bool:
    id = str(id)
    for size in range(1, len(id)):
        valid = True
        if len(id) % size == 0:
            values = []
            pattern = id[0:size]
            for i in range(0, len(id), size):
                if id[i:i+size] != pattern:
                    valid = False
                    break
            if valid:
                return True
    return False
    

def part2():
    input_path = Path("inputs/day2.txt")

    result = 0
    with input_path.open() as file:
        for line in file:
            line = line.strip()
            if not line:
                continue
            intervals = line.split(",")
            for interval in intervals:
                if not interval:
                    continue
                if "-" not in interval:
                    raise Exception(f"{interval} does not have -")
                numbers = interval.split("-")
                if len(numbers) != 2:
                    raise Exception(f"{numbers}")
                first = int(numbers[0])
                last = int(numbers[1])
                for id in range(first, last + 1):
                    if check_number(id):
                        print(id)
                        result += int(id)
                        
    print(f"Result is {result}")
                        
    
def part1():
    input_path = Path("inputs/day2.txt")

    result = 0
    with input_path.open() as file:
        for line in file:
            line = line.strip()
            if not line:
                continue
            intervals = line.split(",")
            for interval in intervals:
                if not interval:
                    continue
                if "-" not in interval:
                    raise Exception(f"{interval} does not have -")
                numbers = interval.split("-")
                if len(numbers) != 2:
                    raise Exception(f"{numbers}")
                first = int(numbers[0])
                last = int(numbers[1])
                for id in range(first, last + 1):
                    id = str(id)
                    if len(id) % 2 != 0:
                        continue
                    middle = len(id) // 2
                    left = id[:middle]
                    right = id[middle:]
                    if left == right:
                        print(id)
                        result += int(id)
    print(f"Result is {result}")
                

def main() -> None:
    print("Hello from day2.py!")
    part2()


if __name__ == "__main__":
    main()
