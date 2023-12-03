package day02

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
		{pkg, ReadFileOrDie(fmt.Sprintf("%v/rsrc/inputs/%v/tests/practice_01.txt", root, pkg)), 2286},
		{pkg, ReadFileOrDie(fmt.Sprintf("%v/rsrc/inputs/%v/tests/final.txt", root, pkg)), 59795},
	})
}
