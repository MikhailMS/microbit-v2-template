# Project template for Micro:bit V2

Small template for any new micro:bit V2 projects I may have in the future

Taken from incredible (Discovery book)[https://docs.rust-embedded.org/discovery/microbit/index.html] and slightly changed to only work on micro:bit V2


## How to use
1. `git clone` this project
2. Enable required libraries in `Cargo.toml` - by default only following are enabled
```
cortex-m    = "0.7.3"
cortex-m-rt = "0.7.0"
microbit-v2 = "0.12.0"
panic-halt  = "0.2.0"
```
3. Add your code in `src/main.rs`
4. Use `cargo embed` to flash your application onto micro:bit V2
