input_file = "./input1.txt"
dial_position = 50
zero_count = 0

def process_line(line):
    global dial_position
    global zero_count
    rotation_direction = line[:1]
    rotation_number = int(line[1:])
    if rotation_direction == "L":
        for i in range(1, rotation_number + 1):
            if dial_position == 0:
                dial_position = 99
            else:
                dial_position -= 1
                if dial_position == 0:
                    zero_count += 1
    elif rotation_direction == "R":
        for i in range(1, rotation_number + 1):
            if dial_position == 99:
                dial_position = 0
                zero_count += 1
            else:
                dial_position += 1

    print(dial_position)
    
with open(input_file) as f:
    for line in f.readlines():
        process_line(line)
    print('ZERO COUNT IS:', zero_count)
