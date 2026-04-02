pub mod commands;
pub mod metadata;
pub mod metapac;
pub mod pac;
pub mod util;

pub struct ChipDescription {
    /// The name of the chip to generate.
    chip: &'static str,

    /// Metadata file for this chip.
    metadata: &'static str,

    /// Cores to generate. If the chip has a single core, then this is the same as the
    /// [`name`](Feature::name) of the chip.
    cores: &'static [&'static str],

    /// When true, the source of the chip won't be the SVD anymore, but the manually curated peripherals
    metapac: bool,
}

/// Parts (and cores) to generate.
#[rustfmt::skip]
pub const CHIPS: &[ChipDescription] = &[
    ChipDescription { chip: "MIMXRT1011", metadata: "MIMXRT1011", cores: &["MIMXRT1011"], metapac: false },
    ChipDescription { chip: "MIMXRT1062", metadata: "MIMXRT106x", cores: &["MIMXRT1062"], metapac: false },
    ChipDescription { chip: "MIMXRT1064", metadata: "MIMXRT106x", cores: &["MIMXRT1064"], metapac: false },
    ChipDescription { chip: "MIMXRT685S", metadata: "", cores: &["MIMXRT685S_cm33"], metapac: false },
    ChipDescription { chip: "LPC55S16", metadata: "LPC55S16", cores: &["LPC55S16"], metapac: false },
    ChipDescription { chip: "LPC55S69", metadata: "LPC55S6x", cores: &["LPC55S69_cm33_core0", "LPC55S69_cm33_core1"], metapac: false },
    ChipDescription { chip: "MCXN947", metadata: "", cores: &["MCXN947_cm33_core0", "MCXN947_cm33_core1"], metapac: false },
    ChipDescription { chip: "MCXA256", metadata: "MCXA2xx", cores: &["MCXA256"], metapac: true },
    ChipDescription { chip: "MCXA577", metadata: "MCXA5xx", cores: &["MCXA577"], metapac: true },
];
