import time
f = open("Day15_input.txt", 'r')

aoc_inp = f.read().split("\n")
f.close()


def initialize_disks():
    d = []
    for disc in aoc_inp:
        # print("-----------------------------")
        positions = int(disc[disc.find("as") + 3:disc.find("pos") - 1])
        # print(positions)
        start_pos = int(disc[disc.rfind("on") + 3:-1])
        # print(start_pos)
        d.append([positions, start_pos])
    return d

discs = initialize_disks()
discs.append([11, 0])
print(discs)

clock = 0


def update_disc(d):
    if d[1] == d[0] - 1:
        return 0
    else:
        return d[1] + 1


def update_disc_n(d, n):
    discpos = d[1]
    for _ in range(n):
        discpos = update_disc([d[0], discpos])
    return discpos


def update_discs(ds):
    for disc in ds:
        disc[1] = update_disc(disc)


def check_alignment(ds):
    for i in range(len(ds)):
        if update_disc_n(ds[i], i+1) != 0:
            return False
    return True

while True:
    update_discs(discs)
    clock += 1
    # print("clock:", clock)
    # print(discs)
    if check_alignment(discs):
        print("HOORAY!")
        print("clock:", clock)
        print(discs)
    # time.sleep(1)
