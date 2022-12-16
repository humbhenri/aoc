package main

import (
	"fmt"
	"io/ioutil"
	"sort"
	"strconv"
	"strings"
)

func count_calories(group string) int {
	total := 0
	for _, calories := range strings.Split(group, "\n") {
		if strings.TrimSpace(calories) == "" {
			continue
		}
		calories, err := strconv.Atoi(strings.TrimSpace(calories))
		if err != nil {
			panic(err)
		}
		total += calories
	}
	return total
}

func part1(file string) int {
	elves := strings.Split(file, "\r\n\r\n")
	max := 0
	for _, elf := range elves {
		total := count_calories(elf)
		if total > max {
			max = total
		}
	}
	return max
}

func part2(file string) int {
	elves := strings.Split(file, "\r\n\r\n")
	var calories []int
	for _, elf := range elves {
		calories = append(calories, count_calories(elf))
	}
	sort.Sort(sort.Reverse(sort.IntSlice(calories)))
	return calories[0] + calories[1] + calories[2]
}

func main() {
	read, err := ioutil.ReadFile("input1.txt")
	if err != nil {
		panic(err)
	}
	file := strings.TrimSpace(string(read))
	fmt.Println(part2(file))
}
