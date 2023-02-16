package build

import (
	"os"
	"os/exec"
	"strings"

	"github.com/RedstoneWizard08/kernel/tasks"
	"github.com/RedstoneWizard08/kernel/tasks/clean"
	"github.com/fatih/color"
)

var kernelLogger tasks.Logger = tasks.NewLogger("Kernel Build", *color.New(color.FgBlue), *color.New(color.Reset))

func BuildKernelBin(config BuildConfig, args tasks.Arguments) {
	kernelLogger.Log("[1/1] [Bin] Building the kernel...")

	commandArgs := append([]string{"rustc"}, config.RustcArgs...)

	cmd := exec.Command("cargo", commandArgs...)

	cmd.Env = os.Environ()
	cmd.Env = append(cmd.Env, "RUSTFLAGS="+strings.Join(config.RustFlags, " "))
	cmd.Env = append(cmd.Env, "LD_SCRIPT_PATH="+config.LdScriptPath)
	cmd.Env = append(cmd.Env, "KERNEL_SYMBOLS_DEMANGLED_RS="+config.KernelSymbolsElf+"_demangled.rs")

	if args.Verbose {
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr
	}

	err := cmd.Run()

	if err != nil {
		kernelLogger.Log("[1/1] [Bin] >> ERROR << Could not compile the kernel!")
		os.Exit(1)
	}
}

func BuildKernelImg(config BuildConfig, args tasks.Arguments) {
	kernelLogger.Log("[1/2] [Image] Patching final ELF...")

	cmd := exec.Command("ruby", tasks.ResolveRoot("tools/kernel_symbols_tool/main.rb"), "--patch_data", config.KernelElfTTablesSyms, config.KernelSymbolsElf+"_stripped")

	cmd.Env = os.Environ()
	cmd.Env = append(cmd.Env, "LD_SCRIPT_PATH="+config.LdScriptPath)
	cmd.Env = append(cmd.Env, "KERNEL_SYMBOLS_DEMANGLED_RS="+config.KernelSymbolsElf+"_demangled.rs")

	if args.Verbose {
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr
	}

	err := cmd.Run()

	if err != nil {
		kernelLogger.Log("[1/2] [Image] >> ERROR << Unable to patch final ELF!")
		os.Exit(1)
	}

	kernelLogger.Log("[2/2] [Image] Stripping final binary...")

	cmd = exec.Command(config.ObjCopy, "--strip-all", "-O", "binary", config.KernelElfTTablesSyms, config.KernelBin)

	cmd.Env = os.Environ()
	cmd.Env = append(cmd.Env, "LD_SCRIPT_PATH="+config.LdScriptPath)
	cmd.Env = append(cmd.Env, "KERNEL_SYMBOLS_DEMANGLED_RS="+config.KernelSymbolsElf+"_demangled.rs")

	if args.Verbose {
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr
	}

	err = cmd.Run()

	if err != nil {
		kernelLogger.Log("[2/2] [Image] >> ERROR << Unable to strip final binary!")
		os.Exit(1)
	}
}

func BuildKernel(config BuildConfig, args tasks.Arguments) {
	if args.Build.Clean {
		kernelLogger.Log("[Pre-build] Cleaning...")

		clean.Clean()
	}

	kernelLogger.Log("[1/3] Building kernel binary...")

	BuildKernelBin(config, args)

	kernelLogger.Log("[2/3] Building kernel symbols...")

	BuildKernelSymbols(config, args)

	kernelLogger.Log("[3/3] Building kernel image...")

	BuildKernelImg(config, args)

	kernelLogger.Log("Build successful!")
}
