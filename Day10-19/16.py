import numpy as np

patterns = dict()


def gen_pattern(digit, length):
    if (digit, length) in patterns:
        return patterns[(digit, length)]
    pattern = np.concatenate((
        np.repeat(0, digit),
        np.repeat(1, digit),
        np.repeat(0, digit),
        np.repeat(-1, digit),
    ))
    pattern = np.resize(pattern, length + 1)[1:]
    patterns[(digit, length)] = pattern
    return pattern


def perform_phase(inp, offset=0):
    out = np.zeros(len(inp), dtype=int)

    if offset > len(inp) // 2:
        s = sum(inp[offset:])
        for idx in range(offset, len(inp)):
            out[idx] = s % 10
            s -= inp[idx]
    else:
        for idx in range(len(inp)):
            pattern = gen_pattern(idx + 1, len(inp))
            out[idx] = np.abs(np.sum(inp * pattern)) % 10
    return out


if __name__ == "__main__":
    with open('16.txt') as f:
        data = f.read()

    inp = np.array(list(int(i) for i in data.strip()), dtype=int)

    p1_inp = np.copy(inp)
    for _ in range(100):
        p1_inp = perform_phase(p1_inp)
    print("Part 1: {}".format(''.join(map(str, p1_inp[:8]))))

    offset = int(''.join(str(n) for n in inp[:7]))
    p2_inp = np.tile(inp, 10000)
    for idx in range(100):
        print(idx)
        p2_inp = perform_phase(p2_inp, offset)
    print("Part 2: {}".format(''.join(str(i)
                                      for i in p2_inp[offset:offset + 8])))
