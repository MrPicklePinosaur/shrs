
default: example

example:
    cd shrs_example && cargo run shrs_example

install:
    cargo install --profile=release --path shrs_example

devsetup:
    cp dev/hooks/* .git/hooks

fmt:
    cargo +nightly fmt --all

check:
    cargo check --workspace

lint:
    cargo clippy -- -W clippy::unwrap_used -W clippy::cargo

flamegraph:
    cargo flamegraph --profile=release

doc:
    cargo doc --workspace --all-features

book:
    cd docs && zola serve
