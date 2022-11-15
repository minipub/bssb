package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Fprint(os.Stderr, "Please input bytes argument.\n")
		os.Exit(1)
	}

	s := strings.TrimPrefix(os.Args[1], "[")
	s = strings.TrimSuffix(s, "]")

	var bs []byte

	for _, v := range strings.Split(s, " ") {
		i, err := strconv.Atoi(v)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error: %+v.\n", err)
			os.Exit(2)
		}

		bs = append(bs, byte(i))
	}

	fmt.Printf("%s\n", bs)
	os.Exit(0)
}
