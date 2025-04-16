install-tui:
	cargo build --release
	cp target/release/wallpapercl_tui  ~/.local/bin/wallpapercl
