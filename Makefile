#
#
#

BUILD ?= build

default: $(BUILD)
	cargo build --manifest-path boards/feather_m0/Cargo.toml --examples --features usb

$(BUILD):
	mkdir -p $(BUILD)
	mkdir -p $(BUILD)/feather_m0/debug
	mkdir -p $(BUILD)/fk_core/debug

examples: $(BUILD)/feather_m0/debug/blinky_basic.bin $(BUILD)/feather_m0/debug/serial.bin

$(BUILD)/feather_m0/debug/%.bin: boards/feather_m0/target/thumbv6m-none-eabi/debug/examples/%
	arm-none-eabi-objcopy -O binary $^ $@

boards/feather_m0/target/thumbv6m-none-eabi/debug/examples/blinky_basic boards/feather_m0/target/thumbv6m-none-eabi/debug/examples/serial: cargo

cargo: $(BUILD)
	cargo build --manifest-path boards/feather_m0/Cargo.toml --examples

clean:
	rm -rf $(BUILD)
	cargo clean --manifest-path boards/feather_m0/Cargo.toml
