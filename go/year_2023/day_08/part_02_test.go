package day09

import (
	"fmt"
	"reflect"
	"testing"

	. "github.com/nicholaschiasson/adventofcode/utils"
)

func TestPart02(t *testing.T) {
	type void struct{}
	pkgPath := reflect.TypeOf(void{}).PkgPath()
	root, pkg := GetModuleRootAndRelativePackagePath(pkgPath)
	RunTestCases(t, Part02, []TestCase{
		{Name: pkg, Input: ReadFileOrDie(fmt.Sprintf("%v/../rsrc/inputs/%v/tests/practice_03.txt", root, pkg)), Want: 6},
		{Name: pkg, Input: ReadFileOrDie(fmt.Sprintf("%v/../rsrc/inputs/%v/tests/final.txt", root, pkg)), Want: 9064949303801},
	})
}
