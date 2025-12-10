package main

import (
	"bufio"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

type Coordinate struct {
	x int
	y int
}

type Rectangle struct {
	area  int
	min_x int
	max_x int
	min_y int
	max_y int
}

type Edge struct {
	min_x int
	max_x int
	min_y int
	max_y int
}

func solve1(lines []string) int {
	points := []Coordinate{}
	for _, line := range lines {
		parts := strings.Split(line, ",")
		x, err := strconv.Atoi(strings.TrimSpace(parts[0]))
		y, err := strconv.Atoi(strings.TrimSpace(parts[1]))

		if err != nil {
			log.Fatalf("Error when parse number")
		}

		points = append(points, Coordinate{x: int(x), y: int(y)})
	}
	max_area := 0

	for i := 0; i < len(points); i++ {
		for j := i; j < len(points); j++ {
			point_a := points[i]
			point_b := points[j]
			min_x := min(point_a.x, point_b.x)
			max_x := max(point_a.x, point_b.x)
			min_y := min(point_a.y, point_b.y)
			max_y := max(point_a.y, point_b.y)
			area := (max_x - min_x + 1) * (max_y - min_y + 1)
			if area > max_area {
				max_area = area
			}
		}
	}

	return max_area
}

func solve2(lines []string) int {
	points := []Coordinate{}
	for _, line := range lines {
		parts := strings.Split(line, ",")
		x, err := strconv.Atoi(strings.TrimSpace(parts[0]))
		y, err := strconv.Atoi(strings.TrimSpace(parts[1]))

		if err != nil {
			log.Fatalf("Error when parse number")
		}

		points = append(points, Coordinate{x: int(x), y: int(y)})
	}

	rectangles := []Rectangle{}
	edges := []Edge{}

	for i := 0; i < len(points); i++ {
		for j := i; j < len(points); j++ {
			point_a := points[i]
			point_b := points[j]
			min_x := min(point_a.x, point_b.x)
			max_x := max(point_a.x, point_b.x)
			min_y := min(point_a.y, point_b.y)
			max_y := max(point_a.y, point_b.y)
			area := (max_x - min_x + 1) * (max_y - min_y + 1)
			rectangles = append(rectangles, Rectangle{area: area, min_x: min_x, max_x: max_x, min_y: min_y, max_y: max_y})
		}
	}

	for i := 0; i < len(points); i++ {
		a := i
		b := i + 1
		if i == len(points)-1 {
			b = 0
		}
		min_x := min(points[a].x, points[b].x)
		max_x := max(points[a].x, points[b].x)
		min_y := min(points[a].y, points[b].y)
		max_y := max(points[a].y, points[b].y)
		edges = append(edges, Edge{min_x: min_x, max_x: max_x, min_y: min_y, max_y: max_y})
	}

	sort.Slice(rectangles, func(i, j int) bool {
		return rectangles[i].area > rectangles[j].area
	})

	max_area := 0

	for _, rectangle := range rectangles {
		is_edge_in_rectangle := false
		for _, edge := range edges {
			is_left := edge.max_x <= rectangle.min_x
			is_right := edge.min_x >= rectangle.max_x
			is_upper := edge.min_y >= rectangle.max_y
			is_lower := edge.max_y <= rectangle.min_y
			if !is_left && !is_right && !is_lower && !is_upper {
				is_edge_in_rectangle = true
			}
		}

		if !is_edge_in_rectangle {
			max_area = rectangle.area
			break
		}
	}

	return max_area
}

func main() {
	file, err := os.Open("day9.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
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
