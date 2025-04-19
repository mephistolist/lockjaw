.DEFAULT_GOAL := all
BIN_NAME := lockjaw
CONFIG_SCRIPT := configure
RELEASE_PATH := target/release/$(BIN_NAME)
all: $(CONFIG_SCRIPT) build strip
$(CONFIG_SCRIPT):
	chmod +x $(CONFIG_SCRIPT)
	./$(CONFIG_SCRIPT)
build: $(CONFIG_SCRIPT)
	RUSTFLAGS="-C opt-level=3 -C panic=abort -C target-cpu=native" cargo build --release
strip:
	@if [ -f "$(RELEASE_PATH)" ]; then \
		echo "Stripping binary..."; \
		strip $(RELEASE_PATH); \
	else \
		echo "Binary not found to strip."; \
	fi
install: build strip
	@echo "Installing to /usr/bin/$(BIN_NAME)..."
	sudo cp $(RELEASE_PATH) /usr/bin/$(BIN_NAME)
	cargo clean
uninstall:
	@echo "Removing /usr/bin/$(BIN_NAME)..."
	sudo rm -f /usr/bin/$(BIN_NAME)
clean:
	cargo clean
run:
	cargo run --release
.PHONY: all $(CONFIG_SCRIPT) build strip install uninstall clean run
