import itertools
import string
from collections import defaultdict, deque


def parse_map(lines):
    portals = defaultdict(list)
    for (x, y) in itertools.product(range(len(lines)), range(len(lines[0]))):
        if lines[x][y] != '.':
            continue
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
            if lines[x + dx][y + dy] in string.ascii_letters:
                portal_str = lines[x + dx][y + dy] + \
                    lines[x + 2 * dx][y + 2 * dy]
                key = ''.join(sorted(portal_str))
                portals[key].append((x, y))
    start = end = (-1, -1)
    connections = {}
    for k, v in portals.items():
        if k == 'AA':
            start = v[0]
        elif k == 'ZZ':
            end = v[0]
        else:
            a, b = v
            connections[a], connections[b] = b, a
    return start, end, connections, portals


if __name__ == "__main__":
    with open('20.txt') as f:
        lines = [l.replace('\n', '') for l in f.readlines()]

    start, end, portals, labels = parse_map(lines)
    dists = {start: 0}
    queue = deque([start])
    visited = set([start])
    while queue:
        c = queue.popleft()
        if c == end:
            print("Part 1: {}".format(dists[c]))
            break
        x, y = c
        neighbors = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
        if c in portals:
            neighbors.append(portals[c])
        for n in neighbors:
            nx, ny = n
            if n in visited or lines[nx][ny] != '.':
                continue
            dists[n] = dists[c] + 1
            visited.add(n)
            queue.append(n)

    sx, sy = start
    end = (end[0], end[1], 0)
    dists = {(sx, sy, 0): 0}
    queue = deque([(sx, sy, 0)])
    visited = set([(sx, sy, 0)])
    while queue:
        c = queue.popleft()
        if c == end:
            print("Part 2: {}".format(dists[c]))
            break
        x, y, l = c
        neighbors = [(x + 1, y, l), (x - 1, y, l),
                     (x, y + 1, l), (x, y - 1, l)]
        if (x, y) in portals:
            is_outer = (
                x == 2 or y == 2 or
                x == len(lines) - 3 or y == len(lines[0]) - 3)
            if is_outer:
                if l > 0:
                    neighbors.append((*portals[(x, y)], l - 1))
            else:
                neighbors.append((*portals[(x, y)], l + 1))
        for n in neighbors:
            nx, ny, nl = n
            if n in visited or lines[nx][ny] != '.':
                continue
            dists[n] = dists[c] + 1
            visited.add(n)
            queue.append(n)
