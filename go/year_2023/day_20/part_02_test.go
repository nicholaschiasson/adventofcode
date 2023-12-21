package day20

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
		{Name: pkg, Input: ReadFileOrDie(fmt.Sprintf("%v/../rsrc/inputs/%v/tests/practice_02.txt", root, pkg)), Want: 1},
		{Name: pkg, Input: ReadFileOrDie(fmt.Sprintf("%v/../rsrc/inputs/%v/tests/final.txt", root, pkg)), Want: 240914003753369},
	})
}
