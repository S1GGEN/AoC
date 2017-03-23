import copy

f = open("Day08_input.txt", 'r')

aoc_inp = f.read().split('\n')

screen = []

for _ in range(6):
    screen.append(50*["~"])


def show_screen(s):
    print(" __________________________________________________")
    for r in s:
        row_show = "|"
        for col in r:
            row_show += col
        row_show += "|"
        print(row_show)
    print(" ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯")


show_screen(screen)


def rotate(length, r, c, by, direc, scr):
    new_screen = copy.deepcopy(scr)
    for i in range(length):
        print("NY")
        print("i:", i)
        a = int(str(i))
        by %= length
        if a + by >= length:
            a = a + by - length
        else:
            a = a + by
        print("a:", a)
        if direc == "x":
            print(new_screen[a][c])
            print(scr[i][c])
            new_screen[a][c] = str(scr[i][c])
            print(new_screen[a][c])
        else:
            new_screen[r][a] = str(scr[r][i])
        print(scr)
        print("i:", i)
        print("OLD:")
        print(screen)
        print("NEW:")
        print(new_screen)
        print("OLD:")
        show_screen(screen)
        print("NEW:")
        show_screen(new_screen)

    return new_screen


for line in aoc_inp:
    print("--------------------------------------------------------------------------------------------")
    print(line)
    if line.find("rect") > -1:
        print("GET REKT")
        a = int(line[5:line.find("x")])
        b = int(line[line.find("x") + 1:])
        for i in range(b):
            screen[i][0:a] = ["#"]*a
        show_screen(screen)
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

        print(screen)

        if direction == "x":
            a = len(screen)
            print(a)
            screen = rotate(a, row, column, by, direction, screen)
            show_screen(screen)
        else:
            a = len(screen[0])
            print(a)
            screen = rotate(a, row, column, by, direction, screen)
            show_screen(screen)

        # new_screen[0][0:] = ['æ']*50
        print(screen)

        # screen = new_screen
        show_screen(screen)
