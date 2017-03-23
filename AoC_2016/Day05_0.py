import hashlib

f = open("Day05_input.txt", 'r')

aoc_inp = f.read()

# aoc_inp = "abc"

password = ""
i = 0
while True:
    my_hash = hashlib.md5(str(aoc_inp + str(i)).encode('ASCII')).hexdigest()
    if my_hash[:5] == "00000":
        print("wohoo")
        password += my_hash[5:6]
    if len(password) == 8:
        break
    i += 1

print(password)
