BUILDER_ARCH = $(shell uname -m)
BUILDER_TARGET = $(BUILDER_ARCH)-unknown-linux-gnu

all:
	@cargo build --target $(BUILDER_TARGET) --release --bin builder
