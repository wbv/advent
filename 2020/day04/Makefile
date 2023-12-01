TARGET := target/debug/$(notdir $(CURDIR))
REL_TARGET := target/release/$(notdir $(CURDIR))

all: $(TARGET)

$(TARGET): $(wildcard src/*)
	cargo build

$(REL_TARGET): $(wildcard src/*)
	cargo build

test: $(TARGET) invalid.txt valid.txt ex.txt input.txt
	cat invalid.txt | ./$(TARGET)
	cat valid.txt | ./$(TARGET)
	cat ex.txt | ./$(TARGET)
	cat input.txt | ./$(TARGET)

benchmark: $(REL_TARGET) input.txt
	cat input.txt | $(REL_TARGET)

