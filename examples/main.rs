use include_files::IncludeFiles;

fn main() {
    IncludeFiles::new("./assets", "txt", "./").build().unwrap();
}
