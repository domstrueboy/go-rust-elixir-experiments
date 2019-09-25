package main

import (
	"fmt"
	"strings"
)

func missing(input []string) string {
	const alphabet = "abcdefghijklmnopqrstuvwxyz"
	const length = len(alphabet)
	for i := 0; i < length; i++ {
		fmt.Println(string(alphabet[i]))
	}
	output := strings.Join(input[:], ":")
	return output
}

func main() {
	letter := missing([]string{"a", "b", "c", "d", "f", "g"})
	fmt.Println(letter)
}
