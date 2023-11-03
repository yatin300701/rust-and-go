package greeting

import "fmt"

func HelloName(name string) string {
	message := fmt.Sprintf("Hi,%v. Welcome!!", name)
	return message
}
