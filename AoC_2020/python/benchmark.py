

def timereps(reps, func, arg):
    from time import time
    start = time()
    for i in range(0, reps):
        func(arg)
    end = time()
    return "{:.10f}".format((end - start) / reps)


def d07():
    from day07.day7 import one as d07p1, two as d07p2, two_old as d07p2_old
    print("\nBENCHMARK DAY 07:")
    number = 1000
    # with open('day07/input.txt', 'r') as f:
        # day7_input = open('day07/input.txt', 'r')
        # for line in day7_input:
        #     print(line)
        # print(f"opened input {number} times, average result: {timereps(number, open('day07/input.txt', 'r'))} seconds")
        # print(f"Ran part1() {number} times, average result: {timereps(number, d07p1, f)} seconds")
        # print(f"Ran part2() {number} times, average result: {timereps(number, d07p2, f)} seconds")
        # print(f"Ran part2() old version {number} times, average result: {timereps(number, d07p2_old, f)} seconds")


d07()
