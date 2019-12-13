package main

import (
	"fmt"
	"io/ioutil"
	"regexp"
	"strconv"
	"strings"
)

var inRegex = regexp.MustCompile(`<x=(.*), y=(.*), z=(.*)>`)

type Moon struct {
	x  int
	y  int
	z  int
	vx int
	vy int
	vz int
}

func (m *Moon) step() {
	m.x += m.vx
	m.y += m.vy
	m.z += m.vz
}

func (m *Moon) energy() int {
	return (abs(m.vx) + abs(m.vy) + abs(m.vz)) * (abs(m.x) + abs(m.y) + abs(m.z))
}

func abs(i int) int {
	if i < 0 {
		return -i
	}
	return i
}

func (m *Moon) String() string {
	return fmt.Sprintf(
		"pos=<x=%d, y=%d, z=%d>, vel=<x=%d, y=%d, z=%d>",
		m.x, m.y, m.z, m.vx, m.vy, m.vz)
}

func check(err error) {
	if err != nil {
		panic(err)
	}
}

func gravityForce(p1, p2 int) (int, int) {
	if p1 == p2 {
		return 0, 0
	}
	if p1 < p2 {
		return 1, -1
	}
	return -1, 1
}

func applyGravity(m []*Moon) {
	for i1, m1 := range m {
		for _, m2 := range m[i1+1:] {
			dx1, dx2 := gravityForce(m1.x, m2.x)
			m1.vx += dx1
			m2.vx += dx2

			dy1, dy2 := gravityForce(m1.y, m2.y)
			m1.vy += dy1
			m2.vy += dy2

			dz1, dz2 := gravityForce(m1.z, m2.z)
			m1.vz += dz1
			m2.vz += dz2
		}
	}
}

func part1(moons []*Moon) {
	for i := 0; i < 1000; i++ {
		applyGravity(moons)
		for _, m := range moons {
			m.step()
		}
	}
	var e int
	for _, m := range moons {
		e += m.energy()
	}
	fmt.Printf("Part 1: %d\n", e)
}

func hashMoons(moons []*Moon, keyFunc func(*Moon) (int, int)) string {
	var h string
	for _, m := range moons {
		pos, vel := keyFunc(m)
		h += fmt.Sprintf("%d;%d;", pos, vel)
	}
	return h
}

// https://play.golang.org/p/SmzvkDjYlb
func GCD(a, b int) int {
	for b != 0 {
		t := b
		b = a % b
		a = t
	}
	return a
}

// https://play.golang.org/p/SmzvkDjYlb
func LCM(a, b int, integers ...int) int {
	result := a * b / GCD(a, b)

	for i := 0; i < len(integers); i++ {
		result = LCM(result, integers[i])
	}

	return result
}

func part2(moons []*Moon) {
	xRep, yRep, zRep := -1, -1, -1
	seenX, seenY, seenZ := make(map[string]bool), make(map[string]bool), make(map[string]bool)
	for i := 0; true; i++ {
		applyGravity(moons)
		for _, m := range moons {
			m.step()
		}
		if xRep > 0 && yRep > 0 && zRep > 0 {
			break
		}
		if xRep <= 0 {
			xHash := hashMoons(moons, func(m *Moon) (int, int) { return m.x, m.vx })
			if _, ok := seenX[xHash]; ok {
				xRep = i
			} else {
				seenX[xHash] = true
			}
		}
		if yRep <= 0 {
			yHash := hashMoons(moons, func(m *Moon) (int, int) { return m.y, m.vy })
			if _, ok := seenY[yHash]; ok {
				yRep = i
			} else {
				seenY[yHash] = true
			}
		}
		if zRep <= 0 {
			zHash := hashMoons(moons, func(m *Moon) (int, int) { return m.z, m.vz })
			if _, ok := seenZ[zHash]; ok {
				zRep = i
			} else {
				seenZ[zHash] = true
			}
		}
	}
	fmt.Printf("Part 2: %d\n", LCM(xRep, yRep, zRep))
}

func getInput() []*Moon {
	data, err := ioutil.ReadFile("12-test.txt")
	check(err)

	var moons []*Moon
	for _, line := range strings.Split(string(data), "\n") {
		matches := inRegex.FindStringSubmatch(line)
		if len(matches) < 2 {
			continue
		}

		x, _ := strconv.Atoi(matches[1])
		y, _ := strconv.Atoi(matches[2])
		z, _ := strconv.Atoi(matches[3])
		moons = append(moons, &Moon{x: x, y: y, z: z})
	}
	return moons
}

func main() {
	part1(getInput())
	part2(getInput())
}
