# Writing scripts

Let's begin by writing some scripts.

Since evscript supports several script kinds simultaneously, the script's kind must be specified; let's say it's `npc` for now.
(We will discuss how to define script kinds, or *environments*, after explaining what scripts can do.)
Obviously, the script must have a name as well.

```evscript
npc TheScriptName {
	// ...
}
```

The script's contents will be written between the braces.
(Let's not worry about the functions we call for now; assume they exist.)
For example, we could make a NPC that simply walks right once.

```evscript
npc SingleStep {
	move_right();
	wait();
	die();
}
```

Or a NPC that walks right twice:

```evscript
npc TwoSteps {
	move_right();
	wait();
	move_right();
	wait();
	die();
}
```

## Repetition

The above is neat, but not very scalable.
Fortunately, evscript supports several kinds of loops!

The first one, `repeat`, simply repeats its contents a fixed amount of time:

```evscript
npc BabySteps {
	repeat 42 {
		move_right();
		wait();
	}
	die();
}
```

The second one, `loop`, repeats its contents *forever*.

```evscript
npc ToInfinityAndBeyond {
	loop {
		move_right();
		wait();
	}
	die(); // This will never be executed!
}
```

`do ... while` is a little more complex: its contents are repeated, until a certain expression becomes different from 0:

```evscript
// This is a more complicated version of `BabySteps` above,
// for demonstration purposes only; please use `repeat 42` instead.
npc BabyStepsAgain {
	u8 nb_steps = 42;
	do {
		move_right();
		wait();
		nb_steps -= 1;
	} while nb_steps;
}
```

Whereas `do ... while` always executes its contents at least once, `while` can have them never executed at all:

```evscript
npc LoopDemo {
	do {
		move_right();
		wait();
	} while 0;
	while 0 {
		move_left();
		wait();
	}
	die();
}
```

This NPC will move right once, but won't move left.

## Variables

Those were already shown in the above `do ... while` example, so let's talk about them.
Variables are things that can be used to store things—primarily numbers.
There are two types of variables: `u8` ones can store integers from 0 to 255, `u16` from 0 to 65535 (both inclusive).

As you saw in that example, variables can be modified.
`nb_steps -= 1;` is a shorthand for `nb_steps = nb_steps - 1;`—which is to say, "read the number contained in `nb_steps`, subtract 1, and write that back into `nb_steps`".

(TODO: come up with a good example for variables?)

And, as you have seen, this brings us to...

## Expressions

Again as you've seen, evscript supports a little bit of calculus.
Specifically, the following mathematical operators are supported: `+`, `-`, `*`, `/`, `%` (modulo), with the usual precedence; `-` can be used for both subtraction and negation.
Comparison is also available: `==` (is equal to), `!=` (is different from), `<`, `>`, `<=`, and `>=`.

The elementary bitwise operators `&`, `|`, and `^` are supported, as well as the bitshift operators `<<` and `>>` (arithmetic/signed left shift); as is (one's) complement, notated `!`.
And finally, the two logic combinators `&&` (and) and `||` are supported, though neither is short-circuiting (if you don't know what that means: both sides are always computed).

Two extra operators are available: `&` can be used like in C to obtain the address of a variable, and `[...]` can be used to read memory at a certain address.

## A more complex example

Let's showcase everything we've seen so far!

```evscript
// “I will look for you, I will find you, and I will kill you.”
npc LiamNeeson {
	u8 npcPos;
	loop {
		npc_x(npcPos); // Read the NPC's X coordinate into the `npcPos` variable.
		if [wPlayerX] < npcPos {
			move_left();
		} else if [wPlayerX] > npcPos {
			move_right();
		} else {
			npc_y(npcPos);
			if playerCoord < npcPos {
				move_up();
			} else if playerCoord > npcPos {
				move_down();
			} else {
				harm_player(5); // 5 HP damage.
			}
		}

		yield;
	}
}
```

...and you may be wondering what this `yield;` line is.
The answer lies in the next chapter!
