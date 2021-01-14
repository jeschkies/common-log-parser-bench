package main

import (
    "bufio"
    "fmt"
    "log"
    "os"
	"regexp"
)	
	

func main() {
    if err := runLine(); err != nil {
		log.Fatal(err)
	}
}


func runLine() error {
    re := regexp.MustCompile(`([0-9a-f\.:]+) ([\S\-]+) ([\S\-]+) \[([^\]]+)\] "(.*)" ([0-9]{3}) ([0-9]*).*$`)

	f, err := os.Open("data/small_access.log")
	if err != nil {
		return err
	}

	count := 0
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
        line := scanner.Text()
		caps := re.FindStringSubmatch(line)
		if caps != nil {
			count++
		}
	}
	if err := scanner.Err(); err != nil {
		return err
	}

	expected := 161761
	if count != expected {
		return fmt.Errorf("Exptected %d matches but found %d", expected, count)
	}
	return nil
}