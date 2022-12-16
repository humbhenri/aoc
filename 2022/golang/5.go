package main

import (
	"fmt"
	"io/ioutil"
	// "io/ioutil"
	"regexp"
	"strconv"
	"strings"
)

type step struct {
	qty, from, to int
}

type stack []string

type stacks []stack

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

func parse_file(content string) (string, string) {
	parts := strings.Split(content, "\n\n")
	return parts[0], parts[1]
}

func parse_stack_line(line string) []string {
	res := []string{}
	crate_len := 3
	for i := 0; i < len(line); {
		crate := line[i : i+crate_len]
		res = append(res, crate)
		i = i + crate_len + 1
	}
	return res
}

func transpose(slice [][]string) [][]string {
	xl := len(slice[0])
	yl := len(slice)
	result := make([][]string, xl)
	for i := range result {
		result[i] = make([]string, yl)
	}
	for i := 0; i < xl; i++ {
		for j := 0; j < yl; j++ {
			result[i][j] = slice[j][i]
		}
	}
	return result
}

func (s *stack) push(crate string) {
	*s = append(*s, crate)
}

func (s *stack) pop() (string, bool) {
	if len(*s) == 0 {
		return "", false
	}
	index := len(*s) - 1
	el := (*s)[index]
	*s = (*s)[:index]
	return el, true
}

func reverse[S ~[]E, E any](s S) {
	for i, j := 0, len(s)-1; i < j; i, j = i+1, j-1 {
		s[i], s[j] = s[j], s[i]
	}
}

func parse_stack(text string) stacks {
	lines := strings.Split(text, "\n")
	s := [][]string{}
	for _, line := range lines {
		s = append(s, parse_stack_line(line))
	}
	matrix := transpose(s[:len(s)-1])
	res := []stack{}
	brackets := regexp.MustCompile("\\[|\\]")
	for i, row := range matrix {
		res = append(res, []string{})
		reverse(row)
		for _, crate := range row {
			if string(crate[0]) == "[" {
				res[i] = append(res[i], brackets.ReplaceAllString(crate, ""))
			}
		}
	}
	return res
}

func (stacks *stacks) execute(s step, retain_order bool) {
	from := &(*stacks)[s.from-1]
	to := &(*stacks)[s.to-1]
	crates := []string{}
	for i := 0; i < s.qty; i++ {
		crate, ok := from.pop()
		if !ok {
			panic("empty stack")
		}
		crates = append(crates, crate)
	}
	if retain_order {
		reverse(crates)
	}
	for _, crate := range crates {
		to.push(crate)
	}
}

func part1(file_content string) {
	execute_procedure(file_content, false)
}

func part2(file_content string) {
	execute_procedure(file_content, true)
}

func execute_procedure(file_content string, retain_order bool) {
	drawing, procedure := parse_file(file_content)
	stacks := parse_stack(drawing)
	for _, line := range strings.Split(strings.TrimSpace(procedure), "\n") {
		step := parse_step(line)
		stacks.execute(step, retain_order)
	}
	top := ""
	for _, stack := range stacks {
		crate, _ := stack.pop()
		top += crate
	}
	fmt.Println(top)
}

func main() {
	bytes, err := ioutil.ReadFile("5.input")
	if err != nil {
		panic(err)
	}
	part2(string(bytes))
}
