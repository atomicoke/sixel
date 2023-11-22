package sixel

import (
	"fmt"
	"testing"
)

func TestName(t *testing.T) {
	_, err := crop("/home/like/Pictures/2023-11-03_09-14.png", 100, 100)
	if err != nil {
		fmt.Println(err)
	}
}
