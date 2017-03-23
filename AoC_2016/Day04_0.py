f = open("Day04_input.txt", 'r')

aoc_inp = f.read().split('\n')


def char_sum(s):
    f = [0] * 26
    checksum = ""
    for c in s:
        if c != "-":
            f[ord(c) - ord('a')] += 1

    i = max(f)

    while i > 0:
        for g in range(len(f)):
            if f[g] == i:
                checksum += chr(g + ord('a'))
        i -= 1
    checksum = checksum[:5]
    return checksum


def find_valid():
    sector_sum = 0
    for l in aoc_inp:
        a = l.rfind("-")
        b = l.rfind("[")
        checksum = l[b + 1: -1]
        sector = int(l[a + 1: b])
        encrypted = l[:a]
        if checksum == char_sum(encrypted):
            sector_sum += sector

    print(sector_sum)
find_valid()
