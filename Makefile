.PHONY: build.linux build build.docker clean build.release tag

TAG ?= $(shell git describe --tags --always --dirty)

tag:
	@echo $(TAG)

clean:
	cargo clean

build:
	cargo build

build.release:
	cargo build --release

build.linux:
	cargo build --target x86_64-unknown-linux-musl --release

build.docker: build.linux
	docker build -t ghcr.io/zhubby/warp-api-template:$(TAG) -f Dockerfile .