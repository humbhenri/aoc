package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strconv"
	"strings"
)

type dot struct {
	x, y int
}

func parseDot(line string) (*dot, error) {
	xs := strings.Split(strings.TrimSpace(line), ",")
	x, err := strconv.Atoi(xs[0])
	if err != nil {
		return nil, err
	}
	y, err := strconv.Atoi(xs[1])
	if err != nil {
		return nil, err
	}
	return &dot{x, y}, nil
}

type paper struct {
	dots map[dot]bool
}

func (paper *paper) add(dot dot) {
	paper.dots[dot] = true
}

func (paper *paper) size() int {
	return len(paper.dots)
}

func (paper *paper) lines() int {
	lines := 0
	for d := range paper.dots {
		if d.y > lines {
			lines = d.y
		}
	}
	return lines
}

func (paper *paper) columns() int {
	columns := 0
	for d := range paper.dots {
		if d.x > columns {
			columns = d.x
		}
	}
	return columns
}

func (paper *paper) foldUp(ammount int) {
	fmt.Printf("fold up %d\n", ammount)
	for d := range paper.dots {
		delete(paper.dots, d)
		y := d.y
		if d.y > ammount {
			y = d.y - 2*ammount
			if y < 0 {
				y = -y
			}
		}
		newDot := dot{d.x, y}
		paper.dots[newDot] = true
	}
}

func (paper *paper) foldLeft(ammount int) {
	fmt.Printf("fold left %d\n", ammount)
	for d := range paper.dots {
		delete(paper.dots, d)
		x := d.x
		if d.x > ammount {
			x = d.x - 2*ammount
			if x < 0 {
				x = -x
			}
		}
		newDot := dot{x, d.y}
		paper.dots[newDot] = true
	}
}

func parseInput(filename string) (*paper, []string, error) {
	bytes, err := ioutil.ReadFile(filename)
	if err != nil {
		return nil, nil, err
	}
	s := strings.Split(string(bytes), "\r\n\r\n")
	dots := s[0]
	instructions := strings.Split(s[1], "\r\n")
	paper := &paper{}
	paper.dots = make(map[dot]bool)
	for _, line := range strings.Split(strings.TrimSpace(dots), "\n") {
		if strings.TrimSpace(line) == "" {
			break
		}
		dot, err := parseDot(strings.TrimSpace(line))
		if err != nil {
			return nil, nil, err
		}
		paper.add(*dot)
	}

	return paper, instructions, nil
}

func extractAmmount(line string) (int, error) {
	s := strings.Split(line, "=")
	ammount, err := strconv.Atoi(s[1])
	if err != nil {
		return -1, err
	}
	return ammount, nil
}

func main() {
	// paper, err := parseInput("example13.txt")
	paper, instructions, err := parseInput("input13.txt")
	if err != nil {
		log.Fatal(err)
	}
	for _, instruction := range instructions {
		if strings.Contains(instruction, "fold along x") {
			ammount, err := extractAmmount(instruction)
			if err != nil {
				log.Fatal(err)
			}
			paper.foldLeft(ammount)
		} else if strings.Contains(instruction, "fold along y") {
			ammount, err := extractAmmount(instruction)
			if err != nil {
				log.Fatal(err)
			}
			paper.foldUp(ammount)
		}
	}
	fmt.Printf("paper size %d\n", paper.size())
}
