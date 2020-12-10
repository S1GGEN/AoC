
def do_search_2(lines):
    for line1 in lines:
        for line2 in lines:
            if line1 + line2 == 2020:
                return line1 * line2


def do_search_3(lines):
    for line1 in lines:
        for line2 in lines:
            for line3 in lines:
                if line1 + line2 + line3 == 2020:
                    return line1 * line2 * line3


def main():
    lines = [int(x) for x in open('input.txt', 'r')]
    print(do_search_2(lines))
    print(do_search_3(lines))


main()
