
default: example

example:
    cd shrs_example && cargo run shrs_example

install:
    ./dev/scripts/install

devsetup:
    cp dev/hooks/* .git/hooks

fmt:
    cargo +nightly fmt --all

check:
    cargo check --workspace --examples --tests --all-features

lint:
    cargo clippy -- -W clippy::unwrap_used -W clippy::cargo

spellfix:
    typos -w crates plugins src shrs_example docs/content

flamegraph:
    cargo flamegraph --profile=release

test:
    cargo test --workspace 

doc:
    cargo doc --workspace --all-features --no-deps --open

book:
    cd docs && zola serve
