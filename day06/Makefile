TARGET := target/debug/$(notdir $(CURDIR))
REL_TARGET := target/release/$(notdir $(CURDIR))

all: $(TARGET)

$(TARGET): $(wildcard src/*)
	cargo build

$(REL_TARGET): $(wildcard src/*)
	cargo build --release

test: $(TARGET) ex.txt input.txt
	cat ex.txt | ./$(TARGET)
	cat input.txt | ./$(TARGET)

benchmark: $(REL_TARGET) input.txt
	cat input.txt | $(REL_TARGET)

