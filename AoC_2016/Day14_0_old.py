import hashlib

f = open("Day14_input.txt", 'r')

aoc_inp = f.read()
f.close()

aoc_inp = "abc"


def find_aaa(s):
    for i in range(len(s) - 2):
        # print(s[i:i+3])
        if s[i] == s[i+1] == s[i+2]:
            print("EUREKA:", s[i]*3)
            return s[i]


def find_aaaaa(s, c):
    for i in range(len(s) - 4):
        # print(s[i:i+5])
        if s[i] == c:
            if s[i] == s[i+1] == s[i+2] == s[i+3] == s[i+4]:
                print("EUREKA:", s[i]*5)
                return True


def make_hash(i):
    return hashlib.md5(str(aoc_inp + str(i)).encode('ASCII')).hexdigest()


def validate_experimental(i, c, i_li, o_li):
    for sublist in i_li:
        if i - sublist[0] <= 1000:
            if i != sublist[0]:
                if sublist[1] == c:
                    o_li.append(sublist)
        else:
            del sublist


find_aaa("abcdeeefg")
find_aaaaa("abcdeeeeefg", "e")


print(make_hash(18))


def main():
    valid_keys = []
    experimental_keys = []

    i = 0
    while len(valid_keys) <= 64:
        # print(i)
        my_hash = make_hash(i)
        triplet = find_aaa(my_hash)
        if triplet:
            experimental_keys.append([i, triplet])
            print(experimental_keys)
            quintuplet = find_aaaaa(my_hash, triplet)
            if quintuplet:
                validate_experimental(i, triplet, experimental_keys, valid_keys)

        i += 1

    valid_keys = sorted(valid_keys, key=lambda x: x[0])[:64]
    print(valid_keys)
    print(valid_keys[63])
    print(len(valid_keys))
main()