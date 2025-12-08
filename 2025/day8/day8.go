package main

import (
	"bufio"
	"log"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

type Coordinate struct{
	x float64
	y float64
	z float64
}

type PairJunctionBox struct{
	A Coordinate
	B Coordinate
}

func (t PairJunctionBox) distance() float64{
	return math.Sqrt(math.Pow(t.A.x - t.B.x,2) + math.Pow(t.A.y - t.B.y,2) + math.Pow(t.A.z - t.B.z,2))
}

type Circuit struct{
	coordinates []Coordinate
}

func (t Circuit) isExist(junction_box Coordinate) bool{
	for _, box := range t.coordinates{
		if box.x == junction_box.x && box.y == junction_box.y && box.z == junction_box.z{
			return true
		}
	}
	return false
}

func (t *Circuit) add(junction_box Coordinate){
	t.coordinates = append(t.coordinates, junction_box)
}

func (t *Circuit) merge(list_junction_box []Coordinate){
	for _, boxB := range list_junction_box{
		isDup := false
		for _, boxA := range t.coordinates{
			if boxA.x == boxB.x && boxA.y == boxB.y && boxA.z == boxB.z{
				isDup = true
			}
		}
		if !isDup{
			t.coordinates = append(t.coordinates, boxB)
		}
	}
}

func (t *Circuit) remove_junction_box(){
	t.coordinates = []Coordinate{}
}

func NewCircuit() *Circuit{
	return &Circuit{}
}

func solve1(lines []string, num_box_connect int) int {
	junction_boxes :=  []Coordinate{}

	for _, line := range lines{
		parts := strings.Split(line, ",")
		if len(parts) == 3{
			x, err := strconv.Atoi(parts[0])
			y, err := strconv.Atoi(parts[1])
			z, err := strconv.Atoi(parts[2])
			if err != nil {
				log.Fatalf("Error when parse number")
			}
			junction_boxes = append(junction_boxes, Coordinate{x: float64(x), y: float64(y), z: float64(z)})
		}
	}

	pair_junction_boxes := []PairJunctionBox{}

	for i := 0; i < len(junction_boxes); i++{
		for j := i + 1; j < len(junction_boxes); j++{
			pair_junction_boxes = append(pair_junction_boxes, PairJunctionBox{A: junction_boxes[i], B: junction_boxes[j]})
		}
	}

	sort.Slice(pair_junction_boxes, func(i, j int) bool {
		return pair_junction_boxes[i].distance() - pair_junction_boxes[j].distance() <= 0
	})

	circuits := []Circuit{}

	for i := 0; i < num_box_connect; i++{
		box_A := pair_junction_boxes[i].A
		box_B := pair_junction_boxes[i].B

		if len(circuits) == 0{
			new_circuit := NewCircuit()
			new_circuit.add(box_A)
			new_circuit.add(box_B)
			circuits = append(circuits, *new_circuit)
			continue
		}
		num_added := 0
		track_circuit_add := []int{}
		for i, circuit := range circuits{
			point_circuit := &circuits[i]
			is_boxA_in_circuit := circuit.isExist(box_A)
			is_boxB_in_circuit := circuit.isExist(box_B)
			if !is_boxA_in_circuit && is_boxB_in_circuit{
				point_circuit.add(box_A)
				num_added++
				track_circuit_add = append(track_circuit_add, i)
				continue
			}

			if is_boxA_in_circuit && !is_boxB_in_circuit{
				point_circuit.add(box_B)
				num_added++
				track_circuit_add = append(track_circuit_add, i)
				continue
			}

			if is_boxA_in_circuit && is_boxB_in_circuit{
				num_added--
			}
		}

		if num_added == 0{
			new_circuit := NewCircuit()
			new_circuit.add(box_A)
			new_circuit.add(box_B)
			circuits = append(circuits, *new_circuit)
		}

		if num_added == 2{
			circuitA := &circuits[track_circuit_add[0]]
			circuitB := &circuits[track_circuit_add[1]]
			circuitA.merge(circuitB.coordinates)
			circuits = append(circuits[:track_circuit_add[1]],circuits[track_circuit_add[1]+1:]...)
		}
	}

	sort.Slice(circuits, func(i, j int) bool {
		return len(circuits[i].coordinates) >= len(circuits[j].coordinates)
	})

	res := 1

	for i := 0; i < 3; i++{
		res *= len(circuits[i].coordinates)
	}

	return res
}


func solve2(lines []string) int {
	junction_boxes :=  []Coordinate{}

	for _, line := range lines{
		parts := strings.Split(line, ",")
		if len(parts) == 3{
			x, err := strconv.Atoi(parts[0])
			y, err := strconv.Atoi(parts[1])
			z, err := strconv.Atoi(parts[2])
			if err != nil {
				log.Fatalf("Error when parse number")
			}
			junction_boxes = append(junction_boxes, Coordinate{x: float64(x), y: float64(y), z: float64(z)})
		}
	}

	pair_junction_boxes := []PairJunctionBox{}

	for i := 0; i < len(junction_boxes); i++{
		for j := i + 1; j < len(junction_boxes); j++{
			pair_junction_boxes = append(pair_junction_boxes, PairJunctionBox{A: junction_boxes[i], B: junction_boxes[j]})
		}
	}

	sort.Slice(pair_junction_boxes, func(i, j int) bool {
		return pair_junction_boxes[i].distance() <= pair_junction_boxes[j].distance()
	})

	circuits := []Circuit{}
	var last_boxA Coordinate
	var last_boxB Coordinate

	for i := 0; i < len(pair_junction_boxes); i++{
		box_A := pair_junction_boxes[i].A
		box_B := pair_junction_boxes[i].B

		if len(circuits) == 0{
			new_circuit := NewCircuit()
			new_circuit.add(box_A)
			new_circuit.add(box_B)
			circuits = append(circuits, *new_circuit)
			continue
		}
		num_added := 0
		track_circuit_add := []int{}
		for i, circuit := range circuits{
			point_circuit := &circuits[i]
			is_boxA_in_circuit := circuit.isExist(box_A)
			is_boxB_in_circuit := circuit.isExist(box_B)
			if !is_boxA_in_circuit && is_boxB_in_circuit{
				point_circuit.add(box_A)
				num_added++
				track_circuit_add = append(track_circuit_add, i)
				continue
			}

			if is_boxA_in_circuit && !is_boxB_in_circuit{
				point_circuit.add(box_B)
				num_added++
				track_circuit_add = append(track_circuit_add, i)
				continue
			}

			if is_boxA_in_circuit && is_boxB_in_circuit{
				num_added--
			}
		}

		if num_added == 0{
			new_circuit := NewCircuit()
			new_circuit.add(box_A)
			new_circuit.add(box_B)
			circuits = append(circuits, *new_circuit)
		}

		if num_added == 2{
			circuitA := &circuits[track_circuit_add[0]]
			circuitB := &circuits[track_circuit_add[1]]
			circuitA.merge(circuitB.coordinates)
			circuits = append(circuits[:track_circuit_add[1]],circuits[track_circuit_add[1]+1:]...)
		}

		if num_added != -1{
			last_boxA = box_A
			last_boxB = box_B
		}
	}

	return int(last_boxA.x) * int(last_boxB.x)
}

func main(){
	file, err  := os.Open("day8.txt")
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

	res1 := solve1(lines, 1000)
	res2 := solve2(lines)

	log.Printf("result 1: %d", res1)
	log.Printf("result 2: %d", res2)
}