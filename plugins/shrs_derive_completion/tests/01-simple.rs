use shrs_derive_completion::Completion;

#[derive(Completion)]
struct MyCompletion {
    #[flag(long = "help")]
    help: bool,
}

fn main() {}
