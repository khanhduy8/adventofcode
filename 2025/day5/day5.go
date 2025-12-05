package main

import (
	"bufio"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

func solve1(lines []string) int {
	type Range struct{
		From int
		To int
	}

	type Ingredient struct{
		Id int
		IsFresh bool
	}

	ingredient_range_list := []Range{}
	ingredient_ids := []Ingredient{}
	res := 0

	for _, line := range lines{
		line = strings.Trim(line, "")
		parts := strings.Split(line, "-")


		if len(parts) == 2 {
			from, err1 := strconv.Atoi(parts[0])
			to, err2 := strconv.Atoi(parts[1])

			if err1 != nil || err2 != nil{
				log.Fatal("Error when parse id range")
			}
			ingredient_range := Range{From: from, To: to}
			ingredient_range_list = append(ingredient_range_list, ingredient_range)
		} else if len(line) > 0{
			id, err := strconv.Atoi(line)
			if err != nil{
				log.Fatal("Error when parse id")
			}
			ingredient_ids = append(ingredient_ids, Ingredient{Id: id, IsFresh: false})
		}
	}

	for i, ingredient := range ingredient_ids{
		for _, ingredient_range := range ingredient_range_list{
			if ingredient.Id >= ingredient_range.From && ingredient.Id <= ingredient_range.To {
				ingredient_ids[i].IsFresh = true
			}
		}
	}

	for _, ingredient := range ingredient_ids{
		if ingredient.IsFresh {
			res++
		}
	}

	return res
}

func solve2(lines []string) int {
	type Range struct{
		From int
		To int
	}

	ingredient_range_list := []Range{}
	res := 0

	for _, line := range lines{
		line = strings.Trim(line, "")
		parts := strings.Split(line, "-")


		if len(parts) == 2 {
			from, _ := strconv.Atoi(parts[0])
			to, _ := strconv.Atoi(parts[1])

			ingredient_range := Range{From: from, To: to}
			ingredient_range_list = append(ingredient_range_list, ingredient_range)
		} else if len(line) == 0 {
			break
		}
	}

	sort.Slice(ingredient_range_list, func(i,j int) bool{
		if ingredient_range_list[i].From != ingredient_range_list[j].From {
			return ingredient_range_list[i].From < ingredient_range_list[j].From
		}
		return ingredient_range_list[i].To <= ingredient_range_list[j].To
	})

	current_range := ingredient_range_list[0]
	for i := 1; i < len(ingredient_range_list); i++{
		next_range := ingredient_range_list[i]
		if next_range.From > current_range.To{
			res += current_range.To - current_range.From + 1
			current_range = next_range
			continue
		}

		if next_range.From <= current_range.To{
			current_range.To = max(next_range.To, current_range.To)
			continue
		}
		
	}
	res += current_range.To - current_range.From + 1

	return res
}

func main(){
	file, err  := os.Open("day5.txt")
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