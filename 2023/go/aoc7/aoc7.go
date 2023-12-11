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

// the types of the hand, from weakest to the strongest
const (
	HighCard HandType = iota
	OnePair
	TwoPair
	ThreeOK
	FullHouse
	FourOK
	FiveOK
)

var handTypeNames []string = []string{
	"HighCard",
	"OnePair",
	"TwoPair",
	"ThreeOK",
	"FullHouse",
	"FourOK",
	"FiveOK",
}

type Hand string

const JOKER = 'J'

// find the type of a hand of 5 cards
func typeOfHand(hand Hand, joker bool) HandType {
	cardCount := make(map[rune]int)
	for _, c := range hand {
		cardCount[c]++
	}
	// joker is wildcard that acts like the most common card
	var jokerCount int
	var mostCommonCard rune
	var biggest int
	for card, count := range cardCount {
		if count > biggest {
			if joker && card == JOKER {
				continue // joker cannot be the most common card because it is a wildcard
			}
			biggest = count
			mostCommonCard = card
		}
	}
	if joker {
		jokerCount = cardCount[JOKER]
		delete(cardCount, JOKER)
		cardCount[mostCommonCard] += jokerCount
	}
	differentCards := len(cardCount)
	mostCommonCardCount := cardCount[mostCommonCard]
	var handType HandType
	switch differentCards {
	case 1:
		handType = FiveOK
	case 2:
		if mostCommonCardCount == 4 {
			handType = FourOK
		} else {
			handType = FullHouse
		}
	case 3:
		if mostCommonCardCount == 3 {
			handType = ThreeOK
		} else {
			handType = TwoPair
		}
	case 4:
		handType = OnePair
	case 5:
		handType = HighCard
	default:
		panic("Invalid hand")
	}
	// restore the hand with the jokers for the purpose of breaking ties
	if joker {
		cardCount[JOKER] = jokerCount
		cardCount[mostCommonCard] -= jokerCount
	}
	return handType
}

// represents a line of the input
type Entry struct {
	hand Hand
	bid  int
}

// return true if hand h1 is less than hand h2
func lessHand(h1 Hand, h2 Hand, joker bool) bool {
	handTypeA := typeOfHand(h1, joker)
	handTypeB := typeOfHand(h2, joker)
	if handTypeA != handTypeB {
		return handTypeA < handTypeB
	}
	for i := 0; i < len(h1); i++ {
		if h1[i] == h2[i] {
			continue
		}
		cardValue1 := cardValue(h1[i], joker)
		cardValue2 := cardValue(h2[i], joker)
		return cardValue1 < cardValue2
	}
	return false
}

// break ties when the type of two hands are equal
func cardValue(b byte, joker bool) int {
	switch string(b) {
	case "A":
		return 14
	case "K":
		return 13
	case "Q":
		return 12
	case "J":
		// to break ties Joker acts like the weakest card in part 2
		if joker {
			return 1
		} else {
			return 11
		}
	case "T":
		return 10
	default:
		return int(b) - 48
	}
}

// sort entries first by type and if equal then by highest card value, from
// left to right
func sortEntries(entries []Entry, joker bool) {
	sort.Slice(entries, func(i, j int) bool {
		entryA := entries[i]
		entryB := entries[j]
		return lessHand(entryA.hand, entryB.hand, joker)
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

// totalWinnings is the sum of each hand multiplied by its rank with its bid
// the rank is the order or the hand in the set
func totalWinnings(joker bool) {
	fileName := "/home/humberto/projects/aoc/2023/07.input"
	entries := parseInput(fileName)
	sortEntries(entries, joker)
	winnings := 0
	for i, entry := range entries {
		winnings += (i + 1) * entry.bid
	}
	fmt.Println(winnings)
}

func part1() {
	totalWinnings(false)
}

func part2() {
	totalWinnings(true)
}

func main() {
	part1()
	part2()
}
