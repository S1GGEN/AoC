f = open("Day12_input.txt", 'r')

aoc_inp = f.read().split("\n")
f.close()

a = 0
b = 0
c = 1
d = 0

i = 0
while i < len(aoc_inp):
    ins = aoc_inp[i]
    # print("-------------------------------------------")
    # print(ins)
    if ins[:3] == "cpy":
        reg = ins[-1]
        # print("reg:", reg)
        try:
            val = int(ins[-len(ins) + 4:-2])
        except ValueError:
            if ins[-len(ins) + 4:-2] == "a":
                val = a
            elif ins[-len(ins) + 4:-2] == "b":
                val = b
            elif ins[-len(ins) + 4:-2] == "c":
                val = c
            elif ins[-len(ins) + 4:-2] == "d":
                val = d

        # print("val:", val)
        if reg == "a":
            a = val
        elif reg == "b":
            b = val
        elif reg == "c":
            c = val
        elif reg == "d":
            d = val
    elif ins[:3] == "inc":
        reg = ins[-1]
        # print("reg:", reg)
        if reg == "a":
            a += 1
        elif reg == "b":
            b += 1
        elif reg == "c":
            c += 1
        elif reg == "d":
            d += 1
    elif ins[:3] == "dec":
        reg = ins[-1]
        # print("reg:", reg)
        if reg == "a":
            a -= 1
        elif reg == "b":
            b -= 1
        elif reg == "c":
            c -= 1
        elif reg == "d":
            d -= 1
    if ins[:3] == "jnz":
        reg = ins[4:5]
        # print("reg:", reg)
        if reg == "a":
            val = a
        elif reg == "b":
            val = b
        elif reg == "c":
            val = c
        else:
            val = d
        # print("int(ins[6:]):", int(ins[6:]))
        if val != 0:
            i += int(ins[6:])
        else:
            i += 1
    else:
        i += 1
    # time.sleep(1)

print("a:", a)
print("b:", b)
print("c:", c)
print("d:", d)
