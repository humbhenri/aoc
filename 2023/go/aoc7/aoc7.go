package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

type HandType int

const (
	HighCard HandType = iota
	OnePair
	TwoPair
	ThreeOK
	FullHouse
	FourOK
	FiveOK
)

type Hand string

// find the type of a hand of 5 cards
func typeOfHand(hand Hand) HandType {
	chars := make(map[rune]int)
	for _, c := range hand {
		chars[c]++
	}
	differentCards := len(chars)
	var quantities []int
	for _, quantity := range chars {
		quantities = append(quantities, quantity)
	}
	sort.Ints(quantities)
	mostRepeatedCardQuantity := quantities[len(quantities)-1]
	switch differentCards {
	case 1:
		return FiveOK
	case 2:
		if mostRepeatedCardQuantity == 4 {
			return FourOK
		}
		return FullHouse
	case 3:
		if mostRepeatedCardQuantity == 3 {
			return ThreeOK
		}
		return TwoPair
	case 4:
		return OnePair
	case 5:
		return HighCard
	default:
		panic("Invalid hand")
	}
}

// represents a line of the input
type Entry struct {
	hand Hand
	bid  int
}

// return true if hand h1 is less than hand h2
func lessHand(h1 Hand, h2 Hand) bool {
	for i := 0; i < len(h1); i++ {
		if h1[i] == h2[i] {
			continue
		}
		return cardValue(h1[i]) < cardValue(h2[i])
	}
	return false
}

func cardValue(b byte) int {
	switch string(b) {
	case "A":
		return 14
	case "K":
		return 13
	case "Q":
		return 12
	case "J":
		return 11
	case "T":
		return 10
	default:
		return int(b) - 48
	}
}

// sort entries first by type and if equal then by highest card value, from
// left to right
func sortEntries(entries []Entry) {
	sort.Slice(entries, func(i, j int) bool {
		entryA := entries[i]
		entryB := entries[j]
		handTypeA := typeOfHand(entryA.hand)
		handTypeB := typeOfHand(entryB.hand)
		if handTypeA == handTypeB {
			return lessHand(entryA.hand, entryB.hand)
		}
		return handTypeA < handTypeB
	})
}

// parse a line of the input into a Entry
func parseEntry(line string) Entry {
	parts := strings.Split(line, " ")
	bid, err := strconv.Atoi(parts[1])
	if err != nil {
		panic("invalid entry " + line)
	}
	return Entry{Hand(parts[0]), bid}
}

func parseInput(fileName string) []Entry {
	file, err := os.Open(fileName)
	if err != nil {
		panic(err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	var entries []Entry
	for scanner.Scan() {
		line := scanner.Text()
		entry := parseEntry(line)
		entries = append(entries, entry)
	}
	if err := scanner.Err(); err != nil {
		fmt.Println("Error reading file:", err)
	}
	return entries
}

func part1() {
	fileName := "/home/humberto/projects/aoc/2023/07.input"
	entries := parseInput(fileName)
	sortEntries(entries)
	winnings := 0
	for i, entry := range entries {
		winnings += (i + 1) * entry.bid
	}
	fmt.Println(winnings)
}

func main() {
	part1()
}
