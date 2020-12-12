from collections import defaultdict
from itertools import product

N = 5


def evolve(grid):
    new_grid = [l[:] for l in grid]
    for x in range(N):
        for y in range(N):
            num_neighbors = 0
            for nx, ny in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]:
                if not (0 <= nx < N and 0 <= ny < N):
                    continue
                if grid[nx][ny] == "#":
                    num_neighbors += 1
            if grid[x][y] == "#" and num_neighbors != 1:
                new_grid[x][y] = "."
            elif grid[x][y] != "#" and num_neighbors in (1, 2):
                new_grid[x][y] = "#"
    return new_grid


def neighbors_3d(x, y, z):
    neighbors = set()
    for dx, dy in [(-1, 0), (0, 1), (1, 0), (0, -1)]:
        cx, cy = x + dx, y + dy

        added = False
        if not (0 <= cx < N):
            neighbors.add((2 + dx, 2, z + 1))
            added = True
        if not (0 <= cy < N):
            neighbors.add((2, 2 + dy, z + 1))
            added = True

        if cx == 2 and cy == 2:
            added = True
            for k in range(N):
                nx = k if dx == 0 else (0 if dx == 1 else 4)
                ny = k if dy == 0 else (0 if dy == 1 else 4)
                neighbors.add((nx, ny, z - 1))

        if not added:
            neighbors.add((cx, cy, z))
    return neighbors


def evolve2(input_grid):
    grid = {}
    for j, row in enumerate(input_grid):
        for i, val in enumerate(row):
            if (i, j) == (2, 2):
                continue
            grid[(i, j, 0)] = val

    min_level, max_level = -1, 1
    for _ in range(200):
        next_grid = {}
        for z in range(min_level, max_level + 1):
            for (x, y) in product(range(N), range(N)):
                if (x, y) == (2, 2):
                    continue

                num_neighbors = sum(
                    1 for n in neighbors_3d(x, y, z) if grid.get(n, ".") == "#"
                )
                curr_val = grid.get((x, y, z), ".")
                to_evolve = (curr_val == "#" and num_neighbors == 1) or (
                    curr_val == "." and (num_neighbors in (1, 2))
                )
                if to_evolve:
                    next_grid[(x, y, z)] = "#"
                    min_level = min(z - 1, min_level)
                    max_level = max(z + 1, max_level)
        grid = next_grid
    return sum(1 for v in grid.values() if v == "#")


def biodiversity(grid):
    v = 1
    total = 0
    for x in range(N):
        for y in range(N):
            if grid[x][y] == "#":
                total += v
            v *= 2
    return total


if __name__ == "__main__":
    with open("24.txt") as f:
        lines = f.readlines()
    orig = [list(l.strip()) for l in lines]

    grid = orig
    seen = set("\n".join("".join(l) for l in grid))
    while True:
        grid = evolve(grid)
        grid_str = "\n".join("".join(l) for l in grid)
        if grid_str in seen:
            print("Part 1: {}".format(biodiversity(grid)))
            break
        seen.add(grid_str)

    grid = orig
    print("Part 2: {}".format(evolve2(grid)))
