TARGET := test

build: compile link fix

compile:
	rgbasm $(TARGET).asm -i "hardware.inc" -o build/$(TARGET).o -Wall

link:
	rgblink -o build/$(TARGET).gb --map output.map build/$(TARGET).o

fix:
	rgbfix -v build/$(TARGET).gb -p 0xff

# Running: rgbasm main.asm -o output.o -Wall
# Running: rgblink -o output.gb --map output.map main.asm.o
# Running: rgbfix -v output.gb -p 0xff
