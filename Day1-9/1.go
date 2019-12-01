package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func fuelReq(module int) int {
	if requirement := (module / 3) - 1; requirement > 0 {
		return requirement
	}
	return 0
}

func part1(modules []int) int {
	out := 0
	for _, m := range modules {
		out += fuelReq(m)
	}
	return out
}

func part2(modules []int) int {
	out := 0
	for len(modules) > 0 {
		var m int
		m, modules = modules[len(modules)-1], modules[:len(modules)-1]
		if fuel := fuelReq(m); fuel > 0 {
			out += fuel
			modules = append(modules, fuel)
		}
	}
	return out
}

func check(err error) {
	if err != nil {
		panic(err)
	}
}

func main() {
	input, err := ioutil.ReadFile("1.txt")
	check(err)
	lines := strings.Split(string(input), "\n")

	modules := make([]int, len(lines))
	for idx, l := range lines {
		modules[idx], _ = strconv.Atoi(l)
	}

	fmt.Printf("Part 1: %d\n", part1(modules))
	fmt.Printf("Part 2: %d\n", part2(modules))
}
