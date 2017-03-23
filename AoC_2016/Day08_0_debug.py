import copy

f = open("Day08_input.txt", 'r')

aoc_inp = f.read().split('\n')

screen = []

for _ in range(6):
    screen.append(50*["~"])


def show_screen():
    print(" __________________________________________________")
    for row in screen:
        row_show = "|"
        for col in row:
            row_show += col
        row_show += "|"
        print(row_show)
    print(" ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯")


show_screen()

for line in aoc_inp:
    print("--------------------------------------------------------------------------------------------")
    print(line)
    if line.find("rect") > -1:
        print("GET REKT")
        a = int(line[5:line.find("x")])
        b = int(line[line.find("x") + 1:])
        for i in range(b):
            screen[i][0:a] = ["#"]*a
        show_screen()
    else:
        print("rotate")
        if line.find("row") > -1:
            row = int(line[13:line.find(" by ")])
        else:
            row = 0
        if line.find("column") > -1:
            column = int(line[16:line.find(" by ")])
        else:
            column = 0
        by = int(line[line.find(" by ") + 3:])

        if line.find("x") > -1:
            direction = "x"
        else:
            direction = "y"

        print("row:", row)
        print("column:", column)
        print("direction:", direction)
        print("by:", by)

        new_screen = copy.deepcopy(screen)
        print(screen)

        if direction == "x":
            for i in range(len(screen)):
                print("NY")
                print("i:", i)
                a = int(str(i))
                by %= len(screen)
                if a + by >= len(screen):
                    a = a + by - len(screen)
                else:
                    a = a + by
                print("a:", a)
                new_screen[a][column] = str(screen[i][column])
                print(screen)
                print("i:", i)
        else:
            for i in range(len(screen[0])):
                print("NY")
                print("i:", i)
                a = int(str(i))
                by %= len(screen[0])
                if a + by >= len(screen[0]):
                    a = a + by - len(screen[0])
                else:
                    a = a + by
                print("a:", a)
                new_screen[row][a] = str(screen[row][i])
                print(screen)
                print("i:", i)

        # new_screen[0][0:] = ['æ']*50
        print(screen)
        print(new_screen)
        screen = new_screen
        show_screen()
