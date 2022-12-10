package main

import "syscall/js"

var counter int = 0

func incfunc(this js.Value, args []js.Value) interface{} {
	counter++
	js.Global().Get("document").Call("getElementById", "int").Set("innerHTML", counter)
	return nil
}

func decrementfunc(this js.Value, args []js.Value) interface{} {
	counter--
	js.Global().Get("document").Call("getElementById", "int").Set("innerHTML", counter)
	return nil
}

func resetfunc(this js.Value, args []js.Value) interface{} {
	counter = 0
	js.Global().Get("document").Call("getElementById", "int").Set("innerHTML", counter)
	return nil
}

func registerCallbacks() {
	js.Global().Set("incfunc", js.FuncOf(incfunc))
	js.Global().Set("decrementfunc", js.FuncOf(decrementfunc))
	js.Global().Set("resetfunc", js.FuncOf(resetfunc))
}

func main() {
	c := make(chan struct{}, 0)
	registerCallbacks()
	<-c
}
