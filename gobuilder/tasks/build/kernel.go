package build

import (
	"context"
	"os"
	"strings"

	"github.com/RedstoneWizard08/kernel/builder/tasks"
	"github.com/RedstoneWizard08/kernel/builder/tasks/clean"
	"github.com/docker/docker/client"
	"github.com/fatih/color"
)

var kernelLogger tasks.Logger = tasks.NewLogger("Kernel Build", *color.New(color.FgBlue), *color.New(color.Reset))

func gwd() string {
	wd, _ := os.Getwd()

	return wd
}

func BuildKernelBin(config tasks.Config, args tasks.Arguments, ctx context.Context, cli *client.Client) {
	kernelLogger.Log("[1/1] [Bin] Building the kernel...")

	commandArgs := append([]string{"cargo", "rustc"}, config.RustcArgs...)

	kernelLogger.Log("%s", commandArgs)

	env := []string{
		"RUSTFLAGS=" + strings.Join(config.RustFlags, " "),
		"LD_SCRIPT_PATH=" + config.LdScriptPath,
		"KERNEL_SYMBOLS_DEMANGLED_RS=" + config.KernelSymbolsElf + "_demangled.rs",
	}

	err := RunDocker(commandArgs, env, config, ctx, cli, args.Verbose)

	if err != nil {
		kernelLogger.Log("[1/1] [Bin] >> ERROR << Could not compile the kernel!")
		os.Exit(1)
	}
}

func BuildKernelImg(config tasks.Config, args tasks.Arguments, ctx context.Context, cli *client.Client) {
	kernelLogger.Log("[1/2] [Image] Patching final ELF...")

	commandArgs := []string{
		config.ObjCopy,
		"--strip-all",
		"-O",
		"binary",
		config.RawKernelElf + "_symbols",
		config.RawKernelElf + "_symbols_stripped",
	}

	env := []string{
		"RUSTFLAGS=" + strings.Join(config.RustFlags, " "),
		"LD_SCRIPT_PATH=" + config.LdScriptPath,
		"KERNEL_SYMBOLS_DEMANGLED_RS=" + config.KernelSymbolsElf + "_demangled.rs",
	}

	err := RunDocker(commandArgs, env, config, ctx, cli, args.Verbose)

	if err != nil {
		kernelLogger.Log("[1/2] [Image] >> ERROR << Unable to patch final ELF!")
		os.Exit(1)
	}

	kernelLogger.Log("[2/2] [Image] Patching final ELF...")

	commandArgs = []string{
		"ruby",
		tasks.ResolveRoot("tools/kernel_symbols_tool/main.rb"),
		"--patch_data",
		config.KernelElfTTablesSyms,
		config.RawKernelElf + "_symbols_stripped",
	}

	err = RunDocker(commandArgs, env, config, ctx, cli, args.Verbose)

	if err != nil {
		kernelLogger.Log("[2/2] [Image] >> ERROR << Unable to patch final ELF!")
		os.Exit(1)
	}

	kernelLogger.Log("[2/2] [Image] Stripping final binary...")

	commandArgs = []string{
		config.ObjCopy,
		"--strip-all",
		"-O",
		"binary",
		config.KernelElfTTablesSyms,
		config.KernelBin,
	}

	err = RunDocker(commandArgs, env, config, ctx, cli, args.Verbose)

	if err != nil {
		kernelLogger.Log("[2/2] [Image] >> ERROR << Unable to strip final binary!")
		os.Exit(1)
	}
}

func BuildKernel(config tasks.Config, args tasks.Arguments, ctx context.Context, cli *client.Client) {
	if args.Build.Clean {
		kernelLogger.Log("[Pre-build] Cleaning...")

		clean.Clean()
	}

	kernelLogger.Log("[1/3] Building kernel binary...")

	BuildKernelBin(config, args, ctx, cli)

	kernelLogger.Log("[2/3] Building kernel symbols...")

	BuildKernelSymbols(config, args, ctx, cli)

	kernelLogger.Log("[3/3] Building kernel image...")

	BuildKernelImg(config, args, ctx, cli)

	kernelLogger.Log("Build successful!")
}
