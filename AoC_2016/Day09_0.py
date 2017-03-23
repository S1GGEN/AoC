f = open("Day09_input.txt", 'r')

aoc_inp = f.read()
aoc_inp = "".join(aoc_inp.split())
f.close()


def marker_start(s):
    if s == "(":
        return True


def get_marker(s):
    return s[1:s.find(')')]

# print(get_marker("2x2)dsdsdsd"))


def get_marker_a_b(s):
    return int(s[:s.find('x')]), int(s[s.find('x') + 1:])


def traverse_string(s):
    i = 0
    # print(len(s))
    print("-----------------------------------------------------------------------------------------------------------")
    while i < len(s):
        print("bokstav:", s[i])
        if marker_start(s[i]):
            print("-------------------------------------------------------")
            marker = get_marker(s[i:])
            if marker.find('x') > -1:
                print("Marker:", marker)
                a, b = get_marker_a_b(marker)
                print("Number of characters:", a)
                print("Number of times to duplicate:", b)
                to_copy = s[i + len(marker) + 2: i + len(marker) + 2 + a]
                print("to_copy:", to_copy)
                print("len(to_copy):", len(to_copy))
                s = str(s[:i]) + str(to_copy * b) + str(s[i + len(marker) + a + 2:])
                i += len(str(to_copy * b)) -1

        i += 1
    print(s)
    print("length:", len(s))

    opened = 0
    closed = 0

    for c in s:
        if c == "(":
            opened += 1
        elif c == ")":
            closed += 1

    print("opened:", opened)
    print("closed:", closed)

traverse_string(aoc_inp)

# traverse_string("ADVENT")
# traverse_string("A(1x5)BC")
# traverse_string("(3x3)XYZ")
# traverse_string("A(2x2)BCD(2x2)EFG")
# traverse_string("(6x1)(1x3)A")
# traverse_string("X(8x2)(3x3)ABCY")
# traverse_string("IKKEKOPIERMEG(33x2)KOPIERHERFRA(3x4)(99x77)OGHELTHIT")
# traverse_string("IKKEKOPIERMEG(33x2)KOPIERHERFRA(3x4)(99x77)OGHELTHIT(3x3)XYZ")
