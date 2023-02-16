package run

func Setup() [6]string {
	var targetDirs [6]string

	return targetDirs
}

func Run() error {
	// QEMU: qemu-system-aarch64 -M virt,highmem=off -smp 8 -m 2G -cpu $(TargetCPU) -serial stdio -display none -kernel $(KernelBin)

	return nil
}
