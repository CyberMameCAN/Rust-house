# README

## Setting

    cargo add clap --features derive
    cargo add anyhow
    cargo add indicatif
    cargo add log
    cargo add env_logger

## Command

    cargo run -- some-pattern some-file

### env_logger

コマンドラインでこんな感じで使用する。

    RUST_LOG=info cargo run ./main
    RUST_LOG=info cargo test ./main
