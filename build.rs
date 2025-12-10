use std::fs;
use std::path::Path;

fn main() {
    // Copy assets folder
    let src_assets = Path::new("src/assets");
    let dst_assets = Path::new("target/release/assets");

    if src_assets.exists() {
        copy_dir_all(src_assets, dst_assets).unwrap();
    }

    // Copy config file
    let src_config = Path::new("src/config.toml");
    let dst_config = Path::new("target/release/config.toml");

    if src_config.exists() {
        fs::copy(src_config, dst_config).unwrap();
    }

    println!("cargo:rerun-if-changed=src/assets");
    println!("cargo:rerun-if-changed=src/config.toml");
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}