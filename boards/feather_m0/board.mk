#

board = feather_m0
examples = blink_simple serial usb_serial rtfm rtfm_blink
bins = $(examples:%=$(BUILD)/$(board)/debug/%.bin)
elfs = $(examples:%=$(BUILD)/$(board)/debug/%.elf)

$(BUILD)/$(board)/debug/%.bin: boards/$(board)/target/thumbv6m-none-eabi/debug/examples/%
	arm-none-eabi-objcopy -O binary $< $@

$(BUILD)/$(board)/debug/%.elf: boards/$(board)/target/thumbv6m-none-eabi/debug/examples/%
	cp $< $@

$(eval ALL_EXAMPLES += $(bins) $(elfs))
$(eval ALL_BOARDS += $(board))
