from collections import defaultdict
from collections import deque


def num_orbits(planet, depth, graph):
    total = depth
    for child in graph[planet]:
        total += num_orbits(child, depth + 1, graph)
    return total


def path(start, end, graph):
    if start == end:
        return [end]
    if not graph[start]:
        return None
    for child in graph[start]:
        p = path(child, end, graph)
        if p is None:
            continue
        return [start] + p
    return None


def part2(graph):
    p1 = deque(path('COM', 'YOU', graph))
    p2 = deque(path('COM', 'SAN', graph))

    lca_len = 0
    while p1[0] == p2[0]:
        lca_len += 1
        p1.popleft()
        p2.popleft()

    return len(p1) + len(p2) - 2


if __name__ == "__main__":
    with open('6.txt') as f:
        orbits = [l.strip().split(')') for l in f.readlines()]

    graph = defaultdict(list)
    for src, dst in orbits:
        graph[dst]
        graph[src].append(dst)

    print("Part 1: {}".format(num_orbits('COM', 0, graph)))
    print("Part 2: {}".format(part2(graph)))
