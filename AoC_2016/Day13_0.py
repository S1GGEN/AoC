f = open("Day13_input.txt", 'r')

aoc_inp = f.read()
f.close()

floor_map = []

for _ in range(50):
    floor_map.append(50*["."])

floor_map[39][31] = "|"


def show_map(f_m):
    col_numbers0 = "   "
    col_numbers1 = "   "
    for i in range(len(f_m[0])):
        if i < 10:
            col_numbers0 += " "
            col_numbers1 += str(i)
        else:
            col_numbers0 += str(i)[0]
            col_numbers1 += str(i)[1]
    # print(" ", "_"*len(f_m[0]))
    print(col_numbers0)
    print(col_numbers1)
    for r in range(len(f_m)):
        if r < 10:
            row_show = " " + str(r) + "|"
        else:
            row_show = str(r) + "|"
        for col in f_m[r]:
            row_show += col
        row_show += "|"
        print(row_show)
    print("  ", "¯" * len(f_m[0]))

show_map(floor_map)


def populate_map(f_m):
    for y in range(len(f_m)):
        for x in range(len(f_m[0])):
            # print("x:", x, "y:", y)
            if not is_open(x, y):
                f_m[y][x] = "#"


def is_open(x, y):
    dec_num = x*x + 3*x + 2*x*y + y + y*y + int(aoc_inp)
    # print("dec_num:", dec_num)
    bin_num = bin(dec_num)[2:]
    # print("bin_num:", bin_num)
    num_1 = str(bin_num).count('1')
    # print("This number of 1s:", num_1)

    if num_1 % 2 == 0:
        # print("open")
        return True
    else:
        # print("wall")
        return False


populate_map(floor_map)
show_map(floor_map)
