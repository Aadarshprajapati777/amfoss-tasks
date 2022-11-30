package main

import (
	"net/http"
)

func incrementfunc() {
	fmt.println("increment is clicked")
}

func main() {
	http.ListenAndServe(":8080", http.FileServer(http.Dir(".")))
	incrementfunc()
}
