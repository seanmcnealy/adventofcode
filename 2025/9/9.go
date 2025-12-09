package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

const example = `7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
`

func parse(file io.Reader) [][2]int {
	scanner := bufio.NewScanner(file)
	var answer [][2]int = [][2]int{}

	for scanner.Scan() {
		var line = strings.Split(scanner.Text(), ",")
		var x, _ = strconv.Atoi(line[0])
		var y, _ = strconv.Atoi(line[1])
		answer = append(answer, [2]int{x, y})
	}
	return answer
}

func absInt(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func part1(coordinates [][2]int) int {
	var answer = 0
	for i := range len(coordinates) {
		for j := range len(coordinates) {
			var area = absInt(coordinates[i][0]-coordinates[j][0]+1) * absInt(coordinates[i][1]-coordinates[j][1]+1)
			if area > answer {
				answer = area
			}
		}
	}
	return answer
}

func isInsidePath(coordinates [][2]int, point [2]int) bool {
	var left = 0
	var right = 0
	var up = 0
	var down = 0

	// complete path
	coordinates = append(coordinates, coordinates[0])

	var last = coordinates[0]
	var goingDown = true
	var goingRight = true
	for _, next := range coordinates {
		if next[0] == point[0] && next[1] == point[1] {
			return true
		}
		if next[0] == last[0] && next[1] > last[1] {
			// path going right
			if last[0] == point[0] && last[1] <= point[1] && next[1] >= point[1] {
				return true
			}
			if last[1] <= point[1] && point[1] <= next[1] {
				if next[0] < point[0] || (next[0] == point[0] && !goingDown) {
					if (last[1] == point[1] && !goingRight) || (last[1] != point[1]) {
						up += 1
					}
				} else {
					if (last[1] == point[1] && !goingRight) || (last[1] != point[1]) {
						down -= 1
					}
				}
			}
			goingRight = true
		} else if next[0] == last[0] && next[1] < last[1] {
			// path going left
			if last[0] == point[0] && last[1] >= point[1] && next[1] <= point[1] {
				return true
			}
			if last[1] >= point[1] && point[1] >= next[1] {
				if next[0] < point[0] || (next[0] == point[0] && !goingDown) {
					if (last[1] == point[1] && goingRight) || (last[1] != point[1]) {
						up -= 1
					}
				} else {
					if (last[1] == point[1] && goingRight) || (last[1] != point[1]) {
						down += 1
					}
				}
			}
			goingRight = false
		} else if next[1] == last[1] && next[0] > last[0] {
			// path going down
			if last[1] == point[1] && last[0] <= point[0] && next[0] >= point[0] {
				return true
			}
			if last[0] <= point[0] && point[0] <= next[0] {
				if next[1] < point[1] || (next[1] == point[1] && !goingRight) {
					if (last[0] == point[0] && !goingDown) || (last[0] != point[0]) {
						left -= 1
					}
				} else {
					if (last[0] == point[0] && !goingDown) || (last[0] != point[0]) {
						right += 1
					}
				}
			}
			goingDown = true
		} else if next[1] == last[1] && next[0] < last[0] {
			// path going up
			if last[1] == point[1] && last[0] >= point[0] && next[0] <= point[0] {
				return true
			}
			if last[0] >= point[0] && point[0] >= next[0] {
				if next[1] < point[1] || (next[1] == point[1] && !goingRight) {
					if (last[0] == point[0] && goingDown) || (last[0] != point[0]) {
						left += 1
					}
				} else {
					if (last[0] == point[0] && goingDown) || (last[0] != point[0]) {
						right -= 1
					}
				}
			}
			goingDown = false
		}
		last = next
	}
	return up == -1 && down == -1 && left == -1 && right == -1
}

func part2(coordinates [][2]int) int {
	var answer = 0
	for i := range len(coordinates) {
		for j := range len(coordinates) {

			var topLeft = [2]int{min(coordinates[i][0], coordinates[j][0]), min(coordinates[i][1], coordinates[j][1])}
			var bottomRight = [2]int{max(coordinates[i][0], coordinates[j][0]), max(coordinates[i][1], coordinates[j][1])}

			var area = (absInt(coordinates[i][0]-coordinates[j][0]) + 1) * (absInt(coordinates[i][1]-coordinates[j][1]) + 1)
			if area > answer {
				var interestingPoints = [][2]int{}
				var last = coordinates[0]
				for _, p := range coordinates {
					var x = p[0] - 1
					var y = p[1] - 1
					if x >= topLeft[0] && y >= topLeft[1] && x <= bottomRight[0] && y <= bottomRight[1] {
						interestingPoints = append(interestingPoints, [2]int{x, y})
					}
					x = p[0] - 1
					y = p[1] + 1
					if x >= topLeft[0] && y >= topLeft[1] && x <= bottomRight[0] && y <= bottomRight[1] {
						interestingPoints = append(interestingPoints, [2]int{x, y})
					}
					x = p[0] + 1
					y = p[1] - 1
					if x >= topLeft[0] && y >= topLeft[1] && x <= bottomRight[0] && y <= bottomRight[1] {
						interestingPoints = append(interestingPoints, [2]int{x, y})
					}
					x = p[0] + 1
					y = p[1] + 1
					if x >= topLeft[0] && y >= topLeft[1] && x <= bottomRight[0] && y <= bottomRight[1] {
						interestingPoints = append(interestingPoints, [2]int{x, y})
					}
					x = p[0]
					y = p[1] - 1
					if x >= topLeft[0] && y >= topLeft[1] && x <= bottomRight[0] && y <= bottomRight[1] {
						interestingPoints = append(interestingPoints, [2]int{x, y})
					}
					x = p[0]
					y = p[1] + 1
					if x >= topLeft[0] && y >= topLeft[1] && x <= bottomRight[0] && y <= bottomRight[1] {
						interestingPoints = append(interestingPoints, [2]int{x, y})
					}
					x = p[0] - 1
					y = p[1]
					if x >= topLeft[0] && y >= topLeft[1] && x <= bottomRight[0] && y <= bottomRight[1] {
						interestingPoints = append(interestingPoints, [2]int{x, y})
					}
					x = p[0] + 1
					y = p[1]
					if x >= topLeft[0] && y >= topLeft[1] && x <= bottomRight[0] && y <= bottomRight[1] {
						interestingPoints = append(interestingPoints, [2]int{x, y})
					}

					if p[0] > bottomRight[0] && last[0] < topLeft[0] && p[1] > topLeft[1] && p[1] < bottomRight[1] {
						interestingPoints = append(interestingPoints, [2]int{topLeft[0] + 1, p[1] - 1})
						interestingPoints = append(interestingPoints, [2]int{topLeft[0] + 1, p[1] + 1})
						interestingPoints = append(interestingPoints, [2]int{bottomRight[0] - 1, p[1] - 1})
						interestingPoints = append(interestingPoints, [2]int{bottomRight[0] - 1, p[1] + 1})
					}
					if p[0] < topLeft[0] && last[0] > bottomRight[0] && p[1] > topLeft[1] && p[1] < bottomRight[1] {
						interestingPoints = append(interestingPoints, [2]int{topLeft[0] + 1, p[1] - 1})
						interestingPoints = append(interestingPoints, [2]int{topLeft[0] + 1, p[1] + 1})
						interestingPoints = append(interestingPoints, [2]int{bottomRight[0] - 1, p[1] - 1})
						interestingPoints = append(interestingPoints, [2]int{bottomRight[0] - 1, p[1] + 1})
					}

					if p[1] > bottomRight[1] && last[1] < topLeft[1] && p[0] > topLeft[0] && p[0] < bottomRight[0] {
						interestingPoints = append(interestingPoints, [2]int{p[0] - 1, topLeft[1] + 1})
						interestingPoints = append(interestingPoints, [2]int{p[0] + 1, topLeft[1] + 1})
						interestingPoints = append(interestingPoints, [2]int{p[0] - 1, bottomRight[1] - 1})
						interestingPoints = append(interestingPoints, [2]int{p[0] + 1, bottomRight[1] - 1})
					}
					if p[1] < topLeft[1] && last[1] > bottomRight[1] && p[0] > topLeft[0] && p[0] < bottomRight[0] {
						interestingPoints = append(interestingPoints, [2]int{p[0] - 1, topLeft[1] + 1})
						interestingPoints = append(interestingPoints, [2]int{p[0] + 1, topLeft[1] + 1})
						interestingPoints = append(interestingPoints, [2]int{p[0] - 1, bottomRight[1] - 1})
						interestingPoints = append(interestingPoints, [2]int{p[0] + 1, bottomRight[1] - 1})
					}

					last = p
				}
				var allInside = true
				for _, p := range interestingPoints {
					if !isInsidePath(coordinates, p) {
						allInside = false
						break
					}
				}
				if allInside {
					answer = area
				}
			}
			//}
		}
	}
	return answer
}

func main() {
	var exampleFile = strings.NewReader(example)
	var ex = parse(exampleFile)
	file, err := os.Open("data/9")
	if err != nil {
		panic(err)
	}
	var fd = parse(file)

	var part1ExampleResult = part1(ex)
	if part1ExampleResult != 50 {
		panic("example 1 failed")
	}

	var part1Result = part1(fd)
	fmt.Printf("Part 1: %d\n", part1Result)

	var part2ExampleResult = part2(ex)
	if part2ExampleResult != 24 {
		panic("example 2 failed")
	}

	var part2Result = part2(fd)
	fmt.Printf("Part 2: %d\n", part2Result)
}
