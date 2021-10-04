package main

import (
    "bufio"
    "fmt"
    "log"
	"os"
	rosie "github.com/jeschkies/rosie-go/pkg"
)	

func main() {
    if err := runLine(); err != nil {
		log.Fatal(err)
	}
}


func runLine() error {
	engine, err := rosie.New("matcher")
	if err != nil {
		return err
	}
	pat, _, err := engine.Compile(`net.ip "-" "-" "["date.day"/"date.month_name"/"date.year":"time.rfc2822 time.rfc2822_zone"]" "\""net.http_command_name net.path net.http_version"\"" [:digit:]+ [:digit:]`)

	f, err := os.Open("data/small_access.log")
	if err != nil {
		return err
	}

	count := 0
	var match *rosie.Match
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
        line := scanner.Text()
		match, err = pat.MatchString(line)
		if err != nil {
			return err
		}
		if match != nil {
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
