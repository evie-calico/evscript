# Writing a driver (and teaching it to evscript)

An `env` block describes how the bytecode interpreter (or "driver") is set up.
Since a game may contain multiple kinds of bytecode, `evscript` supports multiple `env` blocks; each of them must be given a name.

```evscript
env script {
	// TODO
}
```
