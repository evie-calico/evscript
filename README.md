# evscript
## An extensible bytecode-based scripting engine

Some examples can be found in the `scripts/` folder.

To install the latest release, just run `cargo install evscript`.

## Todo

- Script definitions could be given "arguments", syntactic sugar for defining variables at the very beginning of the pool.
- Similarly, if `return` is given a value, this could be copied to the very beginning of the script pool, as convienience
  - Despite evscript's lack of functions, these features would still be very useful for interating with assembly or other scripts if a custom runtime provides function call support.
- `repeat` could name its index variable if given a second argument (such as `repeat i, n`), similar to the common `for (int i = 0; i < n; i++)` pattern that `repeat` aims to replace.

## Credits

- [poryscript](https://github.com/huderlem/poryscript) for inspiring this project.
- And everyone at gbdev who helped me along the way :)
