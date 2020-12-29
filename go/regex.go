package main

import (
    "fmt"
    "io/ioutil"
	"regexp"
)	
	

func main() {
    re := regexp.MustCompile(`([\da-f\.:]*) (.*) (.*) \[(.*)\] "(.*)" (\d{3}) (\d*).*\n`)

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
