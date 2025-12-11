package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"slices"
	"strings"
)

const example = `aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
`

const example2 = `svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
`

func parse(file io.Reader) map[string][]string {
	scanner := bufio.NewScanner(file)
	var answer = map[string][]string{}

	for scanner.Scan() {
		var line = strings.Fields(scanner.Text())

		answer[line[0][0:len(line[0])-1]] = line[1:]
	}
	return answer
}

func part1(machines map[string][]string) int {
	var answer = 0

	var queue = []string{"you"}
	for len(queue) > 0 {
		var m = queue[0]
		queue = queue[1:]
		for _, next := range machines[m] {
			queue = append(queue, next)
		}
		if m == "out" {
			answer += 1
		}
	}

	return answer
}

func paths(machines map[string][]string, from string, to string, avoid []string) int {
	var queue = [][]string{}
	var memos = map[string]int{}
	memos[to] = 1
	avoid = append(avoid, to)

	for !slices.Contains(avoid, from) {
		queue = [][]string{{from}}
		for len(queue) > 0 {
			var path = queue[0]
			queue = queue[1:]
			var m = path[len(path)-1]

			if slices.Contains(avoid, m) {
				continue
			}

			var addedAny = false
			var memoize = 0
			for _, next := range machines[m] {
				if !slices.Contains(avoid, next) {
					queue = append([][]string{append(path, next)}, queue...)
					addedAny = true
				} else {
					memoize += memos[next]
				}
			}

			if !addedAny {
				avoid = append(avoid, m)
				memos[m] = memoize
			}
		}
	}
	return memos[from]
}

func part2(machines map[string][]string) int {
	var sdfo = paths(machines, "svr", "dac", []string{"fft", "out"}) *
		paths(machines, "dac", "fft", []string{"svr", "out"}) *
		paths(machines, "fft", "out", []string{"svr", "dac"})
	var sfdo = paths(machines, "svr", "fft", []string{"dac", "out"}) *
		paths(machines, "fft", "dac", []string{"svr", "out"}) *
		paths(machines, "dac", "out", []string{"svr", "fft"})
	return sdfo + sfdo
}

func main() {
	var exampleFile = strings.NewReader(example)
	var ex = parse(exampleFile)
	file, err := os.Open("data/11")
	if err != nil {
		panic(err)
	}
	var fd = parse(file)

	var part1ExampleResult = part1(ex)
	if part1ExampleResult != 5 {
		panic("example 1 failed")
	}

	var part1Result = part1(fd)
	fmt.Printf("Part 1: %d\n", part1Result)

	var part2ExampleResult = part2(parse(strings.NewReader(example2)))
	if part2ExampleResult != 2 {
		panic("example 2 failed")
	}

	var part2Result = part2(fd)
	fmt.Printf("Part 2: %d\n", part2Result)
}
