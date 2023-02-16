package build

import "github.com/RedstoneWizard08/kernel/tasks"

func Setup() BuildConfig {
	return DefaultConfig()
}

func Build(args tasks.Arguments) error {
	cfg := Setup()

	BuildKernel(cfg, args)

	return nil
}
