import time

f = open("Day13_input.txt", 'r')

aoc_inp = f.read()
f.close()

floor_map = []

for _ in range(50):
    floor_map.append(50*["."])

floor_map[39][31] = "|"

# visited = {(1, 1)}
# visited.update([(1, 1)])

visited = [[1, 1]]
explorable = 0
steps = 0
part2 = 0


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


def legal_moves(x, y):
    moves = [[0, 1], [1, 0], [0, -1], [-1, 0]]
    legals = []
    for move in moves:
        new_pos = [x + move[0], y + move[1]]
        if is_open(new_pos[0], new_pos[1]):
            if new_pos[0] > -1 and new_pos[1] > -1:
                legals.append([new_pos[0], new_pos[1]])
    return legals


def populate_map(f_m):
    for y in range(len(f_m)):
        for x in range(len(f_m[0])):
            if not is_open(x, y):
                f_m[y][x] = "#"


def is_open(x, y):
    dec_num = x*x + 3*x + 2*x*y + y + y*y + int(aoc_inp)
    bin_num = bin(dec_num)[2:]
    return str(bin_num).count('1') % 2 == 0


def crawl():
    print(visited)
    visited2 = visited.copy()
    # print(visited2)
    global steps
    steps += 1
    print("steps:", steps)
    for loc in visited:
        # print("a")
        # print(loc)
        move_to = legal_moves(loc[0], loc[1])
        print("len(move_to):", len(move_to))
        for move in move_to:
            if move not in visited2:
                # print("move[0]:", move[0])
                # print("move[1]:", move[1])
                visited2.append(move)
                if steps < 50:
                    global part2
                    part2 += 1
                global explorable
                explorable += 1

    if len(visited2) > len(visited):
        visited.append(visited2[-int(len(visited2) - len(visited))])
        time.sleep(2)
        crawl()


show_map(floor_map)
populate_map(floor_map)
show_map(floor_map)
print(legal_moves(0, 1))
crawl()
print(visited)
print("part2:", part2)
print("explorable:", explorable)