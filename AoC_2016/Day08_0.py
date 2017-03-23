import copy


def show_screen(s):
    print(" __________________________________________________")
    for r in s:
        row_show = "|"
        for col in r:
            row_show += col
        row_show += "|"
        print(row_show)
    print(" ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯")


def number_of_pixels(s):
    number = 0
    for r in s:
        for col in r:
            if col == "#":
                number += 1
    print(number, "pixels")


def rotate(length, r, c, shift, direc, scr):
    new_screen = copy.deepcopy(scr)
    for i in range(length):
        a = int(str(i))
        shift %= length
        if a + shift >= length:
            a = a + shift - length
        else:
            a = a + shift
        if direc == "x":

            new_screen[a][c] = str(scr[i][c])
        else:
            new_screen[r][a] = str(scr[r][i])
    return new_screen


def main():
    f = open("Day08_input.txt", 'r')

    aoc_inp = f.read().split('\n')
    f.close()
    screen = []

    for _ in range(6):
        screen.append(50 * [" "])

    show_screen(screen)

    for line in aoc_inp:
        print("--------------------------------------------------------------------------------------------")
        print(line)
        if line.find("rect") > -1:
            for i in range(int(line[line.find("x") + 1:])):
                screen[i][0:int(line[5:line.find("x")])] = ["#"]*int(line[5:line.find("x")])
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

            if direction == "x":
                screen = rotate(len(screen), row, column, by, direction, screen)
            else:
                screen = rotate(len(screen[0]), row, column, by, direction, screen)

            show_screen(screen)
            number_of_pixels(screen)
main()
