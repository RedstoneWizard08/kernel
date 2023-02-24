TargetCPU = cortex-a72
KernelBin = kernel8.img
QEMU_CMD = qemu-system-aarch64 -M virt,highmem=off -smp 8 -m 2G -cpu $(TargetCPU) -serial stdio -display none -kernel $(KernelBin)
BUILDER_ARCH = $(shell uname -m)
BuilderCmd = target/$(BUILDER_ARCH)-unknown-linux-gnu/release/builder

.PHONY: all

all: build

clean: __build
	@$(BuilderCmd) clean -v

build: __build
	@$(BuilderCmd) build -v

test: __build
	@$(BuilderCmd) test -v

run:
	@echo "Run: $(QEMU_CMD)"
	@$(QEMU_CMD)

__build:
	@make -C builder
