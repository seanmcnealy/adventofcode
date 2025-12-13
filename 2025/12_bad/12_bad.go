package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

func parse(file io.Reader) ([][3][3]bool, [][2]int, [][]int) {
	scanner := bufio.NewScanner(file)
	var shapes = [][3][3]bool{}
	var boxes = [][2]int{}
	var requirements = [][]int{}

	var scanBoxes = false
	for scanner.Scan() {
		var line = scanner.Text()
		if !scanBoxes && !strings.ContainsRune(line, 'x') {
			// unnecessary
			// var x, _ = strconv.Atoi(line)
			var shape = [3][3]bool{}
			for i := range 3 {
				scanner.Scan()
				line = scanner.Text()
				for j := range 3 {
					if line[j] == '#' {
						shape[i][j] = true
					} else {
						shape[i][j] = false
					}
				}
			}
			scanner.Scan()
			shapes = append(shapes, shape)
		} else {
			scanBoxes = true
			var fields = strings.Fields(line)
			var sizeX, _ = strconv.Atoi(strings.Split(fields[0][:len(fields[0])-1], "x")[0])
			var sizeY, _ = strconv.Atoi(strings.Split(fields[0][:len(fields[0])-1], "x")[1])
			boxes = append(boxes, [2]int{sizeX, sizeY})

			var required = []int{}
			for _, i := range fields[1:] {
				var x, _ = strconv.Atoi(i)
				required = append(required, x)
			}
			requirements = append(requirements, required)
		}

	}
	return shapes, boxes, requirements
}

func part1(boxes [][2]int, requirements [][]int) int {
	var answer = 0

	for i, req := range requirements {
		var b = boxes[i]
		var area = b[0] * b[1]

		var barea = 0
		for _, r := range req {
			barea += r * 9
		}
		if area >= barea {
			answer += 1
		}
	}

	return answer
}

func main() {
	file, err := os.Open("data/12")
	if err != nil {
		panic(err)
	}
	var _, fd2, fd3 = parse(file)

	var part1Result = part1(fd2, fd3)
	fmt.Printf("Part 1: %d\n", part1Result)
}
