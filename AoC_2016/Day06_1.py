f = open("Day06_input.txt", 'r')

aoc_inp = f.read().split('\n')


def least_common_char(s):
    f = [0] * 26
    for c in s:
        f[ord(c) - ord('a')] += 1

    minimum = 1

    while minimum <= max(f):
        print(minimum)
        if minimum in f:
            break
        else:
            minimum += 1

    least = chr(f.index(minimum) + ord('a'))
    print(least)
    return least

answer = ""

for c in range(len(aoc_inp[0])):
    col = ""
    for r in range(len(aoc_inp)):
        col += aoc_inp[r][c]
    answer += least_common_char(col)

print(answer)
