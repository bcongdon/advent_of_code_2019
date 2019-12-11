from fractions import Fraction
from math import atan2, pi


def dist(a, b):
    x1, y1 = a
    x2, y2 = b
    return abs(x2 - x1) + abs(y2 - y1)


def blocking(x, y, tx, ty, mx, my):
    dx, dy = x - tx, y - ty

    if dx == 0:
        slope_x, slope_y = 0, 1
    elif dy == 0:
        slope_x, slope_y = 1, 0
    else:
        s = Fraction(dx, dy)
        slope_x, slope_y = abs(s.numerator), abs(s.denominator)
    if tx < x:
        slope_x = -abs(slope_x)
    if ty < y:
        slope_y = -abs(slope_y)

    while (0 <= x < mx) and (0 <= y < my):
        x += slope_x
        y += slope_y

        if (0 <= x < mx) and (0 <= y < my):
            yield (x, y)


def visible_asteroids(loc, asteroids, mx, my):
    x, y = loc
    visible = []
    asteroids = sorted(asteroids, key=lambda a: dist(loc, a))

    blocked = set()
    for other in asteroids:
        if other == loc or other in blocked:
            continue
        visible.append(other)
        tx, ty = other
        blocked.update(blocking(x, y, tx, ty, mx, my))
    visible = sorted(
        visible, key=lambda loc: atan2(loc[0] - x, loc[1] - y) - pi / 2, reverse=True
    )
    return visible


def destroy_order(loc, asteroids, mx, my):
    destroyed = []
    while asteroids:
        visible = visible_asteroids(loc, asteroids, mx, my)
        print(visible)
        for v in visible:
            print(atan2(v[0], v[1]))
        destroyed += visible
        asteroids = [a for a in asteroids if a not in visible and a != loc]
    return destroyed


if __name__ == "__main__":
    with open("10.txt") as f:
        lines = f.readlines()
        grid = [list(l.strip()) for l in lines]

    my = len(grid)
    mx = len(grid[0])
    asteroids = [(x, y) for x in range(mx) for y in range(my) if grid[y][x] != "."]

    part1 = max((len(visible_asteroids(a, asteroids, mx, my)), a) for a in asteroids)
    num_visible, loc = part1
    print("Part 1: {}".format(num_visible))

    part2 = destroy_order(loc, asteroids, mx, my)
    print(part2[199][0] * 100 + part2[199][1])
