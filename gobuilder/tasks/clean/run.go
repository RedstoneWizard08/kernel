package clean

import (
	"os"
	"path"

	"github.com/RedstoneWizard08/kernel/builder/tasks"
	"github.com/fatih/color"
)

var cleanLogger tasks.Logger = tasks.NewLogger("Clean", *color.New(color.FgBlue), *color.New(color.Reset))

func Setup() [6]string {
	var targetDirs [6]string

	targetDirs[0] = "target"
	targetDirs[1] = "kernel/target"
	targetDirs[2] = "kernel_symbols/target"
	targetDirs[3] = "libraries/test-types/target"
	targetDirs[4] = "libraries/test-macros/target"
	targetDirs[5] = "libraries/debug-symbol-types/target"

	return targetDirs
}

func Clean() {
	targetDirs := Setup()

	root, _ := os.Getwd()

	for i := 0; i < len(targetDirs); i++ {
		target := targetDirs[i]
		dir := path.Join(root, target)
		err := os.RemoveAll(dir)

		cleanLogger.Log("[1/1] Remove => %s", dir)

		if err != nil {
			cleanLogger.Log("[1/1] >> Error << %s", dir)
			os.Exit(1)
		}
	}
}
