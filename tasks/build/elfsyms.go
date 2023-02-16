package build

import (
	"fmt"
	"os"
	"os/exec"

	"github.com/RedstoneWizard08/kernel/tasks"
	"github.com/fatih/color"
)

var symsLogger tasks.Logger = tasks.NewLogger("Debug Symbols", *color.New(color.FgBlue), *color.New(color.Reset))

func BuildKernelSymbols(config BuildConfig, args tasks.Arguments) {
	symsLogger.Log("[1/8] Copying raw ELF...")

	data, err := os.ReadFile(config.RawKernelElf)

	if err != nil {
		symsLogger.Log("[1/8] >> ERROR << Could not open the input file! Path: %s", config.RawKernelElf)
		os.Exit(1)
	}

	err = os.WriteFile(config.KernelElfTTables, data, 0777)

	if err != nil {
		symsLogger.Log("[1/8] >> ERROR << Could not open the output file! Path: %s", config.KernelElfTTables)
		os.Exit(1)
	}

	symsLogger.Log("[2/8] Generating translation tables...")

	GenerateTranslationTables(config.BSP, config.KernelElfTTables, args.Verbose)

	symsLogger.Log("[3/8] Copying ELF tables...")

	data, err = os.ReadFile(config.KernelElfTTables)

	if err != nil {
		symsLogger.Log("[3/8] >> ERROR << Could not open the input file! Path: %s", config.KernelElfTTables)
		os.Exit(1)
	}

	err = os.WriteFile(config.KernelElfTTablesSyms, data, 0777)

	if err != nil {
		symsLogger.Log("[3/8] >> ERROR << Could not open the output file! Path: %s", config.KernelElfTTablesSyms)
		os.Exit(1)
	}

	symsLogger.Log("[4/8] Generating debug symbols...")

	GenerateDebugSymbols(config.KernelElfTTablesSyms, config.KernelElfTTables+"_symbols.rs", args.Verbose)

	symsLogger.Log("[5/8] Demangling debug symbols...")

	cmd_str := fmt.Sprintf("cat %s_symbols.rs | rustfilt > %s_symbols_demangled.rs", config.KernelElfTTables, config.KernelElfTTables)
	cmd := exec.Command("bash", "-c", cmd_str)

	cmd.Env = os.Environ()
	cmd.Env = append(cmd.Env, "LD_SCRIPT_PATH="+config.LdScriptPath)
	cmd.Env = append(cmd.Env, "KERNEL_SYMBOLS_DEMANGLED_RS="+config.KernelSymbolsElf+"_demangled.rs")

	if args.Verbose {
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr
	}

	err = cmd.Run()

	if err != nil {
		symsLogger.Log("[5/8] >> ERROR << Unable to demangle debug symbols!")
		os.Exit(1)
	}

	symsLogger.Log("[6/8] Finding symbols virtual address...")

	vaddr := GetSymbolsVirtualAddress(config.KernelElfTTablesSyms, args.Verbose)

	symsLogger.Log("[7/8] Building demangled symbols...")

	BuildDemangledSymbols(config.KernelSymbolsLinkerScript, vaddr, config.Target, config.KernelSymbolsManifest, args, config)

	symsLogger.Log("[8/8] Stripping symbols ELF...")

	cmd = exec.Command(config.ObjCopy, "--strip-all", "-O", "binary", config.KernelSymbolsElf, config.KernelSymbolsElf+"_stripped")

	cmd.Env = os.Environ()
	cmd.Env = append(cmd.Env, "LD_SCRIPT_PATH="+config.LdScriptPath)
	cmd.Env = append(cmd.Env, "KERNEL_SYMBOLS_DEMANGLED_RS="+config.KernelSymbolsElf+"_demangled.rs")

	if args.Verbose {
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr
	}

	err = cmd.Run()

	if err != nil {
		symsLogger.Log("[8/8] >> ERROR << Unable to strip symbols ELF!")
		os.Exit(1)
	}
}

func BuildDemangledSymbols(linkerScript string, virtualAddress string, target string, manifest string, args tasks.Arguments, config BuildConfig) {
	symsLogger.Log("[1/1] [Build Demangled Symbols] Building demangled symbols...")

	cmd := exec.Command("cargo", "rustc", "--target="+target, "--release", "--manifest-path", manifest)

	cmd.Env = os.Environ()
	cmd.Env = append(cmd.Env, "RUSTFLAGS=-C link-arg=--script="+linkerScript+" -C link-arg=--section-start=.rodata="+virtualAddress)
	cmd.Env = append(cmd.Env, "LD_SCRIPT_PATH="+config.LdScriptPath)
	cmd.Env = append(cmd.Env, "KERNEL_SYMBOLS_DEMANGLED_RS="+config.KernelSymbolsElf+"_demangled.rs")

	if args.Verbose {
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr
	}

	err := cmd.Run()

	if err != nil {
		symsLogger.Log("[1/1] [Build Demangled Symbols] >> ERROR << Could not build the demangled symbols!")
		os.Exit(1)
	}
}
