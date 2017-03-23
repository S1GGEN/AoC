f = open("Day08_input.txt", 'r')

aoc_inp = f.read().split('\n')

screen = []

for _ in range(6):
    screen.append(50*[0])


def show_screen():
    for row in screen:
        print(row)


show_screen()

for line in aoc_inp:
    print("--------------------------------------------------------------------------------------------")
    print(line)
    if line.find("rect") > -1:
        print("GET REKT")
        print(line[5:])
        a = int(line[5:line.find("x")])
        b = int(line[line.find("x") + 1:])
        print(a)
        print("x")
        print(b)
        for i in range(b):
            print(i)
            print(screen[i])
            print(screen[i][:a])
            print(["#"]*a)
            screen[i][0:a] = ["#"]*a
        show_screen()
    else:
        print("rotate")
