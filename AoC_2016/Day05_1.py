import hashlib

f = open("Day05_input.txt", 'r')

aoc_inp = f.read()

# aoc_inp = "abc"

password = "________"
# a = 1
# my_hash = "0000015"
# password = password[:a] + my_hash[6:7] + password[a + 1:]
print(password)

pas_chars = 0
print(len(password))
i = 0
while True:
    my_hash = hashlib.md5(str(aoc_inp + str(i)).encode('ASCII')).hexdigest()
    if my_hash[:5] == "00000":
        a = my_hash[5:6]
        if a.isdigit():
            a = int(a)
            if a < 8:
                if password[a:a+1] == "_":
                    pas_chars += 1
                    password = password[:a] + my_hash[6:7] + password[a + 1:]
                    print(password)
    if pas_chars == 8:
        break
    i += 1

print(password)
