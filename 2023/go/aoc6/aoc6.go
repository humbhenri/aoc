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
		panic(fmt.Sprintf("%s not a number", s))
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

func part1() {
	content := readInput()
	lines := strings.Split(content, "\n")
	numbersRe := regexp.MustCompile(`(\d+)`)
	times := numbersRe.FindAllString(lines[0], -1)
	maxDistances := numbersRe.FindAllString(lines[1], -1)
	p := 1
	for i, t := range times {
		wins := countWins(toInt(t), toInt(maxDistances[i]))
		p *= wins
	}
	fmt.Println(p)
}

func part2() {
	content := readInput()
	lines := strings.Split(content, "\n")
	numbersRe := regexp.MustCompile(`(\d+)`)
	times := numbersRe.FindAllString(lines[0], -1)
	time := strings.Join(times, "")
	maxDistances := numbersRe.FindAllString(lines[1], -1)
	maxDistance := strings.Join(maxDistances, "")
	wins := countWins(toInt(time), toInt(maxDistance))
	fmt.Println(wins)
}

func main() {
	part1()
	part2()
}
