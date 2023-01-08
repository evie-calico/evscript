# Coroutines and you

One of scripting's primary uses is how it makes coroutines much simpler to implement.
Let's learn how to take advantage of that.

Say you have a simple animation in your game.
You want a sprite to move back and forth across the screen, with a short stop on either side.
You could accomplish this with a littany of flags, comparisons, and frame counters; or you could use `yield`.

`yield` is simple: it returns from a script, but leaves the script pointer on the next instruction.
This is different from `return`, which sets the script pointer to `0` to prevent repeat execution.

Your sprite's animation might somewhat look like this:
```evsscript
script SpriteAnimation {
	u8 x = 0;
	u8 y = 0;

	// Play the animation forever.
	loop {
		// Move right 128 pixels at a speed of 1px/frame.
		repeat 128 {
			x += 1;
			draw_sprite(x, y);
			yield; // Wait for the next frame
		}

		// Now just wait 128 frames.
		repeat 128 {
			yield;
		}

		// Move left by 128 pixels at a speed of 1px/frame.
		repeat 128 {
			x -= 1;
			draw_sprite(x, y);
			yield; // Wait for the next frame
		}

		// Now just wait 128 frames.
		repeat 128 {
			yield;
		}
	}
}
```

This code will loop around forever, so long as you keep calling `ExecuteScript` on it.
Make sure that you save and restore the script pointer (`hl`) before and after each `ExecuteScript` call.
evscript will *not* do this for you.

Note that it's entirely safe to have as many scripts as you want running concurrently.
A script's state is stored only in the variable pool (pointed to by `de`) and the script pointer (`hl`).
