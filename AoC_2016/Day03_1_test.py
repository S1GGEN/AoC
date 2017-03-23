f = open("Day03_input.txt", 'r')

aoc_inp = f.read().split('\n')

num_valid = 0
num_false = 0

inp = []

for l in aoc_inp:
    l = list(l.split())
    inp.append([int(l[0]), int(l[1]), int(l[2])])

print(inp)

l = 0

while l < len(inp) - 2:
    print(" ------------------------------------------------------------------------------------------------- ")
    for col in range(3):
        print(col)
        valid = True
        a = inp[l][col]
        b = inp[l + 1][col]
        c = inp[l + 2][col]
        print(a, b, c)
        if a + b <= c:
            valid = False
            print("a")
        if a + c <= b:
            valid = False
            print("b")
        if c + b <= a:
            valid = False
        print(valid)

        if valid:
            num_valid += 1
            print(" ---------- ")
        else:
            num_false += 1
    l += 3

print("---------------------------------------------------------------------------------------------------------------")
print("END")
print(num_valid)
print(num_false)