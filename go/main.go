package main

import (
	"bufio"
	"bytes"
	"fmt"
	"io"
	"os"
	"time"
)

func search(query string, fileName string) []string {
	var results []string
	file, _ := os.Open(fileName)

	queryBytes := []byte(query)

	defer file.Close()
	reader := bufio.NewReader(file)
	for {
		path, _, e := reader.ReadLine()
		if e == io.EOF {
			break
		}
		match := bytes.Contains(path, queryBytes)
		if match {
			results = append(results, string(query))
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
