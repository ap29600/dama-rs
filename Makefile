BIN_DIR = /usr/local/bin


default: build

install: build strip copy-bins copy-desktop-entry

install-nostrip: build copy-bins copy-desktop-entry

strip:
	strip ./target/release/dama

uninstall:
	sudo rm $(BIN_DIR)/dama
	sudo rm /usr/share/applications/dama.desktop

copy-bins:
	sudo cp target/release/dama $(BIN_DIR)

copy-desktop-entry:
	sudo cp dama.desktop /usr/share/applications/dama.desktop

build:
	cargo build --release

clean:
	cargo clean
