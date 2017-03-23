

def calculate_distance(input):
    north = 0
    west = 0
    heading = 'N'

    instructions = input.split(', ')
    for inst in instructions:
        if 'L' in inst:
            if heading == 'N':
                heading = 'W'
            elif heading == 'W':
                heading = 'S'
            elif heading == 'S':
                heading = 'E'
            else:
                heading = 'N'

        else:
            if heading == 'N':
                heading = 'E'
            elif heading == 'E':
                heading = 'S'
            elif heading == 'S':
                heading = 'W'
            else:
                heading = 'N'

        steps = inst.replace('L', '')
        steps = int(steps.replace('R', ''))

        if heading == 'N':
            north += steps
        elif heading == 'S':
            north -= steps
        elif heading == 'W':
            west += steps
        else:
            west -= steps

    print('North:', north)
    print('West:', west)
    print('Distance:', abs(north) + abs(west))

aoc_input = 'L3, R1, L4, L1, L2, R4, L3, L3, R2, R3, L5, R1, R3, L4, L1, L2, R2, R1, L4, L4, R2, L5, R3, R2, R1, L1, L2, R2, R2, L1, L1, R2, R1, L3, L5, R4, L3, R3, R3, L5, L190, L4, R4, R51, L4, R5, R5, R2, L1, L3, R1, R4, L3, R1, R3, L5, L4, R2, R5, R2, L1, L5, L1, L1, R78, L3, R2, L3, R5, L2, R2, R4, L1, L4, R1, R185, R3, L4, L1, L1, L3, R4, L4, L1, R5, L5, L1, R5, L1, R2, L5, L2, R4, R3, L2, R3, R1, L3, L5, L4, R3, L2, L4, L5, L4, R1, L1, R5, L2, R4, R2, R3, L1, L1, L4, L3, R4, L3, L5, R2, L5, L1, L1, R2, R3, L5, L3, L2, L1, L4, R4, R4, L2, R3, R1, L2, R1, L2, L2, R3, R3, L1, R4, L5, L3, R4, R4, R1, L2, L5, L3, R1, R4, L2, R5, R4, R2, L5, L3, R4, R1, L1, R5, L3, R1, R5, L2, R1, L5, L2, R2, L2, L3, R3, R3, R1'


calculate_distance(aoc_input)
