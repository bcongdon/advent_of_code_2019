def run(codes):
    idx = 0
    while True:
        c = codes[idx]
        if c == 1:
            a, b, x = codes[idx + 1], codes[idx + 2], codes[idx + 3]
            codes[x] = codes[a] + codes[b]
        elif c == 2:
            a, b, x = codes[idx + 1], codes[idx + 2], codes[idx + 3]
            codes[x] = codes[a] * codes[b]
        elif c == 99:
            return codes[0]
        else:
            raise Exception("Invalid opcode: {}".format(c))
        idx += 4


def part2(codes):
    for verb in range(100):
        for noun in range(100):
            copy = codes[:]
            copy[1], copy[2] = noun, verb

            result = run(copy)
            if result == 19690720:
                return(100 * noun + verb)


if __name__ == "__main__":
    with open('2.txt') as f:
        codes = [int(i) for i in f.read().split(',')]
    codes[1] = 12
    codes[2] = 2
    print("Part 1: {}".format(run(codes[:])))
    print("Part 2: {}".format(part2(codes[:])))
