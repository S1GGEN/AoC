f = open("Day07_input_test.txt", 'r')

aoc_inp = f.read().split('\n')


def find_aba(s):
    print("ABA-sjekk:", s)
    i = 0
    while i < len(s) - 2:
        a = s[i:i+3]
        i += 1
        if a[0] == a[2] and a[0] != a[1]:
            print("ABA funnet:", a, "!!!!!!!!!!!!!!!!!")
            return a


valid_addresses = []
num_valid = 0
all_babs = []

for line in aoc_inp:
    print("--------------------------------------------------------------------------------------------")
    print(line)
    valid = False

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
    print("OUTSIDE")
    for a in outside:
        print(a)
        print("len(a):", len(a))
        for i in range(0, len(a)-2):
            print("i:", i)
            word = a[i] + a[i+1] + a[i+2]
            print("word:", word)
            aba = find_aba(word)
            if aba:
                outside_abas.append(aba)

    print("INSIDE")
    for a in inside:
        print(a)
        print("len(a):", len(a))
        for i in range(0, len(a)-2):
            print(i)
            word = a[i] + a[i+1] + a[i+2]
            print(word)
            aba = find_aba(word)
            if aba:
                inside_abas.append(aba)

    # ------------------------------------------------------------------------------------------

    print(inside_abas)
    print(outside_abas)

    for aba in inside_abas:
        bab = aba[1] + aba[0] + aba[1]
        print(bab)
        if bab in outside_abas:
            print("bab found!")
            print(bab, "and", aba)
            babs.append(bab)
            valid = True
            all_babs.append([aba, bab])

    print(babs)

print("--------------------------------------------------------------------------------------------------------------")
print(num_valid)
print(all_babs)
print(len(all_babs))



