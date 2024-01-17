# Pros and cons

Should you use evscript?
These (non-exhaustive) lists of pros and cons may help you decide whether it's worth the effort for you.

## Pros

- Easier and faster to write than assembly.
- Lets people design some game logic even if they can't write assembly. (Think level designers?)
- Coroutine support.
- Typically smaller than equivalent assembly.
- Often easier to debug than equivalent assembly.
- "You only pay for what you use"—no need to define any bytecode functions that you don't use.

## Cons

- Far slower than directly writing the logic in assembly.
- One extra dependency in your build process, albeit no more than a single executable.
- Requires writing a bytecode driver, which can be a little tedious.
