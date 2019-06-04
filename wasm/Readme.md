## Tantivy-wasm 

Compiles tantivy to wasm. 

You can index your files on server, serve wasm library + index to the client to give them a fast browser-side search.

### Requirements

- `wasm-pack` to build and distribute
- `wasm-bindgen` used by wasm-pack to generate JS bindings to the wasm library.
- rust nightly - the `rust-toolchain` file specifies nightly, so cargo and rustc commands will call into nightly toolchain. 

### Building and running

```bash
    $$$ wasm-pack build --no-typescript -t no-modules 
    $$$ cd pkg/
    $$$ firefox pkg/index.html
```

### Features

Most tantivy features apart from "mmap", "no\_fail" and "regex\_query". If/when those crates add wasm support in the future, we will try again.

### Usage

Make an index of documents on the server using tantivy-cli. 
Serialise the index directory to a file with name `index.bin`. 
Place the `index.bin` at the root of this `wasm` directory.
Compile and pack the wasm library. 

Serve the following to a wasm-compatible browser (most modern browsers):
    * tantivy as a wasm library
    * JavaScript for interop


