package main

import (
	"bufio"
	"log"
	"os"
	"strings"
)

func solve1(lines []string) int {
	grid := make([][]string, len(lines))
	
	for i, line := range lines{
		grid[i] = strings.Split(line,"")
	}

	type direction  struct {
		x int
		y int
	}

	directions := []direction{{0,1}, {0,-1}, {-1, 0}, {1, 0}, {1,1}, {-1,1}, {1,-1}, {-1,-1}}
	res := 0

	for y, row := range grid{
		for x, cell := range row{
			count_adjacent := 0
			if cell != "@" {
				continue
			}
			for _, d := range directions{
				value := direction{x: x + d.x, y: y + d.y}
				if value.x < 0 || value.x >= len(row) || value.y < 0 ||  value.y >= len(grid){
					continue
				}
				if grid[value.y][value.x] == "@"{
					count_adjacent++
				} 
			}
			if count_adjacent < 4{
				res++
			}
		}
	}
	return res
}

func solve2(lines []string) int {
	type direction  struct {
		x int
		y int
	}
	type paper struct {
		value string
		is_removed bool		
	}
	grid := make([][]paper, len(lines))
	
	for i, line := range lines{
		row := strings.Split(line,"")
		grid[i] = make([]paper, len(row))
		for j, value := range row{
			grid[i][j] = paper{value: value, is_removed: false}
		}
	}

	directions := []direction{{0,1}, {0,-1}, {-1, 0}, {1, 0}, {1,1}, {-1,1}, {1,-1}, {-1,-1}}
	res := 0
	is_can_removed := true

	for is_can_removed{
		count_removed := 0
		for y, row := range grid{
			for x, cell := range row{
				count_adjacent := 0
				if cell.value != "@" {
					continue
				}
				for _, d := range directions{
					value := direction{x: x + d.x, y: y + d.y}
					if value.x < 0 || value.x >= len(row) || value.y < 0 ||  value.y >= len(grid){
						continue
					}
					if grid[value.y][value.x].value == "@"{
						count_adjacent++
					} 
				}
				if count_adjacent < 4{
					grid[y][x].is_removed = true
					count_removed++
				}
			}
		}


		for i, row := range grid{
			for j, cell := range row{
				if cell.is_removed {
					grid[i][j].value = "."
				}
			}
		}

		res += count_removed

		if count_removed == 0{
			is_can_removed = false
		}
	}

	return res
}

func main(){
	file, err  := os.Open("day4.txt")
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