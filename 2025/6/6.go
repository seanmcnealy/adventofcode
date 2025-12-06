package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

const example = `123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
`

func parse(file io.Reader) ([][]int, []uint8) {
	scanner := bufio.NewScanner(file)
	var grid [][]int = [][]int{}
	var ops []uint8 = []uint8{}

	for scanner.Scan() {
		var line = strings.Fields(scanner.Text())
		if line[0] == "+" || line[0] == "*" {
			var lineResult []uint8 = []uint8{}
			for _, i := range line {
				lineResult = append(lineResult, i[0])
			}
			ops = lineResult
		} else {
			var lineResult []int = []int{}
			for _, i := range line {
				var ii, _ = strconv.Atoi(i)
				lineResult = append(lineResult, ii)
			}
			grid = append(grid, lineResult)
		}
	}
	return grid, ops
}

func parse2(file io.Reader) ([][]string, []uint8) {
	scanner := bufio.NewScanner(file)
	var grid []string = []string{}
	var gridanswer [][]string = [][]string{}
	var ops []uint8 = []uint8{}

	for scanner.Scan() {
		var linetext = scanner.Text()
		var line = strings.Fields(linetext)
		if line[0] == "+" || line[0] == "*" {
			var last = -1
			for i := range len(linetext) {
				var op = linetext[i]
				if op == '+' || op == '*' {
					ops = append(ops, op)
					if last != -1 {
						for j := range len(grid) {
							gridanswer[j] = append(gridanswer[j], grid[j][last:i-1])
						}
					}
					last = i
				}
			}
			for j := range len(grid) {
				gridanswer[j] = append(gridanswer[j], grid[j][last:len(grid[0])])
			}
		} else {
			grid = append(grid, linetext)
			gridanswer = append(gridanswer, []string{})
		}
	}
	return gridanswer, ops
}

func part1(grid [][]int, ops []uint8) int {
	var answer = 0
	for i, op := range ops {
		var acc = 0
		if op == '*' {
			acc = 1
		}
		for j := range len(grid) {
			if op == '+' {
				acc += grid[j][i]
			} else {
				acc *= grid[j][i]
			}
		}
		answer += acc
	}
	return answer
}

func part2(grid [][]string, ops []uint8) int {
	var answer = 0
	for j, op := range ops {
		var acc = 0
		if op == '*' {
			acc = 1
		}
		for k := range len(grid[0][j]) {
			var vert = ""
			for i := range len(grid) {
				if grid[i][j][k] != ' ' {
					vert = vert + grid[i][j][k:k+1]
				}
			}
			var x, _ = strconv.Atoi(vert)
			if op == '+' {
				acc += x
			} else {
				acc *= x
			}
		}
		answer += acc
	}
	return answer
}

func main() {
	var exampleFile = strings.NewReader(example)
	var ex1, ex2 = parse(exampleFile)
	file, err := os.Open("data/6")
	if err != nil {
		panic(err)
	}
	var fd1, fd2 = parse(file)

	var part1ExampleResult = part1(ex1, ex2)
	if part1ExampleResult != 4277556 {
		panic("example 1 failed")
	}

	var part1Result = part1(fd1, fd2)
	fmt.Printf("Part 1: %d\n", part1Result)

	var ex21, ex22 = parse2(strings.NewReader(example))
	file, _ = os.Open("data/6")
	var fd21, fd22 = parse2(file)
	var part2ExampleResult = part2(ex21, ex22)
	if part2ExampleResult != 3263827 {
		panic("example 2 failed")
	}

	var part2Result = part2(fd21, fd22)
	fmt.Printf("Part 2: %d\n", part2Result)
}
