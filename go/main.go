package main

/*
#cgo LDFLAGS: ./lib/libsystemd_parser.a -ldl
#include "./lib/ipaddr.h"
*/
import "C"
import (
	"fmt"
	"unsafe"
)

func main() {
	// call Rust library parser function
	C.parser()

	// instantiate a Rust Person type and set the values and print result
	p := C.new_person()

	// convert the "Go" string into a C string for the struct and ensure the memory gets freed using defer
	cstringName := C.CString("Mike")
	defer C.free(unsafe.Pointer(cstringName))

	p.age = 22
	p.name = cstringName
	fmt.Printf("A rust person {age: %v name: %v}\n", p.age, C.GoString(p.name))

	// change the age field of the Rust struct and print result
	p.age = 43
	fmt.Printf("A rust person {age: %v name: %v}\n", p.age, C.GoString(p.name))
}
