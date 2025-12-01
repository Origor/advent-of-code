def diagnostics(input):
    input_tuple = zip(*input)

    gamma = ''
    epsilon = ''
    for i in input_tuple: 
        counter = 0
        for j in i:
            if j == '1': 
                counter += 1
        if counter > len(i)/2:
            gamma += '1'
            epsilon += '0'
            continue
        gamma += '0'
        epsilon += '1'
    
    return int(gamma, 2)*int(epsilon, 2)

    
def parse_input():
    with open("example_input.txt", "r", encoding="utf-8") as inputFile:
        return [list(line.rstrip('\n')) for line in inputFile]
    
def main():
    print(diagnostics(parse_input()))

if __name__ == "__main__":
    main()
