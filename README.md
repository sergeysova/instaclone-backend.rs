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


## Authorization

Send token in header `Authorization: Token asdqwezxc`.

Where `asdqwezxc` is your token received from `POST /session`.

### `GET /session`

Get info about current session.

Response:

```json
{
  "data": {
    "user_id": 123
  }
}
```

### `POST /session`

Create new token. Log in.

Receive:

```json
{
  "login": "string",
  "password": "string"
}
```

Response:

```json
{
  "data" {
    "token": "asdqwezxc"
  }
}
```

### `DELETE /session`

Destroy current token. Log out.
