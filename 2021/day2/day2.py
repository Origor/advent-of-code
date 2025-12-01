def steer_sub(input_tuple, aim_enabled):
    direction, depth = [], []
    aim = 0
    for current_tuple in input_tuple:
        if(current_tuple[0] == 'forward'):
            y = int(current_tuple[1])
            direction.append(y)
            if (aim_enabled):
                depth.append(aim * y)
        elif(current_tuple[0] == 'down'):
            if (aim_enabled == False):
                depth.append(int(current_tuple[1]))
            aim += int(current_tuple[1])
        else:
            if (aim_enabled == False):
                depth.append(-int(current_tuple[1]))
            aim -= int(current_tuple[1])
    return sum(direction)*(sum(depth))
    
def parse_input():
    with open("input.txt", "r", encoding="utf-8") as inputFile:
        return [tuple(map(str, i.strip().split(' '))) for i in inputFile]

def main():
    #print(parse_input())
    print(steer_sub(parse_input(), False))
    print(steer_sub(parse_input(), True))

if __name__ == "__main__":
    main()
