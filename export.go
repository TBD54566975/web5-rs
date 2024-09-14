package main

/*
#include <stdio.h>
*/
import "C"

//export fortytwo
func fortytwo() C.int {
	return 42
}

//export fortyfour
func fortyfour() C.int {
	return 44
}
