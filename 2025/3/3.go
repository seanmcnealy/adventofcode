package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

const example = `987654321111111
811111111111119
234234234234278
818181911112111
`

func parse(file io.Reader) []string {
	scanner := bufio.NewScanner(file)
	var answer []string
	for scanner.Scan() {
		answer = append(answer, scanner.Text())
	}
	return answer
}

func highest(data string) (int, int) {
	var answerIdx = 0
	var answerVal = -1
	for i, d := range data {
		var dInt, _ = strconv.Atoi(string(d))
		if dInt > answerVal {
			answerIdx = i
			answerVal = dInt
		}
	}
	return answerIdx, answerVal
}

func part1(data []string) int {
	var acc = 0

	for _, d := range data {
		var idx, val = highest(d[:len(d)-1])
		var _, val2 = highest(d[idx+1:])
		var highest, _ = strconv.Atoi(strconv.Itoa(val) + strconv.Itoa(val2))
		acc += highest
	}
	return acc
}

func part2(data []string) int {
	var acc = 0

	for _, d := range data {
		var idx = -1
		var answer = ""
		for i := range 12 {
			var nextIdx, nextVal = highest(d[idx+1 : len(d)-11+i])
			idx = idx + 1 + nextIdx
			answer = answer + strconv.Itoa(nextVal)
		}

		var highest, _ = strconv.Atoi(answer)
		acc += highest
	}

	return acc
}

func main() {
	var exampleFile = strings.NewReader(example)
	var exampleData = parse(exampleFile)
	file, err := os.Open("data/3")
	if err != nil {
		panic(err)
	}
	var fileData = parse(file)

	var part1ExampleResult = part1(exampleData)
	if part1ExampleResult != 357 {
		panic("example 1 failed")
	}

	var part1Result = part1(fileData)
	fmt.Printf("Part 1: %d\n", part1Result)

	var part2ExampleResult = part2(exampleData)
	if part2ExampleResult != 3121910778619 {
		panic("example 2 failed")
	}

	var part2Result = part2(fileData)
	fmt.Printf("Part 2: %d\n", part2Result)
}
