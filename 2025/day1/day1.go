package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
)

func solve1(lines []string) int {
	pos := 50
	res := 0

	for _, line := range lines{
		turn_sign := line[0:1]
		turn := 1
		step, err := strconv.Atoi(line[1:])

		if err != nil {
			log.Fatal("String is not a number")
			panic("")
		}

		if turn_sign == "L"{
			turn = -1
		}


		if pos + turn*step <= 0{
			new_pos := (100 + (pos + turn*step) % 100) % 100
			pos = new_pos
		} else if pos + turn*step > 0{
			new_pos := (pos + turn*step) % 100
			pos = new_pos
		}

		if pos == 0 {
			res += 1
		}
	}
	return res
}

func solve2(lines []string) int {
	pos := 50
	res := 0

	for _, line := range lines{
		turn_sign := line[0:1]
		turn := 1
		step, err := strconv.Atoi(line[1:])

		if err != nil {
			log.Fatal("String is not a number")
			panic("")
		}

		if turn_sign == "L"{
			turn = -1
		}

		if pos + turn*step <= 0{
			new_pos := (100 + (pos + turn*step) % 100) % 100
			res += (step - pos) / 100
			if pos > 0 {
				res += 1
			}
			pos = new_pos
		} else if pos + turn*step > 0{
			new_pos := (pos + turn*step) % 100
			res += (pos + turn*step) / 100
			pos = new_pos
		}
	}
	return res
}

func main(){
	file, err  := os.Open("day1.txt")
	if err != nil{
		log.Fatal(err)
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)

	for scanner.Scan(){
		lines = append(lines, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	res1 := solve1(lines)
	res2 := solve2(lines)

	log.Printf("result 1: %d", res1)
	log.Printf("result 2: %d", res2)
}