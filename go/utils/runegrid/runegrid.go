package runegrid

import (
	"fmt"
	"strings"
)

type RuneGrid struct {
	string
	height int
	width  int
}

func New(s string) RuneGrid {
	width := strings.IndexByte(s, '\n')
	if width < 0 {
		const maxErrStringWidth = 64
		if len(s) > maxErrStringWidth {
			s = s[:maxErrStringWidth] + "..."
		}
		panic(fmt.Sprintf("cannot construct grid from string: %v", s))
	}
	height := len(s) / width
	return RuneGrid{
		string: s,
		height: height,
		width:  width,
	}
}

func (self RuneGrid) Get(x, y int) byte {
	if x < 0 || y < 0 || x >= self.width || y >= self.height {
		panic(fmt.Sprintf("attempt to index out of grid bounds at x: %v, y: %v", x, y))
	}
	return self.string[(self.width+1)*y+x]
}

func (self RuneGrid) Height() int {
	return self.height
}

func (self RuneGrid) Width() int {
	return self.width
}
