import numpy as np


def parse_input(lines):
    for l in lines:
        if 'increment' in l:
            yield ('increment', int(l.split()[-1]))
        elif 'stack' in l:
            yield ('stack',)
        elif 'cut' in l:
            yield ('cut', int(l.split()[-1]))
        else:
            raise Exception("Invalid line")


def modinv(a, m):
    def egcd(a, b):
        if a == 0:
            return (b, 0, 1)
        else:
            g, y, x = egcd(b % a, a)
            return (g, x - (b // a) * y, y)

    g, x, _ = egcd(a, m)
    if g != 1:
        raise Exception('modular inverse does not exist')
    else:
        return x % m


def reverse_deal(D, i):
    return D - 1 - i


def reverse_cut(D, i, N):
    return (i + N + D) % D


def reverse_increment(D, i, N):
    return modinv(N, D) * i % D


def reverse_shuffle(inputs, D, pos):
    for inp in inputs:
        if inp[0] == 'stack':
            pos = reverse_deal(D, pos)
        elif inp[0] == 'increment':
            pos = reverse_increment(D, pos, inp[1])
        elif inp[0] == 'cut':
            pos = reverse_cut(D, pos, inp[1])
        else:
            raise Exception("Invalid line")
    return pos


if __name__ == "__main__":
    with open('22.txt') as f:
        lines = f.readlines()
    N = 10007

    cards = np.arange(0, N)
    inputs = list(parse_input(lines))
    for i in inputs:
        if i[0] == 'stack':
            cards = np.flipud(cards)
        elif i[0] == 'increment':
            n = i[1]
            new = np.zeros(N, dtype=int)
            for idx in range(N):
                new[(idx * n) % len(cards)] = cards[idx]
            cards = new
        elif i[0] == 'cut':
            cards = np.roll(cards, -i[1])
        else:
            raise Exception("Invalid line")
    print("Part 1: {}".format(np.where(cards == 2019)[0][0]))

    # https://www.reddit.com/r/adventofcode/comments/ee0rqi/2019_day_22_solutions/fbnifwk?utm_source=share&utm_medium=web2x
    D = 119315717514047  # deck size
    X = 2020
    Y = reverse_shuffle(inputs[::-1], D, X)
    Z = reverse_shuffle(inputs[::-1], D, Y)
    A = (Y - Z) * modinv(X - Y + D, D) % D
    B = (Y - A * X) % D
    n = 101741582076661
    result = (pow(A, n, D) * X + (pow(A, n, D) - 1) * modinv(A - 1, D) * B) % D
    print("Part 2: {}".format(result))
