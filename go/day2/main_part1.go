package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

const (
	OpRock  = "A"
	OpPaper = "B"
	OpSci   = "C"
	MyRock  = "X"
	MyPaper = "Y"
	MySci   = "Z"

	draw = 3
	loss = 0
	win  = 6

	rock  = 1
	paper = 2
	sci   = 3
)

var (
	myMoveScore map[string]int = map[string]int{
		MyRock:  1,
		MyPaper: 2,
		MySci:   3,
	}

	results map[string]int = map[string]int{
		MyRock + OpRock:  3,
		MyRock + OpPaper: 0,
		MyRock + OpSci:   6,

		MyPaper + OpRock:  6,
		MyPaper + OpPaper: 3,
		MyPaper + OpSci:   0,

		MySci + OpRock:  0,
		MySci + OpPaper: 6,
		MySci + OpSci:   3,
	}
)

func part1() {
	content, err := ioutil.ReadFile("input_prod.txt")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(string(content), "\n")
	type strategy struct {
		myMove     string
		opMove     string
		totalScore int
	}

	var strategies []strategy
	var sum int

	for _, line := range lines {
		if line == "" {
			continue
		}
		splitted := strings.Split(line, " ")
		s := strategy{
			myMove: splitted[1],
			opMove: splitted[0],
		}
		s.totalScore = myMoveScore[s.myMove] + results[s.myMove+s.opMove]
		strategies = append(strategies, s)
		sum += s.totalScore
	}

	fmt.Println(sum)

}
