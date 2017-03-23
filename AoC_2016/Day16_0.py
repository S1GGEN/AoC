f = open("Day16_input.txt", 'r')

aoc_inp = f.read()
f.close()

data = aoc_inp

# data = "10000"
length_to_fill = 272


def bin_swap(s):
    new_string = ""
    for i in range(len(s)):
        if s[i] == "1":
            new_string += "0"
        else:
            new_string += "1"
    return new_string

# print(bin_swap("101010"))

while len(data) < length_to_fill:
    a = data
    b = bin_swap(data[::-1])
    data = a + "0" + b
    # print(data)

data = data[:length_to_fill]
# print(data)


def get_checksum(d):
    chcks = ""
    i = 0
    while i < len(d):
        pair = d[i:i + 2]
        if pair[0] == pair[1]:
            chcks += "1"
        else:
            chcks += "0"
        i += 2
    if len(chcks) % 2 == 0:
        return get_checksum(chcks)
    else:
        return chcks


print(get_checksum(data))
