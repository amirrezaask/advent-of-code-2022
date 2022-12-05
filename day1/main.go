package main

import (
	"fmt"
	"io/ioutil"
	"sort"
	"strconv"
	"strings"
)

type elfCalories int64
type Elfs []elfCalories

func (es Elfs) Len() int {
	return len(es)
}

func (es Elfs) Less(i int, j int) bool {
	return es[i] > es[j]
}

func (es Elfs) Swap(i int, j int) {
	tmp := es[i]
	es[i] = es[j]
	es[j] = tmp
}
func main() {
	content, err := ioutil.ReadFile("input_prod.txt")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(string(content), "\n")
	var currentElf elfCalories
	var elfs []elfCalories
	for _, line := range lines {
		if line == "" {
			// end this elf
			elfs = append(elfs, currentElf)
			currentElf = 0
		} else {
			num, err := strconv.ParseInt(line, 10, 64)
			if err != nil {
				panic(err)
			}
			currentElf += elfCalories(num)
		}
	}

	sort.Sort(Elfs(elfs))

	total := elfs[0] + elfs[1] + elfs[2]
	fmt.Println(total)

}
