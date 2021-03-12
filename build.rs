use definitions::Builder;
use std::path::PathBuf;

fn main() {
    let b = Builder::default();

    b.write_pairs(&PathBuf::from("src/internal/pairs.rs"));
    b.write_objects(&PathBuf::from("src/internal/object.rs"));
}
