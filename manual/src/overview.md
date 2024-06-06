# How evscript works

Think of `evscript` as a fancier (and far more comfortable!) way of writing a giant pile of `db`s (data).
`evscript` will not generate code as in assembly instructions; it is moreso a configurable tool for writing data that *your* game engine will interpret.

Thus, `evscript` input files contain descriptions of how your game engine is set up (mostly `env` blocks), and then... scripts!
As many scripts, of as many kinds as you want!

## Syntax basics

First off, like all languages that strive to be practical, `evscript` supports comments: everything between `//` and the end of the same line, or between `/*` and the next `*/` is completely ignored.
These are useful for the script programmer to take notes within the script file.

Also for convenience, file inclusion is supported: the `include "path/to/other.evs";` directive acts as if the contents of the file at `"path/to/other.evs"` were copy-pasted in place of the `include` directive.
Relative paths are evaluated relative to `evscript`'s own working directory.

Another thing to keep in mind: in evscript, identifiers (think "names") can contain letters, digits, underscores `_` and dots `.`, but they must begin with a letter or underscore.
