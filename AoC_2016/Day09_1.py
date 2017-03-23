f = open("Day09_input.txt", 'r')

aoc_inp = f.read()
aoc_inp = "".join(aoc_inp.split())
f.close()


def marker_start(s):
    if s == "(":
        return True


def get_marker(s):
    return s[1:s.find(')')]


def get_marker_a_b(s):
    return int(s[:s.find('x')]), int(s[s.find('x') + 1:])


def traverse_string(s):
    i = 0
    while len(s) > 0:
        if marker_start(s[0]):
            marker = get_marker(s)
            if marker.find('x') > -1:
                a, b = get_marker_a_b(marker)
                to_copy = s[len(marker) + 2:len(marker) + 2 + a]
                i += traverse_string(to_copy) * b
                s = str(s[len(marker) + a + 2:])
        else:
            s = s[1:]
            i += 1

    # print("i:", i)
    return i

print(traverse_string(aoc_inp))

# traverse_string("ADVENT")
# traverse_string("A(1x5)BC")
# traverse_string("(3x3)XYZ")
# traverse_string("A(2x2)BCD(2x2)EFG")
# traverse_string("(6x1)(1x3)A")
# traverse_string("X(8x2)(3x3)ABCY")
# traverse_string("IKKEKOPIERMEG(33x2)KOPIERHERFRA(3x4)(99x77)OGHELTHIT")
# traverse_string("IKKEKOPIERMEG(33x2)KOPIERHERFRA(3x4)(99x77)OGHELTHIT(3x3)XYZ")
# traverse_string("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")
# traverse_string("(27x12)(20x12)(13x14)(7x10)(1x12)A")
