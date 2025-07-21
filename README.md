# Chess with Bevy
A Chess implementation with the Bevy game engine (in Rust).

Soon a bot will be implemented.

## Requirements
- **Rust** (stable) — install via [rustup](https://rustup.rs)

## Building the project
Clone the repository and build it in release mode:
```bash
git clone https://github.com/CieriA/bevy-chess
cd bevy-chess
cargo build --release
```

## Running the project
```bash
cargo run --release
```

## Controls
- Drag and drop to move the pieces.

## Development Notes
This project uses the following crates:
- bevy
- indexmap

### Docs
To build the documentation locally, run:
```bash
cargo doc --open
```

## License
This project is licensed under [ISC License](LICENSE).
