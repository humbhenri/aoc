package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"unicode"
)

func main() {
	f, err := os.Open("../01.input")
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
	fmt.Println(total)

}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
