package main

import (
	"bufio"
	"log"
	"os"
	"sort"
	"strings"
)

func parse_data(lines []string) map[string][]string{
	paths := make(map[string][]string)

	for _, line := range lines{
		parts := strings.Split(line, " ")
		if len(parts) < 2{
			continue
		}
		key := ""
		if parts[0][len(parts[0]) - 1] == ':'{
			key = strings.Trim(parts[0], ":")
			paths[key] = make([]string,0)
		}

		for i := 1; i < len(parts); i++{
			paths[key] = append(paths[key], strings.TrimSpace(parts[i]))
		}
	}
	
	return paths
}

func travel(paths map[string][]string, visited map[string]bool, start, end string) int{
	if start == end {
		return 1
	}

	visited[start] = true
	total := 0

	for _, next_device := range paths[start]{
		if !visited[next_device]{
			total += travel(paths, visited, next_device, end)
		}
	}

	visited[start] = false
	return total
}

func travel_2(paths map[string][]string, visited map[string]bool, state map[string]map[string]int, start, end string, is_visit_dac bool, is_visit_fft bool, track []string) int{
	if start == end {
		if is_visit_dac && is_visit_fft{
			return 1
		}
		return 0
	}

	track = append(track, start)
	state_list := track
	sort.Strings(state_list)
	state_key := strings.Join(state_list, "")

	if state[start] == nil{
		state[start] = map[string]int{}
	}

	if  v, ok := state[start][state_key]; ok{
		// log.Println(v)
		return v
	}

	total := 0
	visited[start] = true

	for _, next_device := range paths[start]{
		if !visited[next_device]{
			total += travel_2(paths, visited, state, next_device, end, is_visit_dac || next_device == "dac", is_visit_fft || next_device == "fft", track)
		}
		
	}

	visited[start] = false
	state[start][state_key] = total
	return total
}

func solve1(lines []string) int {
	paths := parse_data(lines)
	visited := make(map[string]bool)
	for k := range paths{
		visited[k] = false
	}
	res := travel(paths, visited, "you", "out")
	return res
}

func solve2(lines []string) int {
	paths := parse_data(lines)
	visited := make(map[string]bool)
	state := make(map[string]map[string]int)
	res := travel_2(paths, visited, state, "svr", "out", false, false, []string{})
	return res
}

func main(){
	file, err  := os.Open("day11.txt")
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