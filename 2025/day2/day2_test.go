package main

import (
	"bufio"
	"os"
	"strings"
	"testing"
)

func TestResult1(t *testing.T) {
	tests := []struct {
		name  string
		file string
		want  int
	}{
		{
			name: "default",
			file: "day2_test.txt",
			want: 1227775554,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			f, err := os.Open(tt.file)
			if err != nil {
				t.Fatalf("failed to open file: %v", err)
			}
			defer f.Close()

			scanner := bufio.NewScanner(f)
			var range_list []RangeId
			var range_string_list []string

			for scanner.Scan(){
				line := scanner.Text()
				range_string_list = strings.Split(line, ",")
			}

			for _, r := range range_string_list{
				parts := strings.Split(r, "-")
				range_list = append(range_list, RangeId{from: parts[0], to: parts[1]})
			}

			if err := scanner.Err(); err != nil{
				t.Fatalf("error reading files %v", err)
			}

			got := solve1(range_list)
			if got != tt.want {
				t.Errorf("result = %d, want %d", got, tt.want)
			}
		})
	}
}

func TestResult2(t *testing.T) {
	tests := []struct {
		name  string
		file string
		want  int
	}{
		{
			name: "default",
			file: "day2_test.txt",
			want: 4174379265,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			f, err := os.Open(tt.file)
			if err != nil {
				t.Fatalf("failed to open file: %v", err)
			}
			defer f.Close()

			scanner := bufio.NewScanner(f)
			var range_list []RangeId
			var range_string_list []string

			for scanner.Scan(){
				line := scanner.Text()
				range_string_list = strings.Split(line, ",")
			}

			for _, r := range range_string_list{
				parts := strings.Split(r, "-")
				range_list = append(range_list, RangeId{from: parts[0], to: parts[1]})
			}

			if err := scanner.Err(); err != nil{
				t.Fatalf("error reading files %v", err)
			}

			got := solve2(range_list)
			if got != tt.want {
				t.Errorf("result = %d, want %d", got, tt.want)
			}
		})
	}
}