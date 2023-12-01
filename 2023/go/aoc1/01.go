package aoc1

import (
	"bufio"
	"os"
	"regexp"
	"strconv"
	"unicode"
)

func Part1() int {
	f, err := os.Open("../../01.input")
	check(err)
	defer f.Close()
	scanner := bufio.NewScanner(f)
	total := 0
	for scanner.Scan() {
		line := scanner.Text()
		first := ""
		last := ""
		for _, c := range line {
			if unicode.IsDigit(c) {
				last = string(c)
				if len(first) == 0 {
					first = string(c)
				}
			}
		}
		sum, err := strconv.Atoi(first + last)
		check(err)
		total += sum
	}
	check(scanner.Err())
	return total
}

func Part2() int {
	f, err := os.Open("../../01.input")
	check(err)
	defer f.Close()
	scanner := bufio.NewScanner(f)
	total := 0
	for scanner.Scan() {
		line := scanner.Text()
		first, last := getFirstAndLastFrom(line)
		sum, err := strconv.Atoi(translate(first) + translate(last))
		check(err)
		total += sum
	}
	check(scanner.Err())
	return total
}

var lettersToNumber = map[string]string{
	"one": "1", "two": "2", "three": "3", "four": "4", "five": "5", "six": "6", "seven": "7", "eight": "8", "nine": "9",
}

func translate(s string) string {
	if elem, ok := lettersToNumber[s]; ok {
		return elem
	}
	return s
}

var digits = regexp.MustCompile(`\d|one|two|three|four|five|six|seven|eight|nine`)

func getFirstAndLastFrom(line string) (string, string) {
	first := ""
	last := ""
	for i := 0; i < len(line); i++ {
		matches := digits.FindStringSubmatch(line[i:])
		if len(matches) > 0 {
			last = matches[0]
			if len(first) == 0 {
				first = matches[0]
			}
		}
	}
	return first, last
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
