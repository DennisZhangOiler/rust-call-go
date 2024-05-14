package main

import "C"

//export SerializeStrings
func SerializeStrings(a, b, c *C.char) *C.char {
	result := C.GoString(a) + C.GoString(b) + C.GoString(c)
	return C.CString(result)
}

func main() {
}
