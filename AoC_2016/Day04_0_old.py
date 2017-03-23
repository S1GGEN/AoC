f = open("Day04_input.txt", 'r')

aoc_inp = f.read().split('\n')

a = [22, 221, 21212, 22, 999999, 999999]

print(max(a))


def char_sum(s):
    f = [0] * 26
    checksum = ""
    for c in s:
        if c != "-":
            f[ord(c) - ord('a')] += 1
        # print(c)
    # print(f)

    i = max(f)

    while i > 0:
        for g in range(len(f)):
            if f[g] == i:
                checksum += chr(g + ord('a'))
        i -= 1
    checksum = checksum[:5]
    print("charsum:", checksum)
    return checksum

# char_sum("a-b-c-d-e-f-g-h")


def find_valid():
    sector_sum = 0
    for l in aoc_inp:
        print("-------------------------------------------------------------------------------------------------------")
        print(l)
        a = l.rfind("-")
        b = l.rfind("[")
        checksum = l[b + 1: -1]
        print("checksum:", checksum)
        sector = int(l[a + 1: b])
        print("sector:", sector)
        encrypted = l[:a]
        print("encrypted:", encrypted)
        if checksum == char_sum(encrypted):
            print("VALID")
            sector_sum += sector

    print(sector_sum)
find_valid()
