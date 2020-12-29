package main

import (
    "fmt"
    "io/ioutil"
	"regexp"
)	
	

func main() {
    re := regexp.MustCompile(`([0-9a-f\.:]+) ([\S\-]+) ([\S\-]+) \[([^\]]+)\] "(.*)" ([0-9]{3}) ([0-9]*).*\n`)

    b, err := ioutil.ReadFile("data/small_access.log")
    if err != nil {
        panic(err)
    }

    f := re.FindAllSubmatch(b, -1)
    expected := 161761
    if len(f) != expected || len(f[0]) != 8 {
        panic(fmt.Errorf("Exptected %d matches but found %d", expected, len(f)))
    }
}
