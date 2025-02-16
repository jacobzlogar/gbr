; From https://gbdev.io/gb-asm-tutorial/part1/hello_world.html

INCLUDE "hardware.inc"

SECTION "Header", ROM0[$100]

  jp EntryPoint

  ds $150 - @, 0 ; Make room for the header

EntryPoint:
  ; Shut down audio circuitry
  ld b, 12
  
Foo:
  ld b, 12
  
Bar:
  jr Foo
  
Baz:
  ld b, 12
