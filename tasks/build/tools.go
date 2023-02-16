package build

import (
	"os"
	"os/exec"

	"github.com/RedstoneWizard08/kernel/tasks"
	"github.com/fatih/color"
)

var toolsLogger tasks.Logger = tasks.NewLogger("Build Utils", *color.New(color.FgBlue), *color.New(color.Reset))

func GenerateTranslationTables(target BSP, elfPath string, verbose bool) {
	toolsLogger.Log("[1/1] [Translation Tables] Generating translation tables from ELF...")

	if target != Rpi3 && target != Rpi4 {
		toolsLogger.Log("[1/1] [Translation Tables] >> ERROR << Invalid BSP target!")
		os.Exit(1)
	}

	cmd := exec.Command("ruby", tasks.ResolveRoot("tools/translation_table_tool/main.rb"), string(target), elfPath)

	cmd.Env = os.Environ()

	if verbose {
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr
	}

	err := cmd.Run()

	if err != nil {
		toolsLogger.Log("[1/1] [Translation Tables] >> ERROR << Unable to generate translation tables!")
		os.Exit(1)
	}
}

func GenerateDebugSymbols(inPath string, outPath string, verbose bool) {
	toolsLogger.Log("[1/1] [Debug Symbols] Generating debug symbols from ELF...")

	cmd := exec.Command("ruby", tasks.ResolveRoot("tools/kernel_symbols_tool/main.rb"), "--gen_symbols", inPath, outPath)

	cmd.Env = os.Environ()

	if verbose {
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr
	}

	err := cmd.Run()

	if err != nil {
		toolsLogger.Log("[1/1] [Debug Symbols] >> ERROR << Unable to generate debug symbols!")
		os.Exit(1)
	}
}

func GetSymbolsVirtualAddress(inPath string, verbose bool) string {
	toolsLogger.Log("[1/1] [Symbols VAddr] Getting symbols section's virtual address...")

	cmd := exec.Command("ruby", tasks.ResolveRoot("tools/kernel_symbols_tool/main.rb"), "--get_symbols_section_virt_addr", inPath)

	cmd.Env = os.Environ()

	if verbose {
		cmd.Stderr = os.Stderr
	}

	out, err := cmd.Output()

	if err != nil {
		toolsLogger.Log("[1/1] [Symbols VAddr] >> ERROR << Unable to find the virtual address!")
		os.Exit(1)
	}

	return string(out)
}
