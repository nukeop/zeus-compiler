# zeus-compiler
Compiler for the zeus entertainment system assembly language.
Main project here: https://github.com/nukeop/zeus

## Getting started

`### Building

```shell
$ cargo build
```

### Running

```shell
$ env RUST_LOG=main=info,zeus_compiler=info cargo run -- --source program.zeus --target compiled.zeus
```

### Testing

```shell
$ cargo test
```
