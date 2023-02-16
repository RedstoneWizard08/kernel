package main

import (
	"github.com/RedstoneWizard08/kernel/tasks"
	"github.com/RedstoneWizard08/kernel/tasks/build"
	"github.com/RedstoneWizard08/kernel/tasks/check"
	"github.com/RedstoneWizard08/kernel/tasks/clean"
	"github.com/RedstoneWizard08/kernel/tasks/debug"
	"github.com/RedstoneWizard08/kernel/tasks/run"
	"github.com/RedstoneWizard08/kernel/tasks/test"
	"github.com/alexflint/go-arg"
)

func main() {
	var args tasks.Arguments

	arg.MustParse(&args)

	switch {
	case args.Build != nil:
		build.Build(args)
	case args.Check != nil:
		check.Check()
	case args.Clean != nil:
		clean.Clean()
	case args.Debug != nil:
		debug.Debug()
	case args.Run != nil:
		run.Run()
	case args.Test != nil:
		test.Test()
	}
}
