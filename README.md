# Kea-Rust-Base

This program plugs into ISC's Kea server (version 2.3.1) and does absolutely nothing. It is meant as a proof-of-concept that Rust can be used to write a hook, and as a point to start off from while developing another. This project utilizes Google's autocxx project to generate Rust bindings to the Kea source code (C++), allowing the developer to (hopefully) not need to resort to unsafe code that comes with many C FFI projects.

To use this project:  
- update dependencies and the library name  
- install Kea to the default location (currently mostly /usr/local)  
- build with `cargo build`  
- configure Kea to load the compiled `.so` file inside `target/`  
- start Kea and look for a debug message stating that the hook was loaded, and its version
