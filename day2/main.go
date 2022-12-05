package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

const (
	desiredResultDraw = "Y"
	desiredResultLoss = "X"
	desiredResultWin  = "Z"
)

var (
	moveScore map[string]int = map[string]int{
		OpRock:  1,
		OpPaper: 2,
		OpSci:   3,
	}

	moves map[string]int = map[string]int{
		OpPaper + desiredResultDraw: moveScore[OpPaper] + draw,
		OpSci + desiredResultDraw:   moveScore[OpSci] + draw,
		OpRock + desiredResultDraw:  moveScore[OpRock] + draw,

		OpPaper + desiredResultLoss: moveScore[OpRock] + loss,
		OpRock + desiredResultLoss:  moveScore[OpSci] + loss,
		OpSci + desiredResultLoss:   moveScore[OpPaper] + loss,

		OpPaper + desiredResultWin: moveScore[OpSci] + win,
		OpRock + desiredResultWin:  moveScore[OpPaper] + win,
		OpSci + desiredResultWin:   moveScore[OpRock] + win,
	}
)

func main() {
	content, err := ioutil.ReadFile("input_prod.txt")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(string(content), "\n")
	type strategy struct {
		opMove     string
		desired    string
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
			desired: splitted[1],
			opMove:  splitted[0],
		}
		s.totalScore = moves[s.opMove+s.desired]
		strategies = append(strategies, s)
		sum += s.totalScore
	}

	fmt.Println(sum)

}
