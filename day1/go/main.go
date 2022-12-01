package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

func main() {

	readFile, err := os.Open("input.txt")

	if err != nil {
		fmt.Println(err)
	}

	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)

	calories := make([]int, 0)
	current_calories := 0
	for fileScanner.Scan() {
		line := fileScanner.Text()
		if line == "" {
			calories = append(calories, current_calories)
			current_calories = 0
		} else {
			i, _ := strconv.Atoi(line)
			current_calories += i
		}

	}
	calories = append(calories, current_calories)

	readFile.Close()
	sort.Slice(calories, func(i, j int) bool {
		return calories[i] > calories[j]
	})

	total := 0
	for _, valuex := range calories[:3] {
		total += valuex
	}
	fmt.Printf("%v", total)
}
