package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"time"

	"github.com/glkz/rabinkarp"
)

func main() {
    if err := runLine(); err != nil {
		log.Fatal(err)
	}
}


func runLine() error {

	subs := []string{
		"127", "GET", "HTTP", "Mozilla", "Apple", 
		"www", "online", "Bot", "alternative", "bing",
		"check", "viewer", "FR", "wars", "star",
		"arms", "US", "assassin", "client", "Mobile",
		"dotbot", "robot", "Crawler", "href", "compatible",
		"2020", "score", "HTML", "Dec", "19",
		"127", "GET", "HTTP", "Mozilla", "Apple", 
		"www", "online", "Bot", "alternative", "bing",
		"check", "viewer", "FR", "wars", "star",
		"arms", "US", "assassin", "client", "Mobile",
		"dotbot", "robot", "Crawler", "href", "compatible",
		"2020", "score", "HTML", "Dec", "19",
		"127", "GET", "HTTP", "Mozilla", "Apple", 
		"www", "online", "Bot", "alternative", "bing",
		"check", "viewer", "FR", "wars", "star",
		"arms", "US", "assassin", "client", "Mobile",
		"dotbot", "robot", "Crawler", "href", "compatible",
		"2020", "score", "HTML", "Dec", "19",
		"127", "GET", "HTTP", "Mozilla", "Apple", 
		"www", "online", "Bot", "alternative", "bing",
		"check", "viewer", "FR", "wars", "star",
		"arms", "US", "assassin", "client", "Mobile",
		"dotbot", "robot", "Crawler", "href", "compatible",
		"2020", "score", "HTML", "Dec", "19",
		"127", "GET", "HTTP", "Mozilla", "Apple", 
		"www", "online", "Bot", "alternative", "bing",
		"check", "viewer", "FR", "wars", "star",
		"arms", "US", "assassin", "client", "Mobile",
		"dotbot", "robot", "Crawler", "href", "compatible",
		"2020", "score", "HTML", "Dec", "19",
		"127", "GET", "HTTP", "Mozilla", "Apple", 
		"www", "online", "Bot", "alternative", "bing",
		"check", "viewer", "FR", "wars", "star",
		"arms", "US", "assassin", "client", "Mobile",
		"dotbot", "robot", "Crawler", "href", "compatible",
		"2020", "score", "HTML", "Dec", "19",
		"127", "GET", "HTTP", "Mozilla", "Apple", 
		"www", "online", "Bot", "alternative", "bing",
		"check", "viewer", "FR", "wars", "star",
		"arms", "US", "assassin", "client", "Mobile",
		"dotbot", "robot", "Crawler", "href", "compatible",
		"2020", "score", "HTML", "Dec", "19",
		"127", "GET", "HTTP", "Mozilla", "Apple", 
		"www", "online", "Bot", "alternative", "bing",
		"check", "viewer", "FR", "wars", "star",
		"arms", "US", "assassin", "client", "Mobile",
		"dotbot", "robot", "Crawler", "href", "compatible",
		"2020", "score", "HTML", "Dec", "19",
		"127", "GET", "HTTP", "Mozilla", "Apple", 
		"www", "online", "Bot", "alternative", "bing",
		"check", "viewer", "FR", "wars", "star",
		"arms", "US", "assassin", "client", "Mobile",
		"dotbot", "robot", "Crawler", "href", "compatible",
		"2020", "score", "HTML", "Dec", "19",
		"127", "GET", "HTTP", "Mozilla", "Apple", 
		"www", "online", "Bot", "alternative", "bing",
		"check", "viewer", "FR", "wars", "star",
		"arms", "US", "assassin", "client", "Mobile",
		"dotbot", "robot", "Crawler", "href", "compatible",
		"2020", "score", "HTML", "Dec", "19",
		"127", "GET", "HTTP", "Mozilla", "Apple", 
		"www", "online", "Bot", "alternative", "bing",
		"check", "viewer", "FR", "wars", "star",
		"arms", "US", "assassin", "client", "Mobile",
		"dotbot", "robot", "Crawler", "href", "compatible",
		"2020", "score", "HTML", "Dec", "19",
		"127", "GET", "HTTP", "Mozilla", "Apple", 
		"www", "online", "Bot", "alternative", "bing",
		"check", "viewer", "FR", "wars", "star",
		"arms", "US", "assassin", "client", "Mobile",
		"dotbot", "robot", "Crawler", "href", "compatible",
		"2020", "score", "HTML", "Dec", "19",
	}

	f, err := os.Open("data/small_access.log")
	if err != nil {
		return err
	}

	count := 0
	scanner := bufio.NewScanner(f)

	start := time.Now()

	for scanner.Scan() {
        line := scanner.Text()
		matches := rabinkarp.Search(line, subs)
		if len(matches) == len(subs) {
			count++
		}
	}

	elapsed := time.Since(start)
	log.Printf("Matching took %s", elapsed)

	if err := scanner.Err(); err != nil {
		return err
	}

	expected := 34024
	if count != expected {
		return fmt.Errorf("Exptected %d matches but found %d", expected, count)
	}
	return nil
}
