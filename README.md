# zeus-compiler ![Travis (.org)](https://img.shields.io/travis/nukeop/zeus-compiler.svg)
Compiler for the zeus entertainment system assembly language.
Main project here: https://github.com/nukeop/zeus

## Getting started

### Building

```shell
$ cargo build
```

Release:

```shell
$ cargo build --release
```

### Running

```shell
$ env RUST_LOG=main=info,zeus_compiler=info cargo run -- --source program.zeus --target compiled.zeus
```

### Testing

```shell
$ cargo test
```
