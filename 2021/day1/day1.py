# O(n) time, no extra space
def solve(data, window=1):
    return sum(curr > prev for prev, curr in zip(data, data[window:]))

def part_a(data):
    return solve(data, 1)

def part_b(data):
    return solve(data, 3)

def alternative_part_a():
    with open("example_input.txt", "r", encoding="utf-8") as f:
        lines = f.readlines()

    c = 0
    for i in range(len(lines)):
        if i == 0:
            print(lines[i][:-1], "(N/A)", c)
        elif lines[i - 1] < lines[i]:
            c += 1
            print(lines[i][:-1], "(increased)", c)
        else:
            print(lines[i][:-1], "(decreased)", c)
    return c

def main():
    with open("input.txt", "r", encoding="utf-8") as inputFile:
        data = list(map(int, inputFile.readlines()))

    print(part_a(data))
    print(part_b(data))
    print(alternative_part_a())

if __name__ == "__main__":
    main()
