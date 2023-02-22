package main

import (
	"os"

	"github.com/RedstoneWizard08/kernel/builder/tasks"
	"github.com/RedstoneWizard08/kernel/builder/tasks/build"
	"github.com/RedstoneWizard08/kernel/builder/tasks/check"
	"github.com/RedstoneWizard08/kernel/builder/tasks/clean"
	"github.com/RedstoneWizard08/kernel/builder/tasks/debug"
	"github.com/RedstoneWizard08/kernel/builder/tasks/run"
	"github.com/RedstoneWizard08/kernel/builder/tasks/test"
	"github.com/alexflint/go-arg"
)

func main() {
	var args tasks.Arguments

	parsed := arg.MustParse(&args)

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
		test.Test(args)
	default:
		parsed.WriteHelp(os.Stdout)
	}
}
