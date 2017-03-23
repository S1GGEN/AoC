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


def caesar_cipher(char, shift):
    a = ord(char)
    shift %= 26
    if a + shift >= ord('a') + 26:
        shifted_char = chr(a + shift - 26)
    else:
        shifted_char = chr(a + shift)
    return shifted_char


valid_rooms = []
for l in aoc_inp:
    a = l.rfind("-")
    b = l.rfind("[")
    checksum = l[b + 1: -1]
    sector = int(l[a + 1: b])
    encrypted = l[:a]
    if checksum == char_sum(encrypted):
        valid_rooms.append(l)
print(valid_rooms)

decrypted_rooms = []

for room in valid_rooms:
    print("----------------------------------------------------------------------------------------------------------")
    # l = room
    a = room.rfind("-")
    b = room.rfind("[")
    sector = int(room[a + 1: b])
    print("sector:", sector)
    encrypted = room[:a]
    print("encrypted:", encrypted)
    decrypted = ""
    for c in encrypted:
        if c == "-":
            decrypted += " "
        else:
            decrypted += caesar_cipher(c, sector)
    print("decrypted:", decrypted)
    decrypted_rooms.append([decrypted, sector])

print(decrypted_rooms)

north_pole = []

for room in decrypted_rooms:
    print(room[0])
    if room[0].find("north") > -1:
        north_pole.append(room)

print(north_pole)
