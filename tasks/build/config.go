package build

import (
	"fmt"
	"runtime"

	"github.com/RedstoneWizard08/kernel/tasks"
)

type CPU string
type BSP string

const (
	CortexA53 CPU = "cortex-a53"
	CortexA72 CPU = "cortex-a72"

	Raspi3 CPU = CortexA53
	Raspi4 CPU = CortexA72
)

const (
	Rpi3 BSP = "rpi3"
	Rpi4 BSP = "rpi4"
)

type BuildConfig struct {
	CargoArgs []string
	RustcArgs []string
	RustFlags []string

	TargetCPU CPU
	Target    string
	BSP       BSP

	DebugPrints  bool
	LdScriptPath string

	KernelManifest            string
	KernelSymbolsManifest     string
	KernelLinkerScript        string
	KernelSymbolsLinkerScript string
	RawKernelElf              string
	KernelElfTTables          string
	KernelElfTTablesSyms      string
	KernelBin                 string
	KernelSymbolsElf          string

	ObjCopy string
	ObjDump string
	Nm      string
	Readelf string
}

func DefaultConfig() BuildConfig {
	DefaultTarget := "aarch64-unknown-none-softfloat"
	DefaultCPU := CortexA72
	DefaultLdScriptPath := tasks.ResolveRoot("kernel/src/bsp/raspberrypi")
	DefaultBSP := Rpi4
	DefaultKernelManifest := tasks.ResolveRoot("kernel/Cargo.toml")
	DefaultKernelSymbolsManifest := tasks.ResolveRoot("kernel_symbols/Cargo.toml")
	DefaultKernelLinkerScript := "kernel.ld"
	DefaultKernelSymbolsLinkerScript := "kernel_symbols/kernel_symbols.ld"
	DefaultKernelSymbolsElf := tasks.ResolveRoot(fmt.Sprintf("target/%s/release/kernel_symbols", DefaultTarget))
	DefaultRawKernelElf := tasks.ResolveRoot(fmt.Sprintf("target/%s/release/kernel", DefaultTarget))
	DefaultKernelElfTTables := tasks.ResolveRoot(fmt.Sprintf("target/%s/release/kernel+ttables", DefaultTarget))
	DefaultKernelElfTTablesSyms := tasks.ResolveRoot(fmt.Sprintf("target/%s/release/kernel+ttables+symbols", DefaultTarget))
	DefaultReadelfCommand := "aarch64-none-elf-readelf"
	DefaultKernelBin := tasks.ResolveRoot("kernel8.img")

	if runtime.GOARCH == "arm64" {
		DefaultReadelfCommand = "readelf"
	}

	DefaultRustFlags := []string{
		"-C",
		"target-cpu=" + string(DefaultCPU),
		"-C",
		"force-frame-pointers",
		"-C",
		"link-arg=--library-path=" + DefaultLdScriptPath,
		"-C",
		"link-arg=--script=" + DefaultKernelLinkerScript,
	}

	DefaultCargoArgs := []string{
		"--target=" + DefaultTarget,
		"--features",
		"bsp_" + string(DefaultBSP),
		"--release",
	}

	dRustcArgs := []string{
		"-Z",
		"build-std=core,alloc",
		"--manifest-path",
		DefaultKernelManifest,
	}

	DefaultRustcArgs := append(DefaultCargoArgs, dRustcArgs...)

	return BuildConfig{
		CargoArgs:                 DefaultCargoArgs,
		RustFlags:                 DefaultRustFlags,
		RustcArgs:                 DefaultRustcArgs,
		TargetCPU:                 DefaultCPU,
		Target:                    DefaultTarget,
		LdScriptPath:              DefaultLdScriptPath,
		BSP:                       DefaultBSP,
		DebugPrints:               false,
		KernelManifest:            DefaultKernelManifest,
		KernelLinkerScript:        DefaultKernelLinkerScript,
		RawKernelElf:              DefaultRawKernelElf,
		KernelElfTTables:          DefaultKernelElfTTables,
		ObjCopy:                   "rust-objcopy",
		ObjDump:                   "rust-objdump",
		Nm:                        "rust-nm",
		Readelf:                   DefaultReadelfCommand,
		KernelElfTTablesSyms:      DefaultKernelElfTTablesSyms,
		KernelBin:                 DefaultKernelBin,
		KernelSymbolsLinkerScript: DefaultKernelSymbolsLinkerScript,
		KernelSymbolsManifest:     DefaultKernelSymbolsManifest,
		KernelSymbolsElf:          DefaultKernelSymbolsElf,
	}
}
