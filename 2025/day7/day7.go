package main

import (
	"bufio"
	"log"
	"os"
)

func solve1(lines []string) int {
	res := 0
	beams_grid := make([][]int, len(lines))
	for i, line := range lines{
		beams_grid[i] = make([]int, len(line))
	}

	for i, line := range lines {
		for j, v := range line {
			switch v{
			case 'S':
				beams_grid[i][j] = 1
				if lines[i+1][j] == '.'{
					beams_grid[i+1][j] = 1
				}
			case '.':
				if i == 0 {
					continue
				}
				if beams_grid[i-1][j] == 1{
					beams_grid[i][j] = 1
				}
			case '^':
				if i == 0{
					continue
				}
				if beams_grid[i-1][j] == 1{
					res += 1
					if j > 0{
						if lines[i][j-1] == '.'{
							beams_grid[i][j-1] = 1
						}
					}
					if j < len(beams_grid[0]) - 1{
						if lines[i][j+1] == '.'{
							beams_grid[i][j+1] = 1
						}
					}
				}				
			}
		}
	}

	return res
}


func solve2(lines []string) int {
	res := 0
	beams_grid := make([][]int, len(lines))
	d := make([][]int, len(lines))
	for i, line := range lines{
		beams_grid[i] = make([]int, len(line))
		d[i] = make([]int, len(line))
	}

	for i, line := range lines {
		for j, v := range line {
			switch v{
			case 'S':
				beams_grid[i][j] = 1
				if lines[i+1][j] == '.'{
					beams_grid[i+1][j] = 1
				}
			case '.':
				if i == 0 {
					continue
				}
				if beams_grid[i-1][j] == 1{
					beams_grid[i][j] = 1
				}
			case '^':
				if i == 0{
					continue
				}
				if beams_grid[i-1][j] == 1{
					if j > 0{
						if lines[i][j-1] == '.'{
							beams_grid[i][j-1] = 1
						}
					}
					if j < len(beams_grid[0]) - 1{
						if lines[i][j+1] == '.'{
							beams_grid[i][j+1] = 1
						}
					}
				}				
			}
		}
	}

	for i, row := range beams_grid{
		for j, v := range row{
			if i == 0 && v == 1{
				d[i][j] = 1
			}
			
			if i > 0{
				if lines[i][j] != '^'{
					d[i][j] += d[i - 1][j]
				}

				if j - 1 >= 0{
					if lines[i][j-1] == '^'{
						d[i][j] += d[i - 1][j - 1]
					}
				}
				if j + 1 < len(d[i]){
					if lines[i][j+1] == '^'{
						d[i][j] += d[i - 1][j + 1]
					}
				}
			}
		}
	}
	
	for _, v := range d[len(d)-1]{
		res += v
	}

	// for _, r := range d{
	// 	log.Printf("%v", r)
	// }
	
	return res
}

func main(){
	file, err  := os.Open("day7.txt")
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