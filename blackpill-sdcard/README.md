# SD CARD example

- run terminal 1: `openocd`
- run in terminal 2: `cargo run --release`

## DEFMT

- `Cargo.toml`: uncomment defmt-log feature
- `.cargo/config.toml`: uncomment `"-C", "link-arg=-Tdefmt.x",`
- `src/main.rs`: uncomment until it compiles, comment out the `use cortex_m_semihosting::hprintln;`
- run terminal 1: `export DEFMT_LOG=trace; cargo build --release && openocd | defmt-print -e target/thumbv7em-none-eabihf/release/blackpill-sdcard`
- run terminal 2: `cargo run --release`
