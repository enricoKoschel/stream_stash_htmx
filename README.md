# Stream Stash

Companion for streaming movies, TV shows and anime.

## How to run locally

Run all the following commands in the root directory of the project

### Download Tailwind and daisyUI

Run the included `download_tailwind.sh` bash file.

```bash
./download_tailwind.sh
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

* Frontend
    * (HTMX)[https://htmx.org/]
    * (Tailwind)[https://tailwindcss.com/]
    * (daisyUI)[https://daisyui.com/]
* Backend
    * (Rust)[https://rust-lang.org/]
    * (Axum)[https://github.com/tokio-rs/axum]
    * (Hypertext)[https://github.com/vidhanio/hypertext]
