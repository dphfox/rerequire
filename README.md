# rerequire

Push-button converter for Luau module conventions - in seconds, convert back and
forth between:

- `inner-fs`
	- Inner nesting (`/myModule/init.luau`)
	- Filesystem-based relative paths
- `outer-mod`
	- Outer nesting (`/myModule.luau` + `/myModule/` directory)
	- Module-based relative paths

## Install

TODO

## Usage

Open a terminal in the root of your project, and run `rerequire` with the name
of the module convention you want.

```
$ rerequire inner-fs
$ rerequire outer-mod
```

## Disclaimer

This is [a certified Daniel P H Fox Side Project™](https://fluff.blog/2024/04/10/i-dont-want-to-be-a-maintainer.html),
which I am sharing because I personally wanted it to exist in the world. I might maintain it. I might not.
Contributions are welcome, but I do not make guarantees about those either.

Feel free to use rerequire, but if you're about to depend on it big time, the security audit's on you. If, for whatever
reason, you end up in a spot of bother, you should probably not have used a random project from someone's GitHub without
inspecting what it does properly. I take no responsibility for that.