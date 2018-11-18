# cargo-template-actix-tera 

Project template to use with [cargo-generate](https://crates.io/crates/cargo-generate).


Generates an [actix](https://actix.rs/) web application with [tera](https://github.com/Keats/tera) html templating.

## Prerequisites
```bash
cargo install cargo-generate
```
## Usage

Don't clone this project!
Instead run:
```bash
cargo generate --git https://github.com/otomato-gh/cargo-template-actix-tera.git --name <your-project-name>
cd <your-project-name>
cargo run (or ``cargo watch -x run``)
# Started http server: 127.0.0.1:8080
```
