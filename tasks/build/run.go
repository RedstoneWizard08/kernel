package build

import "github.com/RedstoneWizard08/kernel/tasks"

func Setup() tasks.Config {
	return tasks.DefaultConfig()
}

func Build(args tasks.Arguments) error {
	cfg := Setup()

	BuildKernel(cfg, args)

	return nil
}
