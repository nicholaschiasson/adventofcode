package utils

import (
	"os"
	"strings"
)

func ReadFileOrDie(path string) string {
	input, err := os.ReadFile(path)
	Check(err)
	return strings.TrimSpace(string(input))
}
