package main

import (
	"bufio"
	"fmt"
	"io"
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
)

const example = `162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
`

func parse(file io.Reader) [][3]int {
	scanner := bufio.NewScanner(file)
	var answer [][3]int
	for scanner.Scan() {
		var s = strings.Split(scanner.Text(), ",")
		var s0, _ = strconv.Atoi(s[0])
		var s1, _ = strconv.Atoi(s[1])
		var s2, _ = strconv.Atoi(s[2])
		answer = append(answer, [3]int{s0, s1, s2})
	}
	return answer
}

func distance(a [3]int, b [3]int) int {
	return int(math.Sqrt(float64(
		(a[0]-b[0])*(a[0]-b[0]) +
			(a[1]-b[1])*(a[1]-b[1]) +
			(a[2]-b[2])*(a[2]-b[2]))))
}

func part1(data [][3]int, connections int) int {
	var distances = make([][]int, len(data))
	var connected = make([][]bool, len(data))

	for i := range data {
		distances[i] = make([]int, len(data))
		connected[i] = make([]bool, len(data))

		for j := range data {
			if i == j {
				distances[i][j] = math.MaxInt
			} else {
				distances[i][j] = distance(data[i], data[j])
			}
		}
	}

	for range connections {
		var minx = 0
		var miny = 0
		var minValue = math.MaxInt
		for i := range len(distances) {
			for j := range len(distances) {
				if distances[i][j] < minValue {
					minx = i
					miny = j
					minValue = distances[i][j]
				}
			}
		}

		connected[minx][miny] = true
		connected[miny][minx] = true
		distances[minx][miny] = math.MaxInt
		distances[miny][minx] = math.MaxInt

		var found []int = []int{minx, miny}
		var queue []int = []int{minx, miny}
		for len(queue) > 0 {
			var x = queue[0]
			queue = queue[1:]
			for j := range len(connected) {
				if connected[x][j] == true && slices.Contains(found, j) == false {
					found = append(found, j)
					queue = append(queue, j)

					for _, k := range found {
						connected[j][k] = true
						connected[k][j] = true
						//distances[j][k] = math.MaxInt
						//distances[k][j] = math.MaxInt
					}
				}
			}
		}
	}

	var acc = []int{}
	for i := range len(connected) {
		var startsGroup = true
		for _, c := range connected[i][0:i] {
			if c {
				startsGroup = false
				break
			}
		}
		if startsGroup {
			var groupSize = 0
			for j, c := range connected[i] {
				if c || i == j {
					groupSize += 1
				}
			}
			acc = append(acc, groupSize)
		}
	}
	slices.Sort(acc)
	return acc[len(acc)-1] * acc[len(acc)-2] * acc[len(acc)-3]
}

func fullyConnected(data [][]bool) bool {
	answer := true
	for i := range data[0] {
		if !data[0][i] {
			answer = false
			break
		}
	}
	return answer
}

func part2(data [][3]int, connections int) int {
	var distances = make([][]int, len(data))
	var connected = make([][]bool, len(data))

	for i := range data {
		distances[i] = make([]int, len(data))
		connected[i] = make([]bool, len(data))

		for j := range data {
			if i == j {
				distances[i][j] = math.MaxInt
				connected[i][i] = true
			} else {
				distances[i][j] = distance(data[i], data[j])
			}
		}
	}

	var finalx = -1
	var finaly = -1
	for !fullyConnected(connected) {
		var minx = 0
		var miny = 0
		var minValue = math.MaxInt
		for i := range len(distances) {
			for j := range len(distances) {
				if distances[i][j] < minValue {
					minx = i
					miny = j
					minValue = distances[i][j]
				}
			}
		}

		connected[minx][miny] = true
		connected[miny][minx] = true
		distances[minx][miny] = math.MaxInt
		distances[miny][minx] = math.MaxInt
		finalx = minx
		finaly = miny

		var found []int = []int{minx, miny}
		var queue []int = []int{minx, miny}
		for len(queue) > 0 {
			var x = queue[0]
			queue = queue[1:]
			for j := range len(connected) {
				if connected[x][j] == true && slices.Contains(found, j) == false {
					found = append(found, j)
					queue = append(queue, j)

					for _, k := range found {
						connected[j][k] = true
						connected[k][j] = true
						distances[j][k] = math.MaxInt
						distances[k][j] = math.MaxInt
					}
				}
			}
		}
	}

	return data[finalx][0] * data[finaly][0]
}

func main() {
	var exampleFile = strings.NewReader(example)
	var exampleData = parse(exampleFile)
	file, err := os.Open("data/8")
	if err != nil {
		panic(err)
	}
	var fileData = parse(file)

	var part1ExampleResult = part1(exampleData, 10)
	if part1ExampleResult != 40 {
		panic("example 1 failed")
	}

	var part1Result = part1(fileData, 1000)
	fmt.Printf("Part 1: %d\n", part1Result)

	var part2ExampleResult = part2(exampleData, 10)
	if part2ExampleResult != 25272 {
		panic("example 2 failed")
	}

	var part2Result = part2(fileData, 1000)
	fmt.Printf("Part 2: %d\n", part2Result)
}
