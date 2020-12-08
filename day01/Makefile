TARGET := target/debug/$(notdir $(CURDIR))

all: $(TARGET)

$(TARGET): $(wildcard src/*)
	cargo build

test: $(TARGET)
	cat input.txt | ./$(TARGET)
