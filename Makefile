# Directories
ENV_FILE := .env
TARGET_DIR := target/release
#i don't know which path you working on 
#every branch is different, so just change it here
INTERNALS_DIR := internals
BIN_DIR := commands/src/bin

.PHONY: all clean build copy env

all: clean build copy env

clean:
	@echo "Cleaning $(BIN_DIR)..."
	@rm -rf $(BIN_DIR)
	@mkdir -p $(BIN_DIR)

build:
	@echo "Building workspace..."
	@cargo build --release --workspace

copy:
	@echo "Copying binaries from internals/* to $(BIN_DIR)..."
	@for crate in $(wildcard $(INTERNALS_DIR)/*); do \
		if [ -d $$crate ]; then \
			name=$$(basename $$crate); \
			bin_path=$(TARGET_DIR)/$$name; \
			if [ -f $$bin_path ] && [ -x $$bin_path ]; then \
				cp $$bin_path $(BIN_DIR)/; \
			fi \
		fi \
	done

env:
	@echo "Updating $(ENV_FILE)..."
	@rm -f $(ENV_FILE)
	@touch $(ENV_FILE)
	@for f in $(BIN_DIR)/*; do \
		name=$$(basename $$f); \
		echo "$$name=$$f" >> $(ENV_FILE); \
	done
	@echo "✅ .env updated:"
	@cat $(ENV_FILE)
