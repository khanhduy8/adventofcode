package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
	"strings"
)

func transpose[T any](grid [][]T) [][]T{
	if len(grid) == 0{
		return [][]T{}
	}
	max_length := 0
	for _, row := range grid{
		if len(row) > max_length{
			max_length = len(row)
		}
	}
	transpose_grid := make([][]T, max_length)

	for j := 0; j < len(grid[0]); j++{
		transpose_grid[j] = make([]T, len(grid))
		for i := 0; i < len(grid); i++{
			if j < len(grid[i]){
				transpose_grid[j][i] = grid[i][j]
			}
		}
	}

	return transpose_grid
}

func solve1(lines []string) int {
	numbers := make([][]int, len(lines))
	for i := 0; i < len(lines); i++{
		values := strings.Split(lines[i], " ")
		filtered_values := make([]string, 0)
		for _, value := range values{
			value = strings.TrimSpace(value)
			if len(value) > 0{
				filtered_values = append(filtered_values, value)
			}
		}

		numbers[i] = make([]int, len(filtered_values))

		for j, value := range filtered_values{
			if i == len(lines) - 1{
				numbers[i][j] = int(value[0])
			} else {
				number, err := strconv.Atoi(value)
				if err != nil{
					log.Fatalf("Error when parse number %v", value)
				}
				numbers[i][j] = number
			}	
		}			
	}

	numbers_trans := transpose(numbers)
	res := 0

	for _, numbers := range numbers_trans{
		total := 0
		operator := numbers[len(numbers)-1]
		if operator == int('*'){
			total = 1
		}
		for i := 0; i < len(numbers) - 1; i++{
			number := numbers[i]
			if operator == int('+'){
				total += number
			} else {
				total *= number
			}
		}
		res += total
	}
	
	return res
}

func solve2(lines []string) int {
	number_sheet := make([][]string, len(lines))
	for i, line := range lines{
		values := strings.Split(line, "")
		number_sheet[i] = values
	}

	trans_number_sheet := transpose(number_sheet)
	queue := []int{}
	operator := ""
	total := 0
	res := 0
	for j, col := range trans_number_sheet{
		col_string := strings.Join(col, "")
		col_string = strings.ReplaceAll(col_string," ", "")
		if len(col_string) > 0{
			if col_string[len(col_string) - 1] == '*'{
				total = 1
				operator = "*"
				col_string = strings.Trim(col_string, "*")
			} else if col_string[len(col_string) - 1] == '+'{
				operator = "+"
				col_string = strings.Trim(col_string, "+")
			}
			number, err := strconv.Atoi(col_string)
			if err != nil{
				log.Fatalf("Error when parse number")
			}
			queue = append(queue, number)
		}
		
		if len(strings.TrimSpace(col_string)) == 0 || j == len(trans_number_sheet) - 1{
			for _, number := range queue{
				if operator == "*"{
					total *= number
				} else {
					total += number
				}
			}
			res += total
			queue = []int{}
			operator = ""
			total = 0
		}
	}
	
	return res
}

func main(){
	file, err  := os.Open("day6.txt")
	if err != nil{
		log.Fatal(err)
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)

	for scanner.Scan(){
		lines = append(lines, strings.TrimSpace(scanner.Text()))
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	res1 := solve1(lines)
	res2 := solve2(lines)

	log.Printf("result 1: %d", res1)
	log.Printf("result 2: %d", res2)
}