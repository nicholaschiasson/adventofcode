package utils

import (
	"fmt"
	"testing"
)

type TestCase struct {
	Name, Input string
	Want        int
}

func RunTestCases(t *testing.T, aocFunc AocFunc, testCases []TestCase) {
	for i, tt := range testCases {
		t.Run(fmt.Sprintf("%v#%03v", tt.Name, i), func(t *testing.T) {
			ans := aocFunc(tt.Input)
			if ans != tt.Want {
				t.Errorf("got %d, want %d", ans, tt.Want)
			}
		})
	}
}
