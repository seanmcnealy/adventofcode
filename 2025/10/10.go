package main

import (
	"bufio"
	"fmt"
	"io"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

const example = `[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
`

type machine struct {
	lights   []bool
	buttons  [][]bool
	joltages []int
}

func parse(file io.Reader) []machine {
	scanner := bufio.NewScanner(file)
	var answer = []machine{}

	for scanner.Scan() {
		var line = strings.Fields(scanner.Text())
		var lights = line[0]
		lights = lights[1 : len(lights)-1]
		var lightsBool = []bool{}
		for i, _ := range lights {
			lightsBool = append(lightsBool, lights[i] == '#')
		}
		var buttons = [][]bool{}
		for _, b := range line[1 : len(line)-1] {
			var bb = strings.Split(b[1:len(b)-1], ",")
			var bbb = []bool{}
			for i := range len(lights) {
				var ia = strconv.Itoa(i)
				var found = false
				for _, j := range bb {
					if j == ia {
						found = true
						break
					}
				}
				bbb = append(bbb, found)
			}
			buttons = append(buttons, bbb)
		}
		var j = line[len(line)-1][1 : len(line[len(line)-1])-1]
		var jstr = strings.Split(j, ",")
		var jints = []int{}
		for _, jj := range jstr {
			var jint, _ = strconv.Atoi(jj)
			jints = append(jints, jint)
		}

		answer = append(answer, machine{lightsBool, buttons, jints})
	}
	return answer
}

type state struct {
	lights []bool
	path   []int
}

func xor(a, b bool) bool {
	return (a || b) && !(a && b)
}

func part1(machines []machine) int {
	var answer = 0
	for _, m := range machines {
		var startState = []bool{}
		for range len(m.lights) {
			startState = append(startState, false)
		}
		var found = []state{{startState, []int{}}}
		for len(found) > 0 {
			var s = found[0]
			found = found[1:]

			var solved = true
			for i, _ := range s.lights {
				if s.lights[i] != m.lights[i] {
					solved = false
					break
				}
			}
			if solved {
				answer += len(s.path)
				break
			}

			for i := range len(m.buttons) {
				var nextState = []bool{}
				for j := range m.buttons[i] {
					nextState = append(nextState, xor(s.lights[j], m.buttons[i][j]))
				}
				found = append(found, state{nextState, append(s.path, i)})
			}
		}

	}

	return answer
}

type state2 struct {
	joltages []int
	presses  []int
	distance int
}

/*
*
Note that this doesn't work, I started using A* kind of search, but it's way too slow
for the input size. The solution I found uses scipy.
*/
func part2(machines []machine) int {
	var answer = 0
	for mii, m := range machines {
		var startState = []int{}
		var startPresses = []int{}
		for range len(m.lights) {
			startState = append(startState, 0)
		}
		for range len(m.buttons) {
			startPresses = append(startPresses, 0)
		}
		var search = []state2{{startState, startPresses, math.MaxInt}}
		var found = map[string]bool{}
		for len(search) > 0 {
			var s = search[0]
			search = search[1:]

			var solved = true
			for i, _ := range s.joltages {
				if s.joltages[i] != m.joltages[i] {
					solved = false
					break
				}
			}
			if solved {
				var a = 0
				for _, p := range s.presses {
					a += p
				}
				println("solved ", mii, a)
				answer += a
				break
			}

			var tooLarge = false
			for i, _ := range s.joltages {
				if s.joltages[i] > m.joltages[i] {
					tooLarge = true
					break
				}
			}
			if !tooLarge {
				for i := range len(m.buttons) {
					var nextState = []int{}
					var nextPresses = []int{}
					var pressString = ""
					var distance = 0
					for j := range m.joltages {
						var nextJoltage = s.joltages[j]
						if m.buttons[i][j] {
							nextJoltage += 1
						}
						nextState = append(nextState, nextJoltage)
						distance += m.joltages[j] - nextJoltage
					}
					distance /= 2
					for j := range m.buttons {
						var nextPress = s.presses[j]
						if j == i {
							nextPress += 1
						}
						distance += nextPress
						nextPresses = append(nextPresses, nextPress)
						pressString += strconv.Itoa(nextPress) + ","
					}
					if !found[pressString] {
						search = append(search, state2{nextState, nextPresses, distance})
						sort.Slice(search, func(i, j int) bool {
							return search[i].distance < search[j].distance
						})
						found[pressString] = true
					}
				}
			}
		}
	}

	return answer
}

func main() {
	var exampleFile = strings.NewReader(example)
	var ex = parse(exampleFile)
	file, err := os.Open("data/10")
	if err != nil {
		panic(err)
	}
	var fd = parse(file)

	var part1ExampleResult = part1(ex)
	if part1ExampleResult != 7 {
		panic("example 1 failed")
	}

	var part1Result = part1(fd)
	fmt.Printf("Part 1: %d\n", part1Result)

	var part2ExampleResult = part2(ex)
	if part2ExampleResult != 33 {
		panic("example 2 failed")
	}

	var part2Result = part2(fd)
	fmt.Printf("Part 2: %d\n", part2Result)
}
