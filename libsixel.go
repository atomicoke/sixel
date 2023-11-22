package sixel

/*
#cgo CFLAGS: -I.
#cgo LDFLAGS: -L./libsixel/target/release -llibsixel
#include <stdlib.h>
#include "./libsixel.h"
*/
import "C"
import (
	"fmt"
	"io"
)

type Sixel struct {
	filepath      string
	width, height int32
}

func New(filepath string) *Sixel {
	return &Sixel{
		filepath: filepath,
		width:    -1,
		height:   -1,
	}
}

func (s *Sixel) Crop(width, height int32) *Sixel {
	s.width, s.height = width, height
	return s
}

func (s *Sixel) EncodeTo(w io.Writer) error {
	_, err := fmt.Fprint(w, encodeTo(s.filepath, s.width, s.height))
	return err
}

func encodeTo(filepath string, width, height int32) string {
	o := C.encode_to(C.CString(filepath), C.int(width), C.int(height))
	output := C.GoString(o)
	return output
}
