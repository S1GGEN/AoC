f = open("Day10_input.txt", 'r')

aoc_inp = f.read().split("\n")
f.close()


def find_hi_lo(b):
    if len(b) > 1:
        # print(b)
        # return b[b.index(max(b))], b[b.index(min(b))]
        # print("----------------------------------------------------------------")
        return max(b), min(b)
    else:
        return False, False

exchanges = []
bots_receivers = [""]*210
bots_values = [""]*210
outputs = [""]*25

for i in range(len(outputs)):
    outputs[i] = []

for i in range(len(bots_values)):
    bots_values[i] = []  # to make sure these are individual
    bots_receivers[i] = []  # to make sure these are individual


for inst in aoc_inp:
    if inst.find("value") > -1:
        value = int(inst[inst.find("ue") + 3:inst.find("goe") -1])
        bot_num = int(inst[inst.rfind("t") + 2:])
        bots_values[bot_num].append(value)  # Ignore PyCharm error
        # print(bots)
    else:
        exchanges.append(inst)

print(bots_values)
# print(exchanges)


for inst in exchanges:
    sending_bot = int(inst[inst.find("bot") + 4: inst.find("giv") - 1])

    if inst[inst.find("to"):inst.find("and")].find("output") > -1:
        receiving_out_low = int(inst[inst.find("put") + 4: inst.find("and") - 1])
        bots_receivers[sending_bot].append("out" + str(receiving_out_low))
    else:
        receiving_bot_low = int(inst[inst.find("to bot") + 7: inst.find("and") - 1])
        bots_receivers[sending_bot].append(receiving_bot_low)

    if inst[inst.rfind("to"):].find("output") > -1:
        receiving_out_high = int(inst[inst.rfind("t") + 2:])
        bots_receivers[sending_bot].append("out" + str(receiving_out_high))
    else:
        receiving_bot_high = int(inst[inst.rfind("t") + 2:])
        bots_receivers[sending_bot].append(receiving_bot_high)

running = True
while running:
    empty = True
    for i in range(len(bots_values)):
        if len(bots_values[i]) > 1:
            empty = False
            high, low = find_hi_lo(bots_values[i])
            rec_lo = bots_receivers[i][0]
            rec_hi = bots_receivers[i][1]

            if isinstance(rec_lo, int):
                bots_values[rec_lo].append(low)
            else:
                output = int(rec_lo.replace("out", ""))
                outputs[int(rec_lo.replace("out", ""))].append(low)

            if isinstance(rec_hi, int):
                bots_values[rec_hi].append(high)
            else:
                output = int(rec_hi.replace("out", ""))
                outputs[output].append(high)

            bots_values[i] = []

            if high == 61 and low == 17:
                print("PART 1:")
                print("comparing bot:", i)

    if empty:
        print("No bots with two chips left")
        running = False

print("PART 2:", outputs[0][0] * outputs[1][0] * outputs[2][0])
