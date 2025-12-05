package main

import (
	"bufio"
	"os"
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
			file: "day3_test.txt",
			want: 357,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			f, err := os.Open(tt.file)
			if err != nil {
				t.Fatalf("failed to open file: %v", err)
			}
			defer f.Close()

			var lines []string
			scanner := bufio.NewScanner(f)
			for scanner.Scan(){
				lines = append(lines, scanner.Text())
			}

			if err := scanner.Err(); err != nil{
				t.Fatalf("error reading files %v", err)
			}

			got := solve1(lines)
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
			file: "day3_test.txt",
			want: 3121910778619,
		},		
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			f, err := os.Open(tt.file)
			if err != nil {
				t.Fatalf("failed to open file: %v", err)
			}
			defer f.Close()

			var lines []string
			scanner := bufio.NewScanner(f)
			for scanner.Scan(){
				lines = append(lines, scanner.Text())
			}

			if err := scanner.Err(); err != nil{
				t.Fatalf("error reading files %v", err)
			}

			got := solve2(lines)
			if got != tt.want {
				t.Errorf("result = %d, want %d", got, tt.want)
			}
		})
	}
}