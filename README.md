# Stitchy

A pocket row and stitch counter for knitting and crochet, built in async Rust on a Raspberry Pi Pico (RP2040) with [Embassy](https://embassy.dev).

Stitchy drives a 16×2 RGB-backlit character LCD and three push buttons. It keeps a row count for up to 10 named projects and saves them to on-board flash, so your count survives a power-off.

## Features

- **Row counting.** Count rows per project. The count is saved to flash when you leave the screen.
- **Stitch counting.** A scratch counter for stitches within a row. The backlight shifts colour as the count climbs (blue → green → red → white over roughly 0–150+), so you can see your progress at a glance. Stitch counts are not saved.
- **Multiple projects.** Up to 10 projects, each with a 10-character name and its own row count. Pick a project or create a new one from the project list.
- **Name entry.** Spell project names one character at a time (space, `!`–`/`, `0`–`9`, `:`–`@`, `A`–`Z`).
- **Persistent storage.** State is serialised with [postcard](https://crates.io/crates/postcard) and written to a reserved flash sector.
- **Colour-coded modes.** Each screen has its own backlight colour, and the backlight flashes blue/green/red on boot.

## Hardware

| Part | Notes |
| --- | --- |
| Raspberry Pi Pico (RP2040) | 2 MB flash, 264 KB RAM (see `memory.x`) |
| 16×2 I²C character LCD with RGB backlight | Grove-style JHD1313M1 type: text controller at `0x3E`, backlight controller at `0x60` |
| 3 × momentary push buttons | Wired between GPIO and GND (internal pull-ups are used) |
| Debug probe (optional) | Any probe supported by [probe-rs](https://probe.rs), e.g. a second Pico running Debugprobe |

### Wiring

| Function | Pico pin |
| --- | --- |
| LCD SDA | GPIO 4 (I2C0) |
| LCD SCL | GPIO 5 (I2C0) |
| Select / Menu button (`ButtonA`) | GPIO 17 |
| Down / Decrement button (`ButtonB`) | GPIO 18 |
| Up / Increment button (`ButtonC`) | GPIO 16 |

The I²C bus runs at 400 kHz. Buttons are active-low.

## Controls

The **Select** button understands short presses, long presses (≥ 800 ms) and double presses (within 200 ms). The **Up** and **Down** buttons register a press on each tap.

| Screen | Select | Down | Up |
| --- | --- | --- | --- |
| Row counter | Open menu | Row −1 | Row +1 |
| Stitch counter | Open menu | Stitch −1 | Stitch +1 |
| Menu | Enter highlighted option | Previous option | Next option |
| Project list | Open project (or **New** to create one) | Previous project | Next project |
| Name entry | Press: confirm letter and move on · Long press: save and finish · Double press: delete last letter | Previous character | Next character |

Menu options: **Count Rows**, **Count Stchs**, **Project Sel**, **Name Project**.

On first boot, when no projects have been named yet, Stitchy goes straight to name entry.

## Building and flashing

### Toolchain

The easiest route is the included Nix flake, which provides a stable Rust toolchain with the `thumbv6m-none-eabi` target, `probe-rs`, `picotool` and friends:

```sh
nix develop        # or: direnv allow  (uses .envrc)
```

Without Nix, install these yourself:

```sh
rustup target add thumbv6m-none-eabi
cargo install probe-rs-tools
```

### Build

```sh
cargo build --release
```

The default target (`thumbv6m-none-eabi`) is set in `.cargo/config.toml`. The output is `target/thumbv6m-none-eabi/release/stitchy`.

### Flash and view logs

With a debug probe connected:

```sh
probe-rs run --chip RP2040 target/thumbv6m-none-eabi/release/stitchy
```

Logs are sent over RTT with [defmt](https://defmt.ferrous-systems.com). The default level is `debug` (set by `DEFMT_LOG` in `.cargo/config.toml`).

Without a probe, you can convert the ELF to UF2 with `picotool` (or `elf2uf2-rs`) and drag it onto the Pico in BOOTSEL mode.

## Project layout

```
src/
├── main.rs                 # Entry point: initialises buttons, I²C display and flash, then runs the app loop
├── app/
│   ├── app_runner.rs       # Top-level state machine; routes between modes and handles errors
│   ├── peripheral_controller.rs
│   └── modes/              # One file per screen
│       ├── initialising.rs # Boot flash and loading saved state
│       ├── counting.rs     # Row counter
│       ├── stitch_counting.rs
│       ├── menu.rs
│       ├── project_list.rs
│       ├── rename.rs       # Name entry
│       └── error.rs        # Recoverable / fatal error screens
├── display/
│   ├── driver.rs           # Raw I²C writes to the LCD and backlight chips
│   ├── command.rs          # HD44780-style command encoding
│   └── controller.rs       # Higher-level display API (cursor, clear, text, colour)
├── input/
│   ├── mod.rs              # Button tasks feeding a shared input channel
│   └── button.rs           # Debounce plus long/double-press detection
├── memory_controller.rs    # Saves and loads UserData to flash with postcard
├── data.rs                 # UserData / Page models
└── utility.rs              # Wrapping counters and string helpers
```

### How it fits together

Each button runs in its own Embassy task, debounces its input, classifies the press and pushes a `ButtonPress` onto a shared `INPUT_CHANNEL`. The main loop calls `AppRunner::step()`. Each mode's `run()` takes over the display, consumes presses from the channel and returns the next state. Modes that change user data save it to flash before handing over.

Saved data lives at flash offset `0x100000` (the second megabyte) in a single erase sector.

## Notes

- `Embed.toml` and the `thumbv8m.main-none-eabihf` section of `.cargo/config.toml` refer to the RP2350 (Pico 2). The firmware currently targets the RP2040 (`thumbv6m-none-eabi`, `rp2040` feature on `embassy-rp`). Use `--chip RP2040` when flashing.
- Stitch counts are held in memory only and reset when you leave the stitch counter.
