# include-files

Import files bytes into HashMap

## Usage

Cargo.toml:

```toml
[dependencies]
once_cell = "1.10.0"

[build-dependencies]
include-files = "*"
```

build.rs:

```rust
fn main() {
    let target = std::env::var("OUT_DIR").unwrap();
    include_files::IncludeFiles::new("./assets", "png", target).build().unwrap();
}
```

src/example.rs:
```rust
// Import build.rs output file
include!(env!("INCLUDE_FILES_PATH"));

for item in INCLUDE_FILES {
    println!("{:?}", item);
}
```