package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func runeToPriority(r rune) int32 {
	if r >= 97 && r <= 122 {
		// lower case
		return r - 97 + 1

	}
	if r >= 65 && r <= 90 {
		// upper case
		return r - 65 + 27
	}
	return 0
}

func main() {
	content, err := ioutil.ReadFile("input.prod")
	if err != nil {
		panic(err)
	}
	lines := strings.Split(string(content), "\n")
	totalPriority := int32(0)

	for _, line := range lines {
		if line == "" {
			continue
		}
		firstCompartment := line[0 : len(line)/2]
		secondCompartment := line[len(line)/2:]
		sharedP := map[rune]struct{}{}

		for _, c1 := range firstCompartment {
			for _, c2 := range secondCompartment {
				if c1 == c2 {
					sharedP[c1] = struct{}{}
				}
			}
		}
		total := int32(0)
		for p := range sharedP {
			total += runeToPriority(p)
		}
		totalPriority += total

	}

	fmt.Println(totalPriority)
}
