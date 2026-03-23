#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6a8c2aa 2026-01-27))"]
#[doc = "System Clock Generator."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scg {
    ptr: *mut u8,
}
unsafe impl Send for Scg {}
unsafe impl Sync for Scg {}
impl Scg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID Register."]
    #[inline(always)]
    pub const fn verid(self) -> crate::pac::common::Reg<Verid, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Parameter Register."]
    #[inline(always)]
    pub const fn param(self) -> crate::pac::common::Reg<Param, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Trim Lock register."]
    #[inline(always)]
    pub const fn trim_lock(self) -> crate::pac::common::Reg<TrimLock, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Clock Status Register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::pac::common::Reg<Csr, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Run Clock Control Register."]
    #[inline(always)]
    pub const fn rccr(self) -> crate::pac::common::Reg<Rccr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "SOSC Control Status Register."]
    #[inline(always)]
    pub const fn sosccsr(self) -> crate::pac::common::Reg<Sosccsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "SOSC Configuration Register."]
    #[inline(always)]
    pub const fn sosccfg(self) -> crate::pac::common::Reg<Sosccfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "SIRC Control Status Register."]
    #[inline(always)]
    pub const fn sirccsr(self) -> crate::pac::common::Reg<Sirccsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "SIRC Trim Configuration Register."]
    #[inline(always)]
    pub const fn sirctcfg(self) -> crate::pac::common::Reg<Sirctcfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "SIRC Trim Register."]
    #[inline(always)]
    pub const fn sirctrim(self) -> crate::pac::common::Reg<Sirctrim, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "SIRC Auto-trimming Status Register."]
    #[inline(always)]
    pub const fn sircstat(self) -> crate::pac::common::Reg<Sircstat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "FIRC Control Status Register."]
    #[inline(always)]
    pub const fn firccsr(self) -> crate::pac::common::Reg<Firccsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "FIRC Configuration Register."]
    #[inline(always)]
    pub const fn firccfg(self) -> crate::pac::common::Reg<Firccfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "FIRC Trim Register."]
    #[inline(always)]
    pub const fn firctrim(self) -> crate::pac::common::Reg<Firctrim, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[doc = "ROSC Control Status Register."]
    #[inline(always)]
    pub const fn rosccsr(self) -> crate::pac::common::Reg<Rosccsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "SPLL Control Status Register."]
    #[inline(always)]
    pub const fn spllcsr(self) -> crate::pac::common::Reg<Spllcsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "SPLL Control Register."]
    #[inline(always)]
    pub const fn spllctrl(self) -> crate::pac::common::Reg<Spllctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0604usize) as _) }
    }
    #[doc = "SPLL Status Register."]
    #[inline(always)]
    pub const fn spllstat(self) -> crate::pac::common::Reg<Spllstat, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0608usize) as _) }
    }
    #[doc = "SPLL N Divider Register."]
    #[inline(always)]
    pub const fn spllndiv(self) -> crate::pac::common::Reg<Spllndiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x060cusize) as _) }
    }
    #[doc = "SPLL M Divider Register."]
    #[inline(always)]
    pub const fn spllmdiv(self) -> crate::pac::common::Reg<Spllmdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0610usize) as _) }
    }
    #[doc = "SPLL P Divider Register."]
    #[inline(always)]
    pub const fn spllpdiv(self) -> crate::pac::common::Reg<Spllpdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0614usize) as _) }
    }
    #[doc = "SPLL LOCK Configuration Register."]
    #[inline(always)]
    pub const fn splllock_cnfg(
        self,
    ) -> crate::pac::common::Reg<SplllockCnfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0618usize) as _) }
    }
    #[doc = "SPLL SSCG Status Register."]
    #[inline(always)]
    pub const fn spllsscgstat(
        self,
    ) -> crate::pac::common::Reg<Spllsscgstat, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize) as _) }
    }
    #[doc = "SPLL Spread Spectrum Control 0 Register."]
    #[inline(always)]
    pub const fn spllsscg0(self) -> crate::pac::common::Reg<Spllsscg0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0624usize) as _) }
    }
    #[doc = "SPLL Spread Spectrum Control 1 Register."]
    #[inline(always)]
    pub const fn spllsscg1(self) -> crate::pac::common::Reg<Spllsscg1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0628usize) as _) }
    }
    #[doc = "LDO Control and Status Register."]
    #[inline(always)]
    pub const fn ldocsr(self) -> crate::pac::common::Reg<Ldocsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
}
#[doc = "Clock Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "System Clock Source."]
    #[must_use]
    #[inline(always)]
    pub const fn scs(&self) -> Scs {
        let val = (self.0 >> 24usize) & 0x07;
        Scs::from_bits(val as u8)
    }
    #[doc = "System Clock Source."]
    #[inline(always)]
    pub const fn set_scs(&mut self, val: Scs) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
}
impl Default for Csr {
    #[inline(always)]
    fn default() -> Csr {
        Csr(0)
    }
}
impl core::fmt::Debug for Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csr").field("scs", &self.scs()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Csr {{ scs: {:?} }}", self.scs())
    }
}
#[doc = "FIRC Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Firccfg(pub u32);
impl Firccfg {
    #[doc = "Frequency select."]
    #[must_use]
    #[inline(always)]
    pub const fn freq_sel(&self) -> FreqSel {
        let val = (self.0 >> 1usize) & 0x07;
        FreqSel::from_bits(val as u8)
    }
    #[doc = "Frequency select."]
    #[inline(always)]
    pub const fn set_freq_sel(&mut self, val: FreqSel) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
    }
}
impl Default for Firccfg {
    #[inline(always)]
    fn default() -> Firccfg {
        Firccfg(0)
    }
}
impl core::fmt::Debug for Firccfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Firccfg")
            .field("freq_sel", &self.freq_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Firccfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Firccfg {{ freq_sel: {:?} }}", self.freq_sel())
    }
}
#[doc = "FIRC Control Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Firccsr(pub u32);
impl Firccsr {
    #[doc = "FIRC Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fircen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIRC Enable."]
    #[inline(always)]
    pub const fn set_fircen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIRC Stop Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fircsten(&self) -> Fircsten {
        let val = (self.0 >> 1usize) & 0x01;
        Fircsten::from_bits(val as u8)
    }
    #[doc = "FIRC Stop Enable."]
    #[inline(always)]
    pub const fn set_fircsten(&mut self, val: Fircsten) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "FIRC 45 MHz Clock to peripherals Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn firc_sclk_periph_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "FIRC 45 MHz Clock to peripherals Enable."]
    #[inline(always)]
    pub const fn set_firc_sclk_periph_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "FRO_HF Clock to peripherals Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn firc_fclk_periph_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "FRO_HF Clock to peripherals Enable."]
    #[inline(always)]
    pub const fn set_firc_fclk_periph_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> FirccsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        FirccsrLk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: FirccsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "FIRC Valid status."]
    #[must_use]
    #[inline(always)]
    pub const fn fircvld(&self) -> Fircvld {
        let val = (self.0 >> 24usize) & 0x01;
        Fircvld::from_bits(val as u8)
    }
    #[doc = "FIRC Valid status."]
    #[inline(always)]
    pub const fn set_fircvld(&mut self, val: Fircvld) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "FIRC Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn fircsel(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "FIRC Selected."]
    #[inline(always)]
    pub const fn set_fircsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "FIRC Clock Error."]
    #[must_use]
    #[inline(always)]
    pub const fn fircerr(&self) -> Fircerr {
        let val = (self.0 >> 26usize) & 0x01;
        Fircerr::from_bits(val as u8)
    }
    #[doc = "FIRC Clock Error."]
    #[inline(always)]
    pub const fn set_fircerr(&mut self, val: Fircerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "FIRC Clock Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fircerr_ie(&self) -> FircerrIe {
        let val = (self.0 >> 27usize) & 0x01;
        FircerrIe::from_bits(val as u8)
    }
    #[doc = "FIRC Clock Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_fircerr_ie(&mut self, val: FircerrIe) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "FIRC Accurate Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fircacc_ie(&self) -> FircaccIe {
        let val = (self.0 >> 30usize) & 0x01;
        FircaccIe::from_bits(val as u8)
    }
    #[doc = "FIRC Accurate Interrupt Enable."]
    #[inline(always)]
    pub const fn set_fircacc_ie(&mut self, val: FircaccIe) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "FIRC Frequency Accurate."]
    #[must_use]
    #[inline(always)]
    pub const fn fircacc(&self) -> Fircacc {
        let val = (self.0 >> 31usize) & 0x01;
        Fircacc::from_bits(val as u8)
    }
    #[doc = "FIRC Frequency Accurate."]
    #[inline(always)]
    pub const fn set_fircacc(&mut self, val: Fircacc) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Firccsr {
    #[inline(always)]
    fn default() -> Firccsr {
        Firccsr(0)
    }
}
impl core::fmt::Debug for Firccsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Firccsr")
            .field("fircen", &self.fircen())
            .field("fircsten", &self.fircsten())
            .field("firc_sclk_periph_en", &self.firc_sclk_periph_en())
            .field("firc_fclk_periph_en", &self.firc_fclk_periph_en())
            .field("lk", &self.lk())
            .field("fircvld", &self.fircvld())
            .field("fircsel", &self.fircsel())
            .field("fircerr", &self.fircerr())
            .field("fircerr_ie", &self.fircerr_ie())
            .field("fircacc_ie", &self.fircacc_ie())
            .field("fircacc", &self.fircacc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Firccsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Firccsr {{ fircen: {=bool:?}, fircsten: {:?}, firc_sclk_periph_en: {=bool:?}, firc_fclk_periph_en: {=bool:?}, lk: {:?}, fircvld: {:?}, fircsel: {=bool:?}, fircerr: {:?}, fircerr_ie: {:?}, fircacc_ie: {:?}, fircacc: {:?} }}",
            self.fircen(),
            self.fircsten(),
            self.firc_sclk_periph_en(),
            self.firc_fclk_periph_en(),
            self.lk(),
            self.fircvld(),
            self.fircsel(),
            self.fircerr(),
            self.fircerr_ie(),
            self.fircacc_ie(),
            self.fircacc()
        )
    }
}
#[doc = "FIRC Trim Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Firctrim(pub u32);
impl Firctrim {
    #[doc = "Trim Fine."]
    #[must_use]
    #[inline(always)]
    pub const fn trimfine(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Trim Fine."]
    #[inline(always)]
    pub const fn set_trimfine(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Trim Coarse."]
    #[must_use]
    #[inline(always)]
    pub const fn trimcoar(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Trim Coarse."]
    #[inline(always)]
    pub const fn set_trimcoar(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Trim Temperature."]
    #[must_use]
    #[inline(always)]
    pub const fn trimtemp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Trim Temperature."]
    #[inline(always)]
    pub const fn set_trimtemp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Trim Start."]
    #[must_use]
    #[inline(always)]
    pub const fn trimstart(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Trim Start."]
    #[inline(always)]
    pub const fn set_trimstart(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Firctrim {
    #[inline(always)]
    fn default() -> Firctrim {
        Firctrim(0)
    }
}
impl core::fmt::Debug for Firctrim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Firctrim")
            .field("trimfine", &self.trimfine())
            .field("trimcoar", &self.trimcoar())
            .field("trimtemp", &self.trimtemp())
            .field("trimstart", &self.trimstart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Firctrim {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Firctrim {{ trimfine: {=u8:?}, trimcoar: {=u8:?}, trimtemp: {=u8:?}, trimstart: {=u8:?} }}",
            self.trimfine(),
            self.trimcoar(),
            self.trimtemp(),
            self.trimstart()
        )
    }
}
#[doc = "LDO Control and Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldocsr(pub u32);
impl Ldocsr {
    #[doc = "LDO Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ldoen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LDO Enable."]
    #[inline(always)]
    pub const fn set_ldoen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LDO output voltage select."]
    #[must_use]
    #[inline(always)]
    pub const fn vout_sel(&self) -> VoutSel {
        let val = (self.0 >> 1usize) & 0x07;
        VoutSel::from_bits(val as u8)
    }
    #[doc = "LDO output voltage select."]
    #[inline(always)]
    pub const fn set_vout_sel(&mut self, val: VoutSel) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
    }
    #[doc = "LDO Bypass."]
    #[must_use]
    #[inline(always)]
    pub const fn ldobypass(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "LDO Bypass."]
    #[inline(always)]
    pub const fn set_ldobypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "LDO VOUT OK Inform."]
    #[must_use]
    #[inline(always)]
    pub const fn vout_ok(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LDO VOUT OK Inform."]
    #[inline(always)]
    pub const fn set_vout_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ldocsr {
    #[inline(always)]
    fn default() -> Ldocsr {
        Ldocsr(0)
    }
}
impl core::fmt::Debug for Ldocsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldocsr")
            .field("ldoen", &self.ldoen())
            .field("vout_sel", &self.vout_sel())
            .field("ldobypass", &self.ldobypass())
            .field("vout_ok", &self.vout_ok())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldocsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ldocsr {{ ldoen: {=bool:?}, vout_sel: {:?}, ldobypass: {=bool:?}, vout_ok: {=bool:?} }}",
            self.ldoen(),
            self.vout_sel(),
            self.ldobypass(),
            self.vout_ok()
        )
    }
}
#[doc = "Parameter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "SOSC Clock Present."]
    #[must_use]
    #[inline(always)]
    pub const fn soscclkpres(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Clock Present."]
    #[inline(always)]
    pub const fn set_soscclkpres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SIRC Clock Present."]
    #[must_use]
    #[inline(always)]
    pub const fn sircclkpres(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC Clock Present."]
    #[inline(always)]
    pub const fn set_sircclkpres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FIRC Clock Present."]
    #[must_use]
    #[inline(always)]
    pub const fn fircclkpres(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FIRC Clock Present."]
    #[inline(always)]
    pub const fn set_fircclkpres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "ROSC Clock Present."]
    #[must_use]
    #[inline(always)]
    pub const fn roscclkpres(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "ROSC Clock Present."]
    #[inline(always)]
    pub const fn set_roscclkpres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "SPLL Clock Present."]
    #[must_use]
    #[inline(always)]
    pub const fn spllclkpres(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "SPLL Clock Present."]
    #[inline(always)]
    pub const fn set_spllclkpres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Param {
    #[inline(always)]
    fn default() -> Param {
        Param(0)
    }
}
impl core::fmt::Debug for Param {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Param")
            .field("soscclkpres", &self.soscclkpres())
            .field("sircclkpres", &self.sircclkpres())
            .field("fircclkpres", &self.fircclkpres())
            .field("roscclkpres", &self.roscclkpres())
            .field("spllclkpres", &self.spllclkpres())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Param {{ soscclkpres: {=bool:?}, sircclkpres: {=bool:?}, fircclkpres: {=bool:?}, roscclkpres: {=bool:?}, spllclkpres: {=bool:?} }}",
            self.soscclkpres(),
            self.sircclkpres(),
            self.fircclkpres(),
            self.roscclkpres(),
            self.spllclkpres()
        )
    }
}
#[doc = "Run Clock Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rccr(pub u32);
impl Rccr {
    #[doc = "System Clock Source."]
    #[must_use]
    #[inline(always)]
    pub const fn scs(&self) -> Scs {
        let val = (self.0 >> 24usize) & 0x07;
        Scs::from_bits(val as u8)
    }
    #[doc = "System Clock Source."]
    #[inline(always)]
    pub const fn set_scs(&mut self, val: Scs) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
}
impl Default for Rccr {
    #[inline(always)]
    fn default() -> Rccr {
        Rccr(0)
    }
}
impl core::fmt::Debug for Rccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rccr").field("scs", &self.scs()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rccr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rccr {{ scs: {:?} }}", self.scs())
    }
}
#[doc = "ROSC Control Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rosccsr(pub u32);
impl Rosccsr {
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> RosccsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        RosccsrLk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: RosccsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "ROSC Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn roscvld(&self) -> Roscvld {
        let val = (self.0 >> 24usize) & 0x01;
        Roscvld::from_bits(val as u8)
    }
    #[doc = "ROSC Valid."]
    #[inline(always)]
    pub const fn set_roscvld(&mut self, val: Roscvld) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "ROSC Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn roscsel(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "ROSC Selected."]
    #[inline(always)]
    pub const fn set_roscsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "ROSC Clock Error."]
    #[must_use]
    #[inline(always)]
    pub const fn roscerr(&self) -> Roscerr {
        let val = (self.0 >> 26usize) & 0x01;
        Roscerr::from_bits(val as u8)
    }
    #[doc = "ROSC Clock Error."]
    #[inline(always)]
    pub const fn set_roscerr(&mut self, val: Roscerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for Rosccsr {
    #[inline(always)]
    fn default() -> Rosccsr {
        Rosccsr(0)
    }
}
impl core::fmt::Debug for Rosccsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rosccsr")
            .field("lk", &self.lk())
            .field("roscvld", &self.roscvld())
            .field("roscsel", &self.roscsel())
            .field("roscerr", &self.roscerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rosccsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rosccsr {{ lk: {:?}, roscvld: {:?}, roscsel: {=bool:?}, roscerr: {:?} }}",
            self.lk(),
            self.roscvld(),
            self.roscsel(),
            self.roscerr()
        )
    }
}
#[doc = "SIRC Control Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sirccsr(pub u32);
impl Sirccsr {
    #[doc = "SIRC Stop Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sircsten(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC Stop Enable."]
    #[inline(always)]
    pub const fn set_sircsten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SIRC Clock to Peripherals Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sirc_clk_periph_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC Clock to Peripherals Enable."]
    #[inline(always)]
    pub const fn set_sirc_clk_periph_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "SIRC 12 MHz Trim Enable (SIRCCFG\\[RANGE\\]=1)."]
    #[must_use]
    #[inline(always)]
    pub const fn sirctren(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC 12 MHz Trim Enable (SIRCCFG\\[RANGE\\]=1)."]
    #[inline(always)]
    pub const fn set_sirctren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SIRC Trim Update."]
    #[must_use]
    #[inline(always)]
    pub const fn sirctrup(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC Trim Update."]
    #[inline(always)]
    pub const fn set_sirctrup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SIRC TRIM LOCK."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_lock(&self) -> TrimLock {
        let val = (self.0 >> 10usize) & 0x01;
        TrimLock::from_bits(val as u8)
    }
    #[doc = "SIRC TRIM LOCK."]
    #[inline(always)]
    pub const fn set_trim_lock(&mut self, val: TrimLock) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Coarse Auto Trim Bypass."]
    #[must_use]
    #[inline(always)]
    pub const fn coarse_trim_bypass(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Coarse Auto Trim Bypass."]
    #[inline(always)]
    pub const fn set_coarse_trim_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> SirccsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        SirccsrLk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: SirccsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SIRC Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn sircvld(&self) -> Sircvld {
        let val = (self.0 >> 24usize) & 0x01;
        Sircvld::from_bits(val as u8)
    }
    #[doc = "SIRC Valid."]
    #[inline(always)]
    pub const fn set_sircvld(&mut self, val: Sircvld) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "SIRC Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn sircsel(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC Selected."]
    #[inline(always)]
    pub const fn set_sircsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "SIRC Clock Error."]
    #[must_use]
    #[inline(always)]
    pub const fn sircerr(&self) -> Sircerr {
        let val = (self.0 >> 26usize) & 0x01;
        Sircerr::from_bits(val as u8)
    }
    #[doc = "SIRC Clock Error."]
    #[inline(always)]
    pub const fn set_sircerr(&mut self, val: Sircerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "SIRC Clock Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sircerr_ie(&self) -> SircerrIe {
        let val = (self.0 >> 27usize) & 0x01;
        SircerrIe::from_bits(val as u8)
    }
    #[doc = "SIRC Clock Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_sircerr_ie(&mut self, val: SircerrIe) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Sirccsr {
    #[inline(always)]
    fn default() -> Sirccsr {
        Sirccsr(0)
    }
}
impl core::fmt::Debug for Sirccsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sirccsr")
            .field("sircsten", &self.sircsten())
            .field("sirc_clk_periph_en", &self.sirc_clk_periph_en())
            .field("sirctren", &self.sirctren())
            .field("sirctrup", &self.sirctrup())
            .field("trim_lock", &self.trim_lock())
            .field("coarse_trim_bypass", &self.coarse_trim_bypass())
            .field("lk", &self.lk())
            .field("sircvld", &self.sircvld())
            .field("sircsel", &self.sircsel())
            .field("sircerr", &self.sircerr())
            .field("sircerr_ie", &self.sircerr_ie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sirccsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sirccsr {{ sircsten: {=bool:?}, sirc_clk_periph_en: {=bool:?}, sirctren: {=bool:?}, sirctrup: {=bool:?}, trim_lock: {:?}, coarse_trim_bypass: {=bool:?}, lk: {:?}, sircvld: {:?}, sircsel: {=bool:?}, sircerr: {:?}, sircerr_ie: {:?} }}",
            self.sircsten(),
            self.sirc_clk_periph_en(),
            self.sirctren(),
            self.sirctrup(),
            self.trim_lock(),
            self.coarse_trim_bypass(),
            self.lk(),
            self.sircvld(),
            self.sircsel(),
            self.sircerr(),
            self.sircerr_ie()
        )
    }
}
#[doc = "SIRC Auto-trimming Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sircstat(pub u32);
impl Sircstat {
    #[doc = "CCO Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn ccotrim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "CCO Trim."]
    #[inline(always)]
    pub const fn set_ccotrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "CL Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn cltrim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "CL Trim."]
    #[inline(always)]
    pub const fn set_cltrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for Sircstat {
    #[inline(always)]
    fn default() -> Sircstat {
        Sircstat(0)
    }
}
impl core::fmt::Debug for Sircstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sircstat")
            .field("ccotrim", &self.ccotrim())
            .field("cltrim", &self.cltrim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sircstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sircstat {{ ccotrim: {=u8:?}, cltrim: {=u8:?} }}",
            self.ccotrim(),
            self.cltrim()
        )
    }
}
#[doc = "SIRC Trim Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sirctcfg(pub u32);
impl Sirctcfg {
    #[doc = "Trim Source."]
    #[must_use]
    #[inline(always)]
    pub const fn trimsrc(&self) -> Trimsrc {
        let val = (self.0 >> 0usize) & 0x03;
        Trimsrc::from_bits(val as u8)
    }
    #[doc = "Trim Source."]
    #[inline(always)]
    pub const fn set_trimsrc(&mut self, val: Trimsrc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "SIRC Trim Pre-divider."]
    #[must_use]
    #[inline(always)]
    pub const fn trimdiv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "SIRC Trim Pre-divider."]
    #[inline(always)]
    pub const fn set_trimdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for Sirctcfg {
    #[inline(always)]
    fn default() -> Sirctcfg {
        Sirctcfg(0)
    }
}
impl core::fmt::Debug for Sirctcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sirctcfg")
            .field("trimsrc", &self.trimsrc())
            .field("trimdiv", &self.trimdiv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sirctcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sirctcfg {{ trimsrc: {:?}, trimdiv: {=u8:?} }}",
            self.trimsrc(),
            self.trimdiv()
        )
    }
}
#[doc = "SIRC Trim Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sirctrim(pub u32);
impl Sirctrim {
    #[doc = "CCO Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn ccotrim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "CCO Trim."]
    #[inline(always)]
    pub const fn set_ccotrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "CL Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn cltrim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "CL Trim."]
    #[inline(always)]
    pub const fn set_cltrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Trim Temp."]
    #[must_use]
    #[inline(always)]
    pub const fn tctrim(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Trim Temp."]
    #[inline(always)]
    pub const fn set_tctrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Calibrates the replica voltage in FSU for CCO to get well frequency at initial period."]
    #[must_use]
    #[inline(always)]
    pub const fn fvchtrim(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Calibrates the replica voltage in FSU for CCO to get well frequency at initial period."]
    #[inline(always)]
    pub const fn set_fvchtrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for Sirctrim {
    #[inline(always)]
    fn default() -> Sirctrim {
        Sirctrim(0)
    }
}
impl core::fmt::Debug for Sirctrim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sirctrim")
            .field("ccotrim", &self.ccotrim())
            .field("cltrim", &self.cltrim())
            .field("tctrim", &self.tctrim())
            .field("fvchtrim", &self.fvchtrim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sirctrim {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sirctrim {{ ccotrim: {=u8:?}, cltrim: {=u8:?}, tctrim: {=u8:?}, fvchtrim: {=u8:?} }}",
            self.ccotrim(),
            self.cltrim(),
            self.tctrim(),
            self.fvchtrim()
        )
    }
}
#[doc = "SOSC Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sosccfg(pub u32);
impl Sosccfg {
    #[doc = "External Reference Select."]
    #[must_use]
    #[inline(always)]
    pub const fn erefs(&self) -> Erefs {
        let val = (self.0 >> 2usize) & 0x01;
        Erefs::from_bits(val as u8)
    }
    #[doc = "External Reference Select."]
    #[inline(always)]
    pub const fn set_erefs(&mut self, val: Erefs) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SOSC Range Select."]
    #[must_use]
    #[inline(always)]
    pub const fn range(&self) -> Range {
        let val = (self.0 >> 4usize) & 0x03;
        Range::from_bits(val as u8)
    }
    #[doc = "SOSC Range Select."]
    #[inline(always)]
    pub const fn set_range(&mut self, val: Range) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for Sosccfg {
    #[inline(always)]
    fn default() -> Sosccfg {
        Sosccfg(0)
    }
}
impl core::fmt::Debug for Sosccfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sosccfg")
            .field("erefs", &self.erefs())
            .field("range", &self.range())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sosccfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sosccfg {{ erefs: {:?}, range: {:?} }}",
            self.erefs(),
            self.range()
        )
    }
}
#[doc = "SOSC Control Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sosccsr(pub u32);
impl Sosccsr {
    #[doc = "SOSC Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn soscen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Enable."]
    #[inline(always)]
    pub const fn set_soscen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SOSC Stop Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn soscsten(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Stop Enable."]
    #[inline(always)]
    pub const fn set_soscsten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SOSC Clock Monitor Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sosccm(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Clock Monitor Enable."]
    #[inline(always)]
    pub const fn set_sosccm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SOSC Clock Monitor Reset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sosccmre(&self) -> Sosccmre {
        let val = (self.0 >> 17usize) & 0x01;
        Sosccmre::from_bits(val as u8)
    }
    #[doc = "SOSC Clock Monitor Reset Enable."]
    #[inline(always)]
    pub const fn set_sosccmre(&mut self, val: Sosccmre) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> SosccsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        SosccsrLk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: SosccsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SOSC Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn soscvld(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Valid."]
    #[inline(always)]
    pub const fn set_soscvld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "SOSC Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn soscsel(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Selected."]
    #[inline(always)]
    pub const fn set_soscsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "SOSC Clock Error."]
    #[must_use]
    #[inline(always)]
    pub const fn soscerr(&self) -> Soscerr {
        let val = (self.0 >> 26usize) & 0x01;
        Soscerr::from_bits(val as u8)
    }
    #[doc = "SOSC Clock Error."]
    #[inline(always)]
    pub const fn set_soscerr(&mut self, val: Soscerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "SOSC Valid Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn soscvld_ie(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Valid Interrupt Enable."]
    #[inline(always)]
    pub const fn set_soscvld_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "SOSC clock safety enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sosc_safe_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC clock safety enable."]
    #[inline(always)]
    pub const fn set_sosc_safe_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Sosccsr {
    #[inline(always)]
    fn default() -> Sosccsr {
        Sosccsr(0)
    }
}
impl core::fmt::Debug for Sosccsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sosccsr")
            .field("soscen", &self.soscen())
            .field("soscsten", &self.soscsten())
            .field("sosccm", &self.sosccm())
            .field("sosccmre", &self.sosccmre())
            .field("lk", &self.lk())
            .field("soscvld", &self.soscvld())
            .field("soscsel", &self.soscsel())
            .field("soscerr", &self.soscerr())
            .field("soscvld_ie", &self.soscvld_ie())
            .field("sosc_safe_en", &self.sosc_safe_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sosccsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sosccsr {{ soscen: {=bool:?}, soscsten: {=bool:?}, sosccm: {=bool:?}, sosccmre: {:?}, lk: {:?}, soscvld: {=bool:?}, soscsel: {=bool:?}, soscerr: {:?}, soscvld_ie: {=bool:?}, sosc_safe_en: {=bool:?} }}",
            self.soscen(),
            self.soscsten(),
            self.sosccm(),
            self.sosccmre(),
            self.lk(),
            self.soscvld(),
            self.soscsel(),
            self.soscerr(),
            self.soscvld_ie(),
            self.sosc_safe_en()
        )
    }
}
#[doc = "SPLL Control Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllcsr(pub u32);
impl Spllcsr {
    #[doc = "SPLL Power Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn spllpwren(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPLL Power Enable."]
    #[inline(always)]
    pub const fn set_spllpwren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SPLL Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn spllclken(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SPLL Clock Enable."]
    #[inline(always)]
    pub const fn set_spllclken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SPLL Stop Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn spllsten(&self) -> Spllsten {
        let val = (self.0 >> 2usize) & 0x01;
        Spllsten::from_bits(val as u8)
    }
    #[doc = "SPLL Stop Enable."]
    #[inline(always)]
    pub const fn set_spllsten(&mut self, val: Spllsten) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Free running mode clock stable."]
    #[must_use]
    #[inline(always)]
    pub const fn frm_clockstable(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Free running mode clock stable."]
    #[inline(always)]
    pub const fn set_frm_clockstable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SPLL Clock Monitor."]
    #[must_use]
    #[inline(always)]
    pub const fn spllcm(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SPLL Clock Monitor."]
    #[inline(always)]
    pub const fn set_spllcm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SPLL Clock Monitor Reset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn spllcmre(&self) -> Spllcmre {
        let val = (self.0 >> 17usize) & 0x01;
        Spllcmre::from_bits(val as u8)
    }
    #[doc = "SPLL Clock Monitor Reset Enable."]
    #[inline(always)]
    pub const fn set_spllcmre(&mut self, val: Spllcmre) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> SpllcsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        SpllcsrLk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: SpllcsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SPLL LOCK."]
    #[must_use]
    #[inline(always)]
    pub const fn spll_lock(&self) -> SpllLock {
        let val = (self.0 >> 24usize) & 0x01;
        SpllLock::from_bits(val as u8)
    }
    #[doc = "SPLL LOCK."]
    #[inline(always)]
    pub const fn set_spll_lock(&mut self, val: SpllLock) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "SPLL Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn spllsel(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SPLL Selected."]
    #[inline(always)]
    pub const fn set_spllsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "SPLL Clock Error."]
    #[must_use]
    #[inline(always)]
    pub const fn spllerr(&self) -> Spllerr {
        let val = (self.0 >> 26usize) & 0x01;
        Spllerr::from_bits(val as u8)
    }
    #[doc = "SPLL Clock Error."]
    #[inline(always)]
    pub const fn set_spllerr(&mut self, val: Spllerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "SPLL LOCK Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn spll_lock_ie(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "SPLL LOCK Interrupt Enable."]
    #[inline(always)]
    pub const fn set_spll_lock_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Spllcsr {
    #[inline(always)]
    fn default() -> Spllcsr {
        Spllcsr(0)
    }
}
impl core::fmt::Debug for Spllcsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllcsr")
            .field("spllpwren", &self.spllpwren())
            .field("spllclken", &self.spllclken())
            .field("spllsten", &self.spllsten())
            .field("frm_clockstable", &self.frm_clockstable())
            .field("spllcm", &self.spllcm())
            .field("spllcmre", &self.spllcmre())
            .field("lk", &self.lk())
            .field("spll_lock", &self.spll_lock())
            .field("spllsel", &self.spllsel())
            .field("spllerr", &self.spllerr())
            .field("spll_lock_ie", &self.spll_lock_ie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllcsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllcsr {{ spllpwren: {=bool:?}, spllclken: {=bool:?}, spllsten: {:?}, frm_clockstable: {=bool:?}, spllcm: {=bool:?}, spllcmre: {:?}, lk: {:?}, spll_lock: {:?}, spllsel: {=bool:?}, spllerr: {:?}, spll_lock_ie: {=bool:?} }}",
            self.spllpwren(),
            self.spllclken(),
            self.spllsten(),
            self.frm_clockstable(),
            self.spllcm(),
            self.spllcmre(),
            self.lk(),
            self.spll_lock(),
            self.spllsel(),
            self.spllerr(),
            self.spll_lock_ie()
        )
    }
}
#[doc = "SPLL Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllctrl(pub u32);
impl Spllctrl {
    #[doc = "Bandwidth select R (resistor) value."]
    #[must_use]
    #[inline(always)]
    pub const fn selr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Bandwidth select R (resistor) value."]
    #[inline(always)]
    pub const fn set_selr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Bandwidth select I (interation) value."]
    #[must_use]
    #[inline(always)]
    pub const fn seli(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x3f;
        val as u8
    }
    #[doc = "Bandwidth select I (interation) value."]
    #[inline(always)]
    pub const fn set_seli(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 4usize)) | (((val as u32) & 0x3f) << 4usize);
    }
    #[doc = "Bandwidth select P (proportional) value."]
    #[must_use]
    #[inline(always)]
    pub const fn selp(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "Bandwidth select P (proportional) value."]
    #[inline(always)]
    pub const fn set_selp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "Bypass of the divide-by-2 divider."]
    #[must_use]
    #[inline(always)]
    pub const fn bypasspostdiv2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass of the divide-by-2 divider."]
    #[inline(always)]
    pub const fn set_bypasspostdiv2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Up Limiter."]
    #[must_use]
    #[inline(always)]
    pub const fn limupoff(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Up Limiter."]
    #[inline(always)]
    pub const fn set_limupoff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Control of the bandwidth of the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn banddirect(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Control of the bandwidth of the PLL."]
    #[inline(always)]
    pub const fn set_banddirect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Bypass of the pre-divider."]
    #[must_use]
    #[inline(always)]
    pub const fn bypassprediv(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass of the pre-divider."]
    #[inline(always)]
    pub const fn set_bypassprediv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Bypass of the post-divider."]
    #[must_use]
    #[inline(always)]
    pub const fn bypasspostdiv(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass of the post-divider."]
    #[inline(always)]
    pub const fn set_bypasspostdiv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Free Running Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn frm(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Free Running Mode Enable."]
    #[inline(always)]
    pub const fn set_frm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Clock Source."]
    #[must_use]
    #[inline(always)]
    pub const fn source(&self) -> Source {
        let val = (self.0 >> 25usize) & 0x03;
        Source::from_bits(val as u8)
    }
    #[doc = "Clock Source."]
    #[inline(always)]
    pub const fn set_source(&mut self, val: Source) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
}
impl Default for Spllctrl {
    #[inline(always)]
    fn default() -> Spllctrl {
        Spllctrl(0)
    }
}
impl core::fmt::Debug for Spllctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllctrl")
            .field("selr", &self.selr())
            .field("seli", &self.seli())
            .field("selp", &self.selp())
            .field("bypasspostdiv2", &self.bypasspostdiv2())
            .field("limupoff", &self.limupoff())
            .field("banddirect", &self.banddirect())
            .field("bypassprediv", &self.bypassprediv())
            .field("bypasspostdiv", &self.bypasspostdiv())
            .field("frm", &self.frm())
            .field("source", &self.source())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllctrl {{ selr: {=u8:?}, seli: {=u8:?}, selp: {=u8:?}, bypasspostdiv2: {=bool:?}, limupoff: {=bool:?}, banddirect: {=bool:?}, bypassprediv: {=bool:?}, bypasspostdiv: {=bool:?}, frm: {=bool:?}, source: {:?} }}",
            self.selr(),
            self.seli(),
            self.selp(),
            self.bypasspostdiv2(),
            self.limupoff(),
            self.banddirect(),
            self.bypassprediv(),
            self.bypasspostdiv(),
            self.frm(),
            self.source()
        )
    }
}
#[doc = "SPLL LOCK Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SplllockCnfg(pub u32);
impl SplllockCnfg {
    #[doc = "Configures the number of reference clocks to count before SPLL is considered locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_time(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Configures the number of reference clocks to count before SPLL is considered locked."]
    #[inline(always)]
    pub const fn set_lock_time(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
    }
}
impl Default for SplllockCnfg {
    #[inline(always)]
    fn default() -> SplllockCnfg {
        SplllockCnfg(0)
    }
}
impl core::fmt::Debug for SplllockCnfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SplllockCnfg")
            .field("lock_time", &self.lock_time())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SplllockCnfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SplllockCnfg {{ lock_time: {=u32:?} }}",
            self.lock_time()
        )
    }
}
#[doc = "SPLL M Divider Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllmdiv(pub u32);
impl Spllmdiv {
    #[doc = "Feedback divider ratio (M-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn mdiv(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feedback divider ratio (M-divider)."]
    #[inline(always)]
    pub const fn set_mdiv(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Feedback ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn mreq(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Feedback ratio change request."]
    #[inline(always)]
    pub const fn set_mreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Spllmdiv {
    #[inline(always)]
    fn default() -> Spllmdiv {
        Spllmdiv(0)
    }
}
impl core::fmt::Debug for Spllmdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllmdiv")
            .field("mdiv", &self.mdiv())
            .field("mreq", &self.mreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllmdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllmdiv {{ mdiv: {=u16:?}, mreq: {=bool:?} }}",
            self.mdiv(),
            self.mreq()
        )
    }
}
#[doc = "SPLL N Divider Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllndiv(pub u32);
impl Spllndiv {
    #[doc = "Pre-divider divider ratio (N-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn ndiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Pre-divider divider ratio (N-divider)."]
    #[inline(always)]
    pub const fn set_ndiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Pre-divider ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn nreq(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Pre-divider ratio change request."]
    #[inline(always)]
    pub const fn set_nreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Spllndiv {
    #[inline(always)]
    fn default() -> Spllndiv {
        Spllndiv(0)
    }
}
impl core::fmt::Debug for Spllndiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllndiv")
            .field("ndiv", &self.ndiv())
            .field("nreq", &self.nreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllndiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllndiv {{ ndiv: {=u8:?}, nreq: {=bool:?} }}",
            self.ndiv(),
            self.nreq()
        )
    }
}
#[doc = "SPLL P Divider Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllpdiv(pub u32);
impl Spllpdiv {
    #[doc = "Post-divider divider ratio (P-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn pdiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Post-divider divider ratio (P-divider)."]
    #[inline(always)]
    pub const fn set_pdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Post-divider ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn preq(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Post-divider ratio change request."]
    #[inline(always)]
    pub const fn set_preq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Spllpdiv {
    #[inline(always)]
    fn default() -> Spllpdiv {
        Spllpdiv(0)
    }
}
impl core::fmt::Debug for Spllpdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllpdiv")
            .field("pdiv", &self.pdiv())
            .field("preq", &self.preq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllpdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllpdiv {{ pdiv: {=u8:?}, preq: {=bool:?} }}",
            self.pdiv(),
            self.preq()
        )
    }
}
#[doc = "SPLL Spread Spectrum Control 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllsscg0(pub u32);
impl Spllsscg0 {
    #[doc = "SS_MDIV\\[31:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn ss_mdiv_lsb(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SS_MDIV\\[31:0\\]."]
    #[inline(always)]
    pub const fn set_ss_mdiv_lsb(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Spllsscg0 {
    #[inline(always)]
    fn default() -> Spllsscg0 {
        Spllsscg0(0)
    }
}
impl core::fmt::Debug for Spllsscg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllsscg0")
            .field("ss_mdiv_lsb", &self.ss_mdiv_lsb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllsscg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllsscg0 {{ ss_mdiv_lsb: {=u32:?} }}",
            self.ss_mdiv_lsb()
        )
    }
}
#[doc = "SPLL Spread Spectrum Control 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllsscg1(pub u32);
impl Spllsscg1 {
    #[doc = "SS_MDIV\\[32\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn ss_mdiv_msb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SS_MDIV\\[32\\]."]
    #[inline(always)]
    pub const fn set_ss_mdiv_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SS_MDIV\\[32:0\\] change request."]
    #[must_use]
    #[inline(always)]
    pub const fn ss_mdiv_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SS_MDIV\\[32:0\\] change request."]
    #[inline(always)]
    pub const fn set_ss_mdiv_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Modulation Frequency Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mf(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[doc = "Modulation Frequency Control."]
    #[inline(always)]
    pub const fn set_mf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[doc = "Modulation Depth Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mr(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Modulation Depth Control."]
    #[inline(always)]
    pub const fn set_mr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "Modulation Waveform Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mc(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Modulation Waveform Control."]
    #[inline(always)]
    pub const fn set_mc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Dither Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dither(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Dither Enable."]
    #[inline(always)]
    pub const fn set_dither(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SS_MDIV select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel_ss_mdiv(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SS_MDIV select."]
    #[inline(always)]
    pub const fn set_sel_ss_mdiv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SSCG Power Down."]
    #[must_use]
    #[inline(always)]
    pub const fn ss_pd(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "SSCG Power Down."]
    #[inline(always)]
    pub const fn set_ss_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Spllsscg1 {
    #[inline(always)]
    fn default() -> Spllsscg1 {
        Spllsscg1(0)
    }
}
impl core::fmt::Debug for Spllsscg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllsscg1")
            .field("ss_mdiv_msb", &self.ss_mdiv_msb())
            .field("ss_mdiv_req", &self.ss_mdiv_req())
            .field("mf", &self.mf())
            .field("mr", &self.mr())
            .field("mc", &self.mc())
            .field("dither", &self.dither())
            .field("sel_ss_mdiv", &self.sel_ss_mdiv())
            .field("ss_pd", &self.ss_pd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllsscg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllsscg1 {{ ss_mdiv_msb: {=bool:?}, ss_mdiv_req: {=bool:?}, mf: {=u8:?}, mr: {=u8:?}, mc: {=u8:?}, dither: {=bool:?}, sel_ss_mdiv: {=bool:?}, ss_pd: {=bool:?} }}",
            self.ss_mdiv_msb(),
            self.ss_mdiv_req(),
            self.mf(),
            self.mr(),
            self.mc(),
            self.dither(),
            self.sel_ss_mdiv(),
            self.ss_pd()
        )
    }
}
#[doc = "SPLL SSCG Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllsscgstat(pub u32);
impl Spllsscgstat {
    #[doc = "SS_MDIV change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn ss_mdiv_ack(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SS_MDIV change acknowledge."]
    #[inline(always)]
    pub const fn set_ss_mdiv_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Spllsscgstat {
    #[inline(always)]
    fn default() -> Spllsscgstat {
        Spllsscgstat(0)
    }
}
impl core::fmt::Debug for Spllsscgstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllsscgstat")
            .field("ss_mdiv_ack", &self.ss_mdiv_ack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllsscgstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllsscgstat {{ ss_mdiv_ack: {=bool:?} }}",
            self.ss_mdiv_ack()
        )
    }
}
#[doc = "SPLL Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllstat(pub u32);
impl Spllstat {
    #[doc = "Pre-divider (N) ratio change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn ndivack(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pre-divider (N) ratio change acknowledge."]
    #[inline(always)]
    pub const fn set_ndivack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Feedback (M) divider ratio change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn mdivack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Feedback (M) divider ratio change acknowledge."]
    #[inline(always)]
    pub const fn set_mdivack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Post-divider (P) ratio change acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn pdivack(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Post-divider (P) ratio change acknowledge."]
    #[inline(always)]
    pub const fn set_pdivack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Free running detector (active high)."]
    #[must_use]
    #[inline(always)]
    pub const fn frmdet(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Free running detector (active high)."]
    #[inline(always)]
    pub const fn set_frmdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Spllstat {
    #[inline(always)]
    fn default() -> Spllstat {
        Spllstat(0)
    }
}
impl core::fmt::Debug for Spllstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllstat")
            .field("ndivack", &self.ndivack())
            .field("mdivack", &self.mdivack())
            .field("pdivack", &self.pdivack())
            .field("frmdet", &self.frmdet())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllstat {{ ndivack: {=bool:?}, mdivack: {=bool:?}, pdivack: {=bool:?}, frmdet: {=bool:?} }}",
            self.ndivack(),
            self.mdivack(),
            self.pdivack(),
            self.frmdet()
        )
    }
}
#[doc = "Trim Lock register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrimLock(pub u32);
impl TrimLock {
    #[doc = "TRIM_UNLOCK."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_unlock(&self) -> TrimUnlock {
        let val = (self.0 >> 0usize) & 0x01;
        TrimUnlock::from_bits(val as u8)
    }
    #[doc = "TRIM_UNLOCK."]
    #[inline(always)]
    pub const fn set_trim_unlock(&mut self, val: TrimUnlock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "IFR_DISABLE."]
    #[must_use]
    #[inline(always)]
    pub const fn ifr_disable(&self) -> IfrDisable {
        let val = (self.0 >> 1usize) & 0x01;
        IfrDisable::from_bits(val as u8)
    }
    #[doc = "IFR_DISABLE."]
    #[inline(always)]
    pub const fn set_ifr_disable(&mut self, val: IfrDisable) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "TRIM_LOCK_KEY."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_lock_key(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIM_LOCK_KEY."]
    #[inline(always)]
    pub const fn set_trim_lock_key(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for TrimLock {
    #[inline(always)]
    fn default() -> TrimLock {
        TrimLock(0)
    }
}
impl core::fmt::Debug for TrimLock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrimLock")
            .field("trim_unlock", &self.trim_unlock())
            .field("ifr_disable", &self.ifr_disable())
            .field("trim_lock_key", &self.trim_lock_key())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrimLock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TrimLock {{ trim_unlock: {:?}, ifr_disable: {:?}, trim_lock_key: {=u16:?} }}",
            self.trim_unlock(),
            self.ifr_disable(),
            self.trim_lock_key()
        )
    }
}
#[doc = "Version ID Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "SCG Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn version(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SCG Version Number."]
    #[inline(always)]
    pub const fn set_version(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Verid {
    #[inline(always)]
    fn default() -> Verid {
        Verid(0)
    }
}
impl core::fmt::Debug for Verid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Verid")
            .field("version", &self.version())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Verid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Verid {{ version: {=u32:?} }}", self.version())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Erefs {
    #[doc = "External reference clock selected."]
    EXTERNAL = 0x0,
    #[doc = "Internal crystal oscillator of OSC selected."]
    INTERNAL = 0x01,
}
impl Erefs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Erefs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Erefs {
    #[inline(always)]
    fn from(val: u8) -> Erefs {
        Erefs::from_bits(val)
    }
}
impl From<Erefs> for u8 {
    #[inline(always)]
    fn from(val: Erefs) -> u8 {
        Erefs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fircacc {
    #[doc = "FIRC is not enabled or clock is not accurate."]
    NOT_ENABLED_OR_NOT_VALID = 0x0,
    #[doc = "FIRC is enabled and output clock is accurate after some preparation time which is obtained by counting FRO_HF clock."]
    ENABLED_AND_VALID = 0x01,
}
impl Fircacc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fircacc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fircacc {
    #[inline(always)]
    fn from(val: u8) -> Fircacc {
        Fircacc::from_bits(val)
    }
}
impl From<Fircacc> for u8 {
    #[inline(always)]
    fn from(val: Fircacc) -> u8 {
        Fircacc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FircaccIe {
    #[doc = "FIRCACC interrupt is not enabled."]
    FIRCACCNOT = 0x0,
    #[doc = "FIRCACC interrupt is enabled."]
    FIRCACCYES = 0x01,
}
impl FircaccIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FircaccIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FircaccIe {
    #[inline(always)]
    fn from(val: u8) -> FircaccIe {
        FircaccIe::from_bits(val)
    }
}
impl From<FircaccIe> for u8 {
    #[inline(always)]
    fn from(val: FircaccIe) -> u8 {
        FircaccIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FirccsrLk {
    #[doc = "Control Status Register can be written."]
    WRITE_ENABLED = 0x0,
    #[doc = "Control Status Register cannot be written."]
    WRITE_DISABLED = 0x01,
}
impl FirccsrLk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FirccsrLk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FirccsrLk {
    #[inline(always)]
    fn from(val: u8) -> FirccsrLk {
        FirccsrLk::from_bits(val)
    }
}
impl From<FirccsrLk> for u8 {
    #[inline(always)]
    fn from(val: FirccsrLk) -> u8 {
        FirccsrLk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fircerr {
    #[doc = "Error not detected with the FIRC trimming."]
    ERROR_NOT_DETECTED = 0x0,
    #[doc = "Error detected with the FIRC trimming."]
    ERROR_DETECTED = 0x01,
}
impl Fircerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fircerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fircerr {
    #[inline(always)]
    fn from(val: u8) -> Fircerr {
        Fircerr::from_bits(val)
    }
}
impl From<Fircerr> for u8 {
    #[inline(always)]
    fn from(val: Fircerr) -> u8 {
        Fircerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FircerrIe {
    #[doc = "FIRCERR interrupt is not enabled."]
    ERROR_NOT_DETECTED = 0x0,
    #[doc = "FIRCERR interrupt is enabled."]
    ERROR_DETECTED = 0x01,
}
impl FircerrIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FircerrIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FircerrIe {
    #[inline(always)]
    fn from(val: u8) -> FircerrIe {
        FircerrIe::from_bits(val)
    }
}
impl From<FircerrIe> for u8 {
    #[inline(always)]
    fn from(val: FircerrIe) -> u8 {
        FircerrIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fircsten {
    #[doc = "FIRC is disabled in Deep Sleep mode."]
    DISABLED_IN_STOP_MODES = 0x0,
    #[doc = "FIRC is enabled in Deep Sleep mode."]
    ENABLED_IN_STOP_MODES = 0x01,
}
impl Fircsten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fircsten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fircsten {
    #[inline(always)]
    fn from(val: u8) -> Fircsten {
        Fircsten::from_bits(val)
    }
}
impl From<Fircsten> for u8 {
    #[inline(always)]
    fn from(val: Fircsten) -> u8 {
        Fircsten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fircvld {
    #[doc = "FIRC is not enabled or clock is not valid."]
    NOT_ENABLED_OR_NOT_VALID = 0x0,
    #[doc = "FIRC is enabled and output clock is valid. The clock is valid after there is an output clock from the FIRC analog."]
    ENABLED_AND_VALID = 0x01,
}
impl Fircvld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fircvld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fircvld {
    #[inline(always)]
    fn from(val: u8) -> Fircvld {
        Fircvld::from_bits(val)
    }
}
impl From<Fircvld> for u8 {
    #[inline(always)]
    fn from(val: Fircvld) -> u8 {
        Fircvld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqSel {
    _RESERVED_0 = 0x0,
    #[doc = "45 MHz FIRC clock selected, divided from 180 MHz."]
    FIRC_48MHZ_192S = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "60 MHz FIRC clock selected."]
    FIRC_64MHZ = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "90 MHz FIRC clock selected."]
    FIRC_96MHZ = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "180 MHz FIRC clock selected."]
    FIRC_192MHZ = 0x07,
}
impl FreqSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqSel {
    #[inline(always)]
    fn from(val: u8) -> FreqSel {
        FreqSel::from_bits(val)
    }
}
impl From<FreqSel> for u8 {
    #[inline(always)]
    fn from(val: FreqSel) -> u8 {
        FreqSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IfrDisable {
    #[doc = "IFR write access to SCG trim registers not disabled. The SCG Trim registers are reprogrammed with the IFR values after any system reset."]
    ENABLED = 0x0,
    #[doc = "IFR write access to SCG trim registers during system reset is blocked."]
    DISABLED = 0x01,
}
impl IfrDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IfrDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IfrDisable {
    #[inline(always)]
    fn from(val: u8) -> IfrDisable {
        IfrDisable::from_bits(val)
    }
}
impl From<IfrDisable> for u8 {
    #[inline(always)]
    fn from(val: IfrDisable) -> u8 {
        IfrDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Range {
    #[doc = "Frequency range select of 8-16 MHz."]
    FREQ_16TO20MHZ = 0x0,
    #[doc = "Frequency range select of 16-25 MHz."]
    LOW_FREQ = 0x01,
    #[doc = "Frequency range select of 25-40 MHz."]
    MEDIUM_FREQ = 0x02,
    #[doc = "Frequency range select of 40-50 MHz."]
    HIGH_FREQ = 0x03,
}
impl Range {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Range {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Range {
    #[inline(always)]
    fn from(val: u8) -> Range {
        Range::from_bits(val)
    }
}
impl From<Range> for u8 {
    #[inline(always)]
    fn from(val: Range) -> u8 {
        Range::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RosccsrLk {
    #[doc = "Control Status Register can be written."]
    WRITE_ENABLED = 0x0,
    #[doc = "Control Status Register cannot be written."]
    WRITE_DISABLED = 0x01,
}
impl RosccsrLk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RosccsrLk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RosccsrLk {
    #[inline(always)]
    fn from(val: u8) -> RosccsrLk {
        RosccsrLk::from_bits(val)
    }
}
impl From<RosccsrLk> for u8 {
    #[inline(always)]
    fn from(val: RosccsrLk) -> u8 {
        RosccsrLk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Roscerr {
    #[doc = "ROSC Clock has not detected an error."]
    DISABLED_OR_NO_ERROR = 0x0,
    #[doc = "ROSC Clock has detected an error."]
    ENABLED_AND_ERROR = 0x01,
}
impl Roscerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Roscerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Roscerr {
    #[inline(always)]
    fn from(val: u8) -> Roscerr {
        Roscerr::from_bits(val)
    }
}
impl From<Roscerr> for u8 {
    #[inline(always)]
    fn from(val: Roscerr) -> u8 {
        Roscerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Roscvld {
    #[doc = "ROSC is not enabled or clock is not valid."]
    DISABLED_OR_NOT_VALID = 0x0,
    #[doc = "ROSC is enabled and output clock is valid."]
    ENABLED_AND_VALID = 0x01,
}
impl Roscvld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Roscvld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Roscvld {
    #[inline(always)]
    fn from(val: u8) -> Roscvld {
        Roscvld::from_bits(val)
    }
}
impl From<Roscvld> for u8 {
    #[inline(always)]
    fn from(val: Roscvld) -> u8 {
        Roscvld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scs {
    _RESERVED_0 = 0x0,
    #[doc = "SOSC."]
    SOSC = 0x01,
    #[doc = "SIRC."]
    SIRC = 0x02,
    #[doc = "FIRC."]
    FIRC = 0x03,
    #[doc = "ROSC."]
    ROSC = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "SPLL."]
    SPLL = 0x06,
    _RESERVED_7 = 0x07,
}
impl Scs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scs {
    #[inline(always)]
    fn from(val: u8) -> Scs {
        Scs::from_bits(val)
    }
}
impl From<Scs> for u8 {
    #[inline(always)]
    fn from(val: Scs) -> u8 {
        Scs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SirccsrLk {
    #[doc = "Control Status Register can be written."]
    WRITE_ENABLED = 0x0,
    #[doc = "Control Status Register cannot be written."]
    WRITE_DISABLED = 0x01,
}
impl SirccsrLk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SirccsrLk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SirccsrLk {
    #[inline(always)]
    fn from(val: u8) -> SirccsrLk {
        SirccsrLk::from_bits(val)
    }
}
impl From<SirccsrLk> for u8 {
    #[inline(always)]
    fn from(val: SirccsrLk) -> u8 {
        SirccsrLk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sircerr {
    #[doc = "Error not detected with the SIRC trimming."]
    ERROR_NOT_DETECTED = 0x0,
    #[doc = "Error detected with the SIRC trimming."]
    ERROR_DETECTED = 0x01,
}
impl Sircerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sircerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sircerr {
    #[inline(always)]
    fn from(val: u8) -> Sircerr {
        Sircerr::from_bits(val)
    }
}
impl From<Sircerr> for u8 {
    #[inline(always)]
    fn from(val: Sircerr) -> u8 {
        Sircerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SircerrIe {
    #[doc = "SIRCERR interrupt is not enabled."]
    ERROR_NOT_DETECTED = 0x0,
    #[doc = "SIRCERR interrupt is enabled."]
    ERROR_DETECTED = 0x01,
}
impl SircerrIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SircerrIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SircerrIe {
    #[inline(always)]
    fn from(val: u8) -> SircerrIe {
        SircerrIe::from_bits(val)
    }
}
impl From<SircerrIe> for u8 {
    #[inline(always)]
    fn from(val: SircerrIe) -> u8 {
        SircerrIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sircvld {
    #[doc = "SIRC is not enabled or clock is not valid."]
    DISABLED_OR_NOT_VALID = 0x0,
    #[doc = "SIRC is enabled and output clock is valid."]
    ENABLED_AND_VALID = 0x01,
}
impl Sircvld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sircvld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sircvld {
    #[inline(always)]
    fn from(val: u8) -> Sircvld {
        Sircvld::from_bits(val)
    }
}
impl From<Sircvld> for u8 {
    #[inline(always)]
    fn from(val: Sircvld) -> u8 {
        Sircvld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sosccmre {
    #[doc = "Clock monitor generates an interrupt when an error is detected."]
    GENERATE_INTERRUPT = 0x0,
    #[doc = "Clock monitor generates a reset when an error is detected."]
    GENERATE_RESET = 0x01,
}
impl Sosccmre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sosccmre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sosccmre {
    #[inline(always)]
    fn from(val: u8) -> Sosccmre {
        Sosccmre::from_bits(val)
    }
}
impl From<Sosccmre> for u8 {
    #[inline(always)]
    fn from(val: Sosccmre) -> u8 {
        Sosccmre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SosccsrLk {
    #[doc = "This Control Status Register can be written."]
    WRITE_ENABLED = 0x0,
    #[doc = "This Control Status Register cannot be written."]
    WRITE_DISABLED = 0x01,
}
impl SosccsrLk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SosccsrLk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SosccsrLk {
    #[inline(always)]
    fn from(val: u8) -> SosccsrLk {
        SosccsrLk::from_bits(val)
    }
}
impl From<SosccsrLk> for u8 {
    #[inline(always)]
    fn from(val: SosccsrLk) -> u8 {
        SosccsrLk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Soscerr {
    #[doc = "SOSC Clock Monitor is disabled or has not detected an error."]
    DISABLED_OR_NO_ERROR = 0x0,
    #[doc = "SOSC Clock Monitor is enabled and detected an error."]
    ENABLED_AND_ERROR = 0x01,
}
impl Soscerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Soscerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Soscerr {
    #[inline(always)]
    fn from(val: u8) -> Soscerr {
        Soscerr::from_bits(val)
    }
}
impl From<Soscerr> for u8 {
    #[inline(always)]
    fn from(val: Soscerr) -> u8 {
        Soscerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Source {
    #[doc = "SOSC."]
    SOSC = 0x0,
    #[doc = "FIRC 45 MHz clock. FIRC_SCLK_PERIPH_EN needs to be set to use FIRC 45 MHz clock."]
    FIRC = 0x01,
    #[doc = "ROSC."]
    ROSC = 0x02,
    #[doc = "SIRC 12 MHz clock."]
    SIRC = 0x03,
}
impl Source {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Source {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Source {
    #[inline(always)]
    fn from(val: u8) -> Source {
        Source::from_bits(val)
    }
}
impl From<Source> for u8 {
    #[inline(always)]
    fn from(val: Source) -> u8 {
        Source::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpllLock {
    #[doc = "SPLL is not powered on or not locked."]
    DISABLED_OR_NOT_VALID = 0x0,
    #[doc = "SPLL is locked."]
    ENABLED_AND_VALID = 0x01,
}
impl SpllLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpllLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpllLock {
    #[inline(always)]
    fn from(val: u8) -> SpllLock {
        SpllLock::from_bits(val)
    }
}
impl From<SpllLock> for u8 {
    #[inline(always)]
    fn from(val: SpllLock) -> u8 {
        SpllLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spllcmre {
    #[doc = "Clock monitor generates an interrupt when an error is detected."]
    GENERATE_INTERRUPT = 0x0,
    #[doc = "Clock monitor generates a reset when an error is detected."]
    GENERATE_RESET = 0x01,
}
impl Spllcmre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spllcmre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spllcmre {
    #[inline(always)]
    fn from(val: u8) -> Spllcmre {
        Spllcmre::from_bits(val)
    }
}
impl From<Spllcmre> for u8 {
    #[inline(always)]
    fn from(val: Spllcmre) -> u8 {
        Spllcmre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpllcsrLk {
    #[doc = "Control Status Register can be written."]
    WRITE_ENABLED = 0x0,
    #[doc = "Control Status Register cannot be written."]
    WRITE_DISABLED = 0x01,
}
impl SpllcsrLk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpllcsrLk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpllcsrLk {
    #[inline(always)]
    fn from(val: u8) -> SpllcsrLk {
        SpllcsrLk::from_bits(val)
    }
}
impl From<SpllcsrLk> for u8 {
    #[inline(always)]
    fn from(val: SpllcsrLk) -> u8 {
        SpllcsrLk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spllerr {
    #[doc = "SPLL Clock Monitor is disabled or has not detected an error."]
    DISABLED_OR_NO_ERROR = 0x0,
    #[doc = "SPLL Clock Monitor is enabled and detected an error."]
    ENABLED_AND_ERROR = 0x01,
}
impl Spllerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spllerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spllerr {
    #[inline(always)]
    fn from(val: u8) -> Spllerr {
        Spllerr::from_bits(val)
    }
}
impl From<Spllerr> for u8 {
    #[inline(always)]
    fn from(val: Spllerr) -> u8 {
        Spllerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spllsten {
    #[doc = "SPLL is disabled in Deep Sleep mode."]
    DISABLED_IN_STOP = 0x0,
    #[doc = "SPLL is enabled in Deep Sleep mode."]
    ENABLED_IN_STOP = 0x01,
}
impl Spllsten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spllsten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spllsten {
    #[inline(always)]
    fn from(val: u8) -> Spllsten {
        Spllsten::from_bits(val)
    }
}
impl From<Spllsten> for u8 {
    #[inline(always)]
    fn from(val: Spllsten) -> u8 {
        Spllsten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrimLock {
    #[doc = "SIRC auto trim not locked to target frequency range."]
    SIRC_NOT_LOCKED = 0x0,
    #[doc = "SIRC auto trim locked to target frequency range."]
    SIRC_LOCKED = 0x01,
}
impl TrimLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrimLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrimLock {
    #[inline(always)]
    fn from(val: u8) -> TrimLock {
        TrimLock::from_bits(val)
    }
}
impl From<TrimLock> for u8 {
    #[inline(always)]
    fn from(val: TrimLock) -> u8 {
        TrimLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrimUnlock {
    #[doc = "SCG Trim Registers locked and not writable."]
    LOCKED = 0x0,
    #[doc = "SCG Trim registers unlocked and writable."]
    NOT_LOCKED = 0x01,
}
impl TrimUnlock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrimUnlock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrimUnlock {
    #[inline(always)]
    fn from(val: u8) -> TrimUnlock {
        TrimUnlock::from_bits(val)
    }
}
impl From<TrimUnlock> for u8 {
    #[inline(always)]
    fn from(val: TrimUnlock) -> u8 {
        TrimUnlock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trimsrc {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "SOSC. This option requires that SOSC be divided using the TRIMDIV field to get a frequency of 1 MHz."]
    SOSC = 0x02,
    _RESERVED_3 = 0x03,
}
impl Trimsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trimsrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trimsrc {
    #[inline(always)]
    fn from(val: u8) -> Trimsrc {
        Trimsrc::from_bits(val)
    }
}
impl From<Trimsrc> for u8 {
    #[inline(always)]
    fn from(val: Trimsrc) -> u8 {
        Trimsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VoutSel {
    #[doc = "VOUT = 1V."]
    VOUT_1V_1 = 0x0,
    #[doc = "VOUT = 1V."]
    VOUT_1V_2 = 0x01,
    #[doc = "VOUT = 1V."]
    VOUT_1V_3 = 0x02,
    #[doc = "VOUT = 1.05V."]
    VOUT_105V = 0x03,
    #[doc = "VOUT = 1.1V."]
    VOUT_11V = 0x04,
    #[doc = "VOUT = 1.15V."]
    VOUT_115V = 0x05,
    #[doc = "VOUT = 1.2V."]
    VOUT_12V = 0x06,
    #[doc = "VOUT = 1.25V."]
    VOUT_125V = 0x07,
}
impl VoutSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VoutSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VoutSel {
    #[inline(always)]
    fn from(val: u8) -> VoutSel {
        VoutSel::from_bits(val)
    }
}
impl From<VoutSel> for u8 {
    #[inline(always)]
    fn from(val: VoutSel) -> u8 {
        VoutSel::to_bits(val)
    }
}
