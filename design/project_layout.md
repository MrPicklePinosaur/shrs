
# Project Layout

The crates have the following functionalities:
- `shrs`: top level crate that implements the shell language
- `shrs_core`: implementations for all auxiliary features like environment vars, alias etc
- `shrs_lang`: lexer and parser for the shrs shell language
- `shrs_line`: readline implementation
- `shrs_vi`: parser for the shrs vi implementation
- `shrs_example`: example shell implementation using shrs library
- `shrs_utils`: unrelated utilities that shrs makes use of (could technically separate into a third party crate)

