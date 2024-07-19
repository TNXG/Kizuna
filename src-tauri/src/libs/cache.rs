use std::fs;

pub fn create_cache_directory() -> std::io::Result<()> {
    if !fs::metadata(".\\cache\\").is_ok() {
        fs::create_dir(".\\cache.\\")?;
    }
    Ok(())
}
