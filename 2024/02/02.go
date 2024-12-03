package main

import (
	"fmt"
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
)

func assert(condition bool, message string) {
	if !condition {
		panic(message)
	}
}

func delete(slice []int, index int) []int {
	if (index < 0) || (index >= len(slice)) {
		return slice
	}
	s := slices.Clone(slice)
	s = slices.Delete(s, index, index+1)
	return s
}

func isSafe(numbers []int, problemDampener bool) bool {
	increasing := numbers[1]-numbers[0] > 0
	permittedDiffs := []int{1, 2, 3}
	violates := func(diff int) bool {
		return increasing && diff < 0 || !increasing && diff > 0 || !slices.Contains(permittedDiffs, int(math.Abs(float64(diff))))
	}
	for i := 0; i < len(numbers)-1; i++ {
		diff := numbers[i+1] - numbers[i]
		if violates(diff) {
			if problemDampener {
				return isSafe(delete(numbers, i), false) || isSafe(delete(numbers, i+1), false) || isSafe(delete(numbers, i-1), false)
			}
			return false
		}
	}
	return true
}

func findSafeReports(
	inputFile string,
	problemDampener bool,
) int {
	input, err := os.ReadFile(inputFile)
	if err != nil {
		panic(err)
	}

	safe := 0
	lines := strings.Split(string(input), "\n")
	for _, line := range lines {
		var numbers []int
		for _, number := range strings.Split(line, " ") {
			num, err := strconv.Atoi(number)
			if err != nil {
				panic(err)
			}
			numbers = append(numbers, num)
		}
		if isSafe(numbers, problemDampener) {
			safe++
		}
	}

	return safe
}

func part1(inputFile string) int {
	return findSafeReports(inputFile, false)
}

func part2(inputFile string) int {
	return findSafeReports(inputFile, true)
}

func main() {
	inputFile := "02/02.input"
	part1 := part1(inputFile)
	fmt.Printf("Part 1: %d\n", part1)
	part2 := part2(inputFile)
	fmt.Printf("Part 2: %d\n", part2)
}
