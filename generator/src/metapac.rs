use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::{Context, bail};
use indexmap::IndexSet;
use inflections::Inflect;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::metadata::Metadata;

pub fn export_meta_peripherals(current: &Path) -> anyhow::Result<()> {
    let pac_peri_dir = current.join("nxp-pac/src/meta_peripherals");
    let yaml_peri_dir = current.join("data/metadata/peripherals");

    if !pac_peri_dir.exists() {
        fs::create_dir_all(&pac_peri_dir)?;
    }

    for file in read_dir_all::read_dir_all(&pac_peri_dir)? {
        let file = file?;
        // Erasing is for niceness and is not required. If it doesn't work that's fine
        let _ = fs::remove_file(file.path());
    }

    read_dir_all::read_dir_all(&yaml_peri_dir)?
        .par_bridge()
        .try_for_each(|entry| {
            let entry = entry?;
            let entry_path = entry.path().to_path_buf();

            let relative_entry_path = entry_path.strip_prefix(&yaml_peri_dir)?;
            if relative_entry_path.starts_with("raw")
                || entry_path.is_dir()
                || entry_path.extension().map(|e| e.to_string_lossy()) != Some("yaml".into())
            {
                return Ok(());
            }

            tracing::info!("{}", relative_entry_path.display());

            let mut output_path = pac_peri_dir.join(relative_entry_path);
            output_path.set_extension("rs");
            fs::create_dir_all(
                output_path
                    .parent()
                    .context("Getting parent of output path")?,
            )?;

            let output = Command::new("chiptool")
                .arg("gen-block")
                .arg("--input")
                .arg(
                    entry
                        .path()
                        .canonicalize()
                        .context("Canonicalizing entry path")?,
                )
                .arg("--output")
                .arg(&output_path)
                .arg("--common-module")
                .arg("crate::pac::common")
                .arg("--skip-no-std")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()?;

            if !output.status.success() {
                bail!(
                    "Error generating block {}:\nSTDERR:\n{}",
                    relative_entry_path.display(),
                    String::from_utf8_lossy(&output.stderr)
                );
            }

            let output = Command::new("rustfmt")
                .arg("--edition")
                .arg("2024")
                .arg(output_path)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()?;

            if !output.status.success() {
                bail!(
                    "Error formatting block {}:\nSTDERR:\n{}",
                    relative_entry_path.display(),
                    String::from_utf8_lossy(&output.stderr)
                );
            }

            Ok(())
        })?;

    Ok(())
}

pub fn assemble_metapac(current: &Path, core: &str, mut metadata: Metadata) -> anyhow::Result<()> {
    let chip_dir = current.join("nxp-pac/src/chips").join(core.to_lowercase());
    if !chip_dir.exists() {
        fs::create_dir_all(&chip_dir).context("creating chip dir")?;
    }

    // Remove all peripherals that are defined, but don't have a driver
    let yaml_peri_dir = current.join("data/metadata/peripherals");
    metadata.peripherals.retain(|p| {
        let Some(peripheral_type) = p.peripheral_type.as_ref() else {
            tracing::warn!("Peripheral {} has no type associated. Skipped.", p.name);

            return false;
        };

        if fs::exists(yaml_peri_dir.join(peripheral_type).with_extension("yaml")).unwrap_or(false) {
            true
        } else {
            tracing::warn!(
                "Peripheral {} does not point to an existing driver: `{}`. Skipped.",
                p.name,
                peripheral_type
            );

            false
        }
    });

    export_device_x(&chip_dir, &metadata).context("exporting device.x")?;
    export_mod_rs(&chip_dir, &metadata).context("exporting mod.rs")?;
    export_vectors_rs(&chip_dir, &metadata).context("exporting _vectors.rs")?;
    export_common_rs(&chip_dir).context("exporting common.rs")?;

    Ok(())
}

fn export_device_x(chip_dir: &Path, metadata: &Metadata) -> anyhow::Result<()> {
    use std::fmt::Write;

    let mut contents = String::new();

    for (name, _) in metadata.interrupts.iter() {
        writeln!(&mut contents, "PROVIDE({name} = DefaultHandler);")?;
    }

    fs::write(chip_dir.join("device.x"), contents.as_bytes())
        .context("writing contents to file")?;

    Ok(())
}

fn export_mod_rs(chip_dir: &Path, metadata: &Metadata) -> anyhow::Result<()> {
    use std::fmt::Write;

    let mut contents = String::new();

    writeln!(&mut contents, "{}", create_interrupts(metadata))?;

    // Create instances for all peripherals
    for peripheral in metadata.peripherals.iter() {
        let Some(driver_path) = peripheral.peripheral_type.as_ref() else {
            tracing::warn!(
                "Peripheral {} has no type associated. Skipped.",
                peripheral.name
            );
            continue;
        };

        let driver_name = driver_path
            .split('/')
            .last()
            .with_context(|| format!("Getting driver name from path: {}", driver_path))?;
        let driver_name_mod = Ident::new(&driver_name.to_lowercase(), Span::call_site());
        let driver_name_type = Ident::new(&driver_name.to_pascal_case(), Span::call_site());

        let peripheral_name = Ident::new(&peripheral.name, Span::call_site());
        let Some(peripheral_address) = peripheral.peripheral_address.as_ref() else {
            tracing::warn!(
                "Peripheral {} has no address associated. Skipped.",
                peripheral.name
            );
            continue;
        };
        let periperal_address: TokenStream = peripheral_address
            .parse()
            .expect("peripheral_address parsed as tokenstream");

        writeln!(
            &mut contents,
            "{}",
            quote! {
                pub const #peripheral_name: #driver_name_mod::#driver_name_type = unsafe { #driver_name_mod::#driver_name_type::from_ptr(#periperal_address as _) };
            }
        )?;
    }

    // Create mods with a path pointing into meta_peripherals
    let drivers = metadata
        .peripherals
        .iter()
        .flat_map(|p| p.peripheral_type.clone())
        .collect::<IndexSet<String>>();
    for driver_path in drivers {
        let driver_name = driver_path
            .split('/')
            .last()
            .with_context(|| format!("Getting driver name from path: {}", driver_path))?;
        let driver_name_mod = Ident::new(&driver_name.to_lowercase(), Span::call_site());

        let full_driver_path = format!("../../meta_peripherals/{driver_path}.rs");

        writeln!(
            &mut contents,
            "{}",
            quote! {
                #[path = #full_driver_path]
                pub mod #driver_name_mod;
            }
        )?;
    }

    writeln!(&mut contents, "pub mod common;")?;

    fs::write(chip_dir.join("mod.rs"), contents.as_bytes()).context("writing contents to file")?;

    Ok(())
}

fn create_interrupts(metadata: &Metadata) -> TokenStream {
    let interrupt_tokens = metadata.interrupts.iter().map(|(name, val)| {
        let val_lit = Literal::u32_unsuffixed(*val);
        let name_ident = Ident::new(name, Span::call_site());
        let doc = format!("{val} - {name}");

        quote! {
            #[doc = #doc]
            #name_ident = #val_lit
        }
    });

    let nvic_prio_bits = Literal::u32_unsuffixed(metadata.nvic_prio_bits);

    quote! {
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Interrupt {
            #(#interrupt_tokens),*
        }

        unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
            #[inline(always)]
            fn number(self) -> u16 {
                self as u16
            }
        }

        #[cfg(feature = "rt")]
        mod _vectors;

        #[doc = r" Number available in the NVIC for configuring priority"]
        #[cfg(feature = "rt")]
        pub const NVIC_PRIO_BITS: u8 = #nvic_prio_bits;
        #[cfg(feature = "rt")]
        pub use Interrupt as interrupt;
        #[cfg(feature = "rt")]
        pub use cortex_m_rt::interrupt;
    }
}

fn export_vectors_rs(chip_dir: &Path, metadata: &Metadata) -> anyhow::Result<()> {
    use std::fmt::Write;

    let extern_functions = metadata.interrupts.iter().map(|(name, _)| {
        let name_identifier = Ident::new(name, Span::call_site());
        quote! { fn #name_identifier(); }
    });

    let extern_functions_tokens = quote! {
        unsafe extern "C" {
            #(#extern_functions)*
        }
    };

    let num_interrupts = metadata
        .interrupts
        .iter()
        .map(|(_, num)| *num)
        .max()
        .unwrap_or_default()
        + 1;

    let vectors = (0..num_interrupts).map(|num| {
        match metadata
            .interrupts
            .iter()
            .find(|(_, irq_num)| **irq_num == num)
        {
            Some((name, _)) => {
                let name_ident = Ident::new(name, Span::call_site());
                quote! { Vector { _handler: #name_ident } }
            }
            None => quote! { Vector { _reserved: 0 } },
        }
    });

    let num_interrupts_lit = Literal::u32_unsuffixed(num_interrupts);

    let interrupt_vector_table = quote! {
        pub union Vector {
            _handler: unsafe extern "C" fn(),
            _reserved: u32,
        }
        #[unsafe(link_section = ".vector_table.interrupts")]
        #[unsafe(no_mangle)]
        pub static __INTERRUPTS: [Vector; #num_interrupts_lit] = [
            #(#vectors),*
        ];
    };

    let mut contents = String::new();

    writeln!(&mut contents, "{}", extern_functions_tokens)?;
    writeln!(&mut contents, "{}", interrupt_vector_table)?;

    fs::write(chip_dir.join("_vectors.rs"), contents.as_bytes())
        .context("writing contents to file")?;

    Ok(())
}

fn export_common_rs(chip_dir: &Path) -> anyhow::Result<()> {
    let output = Command::new("chiptool")
        .arg("gen-common")
        .arg("--output")
        .arg(&chip_dir.join("common.rs"))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        bail!(
            "Error generating common:\nSTDERR:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}
