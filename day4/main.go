package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func overlaps(a1 assignment, a2 assignment) bool {
	if (a2.start <= a1.end && a2.end >= a1.start) ||
		(a1.start <= a2.end && a1.end >= a2.start) {
		return true
	}
	return false
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

		if overlaps(first, second) || overlaps(second, first) {
			count++
		}
	}

	fmt.Println(count)
}
