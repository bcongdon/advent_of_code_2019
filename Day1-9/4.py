from collections import Counter

A = 236491
B = 713787


def password_count():
    valid_p1 = 0
    valid_p2 = 0
    for x in range(A, B):
        x_str = str(x)
        if ''.join(sorted(x_str)) != x_str:
            continue

        counter = Counter(x_str)
        if any(v for v in counter.values() if v >= 2):
            valid_p1 += 1
        if any(v for v in counter.values() if v == 2):
            valid_p2 += 1

    return valid_p1, valid_p2


if __name__ == "__main__":
    p1, p2 = password_count()
    print("Part 1: {}".format(p1))
    print("Part 2: {}".format(p2))
