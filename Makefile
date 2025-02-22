TARGET := test

build: compile link fix

compile:
	rgbasm asm/$(TARGET).asm -i "hardware.inc" -o build/$(TARGET).o -Wall

link:
	rgblink -o roms/$(TARGET).gb --map build/output.map build/$(TARGET).o

fix:
	rgbfix -v roms/$(TARGET).gb -p 0xff
