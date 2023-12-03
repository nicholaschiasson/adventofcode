package utils

import (
	"os"
)

func GetModuleRootAndRelativePackagePath(pkgPath string) (moduleRoot, relativePackagePath string) {
	wd, err := os.Getwd()
	Check(err)
	i := 0
	lenWd := len(wd)
	lenPkg := len(pkgPath)
	for i < lenWd && i < lenPkg && wd[lenWd-i-1] == pkgPath[lenPkg-i-1] {
		i++
	}
	return wd[:lenWd-i], wd[lenWd-i+1:]
}
