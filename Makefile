#
#
#

BUILD ?= build

default: examples

$(BUILD):
	mkdir -p $(BUILD)
	mkdir -p $(BUILD)/feather_m0/debug
	mkdir -p $(BUILD)/fk_core/debug

examples: $(BUILD)/feather_m0/debug/blinky_basic.bin \
	$(BUILD)/feather_m0/debug/serial.bin \
  $(BUILD)/feather_m0/debug/usb_serial.bin \
	$(BUILD)/feather_m0/debug/rtfm.bin \
	$(BUILD)/feather_m0/debug/rtfm_blink.bin

$(BUILD)/feather_m0/debug/%.bin: boards/feather_m0/target/thumbv6m-none-eabi/debug/examples/%
	arm-none-eabi-objcopy -O binary $^ $@

boards/feather_m0/target/thumbv6m-none-eabi/debug/examples/blinky_basic \
	boards/feather_m0/target/thumbv6m-none-eabi/debug/examples/serial \
	boards/feather_m0/target/thumbv6m-none-eabi/debug/examples/usb_serial \
	boards/feather_m0/target/thumbv6m-none-eabi/debug/examples/rtfm_blink \
	boards/feather_m0/target/thumbv6m-none-eabi/debug/examples/rtfm: cargo

cargo: $(BUILD)
	cargo build --manifest-path boards/feather_m0/Cargo.toml --examples --features usb

clean:
	rm -rf $(BUILD)
	cargo clean --manifest-path boards/feather_m0/Cargo.toml
