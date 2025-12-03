package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

const example = `11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124`

type tuple struct {
	_1 string
	_2 string
}

func parse(file io.Reader) []tuple {
	scanner := bufio.NewScanner(file)
	var answer []tuple
	scanner.Scan()
	var line = scanner.Text()
	for _, r := range strings.Split(line, ",") {
		var beginAndEnd = strings.Split(r, "-")
		var begin = beginAndEnd[0]
		var end = beginAndEnd[1]
		answer = append(answer, tuple{begin, end})
	}

	return answer
}

func part1(data []tuple) int {
	var acc = 0
	for _, d := range data {
		var start = d._1
		var end = d._2
		var startInt, _ = strconv.Atoi(d._1)
		var endInt, _ = strconv.Atoi(d._2)

		var startFirstHalf, _ = strconv.Atoi(start[:len(start)/2])
		var endFirstHalf, _ = strconv.Atoi(end[:(len(end)+1)/2])

		for startFirstHalf <= endFirstHalf {
			var possible, _ = strconv.Atoi(strconv.Itoa(startFirstHalf) + strconv.Itoa(startFirstHalf))
			if possible >= startInt && possible <= endInt {
				acc += possible
			}
			startFirstHalf += 1
		}
	}
	return acc
}

func part2(data []tuple) int {
	var acc = map[int]bool{}
	for _, d := range data {
		var start = d._1
		var end = d._2
		var startInt, _ = strconv.Atoi(d._1)
		var endInt, _ = strconv.Atoi(d._2)
		var differentLength = len(end) - len(start)

		var primes = []int{2, 3, 5, 7}
		for i := range len(start) / 2 {
			var groupInt, _ = strconv.Atoi(start[:i+1])
			var groupIntTarget, _ = strconv.Atoi(end[:i+1+differentLength])

			for groupInt <= groupIntTarget {
				var group = strconv.Itoa(groupInt)
				for _, p := range primes {
					var possible, _ = strconv.Atoi(strings.Repeat(group, p))
					if possible >= startInt && possible <= endInt {
						acc[possible] = true
					}
				}
				groupInt += 1
			}
		}
		if differentLength > 0 {
			for i := range len(end) / 2 {
				var groupInt, _ = strconv.Atoi(("1" + strings.Repeat("0", len(start)))[:i+1])
				var groupIntTarget, _ = strconv.Atoi(end[:i+1])

				for groupInt <= groupIntTarget {
					var group = strconv.Itoa(groupInt)
					for _, p := range primes {
						var possible, _ = strconv.Atoi(strings.Repeat(group, p))
						if possible >= startInt && possible <= endInt {
							acc[possible] = true
						}
					}
					groupInt += 1
				}
			}
		}
	}
	var result = 0
	for k := range acc {
		result += k
	}
	return result
}

func main() {
	var exampleFile = strings.NewReader(example)
	var exampleData = parse(exampleFile)
	file, err := os.Open("data/2")
	if err != nil {
		panic(err)
	}
	var fileData = parse(file)

	var part1ExampleResult = part1(exampleData)
	if part1ExampleResult != 1227775554 {
		panic("example 1 failed")
	}

	var part1Result = part1(fileData)
	fmt.Printf("Part 1: %d\n", part1Result)

	var part2ExampleResult = part2(exampleData)
	if part2ExampleResult != 4174379265 {
		panic("example 2 failed")
	}

	var part2Result = part2(fileData)
	fmt.Printf("Part 2: %d\n", part2Result)
}
