GO_ENV_ARGS = GOOS=linux CGO_ENABLED=0
GO_ARGS = -ldflags="-extldflags=-static"

define _log
	@tput setaf 4 2> /dev/null || true
	@printf 'Build'
	@tput sgr0 2> /dev/null || true
	@printf ' : %s\n' $(1)
endef

define _logc
	@tput setaf 4 2> /dev/null || true
	@printf 'Clean'
	@tput sgr0 2> /dev/null || true
	@printf ' : %s\n' $(1)
endef

.PHONY: all

all: clean x86_32 x86_64 arm aarch64

x86_32:
	$(call _log, "[1/4] Building x86_32...")
	@$(GO_ENV_ARGS) GOARCH=386 go build $(GO_ARGS) -o build/builder.x86_32 ./main.go

x86_64:
	$(call _log, "[2/4] Building x86_64...")
	@$(GO_ENV_ARGS) GOARCH=amd64 go build $(GO_ARGS) -o build/builder.x86_64 ./main.go

arm:
	$(call _log, "[3/4] Building arm...")
	@$(GO_ENV_ARGS) GOARCH=arm go build $(GO_ARGS) -o build/builder.arm ./main.go

aarch64:
	$(call _log, "[4/4] Building aarch64...")
	@$(GO_ENV_ARGS) GOARCH=arm64 go build $(GO_ARGS) -o build/builder.aarch64 ./main.go

clean:
	$(call _logc, "Cleaning build directory...")
	@if [ -d "build" ]; then rm -rf build; fi
