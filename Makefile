HOME_DIR=$(shell echo $$HOME)
CONFIG_PATH=$(HOME_DIR)/.config/wallpapercl

build:
	@go build -o bin/wallpapercl cmd/wallpapercl/main.go
	@echo "Build complete!"

install: build
	@cp bin/wallpapercl $(HOME_DIR)/.local/bin/wallpapercl


run: build
	@./bin/wallpapercl

.PHONY: build install run
