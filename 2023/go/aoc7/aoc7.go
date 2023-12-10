package main

import (
	"fmt"
	"sort"
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

func typeOfHand(hand Hand) HandType {
	chars := make(map[rune]int)
	for _, c := range hand {
		chars[c]++
	}
	fmt.Println(chars)
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

type Entry struct {
	hand Hand
	bid  int
}

func main() {
	fmt.Println(typeOfHand("32T3K"))
	fmt.Println(typeOfHand("T55J5"))
	fmt.Println(typeOfHand("KK677"))
	fmt.Println(typeOfHand("KTJJT"))
	fmt.Println(typeOfHand("QQQJA"))
}
