package utils

import "testing"

type TestCase struct {
	Name, Input string
	Want        int
}

func RunTestCases(t *testing.T, aocFunc AocFunc, testCases []TestCase) {
	for _, tt := range testCases {
		t.Run(tt.Name, func(t *testing.T) {
			ans := aocFunc(tt.Input)
			if ans != tt.Want {
				t.Errorf("got %d, want %d", ans, tt.Want)
			}
		})
	}
}
