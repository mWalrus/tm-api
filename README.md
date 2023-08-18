# TM API
This is a small API serving my [MapRank plugin](https://github.com/mWalrus/MapRank).

## Requirements
- Rust >= 1.67.0 (`cargo`, toolchain, etc. setup)

## Install
1. On your server: `git clone https://github.com/mWalrus/tm-api.git /var/www/`
2. `install.sh`
3. Configure your webserver with a reverse proxy to `localhost:8000`

## Usage
If you want to host this yourself you'll need to:
1. Clone the repo: `git clone https://github.com/mWalrus/tm-api`
2. Create a new ubisoft account
3. Add that account's credentials to a `auth.key` file in the project root like so: `example@email.com:password`
