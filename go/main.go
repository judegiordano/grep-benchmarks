package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"time"
)

func search(query string, file string) []string {
	var results []string
	f, _ := os.Open(file)

	defer f.Close()
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		line := scanner.Text()
		if strings.Contains(line, query) {
			results = append(results, line)
		}
	}
	return results
}

func main() {
	args := os.Args
	query := args[1]
	file := args[2]

	start := time.Now().UnixNano() / int64(time.Millisecond)

	results := search(query, file)

	end := time.Now().UnixNano() / int64(time.Millisecond)
	diff := end - start

	fmt.Println("matches:", len(results))
	fmt.Println("operation complete in", diff, "\bms")
}
