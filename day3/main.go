package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

type groupIndex map[rune]*struct {
	g1 bool
	g2 bool
	g3 bool
}

func (g groupIndex) add(gr int, c rune) {
	if _, ok := g[c]; !ok {
		g[c] = &struct {
			g1 bool
			g2 bool
			g3 bool
		}{}
	}
	if gr == 0 {
		g[c].g1 = true
	}

	if gr == 1 {
		g[c].g2 = true
	}

	if gr == 2 {
		g[c].g3 = true
	}
}

func (g groupIndex) shareds() int {
	var total int
	for c, data := range g {
		if data.g1 && data.g2 && data.g3 {
			total += int(runeToPriority(c))
		}
	}

	return total
}

func main() {
	content, err := ioutil.ReadFile("input.prod")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(string(content), "\n")

	var total int
	var current []string

	for _, line := range lines {
		if line == "" {
			continue
		}
		current = append(current, line)
		if len(current) == 3 {
			gi := groupIndex{}
			for idx, l := range current {
				for _, c := range l {
					gi.add(idx, c)
				}
			}
			total += gi.shareds()
			current = []string{}
		}
	}

	fmt.Println(total)
}
