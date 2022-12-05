package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type assignment struct {
	start int
	end   int
}

func assignmentOverlap(a1 assignment, a2 assignment) bool {
	if a2.start >= a1.start && a2.end <= a1.end {
		return true
	}
	return false
}

func parseRange(s string) assignment {
	start := strings.Split(s, "-")[0]
	end := strings.Split(s, "-")[1]

	startNum, err := strconv.ParseInt(start, 10, 64)
	if err != nil {
		panic(err)
	}
	endNum, err := strconv.ParseInt(end, 10, 64)
	if err != nil {
		panic(err)
	}

	return assignment{
		start: int(startNum),
		end:   int(endNum),
	}
}

func main() {
	content, err := ioutil.ReadFile("input.prod")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(string(content), "\n")

	var count int
	for _, l := range lines {
		if l == "" {
			continue
		}
		firstRange := strings.Split(l, ",")[0]
		secondRange := strings.Split(l, ",")[1]

		first := parseRange(firstRange)
		second := parseRange(secondRange)

		if assignmentOverlap(first, second) || assignmentOverlap(second, first) {
			count++
		}

	}

	fmt.Println(count)
}
