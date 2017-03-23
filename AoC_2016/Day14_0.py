import hashlib

f = open("Day14_input.txt", 'r')

aoc_inp = f.read()
f.close()

# aoc_inp = "abc"
# aoc_inp = "jlmsuwbz"


def find_aaa(s):
    for i in range(len(s) - 2):
        # print(s[i:i+3])
        if s[i] == s[i+1] == s[i+2]:
            # print("EUREKA:", s[i]*3)
            return s[i]


def find_aaaaa(s, c):
    for i in range(len(s) - 4):
        # print(s[i:i+5])
        if s[i] == c:
            if s[i] == s[i+1] == s[i+2] == s[i+3] == s[i+4]:
                # print("EUREKA:", s[i]*5)
                return True
    return False


def make_hash_i(i):
    return hashlib.md5(str(aoc_inp + str(i)).encode('ASCII')).hexdigest()


def make_hash(s):
    return hashlib.md5(s.encode('ASCII')).hexdigest()


def hash_2016_times(h):
    a = 0
    my_hash = h
    for _ in range(2016):
        my_hash = make_hash(my_hash)

    return my_hash


hash_2016_times("abc0")
print(make_hash_i(0))


def validate_experimental(i, c, i_li, o_li):
    for sublist in i_li:
        if i - sublist[0] < 1000:
            if i != sublist[0]:
                if sublist[1] == c:
                    if sublist not in o_li:
                        o_li.append(sublist)
                        print(len(o_li))
                        del sublist
                    # return
        else:
            del sublist


# find_aaa("abcdeeefg")
# find_aaaaa("abcdeeeeefg", "e")


# print(make_hash(18))

def make_unique(original_list):
    unique_list = []
    [unique_list.append(obj) for obj in original_list if obj not in unique_list]
    return unique_list


def main():
    valid_keys = []
    experimental_keys = []

    i = 0
    while len(valid_keys) <= 64:
        # print("-")
        # print(len(valid_keys))
        # print(i)
        my_hash = make_hash_i(i)
        my_hash = hash_2016_times(my_hash)
        triplet = find_aaa(my_hash)
        if triplet:
            experimental_keys.append([i, triplet])
            # print(experimental_keys)
            quintuplet = find_aaaaa(my_hash, triplet)
            if quintuplet:
                validate_experimental(i, triplet, experimental_keys, valid_keys)
            # else:
                # experimental_keys.append([i, triplet])
        i += 1
    print("---")
    print(len(valid_keys))
    print(len(valid_keys))
    print(experimental_keys)
    print(valid_keys)
    # valid_keys = sorted(valid_keys, key=lambda x: x[0])[:64]
    print(valid_keys)
    print(len(valid_keys))

    print("-------")
    valid_keys = make_unique(valid_keys)
    print(valid_keys)
    print(len(valid_keys))
    print(valid_keys[63])
main()
