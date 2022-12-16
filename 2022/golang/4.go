package main

import (
	"fmt"
	"io/ioutil"
	"regexp"
	"strconv"
	"strings"
)

type pair struct {
	a, b int
}

func contains(p1, p2 pair) bool {
	return (p1.a <= p2.a && p2.b <= p1.b) ||
		(p2.a <= p1.a && p1.b <= p2.b)
}

func atoi(s string) int {
	res, error := strconv.Atoi(s)
	if error != nil {
		panic(error)
	}
	return res
}

func make_pair(line string) (pair, pair) {
	re := regexp.MustCompile(`(?P<a1>\d*)-(?P<b1>\d*),(?P<a2>\d*)-(?P<b2>\d*)`)
	matches := re.FindStringSubmatch(line)
	a1 := re.SubexpIndex("a1")
	b1 := re.SubexpIndex("b1")
	a2 := re.SubexpIndex("a2")
	b2 := re.SubexpIndex("b2")
	return pair{atoi(matches[a1]), atoi(matches[b1])}, pair{atoi(matches[a2]), atoi(matches[b2])}
}

func overlaps(p1, p2 pair) bool {
	return (p1.a >= p2.a && p1.a <= p2.b) ||
		(p2.a >= p1.a && p2.a <= p1.b)
}

func part1(file string) int {
	lines := strings.Split(file, "\n")
	total_contains := 0
	for _, line := range lines {
		p1, p2 := make_pair(line)
		if contains(p1, p2) {
			total_contains++
		}
	}
	return total_contains
}

func part2(file string) int {
	lines := strings.Split(file, "\n")
	total:= 0
	for _, line := range lines {
		p1, p2 := make_pair(line)
		if overlaps(p1, p2) {
			total++
		}
	}
	return total
}

func main() {
	read, err := ioutil.ReadFile("input4.txt")
	if err != nil {
		panic(err)
	}
	file := strings.TrimSpace(string(read))
	fmt.Println(part2(file))
}
