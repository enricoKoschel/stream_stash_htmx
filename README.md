# Stream Stash

Companion for streaming movies, TV shows and anime.

## How to run locally

Run all the following commands in the root directory of the project

### Download Tailwind and daisyUI

Run the included `download_tailwind.sh` bash file.

```bash
./download_tailwind.sh
```

### Request API access to [The Movie Database](https://www.themoviedb.org)

Click [here](https://developer.themoviedb.org/docs/getting-started) for instructions on how to request access

### Setup Google OAuth

1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Create a new project or select an existing one
3. Navigate to "APIs & Services" > "Credentials"
4. Create OAuth 2.0 credentials (Web application type)
5. Add `http://localhost` and `http://localhost:8000` to authorized JavaScript origins
6. Add `http://localhost/login` and `http://localhost:8000/login` to authorized redirect URIs
7. Copy the Client ID for your `.env` file

### Setup environment variables

- Create a `.env` file in the same directory as the `Cargo.toml` file with the following contents:

TODO: Add ip, port, log level
```toml
TMDB_READ_ACCESS_TOKEN="" # Your TMDB API Read Access Token
DATABASE_URL="" # URL to the Stream Stash DB (e.g.: "sqlite:stream_stash.db")
GOOGLE_CLIENT_ID="" # Your Google OAuth Client ID (ends with .apps.googleusercontent.com)
```

- Make sure to use the TMDB API Read Access Token, not the API Key (both can be found [here](https://www.themoviedb.org/settings/api) after your request to access the TMDB API has been granted)

### Setup local database

#### Install sqlx-cli

```bash
cargo install sqlx-cli
```

#### Create local DB

```bash
sqlx database create
```

#### Run migrations

```bash
sqlx migrate run
```

### Install cargo watch (optional, but recommended)

```bash
cargo install cargo-watch
```

### Run the app

With `cargo watch`

```bash
cargo watch -c -x run
# or
cargo watch -c -x "run --release"
```

Without `cargo watch`

```bash
cargo run
#or
cargo run --release
```

### Format the files

```bash
cargo fmt
```

## Stack

- Frontend
  - [HTMX](https://htmx.org/)
  - [Tailwind](https://tailwindcss.com/)
  - [daisyUI](https://daisyui.com/)
- Backend
  - [Rust](https://rust-lang.org/)
  - [Maud](https://github.com/lambda-fairy/maud)
