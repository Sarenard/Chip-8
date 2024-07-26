# Chip-8

A Chip-8 emulator in rust comporting **1347** lines of code, made in the span of 3 days

Run with `cargo run --release -- -f <file path>`

Test with `cargo test -- --test-threads 1`

If the result is right, input `esc` else input anything else.

# Useful stuff

The [spec](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM) for the chip-8 VM

A [test suite](https://github.com/Timendus/chip8-test-suite/blob/main/README.md) with some roms

Some [more things](https://chip-8.github.io/links/) about Chip-8 if interested

# Things that could be done

- Pass the entire flags tests
- Pass the entire quirks tests
- Make some games