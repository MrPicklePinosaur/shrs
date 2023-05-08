use shrs_derive_completion::{self, Completion};

#[derive(Completion)]
struct MyCompletion {
    #[flag]
    help: bool,
}

fn main() {}
