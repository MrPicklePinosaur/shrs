
default: example

example:
    cargo run shrs_example

build:
    cargo build --release --target x86_64-unknown-linux-gnu

install:
    cargo install --profile=release --path shrs_example

devsetup:
    cp dev/hooks/* .git/hooks

fmt:
    cargo +nightly fmt --all

lint:
    cargo clippy -- -W clippy::unwrap_used -W clippy::cargo

flamegraph:
    cargo flamegraph --profile=release

doc:
    cargo doc --workspace --all-features

book:
    cd docs && zola serve
