package aoc1

import (
	"reflect"
	"testing"
)

func TestPart1(t *testing.T) {
	got := Part1()
	want := 55208
	if got != want {
		t.Errorf("got %v, wanted %v", got, want)
	}
}

func TestTranslate(t *testing.T) {
	got := translate("one")
	want := "1"
	if got != want {
		t.Errorf("got %v, wanted %v", got, want)
	}
}

func TestGetFirstAndLastFrom(t *testing.T) {
	first, last := getFirstAndLastFrom("eightwothree")
	got := []string{first, last}
	want := []string{"eight", "three"}
	if !reflect.DeepEqual(want, got) {
		t.Errorf("got %v, wanted %v", got, want)
	}
}

func TestPart2(t *testing.T) {
	got := Part2()
	want := 54578
	if got != want {
		t.Errorf("got %v, wanted %v", got, want)
	}
}
