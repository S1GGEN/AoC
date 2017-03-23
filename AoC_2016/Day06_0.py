f = open("Day06_input.txt", 'r')
aoc_inp = f.read().split('\n')


def most_common_char(s):
    f = [0] * 26
    for c in s:
        f[ord(c) - ord('a')] += 1

    i = max(f)
    most = chr(f.index(i) + ord('a'))
    return most

answer = ""
for c in range(len(aoc_inp[0])):
    col = ""
    for r in range(len(aoc_inp)):
        col += aoc_inp[r][c]
    answer += most_common_char(col)

print(answer)
