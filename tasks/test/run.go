package test

import (
	"os"
	"os/exec"
	"strings"

	"github.com/RedstoneWizard08/kernel/tasks"
	"github.com/fatih/color"
)

var testLogger tasks.Logger = tasks.NewLogger("Tests", *color.New(color.FgBlue), *color.New(color.Reset))

func Setup() tasks.Config {
	return tasks.DefaultConfig()
}

func Test(args tasks.Arguments) {
	RunTests(Setup(), args)
}

func RunTests(config tasks.Config, args tasks.Arguments) {
	testLogger.Log("[1/2] [Integration] Running integration tests...")

	commandArgs := append([]string{"test"}, config.TestArgs...)
	commandArgs = append(commandArgs, "--test", "*")

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
		testLogger.Log("[1/2] [Integration] >> ERROR << Could not run the integration tests!")
		os.Exit(1)
	}

	testLogger.Log("[2/2] [Unit] Running unit tests...")

	commandArgs = append([]string{"test"}, config.TestArgs...)
	commandArgs = append(commandArgs, "--lib")

	cmd = exec.Command("cargo", commandArgs...)

	cmd.Env = os.Environ()
	cmd.Env = append(cmd.Env, "RUSTFLAGS="+strings.Join(config.RustFlags, " "))
	cmd.Env = append(cmd.Env, "LD_SCRIPT_PATH="+config.LdScriptPath)
	cmd.Env = append(cmd.Env, "KERNEL_SYMBOLS_DEMANGLED_RS="+config.KernelSymbolsElf+"_demangled.rs")

	if args.Verbose {
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr
	}

	err = cmd.Run()

	if err != nil {
		testLogger.Log("[2/2] [Unit] >> ERROR << Could not run the unit tests!")
		os.Exit(1)
	}
}
