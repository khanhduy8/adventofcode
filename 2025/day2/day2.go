package main

import (
	"bufio"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

type RangeId struct{
	from string
	to string
}

func solve1(range_list []RangeId) int {
	res := 0

	for _, range_id := range range_list{
		from, err_from := strconv.Atoi(range_id.from)
		to, err_to := strconv.Atoi(range_id.to)
		if err_from != nil || err_to != nil{
			log.Fatalf("Error when parse range number")
		}

		length_max_sequence_number := int(math.Ceil(float64(len(range_id.to)) / 2.0))
		max_twice_sequence_number, err := strconv.Atoi(range_id.to[:length_max_sequence_number + 1])
		
		if err != nil {
			log.Fatalf("Error when parse max twice sequence number")
		}

		for twice_sequence_id := 1; twice_sequence_id <= max_twice_sequence_number; twice_sequence_id++{
			invalid_id := twice_sequence_id * int(math.Pow(10,float64(len(strconv.Itoa(twice_sequence_id))))) + twice_sequence_id
			if invalid_id >= from && invalid_id <= to{
				res += invalid_id
			}
		} 
	}
	return res
}

func solve2(range_list []RangeId) int {
	res := 0

	for _, range_id := range range_list{
		invalid_ids := make(map[int]bool)
		from, err_from := strconv.Atoi(range_id.from)
		to, err_to := strconv.Atoi(range_id.to)
		if err_from != nil || err_to != nil{
			log.Fatalf("Error when parse range number")
		}
		
		for d := 2; d <= len(range_id.to); d++{
			length_max_sequence_number := int(math.Ceil(float64(len(range_id.to)) / float64(d)))
			max_sequence_number, err := strconv.Atoi(range_id.to[:length_max_sequence_number + 1])
			if err != nil {
				log.Fatalf("Error when parse max twice sequence number")
			}

			for sequence_id := 1; sequence_id <= max_sequence_number; sequence_id++{
				invalid_id := sequence_id
				for t := 1; t < d; t++{
					invalid_id = invalid_id * int(math.Pow(10,float64(len(strconv.Itoa(sequence_id))))) + sequence_id
				}
				if invalid_id >= from && invalid_id <= to{
					invalid_ids[invalid_id] = true
				}
			} 
		}

		for invalid_id := range invalid_ids{
			res += invalid_id
		}
	}
	return res
}

func main(){
	file, err  := os.Open("day2.txt")
	if err != nil{
		log.Fatal(err)
	}
	defer file.Close()

	var id_range_string_list []string

	var range_list []RangeId
	scanner := bufio.NewScanner(file)

	for scanner.Scan(){
		line := scanner.Text()
		id_range_string_list = strings.Split(line, ",")
	}

	for _, r := range id_range_string_list{
		parts := strings.Split(r, "-")
		range_id := RangeId{
			from: parts[0],
			to: parts[1],
		}
		range_list = append(range_list, range_id)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	res1 := solve1(range_list)
	res2 := solve2(range_list)

	log.Printf("result 1: %d", res1)
	log.Printf("result 2: %d", res2)
}