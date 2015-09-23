# Rust wrapper for TRANS2QUIK API

## Supported targets

**IMPORTANT!** This library support only 2 targets, because TRANS2QUIK API implemented as 32-bit version for Windows only:
* i686-pc-windows-msvc
* i686-pc-windows-gnu

## Usage

To use `trans2quik`, first add this to your `Cargo.toml`:

```toml
[dependencies]
trans2quik = "*"
```

Then, add this to your crate root:

```rust
extern crate trans2quik;
```

## Build
To build with this library you have to put `Trans2Quik.dll` and `Trans2Quik.lib` in right places (library doesn't have an installer). But you also can set up environment variables.

For **i686-pc-windows-msvc** target set (VS2012 x86 Native Tools Command Prompt):
```cmd
set TRANS2QUIK_PATH=<path_to_your_Trans2QUIKAPI>
set LIB=%TRANS2QUIK_PATH%;%LIB%
```

For **i686-pc-windows-gnu** target set (msys2_shell.bat):
```sh
LIBRARY_PATH=<path_to_your_Trans2QUIKAPI> cargo build
```
