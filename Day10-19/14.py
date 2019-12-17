from collections import defaultdict
import math

# Solution inspired by https://github.com/sparkyb/adventofcode/blob/master/2019/day14.py


def parse_ingredient(ing):
    amt, ing = ing.strip().split(' ')
    return (ing, int(amt))


def parse_dependency(line):
    inp, out = line.split('=>')

    inp = [parse_ingredient(i)
           for i in inp.strip().split(',')]

    out = [parse_ingredient(i)
           for i in out.strip().split(',')][0]
    return ((inp, out[1]), out[0])


def calc_necessary_ore(rules, target, t_amt, leftovers=None):
    if leftovers is None:
        leftovers = defaultdict(int)
    if target == 'ORE':
        return t_amt
    if t_amt < leftovers[target]:
        leftovers[target] -= t_amt
        return 0
    t_amt -= leftovers[target]
    leftovers[target] = 0

    ore_needed = 0
    ingredients, output_amt = rules[target]
    num_reactions = math.ceil(t_amt / output_amt)

    for ing, imp_amt in ingredients:
        imp_amt *= num_reactions
        ore_needed += calc_necessary_ore(rules, ing, imp_amt, leftovers)
    leftovers[target] = output_amt * num_reactions - t_amt
    return ore_needed


if __name__ == "__main__":
    with open('14.txt') as f:
        lines = f.readlines()
    rules = {out: recipe for (recipe, out) in [
        parse_dependency(l) for l in lines]}
    p1 = calc_necessary_ore(rules, 'FUEL', 1)
    print("Part 1: {}".format(p1))

    ore, fuel = 1000000000000, 0
    t_amt = ore // p1
    leftovers = defaultdict(int)
    while ore and t_amt:
        l_new = defaultdict(int, leftovers)
        ore_consumed = calc_necessary_ore(rules, 'FUEL', t_amt, l_new)
        if ore_consumed > ore:
            t_amt //= 2
        else:
            fuel += t_amt
            ore -= ore_consumed
            leftovers = l_new
    print("Part 2: {}".format(fuel))
