//! # Usage
//! Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! once_cell = "1.10.0"
//!
//! [build-dependencies]
//! include-files = "*"
//! ```
//!
//! build.rs:
//!
//! ```-
//! fn main() {
//!     let target = std::env::var("OUT_DIR").unwrap();
//!     include_files::IncludeFiles::new("./assets", "png", target).build().unwrap();
//! }
//! ```
//!
//! src/example.rs:
//! ```-
//! // Import build.rs output file
//! include!(env!("INCLUDE_FILES_PATH"));
//!
//! for item in INCLUDE_FILES {
//!     println!("{:?}", item);
//! }
//! ```
use std::env::current_dir;
use std::fs::{self, File};
use std::io::{Result as IoResult, Write};
use std::path::{Path, PathBuf};

const TEMPLATE: &str = r#"
static INCLUDE_FILES: once_cell::sync::Lazy<std::collections::HashMap<&'static str, &'static [u8]>> = once_cell::sync::Lazy::new(|| {
    let mut map: std::collections::HashMap<&'static str, &'static [u8]> = std::collections::HashMap::new();
    // INSERT
    map
});
"#;

#[derive(Debug, Clone)]
pub struct IncludeFiles {
    input: PathBuf,
    output: PathBuf,
    extension: &'static str,
}

impl IncludeFiles {
    pub fn new<I: AsRef<Path>, O: AsRef<Path>>(
        input_dir: I,
        extension: &'static str,
        output_dir: O,
    ) -> Self {
        Self {
            input: input_dir.as_ref().to_path_buf(),
            output: output_dir.as_ref().to_path_buf(),
            extension,
        }
    }

    pub fn build(&self) -> IoResult<()> {
        let cur = current_dir()?;
        let output = cur.join(&self.output).join("include-files.rs");
        let mut file = File::create(&output)?;
        let mut content = String::new();
        let dir = fs::read_dir(&self.input)?;

        for rst in dir {
            let item = rst?.path();
            if let (Some(ext), Some(name)) = (item.extension(), item.file_stem()) {
                if ext == self.extension {
                    let name = name.to_str().unwrap();
                    content.push_str(&format!(
                        r#"
    map.insert("{}", include_bytes!(r"{}/icons/{}.png"));
    "#,
                        name,
                        cur.display(),
                        name
                    ));
                }
            }
        }

        file.write_all(TEMPLATE.replace("// INSERT", &content).as_bytes())?;
        println!("cargo:rustc-env=INCLUDE_FILES_PATH={}", output.display());
        Ok(())
    }
}
