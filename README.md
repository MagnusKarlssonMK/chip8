My attempt at making a CHIP8 emulator in Rust.

Following the guide at https://tobiasvl.github.io/blog/write-a-chip-8-emulator/

Uses SDL2 for creating the display. However I made an effort to separate the display device from the emulator implementation, to make it possible to swap out to different graphics options without impacting the actual emulator.

Currently only supports simple CHIP-8. I might return to add support for SUPER-CHIP in the future.

OP code quirks and some other settings can be changed in [options](options.toml).
