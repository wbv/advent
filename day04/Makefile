TARGET := target/debug/$(notdir $(CURDIR))

all: $(TARGET)

$(TARGET): $(wildcard src/*)
	cargo build

test: $(TARGET) ex.txt input.txt
	cat ex.txt | ./$(TARGET)
	cat input.txt | ./$(TARGET)
