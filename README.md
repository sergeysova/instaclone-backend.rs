# Instaclone Backend

## Installation

1. Open [rustup.rs](https://rustup.rs) and follow instructions
2. Install Rust 1.26 (nightly)
3. Install PostgreSQL ^9.6.5
4. Create db instaclone: `CREATE DATABASE instaclone;` or change connection in `.env`
5. Install [diesel.rs](http://diesel.rs): `cargo install diesel_cli`
6. Migrate database: `diesel migration run`
7. Review `Rocket.toml`
8. **Start app**: `cargo run`
