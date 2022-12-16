package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

var (
	shapes = map[string]int{
		"X": 1,
		"Y": 2,
		"Z": 3,
	}
	outcome = map[string]int{
		"AX": 3,
		"AY": 6,
		"AZ": 0,
		"BX": 0,
		"BY": 3,
		"BZ": 6,
		"CX": 6,
		"CY": 0,
		"CZ": 3,
	}

	win = map[string]string{
		"A": "Y",
		"B": "Z",
		"C": "X",
	}

	draw = map[string]string{
		"A": "X",
		"B": "Y",
		"C": "Z",
	}

	lose = map[string]string{
		"A": "Z",
		"B": "X",
		"C": "Y",
	}
)

func score(op, you string) int {
	return shapes[you] + outcome[op+you]
}

func score_line(line string) int {
	shapes := strings.Split(line, " ")
	return score(shapes[0], shapes[1])
}

func total_score(input string) int {
	rounds := strings.Split(input, "\n")
	total := 0
	for _, round := range rounds {
		total += score_line(round)
	}
	return total
}

func choose(op, you string) string {
	if you == "X" { // lose
		return lose[op]
	} else if you == "Y" { // draw
		return draw[op]
	} else {
		return win[op]
	}
}

func main() {
	input, err := ioutil.ReadFile("input2.txt")
	if err != nil {
		fmt.Println("error reading input")
		return
	}
	rounds := strings.Split(string(input), "\n")
	total := 0
	for _, round := range rounds {
		shapes := strings.Split(round, " ")
		total += score(shapes[0], choose(shapes[0], shapes[1]))
	}
	fmt.Println(total)
}
