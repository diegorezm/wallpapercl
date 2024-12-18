HOME_DIR=$(shell echo $$HOME)
CONFIG_PATH=$(HOME_DIR)/.config/wallpapercl

bundle: 
	@bun run build

build: bundle 
	@go build -o bin/wallpapercl cmd/wallpapercl/main.go
	@echo "Build complete!"

install: build
	@cp bin/wallpapercl $(HOME_DIR)/.local/bin/wallpapercl

run: build
	@./bin/wallpapercl

server: build
	@./bin/wallpapercl --serve


.PHONY: build install run
