package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
	"strings"
)

func solve1(lines []string) int {
	if len(lines) == 0 || len(lines[0]) == 0{
		return 0
	}

	battery_length := len(lines)

	batteries := make([][]int, battery_length)
	
	for i, line := range lines{
		bank_length := len(line)
		number_string_list := strings.Split(line,"")
		batteries[i] = make([]int, bank_length)
		
		for j, number := range number_string_list{
			value, err := strconv.Atoi(number)
			if err != nil{
				log.Fatalf("Error when parse number")
			}
			batteries[i][j] = value
		}
	}

	res := 0
	for _, bank := range batteries{
		battery_length := len(bank)
		max_value := 0
		for l := 0; l < battery_length; l++{
			battery_one := bank[l]
			for r := l+1; r < battery_length; r++{
				battery_two := bank[r]
				battery_value := battery_one*10+battery_two
				if battery_value > max_value{
					max_value = battery_value
				}
			}
		}
		res += max_value
	}
	return res
}

func solve2(lines []string) int {
	if len(lines) == 0 || len(lines[0]) == 0{
		return 0
	}

	battery_length := len(lines)

	batteries := make([][]int, battery_length)
	
	for i, line := range lines{
		bank_length := len(line)
		number_string_list := strings.Split(line,"")
		batteries[i] = make([]int, bank_length)
		
		for j, number := range number_string_list{
			value, err := strconv.Atoi(number)
			if err != nil{
				log.Fatalf("Error when parse number")
			}
			batteries[i][j] = value
		}
	}

	res := 0
	for _, bank := range batteries{
		bank_length := len(bank)

		MAX_ACTIVE_BATTERY := 12
		d := make([][][]int, MAX_ACTIVE_BATTERY)

		for k := 0; k < MAX_ACTIVE_BATTERY; k++{
			d[k] = make([][]int, bank_length)
			for i := 0 ; i < bank_length; i++ {
				d[k][i] = make([]int, bank_length)
				for j := 0; j < bank_length; j++{
					d[k][i][j] = 0
				}
			}
		}

		for i := 0; i < bank_length; i++ {
			for j := i; j < bank_length; j++{
				max_value := 0
				for z := i; z <= j; z++{
					if bank[z] > max_value{
						max_value = bank[z]
					}
				}
				d[0][i][j] = max_value
			}
		}

		for k := 1; k < MAX_ACTIVE_BATTERY; k++{
			for i := 0; i < bank_length - k; i++{
				max_value := 0
				for j := i + k; j < bank_length; j++{
					value := d[k-1][i][j-1]*10 + bank[j]
					if value > max_value{
						max_value = value
					}
					d[k][i][j] = max_value
				}
			}
		}

		res += d[11][0][bank_length-1]
	}
	return res
}

func main(){
	file, err  := os.Open("day3.txt")
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