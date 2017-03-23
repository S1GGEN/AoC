f = open("Day07_input.txt", 'r')

aoc_inp = f.read().split('\n')


def find_abba(s):
    # print("ABBA-sjekk:")
    # print(s)
    i = 0
    while i < len(s) - 3:
        a = s[i:i+4]
        i += 1
        if a[0] == a[3] and a[1] == a[2] and a[0] != a[1]:
            print("ABBA funnet:", a, "!!!!!!!!!!!!!!!!!")
            return True
    return False

valid_addresses = []
num_valid = 0

for line in aoc_inp:
    print("--------------------------------------------------------------------------------------------")
    print(line)
    valid = False
    super_invalid = False

    outside = []
    inside = []
    last_index = -1
    for i in range(len(line)):
        if line[i] == "[":
            outside.append(line[last_index + 1:i])
            last_index = i
        elif line[i] == "]":
            inside.append(line[last_index + 1:i])
            last_index = i

        if i == len(line) - 1:
            outside.append(line[last_index + 1:])

    # print(inside)
    # print(outside)
    for a in outside:
        for i in range(len(a)-3):
            word = a[i] + a[i+1] + a[i+2] + a[i+3]
            # print(word)
            if find_abba(word):
                valid = True

    for a in inside:
        for i in range(len(a)-3):
            word = a[i] + a[i+1] + a[i+2] + a[i+3]
            # print(word)
            if find_abba(word):
                super_invalid = True

    if valid and not super_invalid:
        num_valid += 1
        print("VALID")


print("--------------------------------------------------------------------------------------------------------------")
print(num_valid)




