package clean

import (
	"log"
	"os"
	"path"
)

var TARGET_DIRS []string

func Setup() {
	TARGET_DIRS[0] = "target"
	TARGET_DIRS[1] = "kernel/target"
	TARGET_DIRS[2] = "kernel_symbols/target"
	TARGET_DIRS[0] = "libraries/test-types/target"
	TARGET_DIRS[0] = "libraries/test-macros/target"
	TARGET_DIRS[0] = "libraries/debug-symbol-types/target"
}

func Clean() {
	root, _ := os.Getwd()

	for _, target := range TARGET_DIRS {
		dir := path.Join(root, target)
		err := os.RemoveAll(dir)

		log.Printf("Clean: %s\n", dir)

		if err != nil {
			log.Fatalf("Clean: Error: %s\n", dir)
			os.Exit(1)
		}
	}
}
