//! build.rs build step for assertables
//! 
//! This build step does proofreading, and doesn't do any transformation.
//! 
//! To see print output:
//! 
//! ```sh
//! cargo -vv build
//! ```

use std::path::Path;
use walkdir::WalkDir;

fn main() {
    WalkDir::new("src")
        .into_iter()
        .map(|e| e.unwrap_or_else(|e| panic!("{:?}", e)))
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name() != "mod.rs")
        .for_each(|e| {
            let file_stem = Path::new(e.file_name()).file_stem().expect("file_stem");
            println!("{} {}", e.file_name().display(), file_stem.display());
        })
}
