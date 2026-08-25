# /// script
# requires-python = ">=3.14"
# dependencies = []
# ///
from pathlib import Path

def part2():
    input_path = Path("inputs/day3.txt")

    result = 0
    joltage_length = 12
    with input_path.open() as file:
        for bank in file:
            bank = bank.strip()
            if not bank:
                continue
            joltage = []
            current_pos = 0
            numbers_still = joltage_length
            length_missing = len(bank)
            for _ in range(joltage_length):
                window = length_missing - numbers_still + 1
                search = bank[current_pos:current_pos+window]

                max_number = max(search)
                max_index = search.index(max_number)

                joltage.append(max_number)

                current_pos += + max_index + 1
                numbers_still -= 1
                length_missing -= max_index + 1
            print(joltage)
            joltage = "".join(joltage)
            result += int(joltage)
                

        print(f"Result is {result}")
    

def part1():
    input_path = Path("inputs/day3.txt")

    result = 0
    with input_path.open() as file:
        for bank in file:
            bank = bank.strip()
            if not bank:
                continue
            print(bank)
            joltage = int(f"{bank[0]}{bank[1]}")
            print(f"Initial joltage: {joltage}")
            for i in range(0, len(bank)):
                for y in range(i+1, len(bank)):
                    possible_joltage = int(f"{bank[i]}{bank[y]}")
                    if possible_joltage > joltage:
                        joltage = possible_joltage

            print(joltage)
            result += joltage
                    
        print(f"Result is {result}")
                
            
    
    
def main() -> None:
    print("Hello from day3.py!")
    part2()


if __name__ == "__main__":
    main()
