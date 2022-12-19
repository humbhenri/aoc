package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func find_common(rucksacks []string) (rune, error) {
	var sets = make([]map[rune]bool, len(rucksacks))
	for i, rucksack := range rucksacks {
		sets[i] = make(map[rune]bool)
		for _, item := range rucksack {
			sets[i][item] = true
		}
	}
	for _, item := range rucksacks[0] {
		found_all := true
		for _, set := range sets {
			_, exists := set[item]
			found_all = found_all && exists
		}
		if found_all {
			return item, nil
		}
	}
	return ' ', fmt.Errorf("No common char found")
}

func split_rucksack(rucksack string) []string {
	half := len(rucksack) / 2
	return []string{rucksack[0:half], rucksack[half:]}
}

func priority(r rune) int {
	res := int(r) - 'a' + 1
	if res > 0 {
		return res
	}
	return int(r) - 'A' + 27
}

func part1(rucksacks []string) int {
	total := 0
	for _, rucksack := range rucksacks {
		compartments := split_rucksack(rucksack)
		common_item, err := find_common(compartments)
		if err != nil {
			panic(err)
		}
		total += priority(common_item)
	}
	return total
}

func part2(rucksacks []string) int {
	size := 3
	var j int
	total := 0
	for i := 0; i < len(rucksacks); i += size {
		j += size
		if j > len(rucksacks) {
			j = len(rucksacks)
		}
		common_item, err := find_common(rucksacks[i:j])
		if err != nil {
			panic(err)
		}
		total += priority(common_item)
	}
	return total
}

func main() {
	read, err := ioutil.ReadFile("input3.txt")
	if err != nil {
		panic(err)
	}
	file := strings.TrimSpace(string(read))
	lines := strings.Split(file, "\n")
	res := part2(lines)
	fmt.Println(res)
}
