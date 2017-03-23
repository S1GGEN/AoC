

def calculate_distance(input):
    north = 0
    west = 0
    heading = 'N'

    instructions = input.split(', ')
    for inst in instructions:
        print(inst)
        if 'L' in inst:
            print('L')
            if heading == 'N':
                heading = 'W'
            elif heading == 'W':
                heading = 'S'
            elif heading == 'S':
                heading = 'E'
            else:
                heading = 'N'

        else:
            print('R')

            if heading == 'N':
                heading = 'E'
            elif heading == 'E':
                heading = 'S'
            elif heading == 'S':
                heading = 'W'
            else:
                heading = 'N'

        print("heading:", heading)
        steps = inst.replace('L', '')
        steps = int(steps.replace('R', ''))
        print(steps)

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


calculate_distance('R5, L5, R5, R3')

