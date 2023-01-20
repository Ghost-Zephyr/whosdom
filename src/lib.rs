
pub mod whois;

pub fn open(file_name: &str) -> std::io::Result<std::fs::File> {
    std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)
}
