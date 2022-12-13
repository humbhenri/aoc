package main

import (
	"fmt"
	"regexp"
	"strconv"
)

type step struct {
	qty, from, to int
}

func atoi(s string) int {
	res, err := strconv.Atoi(s)
	if err != nil {
		panic(err)
	}
	return res
}

func parse_step(line string) step {
	r := regexp.MustCompile("[0-9]+")
	m := r.FindAllString(line, -1)
	s := step{}
	s.qty = atoi(m[0])
	s.from = atoi(m[1])
	s.to = atoi(m[2])
	return s
}

func main() {
	fmt.Println(parse_step("move 9 from 2 to 1"))
}
