package run

import (
	"os"
	"os/exec"

	"github.com/RedstoneWizard08/kernel/builder/tasks"
	"github.com/fatih/color"
)

var runLogger tasks.Logger = tasks.NewLogger("Run", *color.New(color.FgBlue), *color.New(color.Reset))

func Run() {
	cfg := tasks.DefaultConfig()

	cmd := exec.Command("qemu-system-aarch64", "-M", "virt,highmem=off", "-smp", "8", "-m", "2G", "-cpu", string(cfg.TargetCPU), "-serial", "stdio", "-display", "none", cfg.KernelBin)

	runLogger.Log("Launching QEMU...")

	err := cmd.Run()

	if err != nil {
		runLogger.Log("Unable to launch QEMU for running!")
		os.Exit(1)
	}
}
