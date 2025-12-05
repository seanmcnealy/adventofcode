package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

const example = `3-5
10-14
16-20
12-18

1
5
8
11
17
32
`

func parse(file io.Reader) ([]int, []int, []int) {
	scanner := bufio.NewScanner(file)
	var a []int
	var b []int
	var c []int

	var mode = 0
	for scanner.Scan() {
		if mode == 0 {
			var line = strings.Split(scanner.Text(), "-")
			if line[0] == "" {
				mode = 1
			} else {
				var from, _ = strconv.Atoi(line[0])
				var to, _ = strconv.Atoi(line[1])
				a = append(a, from)
				b = append(b, to)
			}
		} else {
			var line = scanner.Text()
			var ing, _ = strconv.Atoi(line)
			c = append(c, ing)
		}

	}
	return a, b, c
}

func part1(from []int, to []int, ing []int) int {
	var acc = 0
	for _, i := range ing {
		var good = false
		for j := range len(from) {
			if from[j] <= i && i <= to[j] {
				good = true
				break
			}
		}
		if good {
			acc += 1
		}
	}
	return acc
}

func part2(from []int, to []int, _ []int) int {
	var ranges = map[int]int{}

	for i := range len(from) {
		var f = from[i]
		var t = to[i]

		for k, v := range ranges {
			if f <= k && t >= v {
				delete(ranges, k)
			} else if f >= k && t <= v {
				delete(ranges, k)
				f = k
				t = v
			} else if t >= k && t <= v {
				delete(ranges, k)
				t = v
			} else if f >= k && f <= v {
				delete(ranges, k)
				f = k
			}
		}
		ranges[f] = t
	}

	var acc = 0
	for k, v := range ranges {
		acc += v - k + 1
	}
	return acc
}

func main() {
	var exampleFile = strings.NewReader(example)
	var ex1, ex2, ex3 = parse(exampleFile)
	file, err := os.Open("data/5")
	if err != nil {
		panic(err)
	}
	var fd1, fd2, fd3 = parse(file)

	var part1ExampleResult = part1(ex1, ex2, ex3)
	if part1ExampleResult != 3 {
		panic("example 1 failed")
	}

	var part1Result = part1(fd1, fd2, fd3)
	fmt.Printf("Part 1: %d\n", part1Result)

	var part2ExampleResult = part2(ex1, ex2, ex3)
	if part2ExampleResult != 14 {
		panic("example 2 failed")
	}

	var part2Result = part2(fd1, fd2, fd3)
	fmt.Printf("Part 2: %d\n", part2Result)
}
