package main

import (
	"bufio"
	"fmt"
	"io"
	"strconv"
	"strings"
)

const example = `0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
`

func parse(file io.Reader) ([][3][3]bool, [][2]int, [][]int) {
	scanner := bufio.NewScanner(file)
	var shapes = [][3][3]bool{}
	var boxes = [][2]int{}
	var requirements = [][]int{}

	var scanBoxes = false
	for scanner.Scan() {
		var line = scanner.Text()
		if !scanBoxes && !strings.ContainsRune(line, 'x') {
			var shape = [3][3]bool{}
			for i := range 3 {
				scanner.Scan()
				line = scanner.Text()
				for j := range 3 {
					if line[j] == '#' {
						shape[i][j] = true
					} else {
						shape[i][j] = false
					}
				}
			}
			scanner.Scan()
			shapes = append(shapes, shape)
		} else {
			scanBoxes = true
			var fields = strings.Fields(line)
			var sizeX, _ = strconv.Atoi(strings.Split(fields[0][:len(fields[0])-1], "x")[0])
			var sizeY, _ = strconv.Atoi(strings.Split(fields[0][:len(fields[0])-1], "x")[1])
			boxes = append(boxes, [2]int{sizeX, sizeY})

			var required = []int{}
			for _, i := range fields[1:] {
				var x, _ = strconv.Atoi(i)
				required = append(required, x)
			}
			requirements = append(requirements, required)
		}

	}
	return shapes, boxes, requirements
}

func isSameShape(shape1 [3][3]bool, shape2 [3][3]bool) bool {
	for i := range 3 {
		for j := range 3 {
			if shape1[i][j] != shape2[i][j] {
				return false
			}
		}
	}
	return true
}

func containsShape(shapes [][3][3]bool, shape [3][3]bool) bool {
	for _, s := range shapes {
		if isSameShape(s, shape) {
			return true
		}
	}
	return false
}

func rotateShape(shape [3][3]bool) [3][3]bool {
	var rotated = [3][3]bool{}
	for i := range 3 {
		for j := range 3 {
			rotated[j][2-i] = shape[i][j]
		}
	}
	return rotated
}

func flipShape(shape [3][3]bool) [3][3]bool {
	var flipped = [3][3]bool{}
	for i := range 3 {
		for j := range 3 {
			flipped[2-i][j] = shape[i][j]
		}
	}
	return flipped
}

func calcShapeDensities(shapes [][3][3]bool) []int {
	var shapeDensity = []int{}
	for _, shape := range shapes {
		var count = 0
		for _, line := range shape {
			for i := range len(line) {
				if line[i] {
					count++
				}
			}
		}
		shapeDensity = append(shapeDensity, count)
	}
	return shapeDensity
}

func recommendLocations(state state) [][2]int {
	var answer = [][2]int{}
	for si, s := range state.placedShapes {
		for i := range 3 {
			for j := range 3 {
				if !s[i][j] {
					answer = append(answer, [2]int{state.placedLocations[si][0] + i, state.placedLocations[si][1] + j})
				}
			}
		}
		answer = append(answer,
			[2]int{state.placedLocations[si][0] + 3, state.placedLocations[si][1] - 1})
		//[2]int{state.placedLocations[si][0] + 3, state.placedLocations[si][1]},
		//[2]int{state.placedLocations[si][0] + 3, state.placedLocations[si][1] + 1},
		//[2]int{state.placedLocations[si][0] - 1, state.placedLocations[si][1] + 3},
		//[2]int{state.placedLocations[si][0], state.placedLocations[si][1] + 3})
		//[2]int{state.placedLocations[si][0] + 1, state.placedLocations[si][1] + 3}
	}
	return answer
}

func doesFit(st state, shape [3][3]bool, x, y, maxx, maxy int) bool {
	if x < 0 || y < 0 {
		return false
	}
	for i, sh := range st.placedShapes {
		var loc = st.placedLocations[i]
		var dx = loc[0] - x
		var dy = loc[1] - y

		for i := range 3 {
			for j := range 3 {
				if sh[i][j] && dx+i >= 0 && dx+i < 3 && dy+j >= 0 && dy+j < 3 && shape[dx+i][dy+j] {
					return false
				}
				if sh[i][j] && x+i >= maxx || y+j >= maxy {
					return false
				}
			}
		}
	}
	return true
}

type state struct {
	placedShapes    [][3][3]bool
	placedLocations [][2]int
	requirement     []int
}

type shapeWithIndex struct {
	shape [3][3]bool
	index int
}

func availableShapes(shapeTransforms [][][3][3]bool, requirements []int) []shapeWithIndex {
	var available = []shapeWithIndex{}
	for i, needed := range requirements {
		if needed > 0 {
			for _, shape := range shapeTransforms[i] {
				available = append(available, shapeWithIndex{shape, i})
			}
		}
	}
	return available
}

func oneLessRequirement(requirements []int, index int) []int {
	var newRequirements = make([]int, len(requirements))
	copy(newRequirements, requirements)
	newRequirements[index] -= 1
	return newRequirements
}

func part1(shapes [][3][3]bool, boxes [][2]int, requirements [][]int) int {
	var answer = 0
	var shapeTransforms = [][][3][3]bool{}
	for _, shape := range shapes {
		var transformed = [][3][3]bool{shape}
		var flipped = flipShape(shape)
		if !containsShape(transformed, flipped) {
			transformed = append(transformed, flipped)
		}
		var rotated = rotateShape(shape)
		if !containsShape(transformed, rotated) {
			transformed = append(transformed, rotated)
		}
		flipped = flipShape(rotated)
		if !containsShape(transformed, flipped) {
			transformed = append(transformed, flipped)
		}
		rotated = rotateShape(rotated)
		if !containsShape(transformed, rotated) {
			transformed = append(transformed, rotated)
		}
		flipped = flipShape(rotated)
		if !containsShape(transformed, flipped) {
			transformed = append(transformed, flipped)
		}
		rotated = rotateShape(rotated)
		if !containsShape(transformed, rotated) {
			transformed = append(transformed, rotated)
		}
		flipped = flipShape(rotated)
		if !containsShape(transformed, flipped) {
			transformed = append(transformed, flipped)
		}
		shapeTransforms = append(shapeTransforms, transformed)
	}

	for i, box := range boxes {
		var requirement = requirements[i]

		var available = availableShapes(shapeTransforms, requirement)
		var search = []state{}
		for _, a := range available {
			search = append(search, state{[][3][3]bool{a.shape}, [][2]int{{0, 0}}, oneLessRequirement(requirement, a.index)})
		}
		for len(search) > 0 {
			var currentState = search[0]
			search = search[1:]

			var done = true
			for _, r := range currentState.requirement {
				if r > 0 {
					done = false
					break
				}
			}
			if done {
				answer += 1
				break
			}

			for _, loc := range recommendLocations(currentState) {
				for _, available := range availableShapes(shapeTransforms, currentState.requirement) {
					if doesFit(currentState, available.shape, loc[0], loc[1], box[0], box[1]) {
						search = append(search, state{
							append(currentState.placedShapes, available.shape),
							append(currentState.placedLocations, loc),
							oneLessRequirement(currentState.requirement, available.index),
						})
					}
				}
			}

		}

	}

	return answer
}

func main() {
	var exampleFile = strings.NewReader(example)
	var ex1, ex2, ex3 = parse(exampleFile)

	var part1ExampleResult = part1(ex1, ex2, ex3)
	if part1ExampleResult != 2 {
		panic("example 1 failed")
	}
	fmt.Println("example 1 passed")

}
