


def timereps(reps, func, argument):
    from time import time
    start = time()
    for i in range(0, reps):
        func(argument)
    end = time()
    return "{:.10f}".format((end - start) / reps)


def d07():
    from day7.day7 import one as d07p1, two as d07p2
    print("\nBENCHMARK DAY 07:")
    number = 1000
    day7_input = open('day7/input.txt', 'r')
    # print(f"opened input {number} times, average result: {timereps(number, open('day7/input.txt', 'r'))} seconds")
    print(f"Ran part1() {number} times, average result: {timereps(number, d07p1, day7_input)} seconds")
    print(f"Ran part2() {number} times, average result: {timereps(number, d07p2, day7_input)} seconds")


d07()
