package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

const example = `L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
`

type tuple struct {
	_1 uint8
	_2 int
}

func parse(file io.Reader) []tuple {
	scanner := bufio.NewScanner(file)
	var answer []tuple
	for scanner.Scan() {
		ff := strings.Fields(scanner.Text())

		n1 := ff[0][0]
		n2, _ := strconv.Atoi(ff[0][1:])

		answer = append(answer, tuple{n1, n2})
	}
	return answer
}

func part1(data []tuple) int {
	var dial = 50
	var zeros = 0
	for _, v := range data {
		if v._1 == 'L' {
			dial -= v._2
		} else {
			dial += v._2
		}
		dial = dial % 100
		if dial == 0 {
			zeros += 1
		}
	}
	return zeros
}

func part2(data []tuple) int {
	var dial = 50
	var zeros = 0
	for _, v := range data {
		var dir = v._1
		var amt = v._2
		for amt >= 100 {
			zeros += 1
			amt -= 100
		}
		if dir == 'L' {
			if amt >= dial && dial != 0 {
				zeros += 1
			}
			dial -= amt
			if dial < 0 {
				dial += 100
			}
		} else {
			if amt >= 100-dial {
				zeros += 1
			}
			dial += amt
			if dial >= 100 {
				dial -= 100
			}
		}
	}
	return zeros
}

func main() {
	var exampleFile = strings.NewReader(example)
	var exampleData = parse(exampleFile)
	file, err := os.Open("data/1")
	if err != nil {
		panic(err)
	}
	var fileData = parse(file)

	var part1ExampleResult = part1(exampleData)
	if part1ExampleResult != 3 {
		panic("example 1 failed")
	}

	var part1Result = part1(fileData)
	fmt.Printf("Part 1: %d\n", part1Result)

	var part2ExampleResult = part2(exampleData)
	if part2ExampleResult != 6 {
		panic("example 2 failed")
	}

	var part2Result = part2(fileData)
	fmt.Printf("Part 2: %d\n", part2Result)
}
