use shrs_derive_completion::Completion;

#[derive(Completion)]
struct MyCompletion {
    #[flag(short = "")]
    help: bool,
}

fn main() {}
