# Conway's Game of Life implemented with WASM Rust

### Running locally
Site needs to be run from a web server since [js modules can't be used from a `file://` address](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Modules#other_differences_between_modules_and_classic_scripts).  
Run `python3 -m http.server` from project root to spin up a simple server.

If you make changes to Rust code you need to recompile for WASM target- `wasm-pack build --target web`
