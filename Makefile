TargetCPU = cortex-a72
KernelBin = kernel8.img
QEMU_CMD = qemu-system-aarch64 -M virt,highmem=off -smp 8 -m 2G -cpu $(TargetCPU) -serial stdio -display none -kernel $(KernelBin)

.PHONY: all

all: build

clean:
	go run . clean -v

build:
	go run . build -v

run:
	@echo "Run: $(QEMU_CMD)"
	@$(QEMU_CMD)
