#

BUILD ?= build
ALL_EXAMPLES ?=
ALL_BOARDS ?=

default: cargo

include boards/fk_core/board.mk
include boards/feather_m0/board.mk

all-examples: $(ALL_EXAMPLES)

$(BUILD):
	mkdir -p $(BUILD)
	mkdir -p $(BUILD)/feather_m0/debug
	mkdir -p $(BUILD)/fk_core/debug

cargo: $(BUILD)
	cargo build --manifest-path boards/feather_m0/Cargo.toml --examples --features usb
	cargo build --manifest-path boards/fk_core/Cargo.toml --examples --features usb
	+$(MAKE) all-examples

clean:
	rm -rf $(BUILD)
	cargo clean --manifest-path boards/feather_m0/Cargo.toml
	cargo clean --manifest-path boards/fk_core/Cargo.toml
