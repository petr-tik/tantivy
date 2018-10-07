## Tantivy-wasm 

This is the wasm library implies that the same version of tantivy. You can index your files on server, serve wasm library + index to the client to give them a fast search.


# Requirements

- `wasm-pack` to build and distribute
- rust nightly

# Features

All likes in the server-side version of tantivy apart from "mmap", "no\_fail" and "regex\_query". If/when those crates add wasm support in the future, we will try again.

# Usage

Make an index of documents on the server. Compile and pack the wasm library. Serve the following to a wasm-compatible browser:
    * tantivy as a wasm library
    * JavaScript for interop
    * index as a binary


# Building and running

    wasm-pack build
    cd pkg/
    ls -al

