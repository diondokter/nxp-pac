use std::{fs, path::Path};

pub fn export_meta_peripherals(current: &Path) -> anyhow::Result<()> {
    let pac_peri_dir = current.join("nxp-pac/meta_peripherals");
    let yaml_peri_dir = current.join("data/metadata/peripherals");

    for entry in read_dir_all::read_dir_all(&yaml_peri_dir)? {
        let entry = entry?;
        let entry_path = entry.path().to_path_buf();

        let relative_entry_path = entry_path.strip_prefix(&yaml_peri_dir)?;
        if relative_entry_path.starts_with("raw") || entry_path.is_dir() {
            continue;
        }

        println!(
            "{}, {}",
            entry.path().display(),
            entry.file_name().display()
        );
    }

    Ok(())
}
