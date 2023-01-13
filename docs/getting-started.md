# Getting started

evscript is an extensible scripting language made for the Game Boy.
This page will teach you how to use evscript in a brand-new project, from start to finish.

To begin using evscript, create a *driver* in your assembly program.
This will be used to execute evscript code.

Begin by downloading this file: [driver.asm](../asm/driver.asm)

`driver.asm` contains a ready-made driver with some, but not all, operations.
For example, an implementation of `add_u8`, called `StdAdd` in the driver code, is provided, but not an implementation of `mul_u8`.
If you need to use multiplication in your program, make sure to add an implementation for it.

The exact interface of the driver doesn't matter much, but there are two key components: the script pointer, and the memory pool.
In the example driver, the script pointer is stored in `hl`, and the memory pool pointer in `de`.
`hl` will always be pointing at the next bytecode to process, while `de` stays at the beginning of the memory pool.
When the driver returns, the script pointer `hl` will either be pointing to the next opcode (if a `yield` was executed) or will contain `0` (if a `return` is executed).
If your script is re-entrant, you are responsible for saving and restoring the script pointer (`hl`)

Let's add an additional *bytecode* to this driver.
Copy the following code to the bottom of `driver.asm`:

```rgbds
SECTION "Print Function", ROM0
PrintFunction:
	push hl
	ld a, [hli]
	ld h, [hl]
	ld l, a
	ld de, $9800 + 32
.next
	ld a, [hli]
	and a, a
	jr z, .exit
	ld [de], a
	inc de
	jr .next

.exit
	pop hl
	inc hl
	inc hl
	ret
```

Now, find the `EvscriptBytecodeTable` near the top of the file, and write this at the very end:

```
	dw PrintFunction
```

Great! We've now added our own *bytecode*, which we'll use to print a string to the screen.
While this particular bytecode isn't very useful, it shows us how to implement our own, and will let us get started with evscript.

There's just a bit more boilerplate essential to creating a Game Boy rom, so download [main.asm](../asm/main.asm) as well.

Finally, let's get started using evscript!

We're going to begin by creating an *environment*.
Create a new file called `script.evs`.
This tells the evscript compiler how your driver is designed; what bytecode is available to it, and in what order.

Copy this environment into `script.evs`:

```evscript
env script {
	def ret();
	def yld();
	def jmp();
	def jmp_if_true();
	def jmp_if_false();
	def put_u8();
	alias put_i8() = put_u8();
	def mov_u8();
	alias mov_i8() = mov_u8();
	def add_u8();
	alias add_i8() = add_u8();
	def sub_u8();
	alias sub_i8() = sub_u8();
	def band_u8();
	alias band_i8() = band_u8();
	def equ_u8();
	alias equ_i8() = equ_u8();
	def nequ_u8();
	alias nequ_i8() = nequ_u8();
	def lt_u8();
	alias lt_i8() = lt_u8();
	def gt_u8();
	alias gt_i8() = gt_u8();
	def lte_u8();
	alias lte_i8() = lte_u8();
	def gte_u8();
	alias gte_i8() = gte_u8();
	def land_u8();
	alias land_i8() = land_u8();
	def lor_u8();
	alias lor_i8() = lor_u8();
}
```

(Note that you can create more environments if your game has multiple drivers or contexts where different bytecode should be available.)

You'll notice two types of statements being used here: `def` and `alias`.

`def` is simple: it creates a new bytecode with a given name.
The names shown here are all recognized by evscript to represent certain operations, like `add_u8` for `+`.

The other statement, `alias`, is used to give multiple names to the same bytecode.
For example, a signed and unsigned add both use the same logic, so we communicate this to evscript by writing `alias add_i8 = add_u8;`.

If you look closely, you might notice that our print function is missing!
for `print`, we need to make use of *arguments*.
While evscript's internal bytecode doesn't need any arguments (the compiler will figure them out for you), anything we add does.

Start by writing the following at the end of the environment:
```evscript
	def print();
```

Now we have a bytecode, but we need to define its argument: a 16-bit pointer.
Since the string won't ever change, we should use `const`.
Update it to look like this:
```evscript
	def print(const u16);
```

Note that the parameter has no name, only a type.

We're almost there now!
To begin writing a script, we start by specifying an environment, and then a name.
Our environment is called `script`, and we'll call the script `ExampleScript` since this is what `main.asm` expects.
(The name of the script becomes an exported label in assembly, like `ExampleScript::`)
```evscript
script ExampleScript {

}
```

Now just write `print("Hello, world!");`, and your program is done!

To build it:
```sh
evscript -o script.asm script.evs
rgbasm -o main.o main.asm 
rgbasm -o driver.o driver.asm
rgbasm -o script.o script.asm
rgblink -o main.gb main.o driver.o script.o
rgbfix -v main.gb
```
