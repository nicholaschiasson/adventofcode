package day09

import (
	"fmt"
	"reflect"
	"testing"

	. "github.com/nicholaschiasson/adventofcode/utils"
)

func TestPart01(t *testing.T) {
	type void struct{}
	pkgPath := reflect.TypeOf(void{}).PkgPath()
	root, pkg := GetModuleRootAndRelativePackagePath(pkgPath)
	RunTestCases(t, Part01, []TestCase{
		{Name: pkg, Input: ReadFileOrDie(fmt.Sprintf("%v/../rsrc/inputs/%v/tests/practice_01.txt", root, pkg)), Want: 2},
		{Name: pkg, Input: ReadFileOrDie(fmt.Sprintf("%v/../rsrc/inputs/%v/tests/practice_02.txt", root, pkg)), Want: 6},
		{Name: pkg, Input: ReadFileOrDie(fmt.Sprintf("%v/../rsrc/inputs/%v/tests/final.txt", root, pkg)), Want: 12737},
	})
}
