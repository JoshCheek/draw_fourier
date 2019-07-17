Draw Fourier
============

Rust translation of https://github.com/JoshCheek/stylish-fourier-hydra

I wanted to do it in Rust because I figured they'd have libraries for working
with SVG and fourier transforms. But in the end, I had to do a lot of that myself,
I'm really not sure why their FFT didn't work, they should achieve the same results,
right? IDK >.< Anyway, it's still a bit of a WIP (eg doesn't look very nice yet).

```
# install dependencies
cargo build

# run against the built program
target/debug/draw_fourier < hydra.svg

# or run dynamically against whatever is in src/main.rs
env RUST_BACKTRACE=1 cargo run < hydra.svg
```
