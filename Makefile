.PHONY: build.linux build image clean build.release tag push run

TAG ?= $(shell git describe --tags --always --dirty)
BINARY ?= warp-api-template
OWNER ?= zhubby

tag:
	@echo $(TAG)

run:
	cargo run

clean:
	cargo clean

build:
	cargo build

build.release:
	cargo build --release

build.linux:
	cargo build --target x86_64-unknown-linux-musl --release

image: build.linux
	docker build -t ghcr.io/$(OWNER)/$(BINARY):$(TAG) -f Dockerfile .

push:
	docker ghcr.io/$(OWNER)/$(BINARY):$(TAG)