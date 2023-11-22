package main

import (
	"flag"
	"os"
	"sixel"
)

func main() {
	width := flag.Int("w", -1, "image width")
	height := flag.Int("h", -1, "image height")
	filepath := flag.String("f", "", "filepath")
	flag.Parse()
	if *filepath == "" {
		if len(os.Args) > 1 {
			filepath = &os.Args[1]
		}

		if *filepath == "" {
			panic("filepath is empty")
		}
	}

	err := sixel.
		New(*filepath).
		Crop(int32(*width), int32(*height)).
		EncodeTo(os.Stdout)
	if err != nil {
		panic(err)
	}
}
