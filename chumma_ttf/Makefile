BASE_FONT=../OpenSans-Regular.ttf
TARGET_FONT=chumma.ttf
WASM=chumma_ttf_bg.wasm
CONF_FILE = ./src/words.conf
$(TARGET_FONT): pkg/$(WASM) $(BASE_FONT)
	../bin/otfsurgeon -i $(BASE_FONT) add -o $(TARGET_FONT) Wasm < pkg/$(WASM)

pkg/$(WASM): src/lib.rs $(CONF_FILE)
	wasm-pack build --target web

