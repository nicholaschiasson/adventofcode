package day02

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/nicholaschiasson/adventofcode/utils"
)

func Part02(input string) int {
	invalids := make([]int, 0)

	for _, r := range strings.Split(input, ",") {
		low, high, _ := strings.Cut(r, "-")
		lower, err := strconv.Atoi(low)
		utils.Check(err)
		upper, err := strconv.Atoi(high)
		utils.Check(err)
		for i := lower; i <= upper; i++ {
			iStr := fmt.Sprint(i)
			iStrLen := len(iStr)
			for e := 1; e*2 <= iStrLen; e++ {
				if iStrLen%e != 0 {
					continue
				}
				segment := iStr[:e]
				matches := true
				for j := e; j < iStrLen; j += e {
					if segment != iStr[j:j+e] {
						matches = false
						break
					}
				}
				if matches {
					invalids = append(invalids, i)
					break
				}
			}
			/// The following works but it's actually slower than just using string buffers...
			// digits := int(math.Log10(float64(i))) + 1
			// for e := 1; e <= digits/2; e++ {
			// 	segment := i / int(math.Pow(10, float64(digits-e)))
			// 	n := 0
			// 	for j := 0; n < i; j++ {
			// 		n += segment * int(math.Pow(10, float64(e*j)))
			// 	}
			// 	if n == i {
			// 		invalids = append(invalids, i)
			// 		break
			// 	}
			// }
		}
	}

	sum := 0

	for _, i := range invalids {
		sum += i
	}

	return sum
}
