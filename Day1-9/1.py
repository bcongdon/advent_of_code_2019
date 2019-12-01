def fuel_requirement(input):
    return max(0, (input // 3) - 2)


if __name__ == "__main__":
    with open("1.txt") as f:
        inputs = list(map(int, f.readlines()))

    part1 = sum(fuel_requirement(i) for i in inputs)
    print("Part 1: {}".format(part1))

    part2 = 0
    modules = list(inputs)
    while modules:
        fuel = fuel_requirement(modules.pop())
        part2 += fuel
        if fuel > 0:
            modules.append(fuel)
    print("Part 2: {}".format(part2))
