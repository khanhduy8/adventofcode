package main

import (
	"bufio"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

type Coordinate struct{
	x float64
	y float64
	is_red bool
}

func (t Coordinate) equals(k Coordinate) bool{
	return t.x == k.x && t.y == k.y
}

type Corner struct{
	point_left Coordinate
	point_right Coordinate
	point_middle Coordinate
}

func (t Corner) is_in_corner(p Coordinate) bool{
	is_in_left := false
	is_in_right := false

	if p.x == t.point_middle.x && (math.Abs(p.y - t.point_left.y) + math.Abs(p.y - t.point_middle.y) <= math.Abs(t.point_middle.y - t.point_left.y)){
		is_in_left = true
	}

	if p.y == t.point_middle.y && (math.Abs(p.x - t.point_right.x) + math.Abs(p.x - t.point_middle.x) <= math.Abs(t.point_middle.x - t.point_right.x)){
		is_in_right = true
	}

	return is_in_left || is_in_right
}

type Rectangle struct{
	a Coordinate
	b Coordinate
	c Coordinate
	d Coordinate
}

func (t Rectangle) is_in_rectangle(p Coordinate) bool{
	length_1 := math.Abs(t.a.x - t.c.x) 
	length_2 := math.Abs(t.a.y - t.c.y)
	if math.Abs(p.x - t.a.x) + math.Abs(p.x - t.c.x) == length_1 && math.Abs(p.y - t.a.y) + math.Abs(p.y - t.c.y) == length_2{
		return true
	}
	return false
}

func (t Rectangle) area() float64{
	return (math.Abs(t.a.x - t.c.x) + 1.0) * (math.Abs(t.a.y - t.c.y) + 1.0)
}

func solve1(lines []string) int {
	points := []Coordinate{}
	for _, line := range lines{
		parts := strings.Split(line, ",")
		x, err := strconv.Atoi(strings.TrimSpace(parts[0]))
		y, err := strconv.Atoi(strings.TrimSpace(parts[1]))

		if err != nil {
			log.Fatalf("Error when parse number")
		}

		points = append(points, Coordinate{x: float64(x), y: float64(y)})
	}
	max_area := 0

	for i := 0; i < len(points); i++{
		for j := i; j < len(points); j++{
			point_a := points[i]
			point_b := points[j]
			area := math.Abs(point_a.x - point_b.x + 1)*math.Abs(point_a.y - point_b.y + 1)
			if area > float64(max_area) {
				max_area = int(area)
			}
		}
	}

	return max_area
}

func solve2(lines []string) int {
	points := []Coordinate{}
	for _, line := range lines{
		parts := strings.Split(line, ",")
		x, err := strconv.Atoi(strings.TrimSpace(parts[0]))
		y, err := strconv.Atoi(strings.TrimSpace(parts[1]))

		if err != nil {
			log.Fatalf("Error when parse number")
		}

		points = append(points, Coordinate{x: float64(x), y: float64(y), is_red: true})
	}

	corners := []Corner{}

	for i := 0; i < len(points); i++{
		for j := i + 1; j < len(points); j++{
			for k := j + 1; k  < len(points); k++{
				point_a := points[i]
				point_b := points[j]
				point_c := points[k]
				switch{
				case (point_a.x == point_b.x && point_a.y == point_c.y):
					corners = append(corners, Corner{point_left: point_b, point_middle: point_a, point_right: point_c})
				case (point_a.x == point_c.x && point_a.y == point_b.y):
					corners = append(corners, Corner{point_left: point_c, point_middle: point_a, point_right: point_b})
				case (point_b.x == point_a.x && point_b.y == point_c.y):
					corners = append(corners, Corner{point_left: point_a, point_middle: point_b, point_right: point_c})
				case (point_b.x == point_c.x && point_b.y == point_a.y):
					corners = append(corners, Corner{point_left: point_c, point_middle: point_b, point_right: point_a})
				case (point_c.x == point_a.x && point_c.y == point_b.y):
					corners = append(corners, Corner{point_left: point_a, point_middle: point_c, point_right: point_b})
				case (point_c.x == point_b.x && point_c.y == point_a.y):
					corners = append(corners, Corner{point_left: point_b, point_middle: point_c, point_right: point_a})
				}
			}
		}
	}

	rectangles := []Rectangle{}

	for i := 0; i < len(corners); i++{
		corner_point_b := corners[i].point_middle
		corner_point_a := corners[i].point_left
		corner_point_c := corners[i].point_right
		corner_point_d := Coordinate{}

		if corner_point_b.x == corner_point_a.x && corner_point_b.y == corner_point_c.y{
			corner_point_d.x = corner_point_c.x
			corner_point_d.y = corner_point_a.y
		} else {
			corner_point_d.x = corner_point_a.x
			corner_point_d.y = corner_point_c.y
		}

		for _, corner := range corners{
			if corner.is_in_corner(corner_point_d){
				for _, point := range points{
					if point.equals(corner_point_d){
						corner_point_d.is_red = true
					}
				}
				rectangles = append(rectangles, Rectangle{a: corner_point_a, b: corner_point_b, c: corner_point_c, d: corner_point_d})
			}
		}
	}
	all_rectangles := rectangles

	for i := 0; i < len(corners); i++{
		corner_point_b := corners[i].point_middle
		corner_point_a := corners[i].point_left
		corner_point_c := corners[i].point_right
		corner_point_d := Coordinate{}

		if corner_point_b.x == corner_point_a.x && corner_point_b.y == corner_point_c.y{
			corner_point_d.x = corner_point_c.x
			corner_point_d.y = corner_point_a.y
		} else {
			corner_point_d.x = corner_point_a.x
			corner_point_d.y = corner_point_c.y
		}


		for _, rectangle := range rectangles{
			if rectangle.is_in_rectangle(corner_point_d){
				for _, point := range points{
					if point.equals(corner_point_d){
						corner_point_d.is_red = true
					}
				}
				all_rectangles = append(all_rectangles, Rectangle{a: corner_point_a, b: corner_point_b, c: corner_point_c, d: corner_point_d})
			}
		}
	}



	// max_grid := 20
	// grid := make([][]string,max_grid)
	// for i := 0; i < max_grid; i++{
	// 	grid[i] = make([]string, max_grid)
	// 	for j := 0; j < max_grid; j++{
	// 		is_in_rectangle := false
	// 		for _, rectange := range rectangles{
	// 			if rectange.is_in_rectangle(Coordinate{x: float64(j), y: float64(i)}){
	// 				is_in_rectangle = true
	// 			}
	// 		}
	// 		if is_in_rectangle{
	// 			grid[i][j] = "X"
	// 		} else{
	// 			grid[i][j] = "."
	// 		}
	// 	}
	// 	log.Printf("%v", grid[i])
	// }

	max_area := 0

	for _, rectangle := range all_rectangles{
		log.Printf("%v %v", rectangle, rectangle.area())
		if rectangle.area() > float64(max_area){
			max_area = int(rectangle.area())
		}
	}
	log.Print(len(rectangles))

	return max_area
}

func main(){
	file, err  := os.Open("day9.txt")
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