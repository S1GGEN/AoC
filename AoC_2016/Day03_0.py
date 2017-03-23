f = open("Day03_input.txt", 'r')

aoc_input = f.read().split('\n')

num_valid = 0
num_false = 0

for l in aoc_input:
    l = list(l.split())
    valid = True
    print(l[0], l[1], l[2])
    a = int(l[0])
    b = int(l[1])
    c = int(l[2])
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
        print(" ----------------------------------------------------------------------------------------------------- ")
    else:
        num_false += 1


print(num_valid)
print(num_false)