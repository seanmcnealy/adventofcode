package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strings"
)

const example = `..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
`

func parse(file io.Reader) []string {
	scanner := bufio.NewScanner(file)
	var answer []string
	for scanner.Scan() {
		answer = append(answer, scanner.Text())
	}
	return answer
}

var directions = [][]int{
	{-1, -1}, {-1, 0}, {-1, 1},
	{0, -1}, {0, 1},
	{1, -1}, {1, 0}, {1, 1},
}

func part1(data []string) int {
	var acc = 0

	for i, d := range data {
		for j := range len(d) {
			var nearby = 0
			for _, dir := range directions {
				if data[i][j] == '@' && 0 <= i+dir[0] && i+dir[0] < len(data) && 0 <= j+dir[1] && j+dir[1] < len(d) && data[i+dir[0]][j+dir[1]] == '@' {
					nearby += 1
				}
			}
			if data[i][j] == '@' && nearby < 4 {
				acc += 1
			}
		}
	}
	return acc
}

func part2(data []string) int {
	var lastacc = -1
	var acc = 0

	for lastacc != acc {
		lastacc = acc
		for i, d := range data {
			for j := range len(d) {
				var nearby = 0
				for _, dir := range directions {
					if data[i][j] == '@' && 0 <= i+dir[0] && i+dir[0] < len(data) && 0 <= j+dir[1] && j+dir[1] < len(d) && data[i+dir[0]][j+dir[1]] == '@' {
						nearby += 1
					}
				}
				if data[i][j] == '@' && nearby < 4 {
					acc += 1
					data[i] = data[i][:j] + "." + data[i][j+1:]
				}
			}
		}
	}
	return acc
}

func main() {
	var exampleFile = strings.NewReader(example)
	var exampleData = parse(exampleFile)
	file, err := os.Open("data/4")
	if err != nil {
		panic(err)
	}
	var fileData = parse(file)

	var part1ExampleResult = part1(exampleData)
	if part1ExampleResult != 13 {
		panic("example 1 failed")
	}

	var part1Result = part1(fileData)
	fmt.Printf("Part 1: %d\n", part1Result)

	var part2ExampleResult = part2(exampleData)
	if part2ExampleResult != 43 {
		panic("example 2 failed")
	}

	var part2Result = part2(fileData)
	fmt.Printf("Part 2: %d\n", part2Result)
}
