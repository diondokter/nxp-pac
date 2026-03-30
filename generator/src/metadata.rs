use std::{fmt::Write, fs, path::Path};

use anyhow::Context;
use indexmap::IndexMap;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use serde::Deserialize;

use crate::rustfmt;

#[derive(Debug, Clone, Deserialize)]
pub struct Metadata {
    pub chips: Vec<String>,
    pub pins: Vec<Pin>,
    pub nvic_prio_bits: u32,
    pub interrupts: IndexMap<String, u32>,
    pub peripherals: Vec<Peripheral>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Pin {
    pub name: String,

    /// Supply for this pin.
    ///
    /// An example of when this is [`None`] is supply for a VREF pin (the pin is itself a supply).
    pub supply: Option<String>,

    /// IOMUXC information for this pin. Only applicable on RT1xxx chips.
    pub iomuxc: Option<PinIomuxc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PinIomuxc {
    /// Some pins only have a mux, thereby not being usable as GPIO.
    pub mux: Option<String>,

    /// Pins that are usable by IOMUXC require a pad register.
    pub pad: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Peripheral {
    pub name: String,
    #[serde(rename = "type")]
    pub peripheral_type: Option<String>,
    #[serde(rename = "address")]
    pub peripheral_address: Option<String>,
    pub signals: Vec<Signal>,
    pub flexcomm: Option<String>,
    #[serde(default)]
    pub dma_muxing: Vec<DmaMux>,
    pub only_in: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Signal {
    pub name: String,
    pub pins: Vec<SignalPin>,

    /// IOMUXC daisy register used for this signal.
    ///
    /// Depending on the peripheral type and instance, this may some be [`None`] even for a
    /// peripheral which usually has a daisy register.
    ///
    /// If this is [`Some`], each pin's [`Signal::iomuxc_daisy`] value must be [`Some`].
    pub iomuxc_daisy: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SignalPin {
    pub pin: String,
    pub alt: u8,

    /// IOMUXC daisy value to write into the daisy register of the parent [`Signal`].
    ///
    /// This is required if [`Signal::iomuxc_daisy`] is [`Some`]
    pub iomuxc_daisy: Option<u8>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DmaMux {
    pub signal: String,
    pub mux: String,
    pub request: u8,
}

fn generate_metadata(name: &str, metadata: &Metadata) -> TokenStream {
    let pins = metadata.pins.iter().map(|pin| {
        let name = &pin.name;
        let iomuxc = pin
            .iomuxc
            .as_ref()
            .map(|iomuxc| {
                let pad = u32::from_str_radix(&iomuxc.pad[2..], 16).unwrap();

                let mux = iomuxc
                    .mux
                    .as_ref()
                    .map(|mux| {
                        let mux = u32::from_str_radix(&mux[2..], 16).unwrap();
                        quote! { Some(#mux) }
                    })
                    .unwrap_or_else(|| quote! { None });

                quote! {
                    Some(PinIomuxc {
                        mux: #mux,
                        pad: #pad,
                    })
                }
            })
            .unwrap_or_else(|| quote! { None });

        quote! {
            Pin {
                name: #name,
                iomuxc: #iomuxc,
            }
        }
    });

    let peripherals = metadata.peripherals.iter().map(|peripheral| {
        let name = &peripheral.name;
        let flexcomm = peripheral
            .flexcomm
            .as_ref()
            .map(|ref fc| quote! { Some(#fc) })
            .unwrap_or_else(|| quote! { None });

        let signals = peripheral.signals.iter().map(|signal| {
            let name = &signal.name;

            let iomuxc_daisy = signal
                .iomuxc_daisy
                .as_ref()
                .map(|iomuxc| {
                    let daisy = u32::from_str_radix(&iomuxc[2..], 16).unwrap();

                    quote! {
                        Some(#daisy)
                    }
                })
                .unwrap_or_else(|| quote! { None });

            let pins = signal.pins.iter().map(|signal| {
                let pin = &signal.pin;
                let alt = signal.alt;
                let iomuxc_daisy = signal
                    .iomuxc_daisy
                    .as_ref()
                    .map(|daisy| quote! { Some(#daisy) })
                    .unwrap_or_else(|| quote! { None });

                quote! {
                    SignalPin {
                        pin: #pin,
                        alt: #alt,
                        iomuxc_daisy: #iomuxc_daisy,
                    }
                }
            });

            quote! {
                Signal {
                    name: #name,
                    pins: &[#(#pins),*],
                    iomuxc_daisy: #iomuxc_daisy,
                }
            }
        });

        let dma_muxing = peripheral.dma_muxing.iter().map(|dma_mux| {
            let signal = &dma_mux.signal;
            let mux = &dma_mux.mux;
            let request = Literal::u8_unsuffixed(dma_mux.request);

            quote! {
                DmaMux {
                    signal: #signal,
                    mux: #mux,
                    request: #request,
                }
            }
        });

        let address = match peripheral.peripheral_address.as_ref() {
            Some(val) => {
                let val: TokenStream = val
                    .parse()
                    .expect("Peripheral address is parsed to tokenstream");
                quote! { #val }
            }
            None => quote! { 0 },
        };

        let driver_name = peripheral.peripheral_type.as_deref().unwrap_or_default();

        quote! {
            Peripheral {
                name: #name,
                address: #address,
                driver_name: #driver_name,
                signals: &[#(#signals),*],
                flexcomm: #flexcomm,
                dma_muxing: &[#(#dma_muxing),*],
            }
        }
    });

    let interrupts = metadata
        .interrupts
        .iter()
        .map(|(name, val)| quote! { (#name, #val) });

    quote! {
        use crate::metadata::*;

        pub const METADATA: Metadata = Metadata {
            name: #name,
            pins: PINS,
            peripherals: PERIPHERALS,
            interrupts: INTERRUPTS,
        };

        pub const PINS: &[Pin] = &[#(#pins),*];
        pub const PERIPHERALS: &[Peripheral] = &[#(#peripherals),*];
        pub const INTERRUPTS: &[(&str, u32)] = &[#(#interrupts),*];
    }
}

pub fn generate_core(
    chips_dir: &Path,
    svd: &Path,
    metadata: &Path,
    core: &str,
) -> anyhow::Result<Metadata> {
    let metadata = fs::read_to_string(metadata).context("Read metadata")?;
    let mut metadata =
        serde_json::from_str::<Metadata>(&metadata).context("Deserialize metadata")?;

    if metadata.interrupts.is_empty() {
        // If the metadata doesn't define the interrupts, we'll get it from the SVD

        let svd_contents = fs::read_to_string(svd).context("Read SVD")?;
        let svd = svd_parser::parse(&svd_contents).context("Parse SVD")?;

        for peripheral in svd.peripherals.iter() {
            for interrupt in peripheral.interrupt.iter() {
                // Rust uses fully capitalized interrupt names for singletons.
                metadata
                    .interrupts
                    .insert(interrupt.name.clone().to_uppercase(), interrupt.value);
            }
        }

        metadata.interrupts.sort_unstable_by_key(|_, val| *val);
    }

    let mut metadata_out = String::new();
    write!(metadata_out, "{}", generate_metadata(core, &metadata))?;

    let metadata_rs = chips_dir.join(core.to_lowercase()).join("metadata.rs");
    if !metadata_rs
        .parent()
        .context("getting metadata.rs parent")?
        .exists()
    {
        fs::create_dir_all(metadata_rs.parent().context("getting metadata.rs parent")?)?;
    }
    fs::write(&metadata_rs, metadata_out)?;
    rustfmt(&metadata_rs).context("Formatting metadata")?;

    Ok(metadata)
}
