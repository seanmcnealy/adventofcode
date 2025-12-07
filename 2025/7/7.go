package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strings"
)

const example = `.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
`

func parse(file io.Reader) []string {
	scanner := bufio.NewScanner(file)
	var answer []string
	for scanner.Scan() {
		answer = append(answer, scanner.Text())
	}
	return answer
}

func part1(data []string) int {
	var acc = 0

	var beams map[int]bool = map[int]bool{}
	var nextBeams map[int]bool = map[int]bool{}
	for i := range len(data[0]) {
		if data[0][i] == 'S' {
			nextBeams[i] = true
		}
	}

	for i := 1; i < len(data); i++ {
		beams = nextBeams
		nextBeams = map[int]bool{}
		for j := range len(data[0]) {
			if data[i][j] == '.' && beams[j] == true {
				nextBeams[j] = true
			} else if data[i][j] == '^' && beams[j] == true {
				nextBeams[j-1] = true
				nextBeams[j+1] = true
				acc += 1
			}
		}
	}

	return acc
}

func part2(data []string) int {
	var acc = 0

	var beams map[int]int = map[int]int{}
	var nextBeams map[int]int = map[int]int{}
	for i := range len(data[0]) {
		if data[0][i] == 'S' {
			nextBeams[i] = 1
		}
	}

	for i := 1; i < len(data); i++ {
		beams = nextBeams
		nextBeams = map[int]int{}
		for j := range len(data[0]) {
			var above, exists = beams[j]
			if data[i][j] == '.' && exists {
				nextBeams[j] += above
			} else if data[i][j] == '^' && exists {
				nextBeams[j-1] += above
				nextBeams[j+1] += above
				acc += above
			}
		}
	}

	return acc + 1
}

func main() {
	var exampleFile = strings.NewReader(example)
	var exampleData = parse(exampleFile)
	file, err := os.Open("data/7")
	if err != nil {
		panic(err)
	}
	var fileData = parse(file)

	var part1ExampleResult = part1(exampleData)
	if part1ExampleResult != 21 {
		panic("example 1 failed")
	}

	var part1Result = part1(fileData)
	fmt.Printf("Part 1: %d\n", part1Result)

	var part2ExampleResult = part2(exampleData)
	if part2ExampleResult != 40 {
		panic("example 2 failed")
	}

	var part2Result = part2(fileData)
	fmt.Printf("Part 2: %d\n", part2Result)
}
