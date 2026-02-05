run:
    cargo fmt --all
    cargo run

build:
    cargo build --release

install: build
    sudo install -Dm 755 target/release/anime-downloader /usr/bin
