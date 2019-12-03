DIRECTIONS = {
    "U": (0, 1),
    "D": (0, -1),
    "L": (-1, 0),
    "R": (1, 0)
}


def wire_to_point_set(wire):
    s = set()
    d = dict()

    x, y, steps = 0, 0, 0
    for w in wire:
        dx, dy = DIRECTIONS[w[0]]
        dist = int(w[1:])
        for _ in range(dist):
            x += dx
            y += dy
            s.add((x, y))

            steps += 1
            if (x, y) not in d:
                d[(x, y)] = steps
    return s, d


if __name__ == "__main__":
    with open('3.txt') as f:
        wires = [line.strip().split(',') for line in f.readlines()]
    w1, w2 = wires
    s1, d1 = wire_to_point_set(w1)
    s2, d2 = wire_to_point_set(w2)

    intersections = s1 & s2

    p1_intersections = [(abs(x) + abs(y), (x, y)) for (x, y) in intersections]
    p1_intersections.sort()
    print("Part 1: {}".format(p1_intersections[0][0]))

    p2_intersections = [(d1[p] + d2[p], p) for p in intersections]
    p2_intersections.sort()
    print("Part 2: {}".format(p2_intersections[0][0]))
