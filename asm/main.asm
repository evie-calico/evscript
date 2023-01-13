DEF rLCDC EQU $FF40
DEF rLY EQU $FF44
DEF rBGP EQU $FF47
DEF rIE EQU $FFFF

SECTION "VBlank", ROM0[$40]
	reti

SECTION "Header", ROM0[$100]
Entry:
	di
	jp Main
	ds $150 - @, 0

SECTION "Main", ROM0
Main:
	ldh a, [rLY]
	cp a, 144
	jr c, Main
	xor a, a
	ldh [rLCDC], a
	inc a
	ldh [rIE], a

	ld bc, Font.end - Font
	ld de, Font
	ld hl, $8000 + 16 * 32
.copy
	ld a, [de]
	inc de
	ld [hli], a
	dec c
	jr nz, .copy
	dec b
	jr nz, .copy

	ld de, wScriptPool
	ld hl, ExampleScript
	call ExecuteScript
	ld a, %10010001
	ldh [rLCDC], a
	ei
.forever
	halt
	ld de, wScriptPool
	call ExecuteScript
	jr .forever

SECTION "Font", ROM0
Font:
	INCBIN "font.2bpp"
.end

SECTION "Script Pool", WRAM0
wScriptPool: ds 16
