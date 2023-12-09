package main

import (
	"fmt"
	"io/ioutil"
	"regexp"
	"strconv"
	"strings"
)

func countWins(time int, maxDist int) int {
	wins := 0
	for t := 1; t < time; t++ {
		dist := (time - t) * t
		if dist > maxDist {
			wins++
		}
	}
	return wins
}

func toInt(s string) int {
	i, err := strconv.Atoi(s)
	if err != nil {
		panic(err)
	}
	return i
}

func readInput() string {
	file, err := ioutil.ReadFile("/home/humberto/projects/aoc/2023/06.input")
	if err != nil {
		panic(err)
	}
	content := string(file)
	return content
}

func main() {
	content := readInput()
	lines := strings.Split(content, "\n")
	numbersRe := regexp.MustCompile(`(\d+)`)
	times := numbersRe.FindAllString(lines[0], -1)
	maxDistances := numbersRe.FindAllString(lines[1], -1)
	fmt.Println(times)
	fmt.Println(maxDistances)
	p := 1
	for i, t := range times {
		wins := countWins(toInt(t), toInt(maxDistances[i]))
		p *= wins
	}
	fmt.Println(p)
}
