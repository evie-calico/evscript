section "evscript Driver", rom0
; @param de: Variable pool
; @param hl: Script pointer
; @param bank: Script bank
; @return hl: New script pointer. 0 after a return.
; @return bank: New script bank.
; @preserves de
ExecuteScript::
	ld a, h
	or a, l
	ret z
.next
	ld a, [hli]
	push hl
	add a, low(EvscriptBytecodeTable >> 1)
	ld l, a
	adc a, high(EvscriptBytecodeTable >> 1)
	sub a, l
	ld h, a
	add hl, hl
	ld a, [hli]
	ld h, [hl]
	ld l, a
	ld b, h
	ld c, l
	pop hl
	push de
	call .callBC
	pop de
	jr ExecuteScript.next

.callBC
	push bc
	ret

section "evscript Bytecode table", rom0, ALIGN[1]
EvscriptBytecodeTable:
	; Control
	dw StdReturn
	dw StdYield
	; goto
	dw StdJump
	dw StdJumpIfTrue
	dw StdJumpIfFalse
	; Moves
	dw StdPut
	dw StdMove
	; 8-bit ops
	dw StdAdd
	dw StdSub
	dw StdBinaryAnd
	dw StdEqu
	dw StdNotEqu
	dw StdLessThan
	dw StdGreaterThan
	dw StdLessThanEqu
	dw StdGreaterThanEqu
	dw StdLogicalAnd
	dw StdLogicalOr

section "evscript Return", rom0
StdReturn:
	ld hl, 0
StdYield:
	pop de ; pop return address
	pop de ; pop pool pointer
	ret

section "evscript Goto", rom0
StdJump:
	ld a, [hli]
	ld h, [hl]
	ld l, a
	ret

StdJumpIfTrue:
	ld a, [hli]
	add a, e
	ld c, a
	adc a, d
	sub a, c
	ld b, a
	ld a, [bc]
	and a, a
	jr nz, StdJump
.fail
	inc hl
	inc hl
	ret

StdJumpIfFalse:
	ld a, [hli]
	add a, e
	ld c, a
	adc a, d
	sub a, c
	ld b, a
	ld a, [bc]
	and a, a
	jr z, StdJump
.fail
	inc hl
	inc hl
	ret

section "evscript Put", rom0
StdPut:
	ld a, [hli]
	add a, e
	ld c, a
	adc a, d
	sub a, c
	ld b, a
	ld a, [hli]
	ld [bc], a
	ret

section "evscript Mov", rom0
StdMove:
	; Load dest
	ld a, [hli]
	add a, e
	ld c, a
	adc a, d
	sub a, c
	ld b, a
	; Load source
	ld a, [hli]
	add a, e
	ld e, a
	adc a, d
	sub a, e
	ld d, a
	; Move
	ld a, [de]
	ld [bc], a
	ret

section "evscript 8-bit Operations", rom0
; @param de: pool
; @param hl: script pointer
; @return a: lhs
; @return b: rhs
OperandPrologue:
	ld a, [hli] ; lhs offset
	add a, e
	ld c, a
	adc a, d
	sub a, c
	ld b, a
	; de is preserved & variable is pointed to by bc
	ld a, [hli]
	push hl
		ld l, a
		ld h, 0
		add hl, de
		ld a, [bc]
		ld b, [hl]
	pop hl
	ret

StdAdd:
	call OperandPrologue
	add a, b ; Here is the actual operation
	jr StoreEpilogue

StdSub:
	call OperandPrologue
	sub a, b ; Here is the actual operation
	jr StoreEpilogue

StdBinaryAnd:
	call OperandPrologue
	and a, b
	jr StoreEpilogue

StdEqu:
	call OperandPrologue
	cp a, b
	ld a, 0
	jr nz, StoreEpilogue
	inc a
	jr StoreEpilogue

StdNotEqu:
	call OperandPrologue
	cp a, b
	ld a, 0
	jr z, StoreEpilogue
	inc a
	jr StoreEpilogue

StdLessThan:
	call OperandPrologue
	cp a, b
	ld a, 0
	jr nc, StoreEpilogue
	inc a
	jr StoreEpilogue

StdGreaterThan:
	call OperandPrologue
	cp a, b
	jr z, .zero
	jr nc, .zero
	ld a, 1
	jr StoreEpilogue
.zero
	xor a, a
	jr StoreEpilogue

StdLessThanEqu:
	call OperandPrologue
	cp a, b
	jr z, .one
	jr c, .one
	xor a, a
	jr StoreEpilogue
.one
	ld a, 1
	jr StoreEpilogue

StdGreaterThanEqu:
	call OperandPrologue
	cp a, b
	ld a, 0
	jr c, StoreEpilogue
	inc a
	jr StoreEpilogue

StdLogicalAnd:
	call OperandPrologue
	and a, a
	jr z, StoreEpilogue
	ld a, b
	and a, a
	jr z, StoreEpilogue
	ld a, 1
	jr StoreEpilogue

StdLogicalOr:
	call OperandPrologue
	and a, a
	jr nz, .true
	ld a, b
	and a, a
	jr z, StoreEpilogue
.true
	ld a, 1
	; fallthrough
; This is stored in the middle so both variable and constant operations can
; reach it.
StoreEpilogue:
	ld b, a
	ld a, [hli]
	add a, e
	ld e, a
	adc a, d
	sub a, e
	ld d, a
	ld a, b
	ld [de], a
	ret
