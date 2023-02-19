BUILDER_ARCH = x86_64

TargetCPU = cortex-a72
KernelBin = kernel8.img
QEMU_CMD = qemu-system-aarch64 -M virt,highmem=off -smp 8 -m 2G -cpu $(TargetCPU) -serial stdio -display none -kernel $(KernelBin)
BuilderCmd = builder/build/builder.$(BUILDER_ARCH)

.PHONY: all

all: build

clean: _builder_build
	@$(BuilderCmd) clean -v

build: _builder_build
	@$(BuilderCmd) build -v

test: _builder_build
	@$(BuilderCmd) test -v

run:
	@echo "Run: $(QEMU_CMD)"
	@$(QEMU_CMD)

_builder_build:
	@make -C builder
