package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func get_packet(content string, marker_len int) {
	for _, line := range strings.Split(content, "\n") {
		buffer := strings.TrimSpace(line)
		for i := marker_len - 1; i < len(buffer); i++ {
			chars := make(map[byte]bool)
			for j := i; j > i-marker_len; j-- {
				present, _ := chars[buffer[j]]
				if present {
					break
				}
				chars[buffer[j]] = true
			}
			if len(chars) == marker_len {
				fmt.Println(i + 1)
				break
			}
		}
	}
}

func main() {
	bytes, err := ioutil.ReadFile("6.input")
	if err != nil {
		panic(err)
	}
	get_packet(string(bytes), 14)
}
