f = open("Day07_input.txt", 'r')

aoc_inp = f.read().split('\n')


def find_aba(s):
    i = 0
    while i < len(s) - 2:
        a = s[i:i+3]
        i += 1
        if a[0] == a[2] and a[0] != a[1]:
            return a


valid_addresses = []
num_babs = 0
all_babs = []

for line in aoc_inp:
    # print("--------------------------------------------------------------------------------------------")
    # print(line)

    outside_abas = []
    inside_abas = []
    babs = []

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
        for i in range(0, len(a)-2):
            word = a[i] + a[i+1] + a[i+2]
            aba = find_aba(word)
            if aba:
                outside_abas.append(aba)

    for a in inside:
        for i in range(0, len(a)-2):
            word = a[i] + a[i+1] + a[i+2]
            aba = find_aba(word)
            if aba:
                inside_abas.append(aba)

    # ------------------------------------------------------------------------------------------

    for aba in inside_abas:
        bab = aba[1] + aba[0] + aba[1]
        if bab in outside_abas:
            print(line)
            print("bab found!")
            print(bab, "and", aba)
            print("-----------------------------------------------------------------------------")
            babs.append(bab)
            valid = True
            all_babs.append([aba, bab])

    if len(babs) > 0:
        num_babs += 1

    # print(babs)

print("--------------------------------------------------------------------------------------------------------------")
print(num_babs)
print(all_babs)
print(len(all_babs))



