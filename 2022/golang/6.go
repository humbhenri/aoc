package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func get_first_packet_where_all_chars_are_different(content string, packet_len int) {
	for _, line := range strings.Split(content, "\n") {
		buffer := strings.TrimSpace(line)
		for i := packet_len - 1; i < len(buffer); i++ {
			chars := make(map[byte]bool)
			for j := i; j > i-packet_len; j-- {
				present, _ := chars[buffer[j]]
				if present {
					break
				}
				chars[buffer[j]] = true
			}
			if len(chars) == packet_len {
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
	get_first_packet_where_all_chars_are_different(string(bytes), 14)
}
