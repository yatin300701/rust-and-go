package main

import (
	"fmt"
	"practice/main/greeting"

	"rsc.io/quote"
)

// import "greeting"

func main() {
	fmt.Println(quote.Go())
	fmt.Println(greeting.HelloName("Yatin"))
}
