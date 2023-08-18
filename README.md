# TM API
This is a small API serving my [MapRank plugin](https://github.com/mWalrus/MapRank).

## Requirements
- Rust >= 1.67.0, `cargo`, toolchain, etc. (see [rustup.rs](https://rustup.rs/))
- `pkg-config`
- `libssl-dev` (Ubuntu) or `openssl-devel` (Fedora)

## Install
1. On your server: `git clone https://github.com/mWalrus/tm-api.git /var/www/`
2. Create a new ubisoft account
3. Add that account's credentials to a `auth.key` file in the project root like so: `example@email.com:password`
4. Run `./install.sh`
5. Configure your webserver with a reverse proxy to `localhost:8000`
