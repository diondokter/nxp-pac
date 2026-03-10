#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6a8c2aa 2026-01-27))"]
#[doc = "PORT."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port1 {
    ptr: *mut u8,
}
unsafe impl Send for Port1 {}
unsafe impl Sync for Port1 {}
impl Port1 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID."]
    #[inline(always)]
    pub const fn verid(self) -> crate::common::Reg<Verid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Global Pin Control Low."]
    #[inline(always)]
    pub const fn gpclr(self) -> crate::common::Reg<Gpclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Global Pin Control High."]
    #[inline(always)]
    pub const fn gpchr(self) -> crate::common::Reg<Gpchr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Configuration."]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Calibration 0."]
    #[inline(always)]
    pub const fn calib0(self) -> crate::common::Reg<Calib0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Calibration 1."]
    #[inline(always)]
    pub const fn calib1(self) -> crate::common::Reg<Calib1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Pin Control 0."]
    #[inline(always)]
    pub const fn pcr0(self) -> crate::common::Reg<Pcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Pin Control 1."]
    #[inline(always)]
    pub const fn pcr1(self) -> crate::common::Reg<Pcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Pin Control 2."]
    #[inline(always)]
    pub const fn pcr2(self) -> crate::common::Reg<Pcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Pin Control 3."]
    #[inline(always)]
    pub const fn pcr3(self) -> crate::common::Reg<Pcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Pin Control 4."]
    #[inline(always)]
    pub const fn pcr4(self) -> crate::common::Reg<Pcr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Pin Control 5."]
    #[inline(always)]
    pub const fn pcr5(self) -> crate::common::Reg<Pcr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Pin Control 6."]
    #[inline(always)]
    pub const fn pcr6(self) -> crate::common::Reg<Pcr6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Pin Control 7."]
    #[inline(always)]
    pub const fn pcr7(self) -> crate::common::Reg<Pcr7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Pin Control 8."]
    #[inline(always)]
    pub const fn pcr8(self) -> crate::common::Reg<Pcr8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Pin Control 9."]
    #[inline(always)]
    pub const fn pcr9(self) -> crate::common::Reg<Pcr9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Pin Control 10."]
    #[inline(always)]
    pub const fn pcr10(self) -> crate::common::Reg<Pcr10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Pin Control 11."]
    #[inline(always)]
    pub const fn pcr11(self) -> crate::common::Reg<Pcr11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "Pin Control 12."]
    #[inline(always)]
    pub const fn pcr12(self) -> crate::common::Reg<Pcr12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Pin Control 13."]
    #[inline(always)]
    pub const fn pcr13(self) -> crate::common::Reg<Pcr13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Pin Control 14."]
    #[inline(always)]
    pub const fn pcr14(self) -> crate::common::Reg<Pcr14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Pin Control 15."]
    #[inline(always)]
    pub const fn pcr15(self) -> crate::common::Reg<Pcr15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "Pin Control 16."]
    #[inline(always)]
    pub const fn pcr16(self) -> crate::common::Reg<Pcr16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Pin Control 17."]
    #[inline(always)]
    pub const fn pcr17(self) -> crate::common::Reg<Pcr17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Pin Control 18."]
    #[inline(always)]
    pub const fn pcr18(self) -> crate::common::Reg<Pcr18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Pin Control 19."]
    #[inline(always)]
    pub const fn pcr19(self) -> crate::common::Reg<Pcr19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Pin Control 20."]
    #[inline(always)]
    pub const fn pcr20(self) -> crate::common::Reg<Pcr20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Pin Control 21."]
    #[inline(always)]
    pub const fn pcr21(self) -> crate::common::Reg<Pcr21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Pin Control 22."]
    #[inline(always)]
    pub const fn pcr22(self) -> crate::common::Reg<Pcr22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Pin Control 23."]
    #[inline(always)]
    pub const fn pcr23(self) -> crate::common::Reg<Pcr23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Pin Control 24."]
    #[inline(always)]
    pub const fn pcr24(self) -> crate::common::Reg<Pcr24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Pin Control 25."]
    #[inline(always)]
    pub const fn pcr25(self) -> crate::common::Reg<Pcr25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Pin Control 26."]
    #[inline(always)]
    pub const fn pcr26(self) -> crate::common::Reg<Pcr26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Pin Control 27."]
    #[inline(always)]
    pub const fn pcr27(self) -> crate::common::Reg<Pcr27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Pin Control 28."]
    #[inline(always)]
    pub const fn pcr28(self) -> crate::common::Reg<Pcr28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Pin Control 29."]
    #[inline(always)]
    pub const fn pcr29(self) -> crate::common::Reg<Pcr29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Pin Control 30."]
    #[inline(always)]
    pub const fn pcr30(self) -> crate::common::Reg<Pcr30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Pin Control 31."]
    #[inline(always)]
    pub const fn pcr31(self) -> crate::common::Reg<Pcr31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
}
#[doc = "Calibration 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Calib0(pub u32);
impl Calib0 {
    #[doc = "Calibration of NMOS Output Driver."]
    #[must_use]
    #[inline(always)]
    pub const fn ncal(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Calibration of NMOS Output Driver."]
    #[inline(always)]
    pub const fn set_ncal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Calibration of PMOS Output Driver."]
    #[must_use]
    #[inline(always)]
    pub const fn pcal(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Calibration of PMOS Output Driver."]
    #[inline(always)]
    pub const fn set_pcal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for Calib0 {
    #[inline(always)]
    fn default() -> Calib0 {
        Calib0(0)
    }
}
impl core::fmt::Debug for Calib0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Calib0")
            .field("ncal", &self.ncal())
            .field("pcal", &self.pcal())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Calib0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Calib0 {{ ncal: {=u8:?}, pcal: {=u8:?} }}",
            self.ncal(),
            self.pcal()
        )
    }
}
#[doc = "Calibration 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Calib1(pub u32);
impl Calib1 {
    #[doc = "Calibration of NMOS Output Driver."]
    #[must_use]
    #[inline(always)]
    pub const fn ncal(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Calibration of NMOS Output Driver."]
    #[inline(always)]
    pub const fn set_ncal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Calibration of PMOS Output Driver."]
    #[must_use]
    #[inline(always)]
    pub const fn pcal(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Calibration of PMOS Output Driver."]
    #[inline(always)]
    pub const fn set_pcal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for Calib1 {
    #[inline(always)]
    fn default() -> Calib1 {
        Calib1(0)
    }
}
impl core::fmt::Debug for Calib1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Calib1")
            .field("ncal", &self.ncal())
            .field("pcal", &self.pcal())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Calib1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Calib1 {{ ncal: {=u8:?}, pcal: {=u8:?} }}",
            self.ncal(),
            self.pcal()
        )
    }
}
#[doc = "Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "Port Voltage Range."]
    #[must_use]
    #[inline(always)]
    pub const fn range(&self) -> Range {
        let val = (self.0 >> 0usize) & 0x01;
        Range::from_bits(val as u8)
    }
    #[doc = "Port Voltage Range."]
    #[inline(always)]
    pub const fn set_range(&mut self, val: Range) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
impl core::fmt::Debug for Config {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Config")
            .field("range", &self.range())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Config {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Config {{ range: {:?} }}", self.range())
    }
}
#[doc = "Global Pin Control High."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpchr(pub u32);
impl Gpchr {
    #[doc = "Global Pin Write Data."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Global Pin Write Data."]
    #[inline(always)]
    pub const fn set_gpwd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe16(&self) -> Gpwe16 {
        let val = (self.0 >> 16usize) & 0x01;
        Gpwe16::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe16(&mut self, val: Gpwe16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe17(&self) -> Gpwe17 {
        let val = (self.0 >> 17usize) & 0x01;
        Gpwe17::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe17(&mut self, val: Gpwe17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe18(&self) -> Gpwe18 {
        let val = (self.0 >> 18usize) & 0x01;
        Gpwe18::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe18(&mut self, val: Gpwe18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe19(&self) -> Gpwe19 {
        let val = (self.0 >> 19usize) & 0x01;
        Gpwe19::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe19(&mut self, val: Gpwe19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe20(&self) -> Gpwe20 {
        let val = (self.0 >> 20usize) & 0x01;
        Gpwe20::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe20(&mut self, val: Gpwe20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe21(&self) -> Gpwe21 {
        let val = (self.0 >> 21usize) & 0x01;
        Gpwe21::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe21(&mut self, val: Gpwe21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe22(&self) -> Gpwe22 {
        let val = (self.0 >> 22usize) & 0x01;
        Gpwe22::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe22(&mut self, val: Gpwe22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe23(&self) -> Gpwe23 {
        let val = (self.0 >> 23usize) & 0x01;
        Gpwe23::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe23(&mut self, val: Gpwe23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe24(&self) -> Gpwe24 {
        let val = (self.0 >> 24usize) & 0x01;
        Gpwe24::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe24(&mut self, val: Gpwe24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe25(&self) -> Gpwe25 {
        let val = (self.0 >> 25usize) & 0x01;
        Gpwe25::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe25(&mut self, val: Gpwe25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe26(&self) -> Gpwe26 {
        let val = (self.0 >> 26usize) & 0x01;
        Gpwe26::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe26(&mut self, val: Gpwe26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe27(&self) -> Gpwe27 {
        let val = (self.0 >> 27usize) & 0x01;
        Gpwe27::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe27(&mut self, val: Gpwe27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe28(&self) -> Gpwe28 {
        let val = (self.0 >> 28usize) & 0x01;
        Gpwe28::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe28(&mut self, val: Gpwe28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe29(&self) -> Gpwe29 {
        let val = (self.0 >> 29usize) & 0x01;
        Gpwe29::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe29(&mut self, val: Gpwe29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe30(&self) -> Gpwe30 {
        let val = (self.0 >> 30usize) & 0x01;
        Gpwe30::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe30(&mut self, val: Gpwe30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe31(&self) -> Gpwe31 {
        let val = (self.0 >> 31usize) & 0x01;
        Gpwe31::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe31(&mut self, val: Gpwe31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Gpchr {
    #[inline(always)]
    fn default() -> Gpchr {
        Gpchr(0)
    }
}
impl core::fmt::Debug for Gpchr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpchr")
            .field("gpwd", &self.gpwd())
            .field("gpwe16", &self.gpwe16())
            .field("gpwe17", &self.gpwe17())
            .field("gpwe18", &self.gpwe18())
            .field("gpwe19", &self.gpwe19())
            .field("gpwe20", &self.gpwe20())
            .field("gpwe21", &self.gpwe21())
            .field("gpwe22", &self.gpwe22())
            .field("gpwe23", &self.gpwe23())
            .field("gpwe24", &self.gpwe24())
            .field("gpwe25", &self.gpwe25())
            .field("gpwe26", &self.gpwe26())
            .field("gpwe27", &self.gpwe27())
            .field("gpwe28", &self.gpwe28())
            .field("gpwe29", &self.gpwe29())
            .field("gpwe30", &self.gpwe30())
            .field("gpwe31", &self.gpwe31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpchr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpchr {{ gpwd: {=u16:?}, gpwe16: {:?}, gpwe17: {:?}, gpwe18: {:?}, gpwe19: {:?}, gpwe20: {:?}, gpwe21: {:?}, gpwe22: {:?}, gpwe23: {:?}, gpwe24: {:?}, gpwe25: {:?}, gpwe26: {:?}, gpwe27: {:?}, gpwe28: {:?}, gpwe29: {:?}, gpwe30: {:?}, gpwe31: {:?} }}",
            self.gpwd(),
            self.gpwe16(),
            self.gpwe17(),
            self.gpwe18(),
            self.gpwe19(),
            self.gpwe20(),
            self.gpwe21(),
            self.gpwe22(),
            self.gpwe23(),
            self.gpwe24(),
            self.gpwe25(),
            self.gpwe26(),
            self.gpwe27(),
            self.gpwe28(),
            self.gpwe29(),
            self.gpwe30(),
            self.gpwe31()
        )
    }
}
#[doc = "Global Pin Control Low."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpclr(pub u32);
impl Gpclr {
    #[doc = "Global Pin Write Data."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Global Pin Write Data."]
    #[inline(always)]
    pub const fn set_gpwd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe0(&self) -> Gpwe0 {
        let val = (self.0 >> 16usize) & 0x01;
        Gpwe0::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe0(&mut self, val: Gpwe0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe1(&self) -> Gpwe1 {
        let val = (self.0 >> 17usize) & 0x01;
        Gpwe1::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe1(&mut self, val: Gpwe1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe2(&self) -> Gpwe2 {
        let val = (self.0 >> 18usize) & 0x01;
        Gpwe2::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe2(&mut self, val: Gpwe2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe3(&self) -> Gpwe3 {
        let val = (self.0 >> 19usize) & 0x01;
        Gpwe3::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe3(&mut self, val: Gpwe3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe4(&self) -> Gpwe4 {
        let val = (self.0 >> 20usize) & 0x01;
        Gpwe4::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe4(&mut self, val: Gpwe4) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe5(&self) -> Gpwe5 {
        let val = (self.0 >> 21usize) & 0x01;
        Gpwe5::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe5(&mut self, val: Gpwe5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe6(&self) -> Gpwe6 {
        let val = (self.0 >> 22usize) & 0x01;
        Gpwe6::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe6(&mut self, val: Gpwe6) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe7(&self) -> Gpwe7 {
        let val = (self.0 >> 23usize) & 0x01;
        Gpwe7::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe7(&mut self, val: Gpwe7) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe8(&self) -> Gpwe8 {
        let val = (self.0 >> 24usize) & 0x01;
        Gpwe8::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe8(&mut self, val: Gpwe8) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe9(&self) -> Gpwe9 {
        let val = (self.0 >> 25usize) & 0x01;
        Gpwe9::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe9(&mut self, val: Gpwe9) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe10(&self) -> Gpwe10 {
        let val = (self.0 >> 26usize) & 0x01;
        Gpwe10::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe10(&mut self, val: Gpwe10) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe11(&self) -> Gpwe11 {
        let val = (self.0 >> 27usize) & 0x01;
        Gpwe11::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe11(&mut self, val: Gpwe11) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe12(&self) -> Gpwe12 {
        let val = (self.0 >> 28usize) & 0x01;
        Gpwe12::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe12(&mut self, val: Gpwe12) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe13(&self) -> Gpwe13 {
        let val = (self.0 >> 29usize) & 0x01;
        Gpwe13::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe13(&mut self, val: Gpwe13) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe14(&self) -> Gpwe14 {
        let val = (self.0 >> 30usize) & 0x01;
        Gpwe14::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe14(&mut self, val: Gpwe14) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Global Pin Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe15(&self) -> Gpwe15 {
        let val = (self.0 >> 31usize) & 0x01;
        Gpwe15::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable."]
    #[inline(always)]
    pub const fn set_gpwe15(&mut self, val: Gpwe15) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Gpclr {
    #[inline(always)]
    fn default() -> Gpclr {
        Gpclr(0)
    }
}
impl core::fmt::Debug for Gpclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpclr")
            .field("gpwd", &self.gpwd())
            .field("gpwe0", &self.gpwe0())
            .field("gpwe1", &self.gpwe1())
            .field("gpwe2", &self.gpwe2())
            .field("gpwe3", &self.gpwe3())
            .field("gpwe4", &self.gpwe4())
            .field("gpwe5", &self.gpwe5())
            .field("gpwe6", &self.gpwe6())
            .field("gpwe7", &self.gpwe7())
            .field("gpwe8", &self.gpwe8())
            .field("gpwe9", &self.gpwe9())
            .field("gpwe10", &self.gpwe10())
            .field("gpwe11", &self.gpwe11())
            .field("gpwe12", &self.gpwe12())
            .field("gpwe13", &self.gpwe13())
            .field("gpwe14", &self.gpwe14())
            .field("gpwe15", &self.gpwe15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpclr {{ gpwd: {=u16:?}, gpwe0: {:?}, gpwe1: {:?}, gpwe2: {:?}, gpwe3: {:?}, gpwe4: {:?}, gpwe5: {:?}, gpwe6: {:?}, gpwe7: {:?}, gpwe8: {:?}, gpwe9: {:?}, gpwe10: {:?}, gpwe11: {:?}, gpwe12: {:?}, gpwe13: {:?}, gpwe14: {:?}, gpwe15: {:?} }}",
            self.gpwd(),
            self.gpwe0(),
            self.gpwe1(),
            self.gpwe2(),
            self.gpwe3(),
            self.gpwe4(),
            self.gpwe5(),
            self.gpwe6(),
            self.gpwe7(),
            self.gpwe8(),
            self.gpwe9(),
            self.gpwe10(),
            self.gpwe11(),
            self.gpwe12(),
            self.gpwe13(),
            self.gpwe14(),
            self.gpwe15()
        )
    }
}
#[doc = "Pin Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr0(pub u32);
impl Pcr0 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr0Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr0Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr0Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr0Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr0Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr0Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr0Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr0Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr0Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Passive Filter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pfe(&self) -> Pcr0Pfe {
        let val = (self.0 >> 4usize) & 0x01;
        Pcr0Pfe::from_bits(val as u8)
    }
    #[doc = "Passive Filter Enable."]
    #[inline(always)]
    pub const fn set_pfe(&mut self, val: Pcr0Pfe) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr0Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr0Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr0Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr0Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr0Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr0Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Pcr0Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        Pcr0Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Pcr0Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr0Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr0Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr0Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr0Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr0Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr0Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr0Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr0Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr0Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr0 {
    #[inline(always)]
    fn default() -> Pcr0 {
        Pcr0(0)
    }
}
impl core::fmt::Debug for Pcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr0")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("pfe", &self.pfe())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr0 {{ ps: {:?}, pe: {:?}, sre: {:?}, pfe: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.pfe(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr1(pub u32);
impl Pcr1 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr1Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr1Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr1Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr1Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr1Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr1Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr1Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr1Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr1Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Passive Filter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pfe(&self) -> Pcr1Pfe {
        let val = (self.0 >> 4usize) & 0x01;
        Pcr1Pfe::from_bits(val as u8)
    }
    #[doc = "Passive Filter Enable."]
    #[inline(always)]
    pub const fn set_pfe(&mut self, val: Pcr1Pfe) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr1Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr1Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr1Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr1Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr1Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr1Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Pcr1Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        Pcr1Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Pcr1Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr1Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr1Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr1Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr1Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr1Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr1Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr1Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr1Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr1Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr1 {
    #[inline(always)]
    fn default() -> Pcr1 {
        Pcr1(0)
    }
}
impl core::fmt::Debug for Pcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr1")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("pfe", &self.pfe())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr1 {{ ps: {:?}, pe: {:?}, sre: {:?}, pfe: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.pfe(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 10."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr10(pub u32);
impl Pcr10 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr10Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr10Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr10Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr10Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr10Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr10Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr10Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr10Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr10Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr10Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr10Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr10Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr10Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr10Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr10Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Pcr10Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        Pcr10Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Pcr10Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr10Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr10Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr10Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr10Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr10Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr10Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr10Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr10Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr10Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr10 {
    #[inline(always)]
    fn default() -> Pcr10 {
        Pcr10(0)
    }
}
impl core::fmt::Debug for Pcr10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr10")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr10 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 11."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr11(pub u32);
impl Pcr11 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr11Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr11Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr11Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr11Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr11Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr11Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr11Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr11Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr11Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr11Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr11Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr11Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr11Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr11Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr11Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Pcr11Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        Pcr11Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Pcr11Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr11Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr11Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr11Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr11Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr11Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr11Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr11Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr11Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr11Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr11 {
    #[inline(always)]
    fn default() -> Pcr11 {
        Pcr11(0)
    }
}
impl core::fmt::Debug for Pcr11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr11")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr11 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 12."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr12(pub u32);
impl Pcr12 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr12Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr12Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr12Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr12Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr12Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr12Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr12Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr12Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr12Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr12Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr12Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr12Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr12Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr12Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr12Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Pcr12Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        Pcr12Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Pcr12Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr12Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr12Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr12Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr12Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr12Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr12Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr12Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr12Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr12Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr12 {
    #[inline(always)]
    fn default() -> Pcr12 {
        Pcr12(0)
    }
}
impl core::fmt::Debug for Pcr12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr12")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr12 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 13."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr13(pub u32);
impl Pcr13 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr13Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr13Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr13Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr13Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr13Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr13Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr13Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr13Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr13Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr13Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr13Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr13Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr13Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr13Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr13Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Pcr13Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        Pcr13Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Pcr13Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr13Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr13Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr13Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr13Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr13Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr13Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr13Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr13Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr13Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr13 {
    #[inline(always)]
    fn default() -> Pcr13 {
        Pcr13(0)
    }
}
impl core::fmt::Debug for Pcr13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr13")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr13 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 14."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr14(pub u32);
impl Pcr14 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr14Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr14Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr14Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr14Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr14Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr14Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr14Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr14Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr14Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr14Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr14Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr14Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr14Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr14Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr14Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Pcr14Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        Pcr14Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Pcr14Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr14Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr14Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr14Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr14Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr14Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr14Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr14Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr14Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr14Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr14 {
    #[inline(always)]
    fn default() -> Pcr14 {
        Pcr14(0)
    }
}
impl core::fmt::Debug for Pcr14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr14")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr14 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 15."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr15(pub u32);
impl Pcr15 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr15Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr15Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr15Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr15Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr15Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr15Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr15Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr15Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr15Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr15Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr15Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr15Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr15Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr15Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr15Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Pcr15Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        Pcr15Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Pcr15Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr15Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr15Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr15Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr15Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr15Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr15Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr15Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr15Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr15Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr15 {
    #[inline(always)]
    fn default() -> Pcr15 {
        Pcr15(0)
    }
}
impl core::fmt::Debug for Pcr15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr15")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr15 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 16."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr16(pub u32);
impl Pcr16 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr16Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr16Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr16Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr16Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr16Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr16Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr16Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr16Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr16Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr16Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr16Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr16Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr16Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr16Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr16Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Pcr16Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        Pcr16Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Pcr16Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr16Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr16Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr16Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr16Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr16Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr16Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr16Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr16Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr16Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr16 {
    #[inline(always)]
    fn default() -> Pcr16 {
        Pcr16(0)
    }
}
impl core::fmt::Debug for Pcr16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr16")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr16 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 17."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr17(pub u32);
impl Pcr17 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr17Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr17Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr17Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr17Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr17Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr17Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr17Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr17Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr17Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr17Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr17Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr17Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr17Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr17Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr17Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Pcr17Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        Pcr17Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Pcr17Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr17Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr17Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr17Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr17Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr17Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr17Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr17Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr17Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr17Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr17 {
    #[inline(always)]
    fn default() -> Pcr17 {
        Pcr17(0)
    }
}
impl core::fmt::Debug for Pcr17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr17")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr17 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr17 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 18."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr18(pub u32);
impl Pcr18 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr18Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr18Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr18Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr18Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr18Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr18Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr18Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr18Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr18Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr18Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr18Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr18Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr18Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr18Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr18Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Pcr18Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        Pcr18Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Pcr18Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr18Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr18Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr18Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr18Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr18Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr18Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr18Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr18Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr18Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr18 {
    #[inline(always)]
    fn default() -> Pcr18 {
        Pcr18(0)
    }
}
impl core::fmt::Debug for Pcr18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr18")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr18 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr18 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 19."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr19(pub u32);
impl Pcr19 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr19Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr19Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr19Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr19Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr19Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr19Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr19Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr19Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr19Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr19Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr19Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr19Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr19Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr19Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr19Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Pcr19Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        Pcr19Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Pcr19Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr19Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr19Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr19Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr19Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr19Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr19Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr19Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr19Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr19Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr19 {
    #[inline(always)]
    fn default() -> Pcr19 {
        Pcr19(0)
    }
}
impl core::fmt::Debug for Pcr19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr19")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr19 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr19 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr2(pub u32);
impl Pcr2 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr2Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr2Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr2Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr2Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr2Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr2Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr2Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr2Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr2Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr2Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr2Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr2Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr2Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr2Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr2Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Pcr2Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        Pcr2Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Pcr2Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr2Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr2Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr2Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr2Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr2Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr2Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr2Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr2Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr2Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr2 {
    #[inline(always)]
    fn default() -> Pcr2 {
        Pcr2(0)
    }
}
impl core::fmt::Debug for Pcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr2")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr2 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 20."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr20(pub u32);
impl Pcr20 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr20Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr20Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr20Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr20Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr20Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr20Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr20Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr20Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr20Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr20Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr20Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr20Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr20Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr20Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr20Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr20Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr20Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr20Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr20Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr20Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr20Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr20Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr20Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr20Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr20 {
    #[inline(always)]
    fn default() -> Pcr20 {
        Pcr20(0)
    }
}
impl core::fmt::Debug for Pcr20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr20")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr20 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 21."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr21(pub u32);
impl Pcr21 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr21Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr21Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr21Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr21Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr21Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr21Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr21Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr21Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr21Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr21Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr21Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr21Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr21Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr21Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr21Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr21Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr21Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr21Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr21Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr21Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr21Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr21Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr21Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr21Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr21 {
    #[inline(always)]
    fn default() -> Pcr21 {
        Pcr21(0)
    }
}
impl core::fmt::Debug for Pcr21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr21")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr21 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr21 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 22."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr22(pub u32);
impl Pcr22 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr22Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr22Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr22Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr22Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr22Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr22Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr22Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr22Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr22Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr22Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr22Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr22Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr22Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr22Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr22Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr22Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr22Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr22Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr22Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr22Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr22Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr22Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr22Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr22Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr22 {
    #[inline(always)]
    fn default() -> Pcr22 {
        Pcr22(0)
    }
}
impl core::fmt::Debug for Pcr22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr22")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr22 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr22 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 23."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr23(pub u32);
impl Pcr23 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr23Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr23Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr23Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr23Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr23Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr23Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr23Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr23Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr23Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr23Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr23Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr23Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr23Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr23Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr23Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr23Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr23Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr23Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr23Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr23Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr23Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr23Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr23Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr23Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr23 {
    #[inline(always)]
    fn default() -> Pcr23 {
        Pcr23(0)
    }
}
impl core::fmt::Debug for Pcr23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr23")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr23 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 24."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr24(pub u32);
impl Pcr24 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr24Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr24Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr24Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr24Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr24Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr24Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr24Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr24Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr24Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr24Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr24Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr24Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr24Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr24Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr24Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr24Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr24Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr24Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr24Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr24Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr24Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr24Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr24Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr24Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr24 {
    #[inline(always)]
    fn default() -> Pcr24 {
        Pcr24(0)
    }
}
impl core::fmt::Debug for Pcr24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr24")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr24 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr24 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 25."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr25(pub u32);
impl Pcr25 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr25Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr25Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr25Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr25Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr25Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr25Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr25Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr25Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr25Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr25Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr25Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr25Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr25Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr25Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr25Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr25Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr25Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr25Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr25Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr25Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr25Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr25Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr25Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr25Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr25 {
    #[inline(always)]
    fn default() -> Pcr25 {
        Pcr25(0)
    }
}
impl core::fmt::Debug for Pcr25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr25")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr25 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr25 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 26."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr26(pub u32);
impl Pcr26 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr26Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr26Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr26Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr26Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr26Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr26Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr26Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr26Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr26Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr26Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr26Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr26Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr26Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr26Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr26Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr26Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr26Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr26Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr26Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr26Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr26Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr26Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr26Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr26Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr26 {
    #[inline(always)]
    fn default() -> Pcr26 {
        Pcr26(0)
    }
}
impl core::fmt::Debug for Pcr26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr26")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr26 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr26 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 27."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr27(pub u32);
impl Pcr27 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr27Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr27Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr27Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr27Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr27Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr27Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr27Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr27Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr27Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr27Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr27Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr27Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr27Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr27Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr27Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr27Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr27Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr27Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr27Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr27Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr27Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr27Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr27Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr27Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr27 {
    #[inline(always)]
    fn default() -> Pcr27 {
        Pcr27(0)
    }
}
impl core::fmt::Debug for Pcr27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr27")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr27 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr27 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 28."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr28(pub u32);
impl Pcr28 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr28Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr28Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr28Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr28Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr28Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr28Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr28Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr28Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr28Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr28Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr28Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr28Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr28Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr28Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr28Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr28Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr28Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr28Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr28Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr28Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr28Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr28Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr28Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr28Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr28 {
    #[inline(always)]
    fn default() -> Pcr28 {
        Pcr28(0)
    }
}
impl core::fmt::Debug for Pcr28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr28")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr28 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr28 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 29."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr29(pub u32);
impl Pcr29 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr29Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr29Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr29Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr29Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr29Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr29Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pv(&self) -> Pcr29Pv {
        let val = (self.0 >> 2usize) & 0x01;
        Pcr29Pv::from_bits(val as u8)
    }
    #[doc = "Pull Value."]
    #[inline(always)]
    pub const fn set_pv(&mut self, val: Pcr29Pv) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr29Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr29Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr29Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Passive Filter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pfe(&self) -> Pcr29Pfe {
        let val = (self.0 >> 4usize) & 0x01;
        Pcr29Pfe::from_bits(val as u8)
    }
    #[doc = "Passive Filter Enable."]
    #[inline(always)]
    pub const fn set_pfe(&mut self, val: Pcr29Pfe) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr29Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr29Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr29Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr29Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr29Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr29Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Pcr29Mux {
        let val = (self.0 >> 8usize) & 0x03;
        Pcr29Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Pcr29Mux) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr29Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr29Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr29Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr29Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr29Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr29Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr29Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr29Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr29Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr29 {
    #[inline(always)]
    fn default() -> Pcr29 {
        Pcr29(0)
    }
}
impl core::fmt::Debug for Pcr29 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr29")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("pv", &self.pv())
            .field("sre", &self.sre())
            .field("pfe", &self.pfe())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr29 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr29 {{ ps: {:?}, pe: {:?}, pv: {:?}, sre: {:?}, pfe: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.pv(),
            self.sre(),
            self.pfe(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr3(pub u32);
impl Pcr3 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr3Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr3Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr3Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr3Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr3Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr3Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr3Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr3Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr3Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr3Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr3Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr3Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr3Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr3Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr3Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Pcr3Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        Pcr3Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Pcr3Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr3Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr3Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr3Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr3Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr3Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr3Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr3Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr3Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr3Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr3 {
    #[inline(always)]
    fn default() -> Pcr3 {
        Pcr3(0)
    }
}
impl core::fmt::Debug for Pcr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr3")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr3 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 30."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr30(pub u32);
impl Pcr30 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr30Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr30Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr30Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr30Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr30Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr30Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pv(&self) -> Pcr30Pv {
        let val = (self.0 >> 2usize) & 0x01;
        Pcr30Pv::from_bits(val as u8)
    }
    #[doc = "Pull Value."]
    #[inline(always)]
    pub const fn set_pv(&mut self, val: Pcr30Pv) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr30Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr30Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr30Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Passive Filter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pfe(&self) -> Pcr30Pfe {
        let val = (self.0 >> 4usize) & 0x01;
        Pcr30Pfe::from_bits(val as u8)
    }
    #[doc = "Passive Filter Enable."]
    #[inline(always)]
    pub const fn set_pfe(&mut self, val: Pcr30Pfe) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr30Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr30Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr30Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr30Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr30Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr30Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse1(&self) -> Pcr30Dse1 {
        let val = (self.0 >> 7usize) & 0x01;
        Pcr30Dse1::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse1(&mut self, val: Pcr30Dse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Pin Multiplex Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Pcr30Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        Pcr30Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Pcr30Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr30Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr30Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr30Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr30Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr30Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr30Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr30Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr30Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr30Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr30 {
    #[inline(always)]
    fn default() -> Pcr30 {
        Pcr30(0)
    }
}
impl core::fmt::Debug for Pcr30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr30")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("pv", &self.pv())
            .field("sre", &self.sre())
            .field("pfe", &self.pfe())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("dse1", &self.dse1())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr30 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr30 {{ ps: {:?}, pe: {:?}, pv: {:?}, sre: {:?}, pfe: {:?}, ode: {:?}, dse: {:?}, dse1: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.pv(),
            self.sre(),
            self.pfe(),
            self.ode(),
            self.dse(),
            self.dse1(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 31."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr31(pub u32);
impl Pcr31 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr31Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr31Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr31Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr31Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr31Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr31Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr31Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr31Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr31Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Passive Filter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pfe(&self) -> Pcr31Pfe {
        let val = (self.0 >> 4usize) & 0x01;
        Pcr31Pfe::from_bits(val as u8)
    }
    #[doc = "Passive Filter Enable."]
    #[inline(always)]
    pub const fn set_pfe(&mut self, val: Pcr31Pfe) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr31Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr31Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr31Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr31Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr31Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr31Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse1(&self) -> Pcr31Dse1 {
        let val = (self.0 >> 7usize) & 0x01;
        Pcr31Dse1::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse1(&mut self, val: Pcr31Dse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Pin Multiplex Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Pcr31Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        Pcr31Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Pcr31Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr31Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr31Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr31Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr31Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr31Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr31Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr31Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr31Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr31Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr31 {
    #[inline(always)]
    fn default() -> Pcr31 {
        Pcr31(0)
    }
}
impl core::fmt::Debug for Pcr31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr31")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("pfe", &self.pfe())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("dse1", &self.dse1())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr31 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr31 {{ ps: {:?}, pe: {:?}, sre: {:?}, pfe: {:?}, ode: {:?}, dse: {:?}, dse1: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.pfe(),
            self.ode(),
            self.dse(),
            self.dse1(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr4(pub u32);
impl Pcr4 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr4Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr4Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr4Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr4Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr4Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr4Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr4Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr4Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr4Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr4Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr4Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr4Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr4Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr4Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr4Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Pcr4Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        Pcr4Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Pcr4Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr4Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr4Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr4Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr4Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr4Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr4Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr4Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr4Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr4Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr4 {
    #[inline(always)]
    fn default() -> Pcr4 {
        Pcr4(0)
    }
}
impl core::fmt::Debug for Pcr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr4")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr4 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr5(pub u32);
impl Pcr5 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr5Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr5Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr5Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr5Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr5Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr5Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr5Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr5Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr5Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr5Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr5Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr5Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr5Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr5Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr5Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Pcr5Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        Pcr5Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Pcr5Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr5Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr5Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr5Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr5Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr5Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr5Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr5Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr5Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr5Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr5 {
    #[inline(always)]
    fn default() -> Pcr5 {
        Pcr5(0)
    }
}
impl core::fmt::Debug for Pcr5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr5")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr5 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 6."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr6(pub u32);
impl Pcr6 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr6Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr6Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr6Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr6Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr6Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr6Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr6Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr6Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr6Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr6Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr6Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr6Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr6Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr6Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr6Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Pcr6Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        Pcr6Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Pcr6Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr6Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr6Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr6Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr6Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr6Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr6Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr6Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr6Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr6Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr6 {
    #[inline(always)]
    fn default() -> Pcr6 {
        Pcr6(0)
    }
}
impl core::fmt::Debug for Pcr6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr6")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr6 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 7."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr7(pub u32);
impl Pcr7 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr7Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr7Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr7Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr7Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr7Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr7Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr7Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr7Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr7Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr7Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr7Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr7Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr7Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr7Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr7Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Pcr7Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        Pcr7Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Pcr7Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr7Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr7Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr7Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr7Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr7Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr7Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr7Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr7Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr7Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr7 {
    #[inline(always)]
    fn default() -> Pcr7 {
        Pcr7(0)
    }
}
impl core::fmt::Debug for Pcr7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr7")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr7 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 8."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr8(pub u32);
impl Pcr8 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr8Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr8Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr8Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr8Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr8Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr8Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pv(&self) -> Pcr8Pv {
        let val = (self.0 >> 2usize) & 0x01;
        Pcr8Pv::from_bits(val as u8)
    }
    #[doc = "Pull Value."]
    #[inline(always)]
    pub const fn set_pv(&mut self, val: Pcr8Pv) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr8Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr8Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr8Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Passive Filter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pfe(&self) -> Pcr8Pfe {
        let val = (self.0 >> 4usize) & 0x01;
        Pcr8Pfe::from_bits(val as u8)
    }
    #[doc = "Passive Filter Enable."]
    #[inline(always)]
    pub const fn set_pfe(&mut self, val: Pcr8Pfe) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr8Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr8Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr8Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr8Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr8Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr8Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse1(&self) -> Pcr8Dse1 {
        let val = (self.0 >> 7usize) & 0x01;
        Pcr8Dse1::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse1(&mut self, val: Pcr8Dse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Pin Multiplex Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Pcr8Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        Pcr8Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Pcr8Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr8Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr8Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr8Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr8Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr8Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr8Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr8Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr8Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr8Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr8 {
    #[inline(always)]
    fn default() -> Pcr8 {
        Pcr8(0)
    }
}
impl core::fmt::Debug for Pcr8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr8")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("pv", &self.pv())
            .field("sre", &self.sre())
            .field("pfe", &self.pfe())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("dse1", &self.dse1())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr8 {{ ps: {:?}, pe: {:?}, pv: {:?}, sre: {:?}, pfe: {:?}, ode: {:?}, dse: {:?}, dse1: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.pv(),
            self.sre(),
            self.pfe(),
            self.ode(),
            self.dse(),
            self.dse1(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 9."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr9(pub u32);
impl Pcr9 {
    #[doc = "Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Pcr9Ps {
        let val = (self.0 >> 0usize) & 0x01;
        Pcr9Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Pcr9Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> Pcr9Pe {
        let val = (self.0 >> 1usize) & 0x01;
        Pcr9Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: Pcr9Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> Pcr9Sre {
        let val = (self.0 >> 3usize) & 0x01;
        Pcr9Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: Pcr9Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Passive Filter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pfe(&self) -> Pcr9Pfe {
        let val = (self.0 >> 4usize) & 0x01;
        Pcr9Pfe::from_bits(val as u8)
    }
    #[doc = "Passive Filter Enable."]
    #[inline(always)]
    pub const fn set_pfe(&mut self, val: Pcr9Pfe) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Open Drain Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> Pcr9Ode {
        let val = (self.0 >> 5usize) & 0x01;
        Pcr9Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable."]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: Pcr9Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> Pcr9Dse {
        let val = (self.0 >> 6usize) & 0x01;
        Pcr9Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: Pcr9Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Drive Strength Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dse1(&self) -> Pcr9Dse1 {
        let val = (self.0 >> 7usize) & 0x01;
        Pcr9Dse1::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable."]
    #[inline(always)]
    pub const fn set_dse1(&mut self, val: Pcr9Dse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Pin Multiplex Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> Pcr9Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        Pcr9Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: Pcr9Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> Pcr9Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        Pcr9Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: Pcr9Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input."]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> Pcr9Inv {
        let val = (self.0 >> 13usize) & 0x01;
        Pcr9Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input."]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: Pcr9Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Pcr9Lk {
        let val = (self.0 >> 15usize) & 0x01;
        Pcr9Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Pcr9Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr9 {
    #[inline(always)]
    fn default() -> Pcr9 {
        Pcr9(0)
    }
}
impl core::fmt::Debug for Pcr9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr9")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("pfe", &self.pfe())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("dse1", &self.dse1())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr9 {{ ps: {:?}, pe: {:?}, sre: {:?}, pfe: {:?}, ode: {:?}, dse: {:?}, dse1: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.pfe(),
            self.ode(),
            self.dse(),
            self.dse1(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Version ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number."]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        Feature::from_bits(val as u16)
    }
    #[doc = "Feature Specification Number."]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: Feature) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number."]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number."]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
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
            .field("feature", &self.feature())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Verid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Verid {{ feature: {:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Basic implementation."]
    pub const FEATURE0: Self = Self(0x0);
}
impl Feature {
    pub const fn from_bits(val: u16) -> Feature {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Feature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("FEATURE0"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "FEATURE0"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Feature {
    #[inline(always)]
    fn from(val: u16) -> Feature {
        Feature::from_bits(val)
    }
}
impl From<Feature> for u16 {
    #[inline(always)]
    fn from(val: Feature) -> u16 {
        Feature::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe0 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe0 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe0 {
        Gpwe0::from_bits(val)
    }
}
impl From<Gpwe0> for u8 {
    #[inline(always)]
    fn from(val: Gpwe0) -> u8 {
        Gpwe0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe1 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe1 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe1 {
        Gpwe1::from_bits(val)
    }
}
impl From<Gpwe1> for u8 {
    #[inline(always)]
    fn from(val: Gpwe1) -> u8 {
        Gpwe1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe10 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe10 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe10 {
        Gpwe10::from_bits(val)
    }
}
impl From<Gpwe10> for u8 {
    #[inline(always)]
    fn from(val: Gpwe10) -> u8 {
        Gpwe10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe11 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe11 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe11 {
        Gpwe11::from_bits(val)
    }
}
impl From<Gpwe11> for u8 {
    #[inline(always)]
    fn from(val: Gpwe11) -> u8 {
        Gpwe11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe12 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe12 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe12 {
        Gpwe12::from_bits(val)
    }
}
impl From<Gpwe12> for u8 {
    #[inline(always)]
    fn from(val: Gpwe12) -> u8 {
        Gpwe12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe13 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe13 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe13 {
        Gpwe13::from_bits(val)
    }
}
impl From<Gpwe13> for u8 {
    #[inline(always)]
    fn from(val: Gpwe13) -> u8 {
        Gpwe13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe14 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe14 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe14 {
        Gpwe14::from_bits(val)
    }
}
impl From<Gpwe14> for u8 {
    #[inline(always)]
    fn from(val: Gpwe14) -> u8 {
        Gpwe14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe15 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe15 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe15 {
        Gpwe15::from_bits(val)
    }
}
impl From<Gpwe15> for u8 {
    #[inline(always)]
    fn from(val: Gpwe15) -> u8 {
        Gpwe15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe16 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe16 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe16 {
        Gpwe16::from_bits(val)
    }
}
impl From<Gpwe16> for u8 {
    #[inline(always)]
    fn from(val: Gpwe16) -> u8 {
        Gpwe16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe17 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe17 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe17 {
        Gpwe17::from_bits(val)
    }
}
impl From<Gpwe17> for u8 {
    #[inline(always)]
    fn from(val: Gpwe17) -> u8 {
        Gpwe17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe18 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe18 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe18 {
        Gpwe18::from_bits(val)
    }
}
impl From<Gpwe18> for u8 {
    #[inline(always)]
    fn from(val: Gpwe18) -> u8 {
        Gpwe18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe19 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe19 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe19 {
        Gpwe19::from_bits(val)
    }
}
impl From<Gpwe19> for u8 {
    #[inline(always)]
    fn from(val: Gpwe19) -> u8 {
        Gpwe19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe2 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe2 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe2 {
        Gpwe2::from_bits(val)
    }
}
impl From<Gpwe2> for u8 {
    #[inline(always)]
    fn from(val: Gpwe2) -> u8 {
        Gpwe2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe20 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe20 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe20 {
        Gpwe20::from_bits(val)
    }
}
impl From<Gpwe20> for u8 {
    #[inline(always)]
    fn from(val: Gpwe20) -> u8 {
        Gpwe20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe21 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe21 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe21 {
        Gpwe21::from_bits(val)
    }
}
impl From<Gpwe21> for u8 {
    #[inline(always)]
    fn from(val: Gpwe21) -> u8 {
        Gpwe21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe22 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe22 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe22 {
        Gpwe22::from_bits(val)
    }
}
impl From<Gpwe22> for u8 {
    #[inline(always)]
    fn from(val: Gpwe22) -> u8 {
        Gpwe22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe23 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe23 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe23 {
        Gpwe23::from_bits(val)
    }
}
impl From<Gpwe23> for u8 {
    #[inline(always)]
    fn from(val: Gpwe23) -> u8 {
        Gpwe23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe24 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe24 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe24 {
        Gpwe24::from_bits(val)
    }
}
impl From<Gpwe24> for u8 {
    #[inline(always)]
    fn from(val: Gpwe24) -> u8 {
        Gpwe24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe25 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe25 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe25 {
        Gpwe25::from_bits(val)
    }
}
impl From<Gpwe25> for u8 {
    #[inline(always)]
    fn from(val: Gpwe25) -> u8 {
        Gpwe25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe26 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe26 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe26 {
        Gpwe26::from_bits(val)
    }
}
impl From<Gpwe26> for u8 {
    #[inline(always)]
    fn from(val: Gpwe26) -> u8 {
        Gpwe26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe27 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe27 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe27 {
        Gpwe27::from_bits(val)
    }
}
impl From<Gpwe27> for u8 {
    #[inline(always)]
    fn from(val: Gpwe27) -> u8 {
        Gpwe27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe28 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe28 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe28 {
        Gpwe28::from_bits(val)
    }
}
impl From<Gpwe28> for u8 {
    #[inline(always)]
    fn from(val: Gpwe28) -> u8 {
        Gpwe28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe29 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe29 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe29 {
        Gpwe29::from_bits(val)
    }
}
impl From<Gpwe29> for u8 {
    #[inline(always)]
    fn from(val: Gpwe29) -> u8 {
        Gpwe29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe3 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe3 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe3 {
        Gpwe3::from_bits(val)
    }
}
impl From<Gpwe3> for u8 {
    #[inline(always)]
    fn from(val: Gpwe3) -> u8 {
        Gpwe3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe30 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe30 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe30 {
        Gpwe30::from_bits(val)
    }
}
impl From<Gpwe30> for u8 {
    #[inline(always)]
    fn from(val: Gpwe30) -> u8 {
        Gpwe30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe31 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe31 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe31 {
        Gpwe31::from_bits(val)
    }
}
impl From<Gpwe31> for u8 {
    #[inline(always)]
    fn from(val: Gpwe31) -> u8 {
        Gpwe31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe4 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe4 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe4 {
        Gpwe4::from_bits(val)
    }
}
impl From<Gpwe4> for u8 {
    #[inline(always)]
    fn from(val: Gpwe4) -> u8 {
        Gpwe4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe5 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe5 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe5 {
        Gpwe5::from_bits(val)
    }
}
impl From<Gpwe5> for u8 {
    #[inline(always)]
    fn from(val: Gpwe5) -> u8 {
        Gpwe5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe6 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe6 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe6 {
        Gpwe6::from_bits(val)
    }
}
impl From<Gpwe6> for u8 {
    #[inline(always)]
    fn from(val: Gpwe6) -> u8 {
        Gpwe6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe7 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe7 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe7 {
        Gpwe7::from_bits(val)
    }
}
impl From<Gpwe7> for u8 {
    #[inline(always)]
    fn from(val: Gpwe7) -> u8 {
        Gpwe7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe8 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe8 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe8 {
        Gpwe8::from_bits(val)
    }
}
impl From<Gpwe8> for u8 {
    #[inline(always)]
    fn from(val: Gpwe8) -> u8 {
        Gpwe8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe9 {
    #[doc = "Not updated."]
    GPWE0 = 0x0,
    #[doc = "Updated."]
    GPWE1 = 0x01,
}
impl Gpwe9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe9 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe9 {
        Gpwe9::from_bits(val)
    }
}
impl From<Gpwe9> for u8 {
    #[inline(always)]
    fn from(val: Gpwe9) -> u8 {
        Gpwe9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr0Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Dse {
        Pcr0Dse::from_bits(val)
    }
}
impl From<Pcr0Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Dse) -> u8 {
        Pcr0Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr0Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Ibe {
        Pcr0Ibe::from_bits(val)
    }
}
impl From<Pcr0Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Ibe) -> u8 {
        Pcr0Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr0Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Inv {
        Pcr0Inv::from_bits(val)
    }
}
impl From<Pcr0Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Inv) -> u8 {
        Pcr0Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr0Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Lk {
        Pcr0Lk::from_bits(val)
    }
}
impl From<Pcr0Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Lk) -> u8 {
        Pcr0Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Mux {
    #[doc = "Alternative 0 (GPIO)."]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)."]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)."]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)."]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)."]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)."]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)."]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)."]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)."]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)."]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)."]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)."]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)."]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)."]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr0Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Mux {
        Pcr0Mux::from_bits(val)
    }
}
impl From<Pcr0Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Mux) -> u8 {
        Pcr0Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr0Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Ode {
        Pcr0Ode::from_bits(val)
    }
}
impl From<Pcr0Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Ode) -> u8 {
        Pcr0Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr0Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Pe {
        Pcr0Pe::from_bits(val)
    }
}
impl From<Pcr0Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Pe) -> u8 {
        Pcr0Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Pfe {
    #[doc = "Disables."]
    PFE0 = 0x0,
    #[doc = "Enables."]
    PFE1 = 0x01,
}
impl Pcr0Pfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Pfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Pfe {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Pfe {
        Pcr0Pfe::from_bits(val)
    }
}
impl From<Pcr0Pfe> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Pfe) -> u8 {
        Pcr0Pfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr0Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Ps {
        Pcr0Ps::from_bits(val)
    }
}
impl From<Pcr0Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Ps) -> u8 {
        Pcr0Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr0Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Sre {
        Pcr0Sre::from_bits(val)
    }
}
impl From<Pcr0Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Sre) -> u8 {
        Pcr0Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr10Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr10Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr10Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr10Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr10Dse {
        Pcr10Dse::from_bits(val)
    }
}
impl From<Pcr10Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr10Dse) -> u8 {
        Pcr10Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr10Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr10Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr10Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr10Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr10Ibe {
        Pcr10Ibe::from_bits(val)
    }
}
impl From<Pcr10Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr10Ibe) -> u8 {
        Pcr10Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr10Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr10Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr10Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr10Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr10Inv {
        Pcr10Inv::from_bits(val)
    }
}
impl From<Pcr10Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr10Inv) -> u8 {
        Pcr10Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr10Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr10Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr10Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr10Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr10Lk {
        Pcr10Lk::from_bits(val)
    }
}
impl From<Pcr10Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr10Lk) -> u8 {
        Pcr10Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr10Mux {
    #[doc = "Alternative 0 (GPIO)."]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)."]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)."]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)."]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)."]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)."]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)."]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)."]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)."]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)."]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)."]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)."]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)."]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)."]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr10Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr10Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr10Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr10Mux {
        Pcr10Mux::from_bits(val)
    }
}
impl From<Pcr10Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr10Mux) -> u8 {
        Pcr10Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr10Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr10Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr10Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr10Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr10Ode {
        Pcr10Ode::from_bits(val)
    }
}
impl From<Pcr10Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr10Ode) -> u8 {
        Pcr10Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr10Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr10Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr10Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr10Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr10Pe {
        Pcr10Pe::from_bits(val)
    }
}
impl From<Pcr10Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr10Pe) -> u8 {
        Pcr10Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr10Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr10Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr10Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr10Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr10Ps {
        Pcr10Ps::from_bits(val)
    }
}
impl From<Pcr10Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr10Ps) -> u8 {
        Pcr10Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr10Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr10Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr10Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr10Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr10Sre {
        Pcr10Sre::from_bits(val)
    }
}
impl From<Pcr10Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr10Sre) -> u8 {
        Pcr10Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr11Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr11Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr11Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr11Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr11Dse {
        Pcr11Dse::from_bits(val)
    }
}
impl From<Pcr11Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr11Dse) -> u8 {
        Pcr11Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr11Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr11Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr11Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr11Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr11Ibe {
        Pcr11Ibe::from_bits(val)
    }
}
impl From<Pcr11Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr11Ibe) -> u8 {
        Pcr11Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr11Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr11Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr11Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr11Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr11Inv {
        Pcr11Inv::from_bits(val)
    }
}
impl From<Pcr11Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr11Inv) -> u8 {
        Pcr11Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr11Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr11Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr11Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr11Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr11Lk {
        Pcr11Lk::from_bits(val)
    }
}
impl From<Pcr11Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr11Lk) -> u8 {
        Pcr11Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr11Mux {
    #[doc = "Alternative 0 (GPIO)."]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)."]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)."]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)."]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)."]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)."]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)."]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)."]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)."]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)."]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)."]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)."]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)."]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)."]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr11Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr11Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr11Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr11Mux {
        Pcr11Mux::from_bits(val)
    }
}
impl From<Pcr11Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr11Mux) -> u8 {
        Pcr11Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr11Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr11Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr11Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr11Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr11Ode {
        Pcr11Ode::from_bits(val)
    }
}
impl From<Pcr11Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr11Ode) -> u8 {
        Pcr11Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr11Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr11Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr11Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr11Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr11Pe {
        Pcr11Pe::from_bits(val)
    }
}
impl From<Pcr11Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr11Pe) -> u8 {
        Pcr11Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr11Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr11Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr11Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr11Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr11Ps {
        Pcr11Ps::from_bits(val)
    }
}
impl From<Pcr11Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr11Ps) -> u8 {
        Pcr11Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr11Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr11Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr11Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr11Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr11Sre {
        Pcr11Sre::from_bits(val)
    }
}
impl From<Pcr11Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr11Sre) -> u8 {
        Pcr11Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr12Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Dse {
        Pcr12Dse::from_bits(val)
    }
}
impl From<Pcr12Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Dse) -> u8 {
        Pcr12Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr12Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Ibe {
        Pcr12Ibe::from_bits(val)
    }
}
impl From<Pcr12Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Ibe) -> u8 {
        Pcr12Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr12Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Inv {
        Pcr12Inv::from_bits(val)
    }
}
impl From<Pcr12Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Inv) -> u8 {
        Pcr12Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr12Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Lk {
        Pcr12Lk::from_bits(val)
    }
}
impl From<Pcr12Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Lk) -> u8 {
        Pcr12Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Mux {
    #[doc = "Alternative 0 (GPIO)."]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)."]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)."]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)."]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)."]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)."]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)."]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)."]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)."]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)."]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)."]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)."]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)."]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)."]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr12Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Mux {
        Pcr12Mux::from_bits(val)
    }
}
impl From<Pcr12Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Mux) -> u8 {
        Pcr12Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr12Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Ode {
        Pcr12Ode::from_bits(val)
    }
}
impl From<Pcr12Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Ode) -> u8 {
        Pcr12Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr12Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Pe {
        Pcr12Pe::from_bits(val)
    }
}
impl From<Pcr12Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Pe) -> u8 {
        Pcr12Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr12Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Ps {
        Pcr12Ps::from_bits(val)
    }
}
impl From<Pcr12Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Ps) -> u8 {
        Pcr12Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr12Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Sre {
        Pcr12Sre::from_bits(val)
    }
}
impl From<Pcr12Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Sre) -> u8 {
        Pcr12Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr13Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Dse {
        Pcr13Dse::from_bits(val)
    }
}
impl From<Pcr13Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Dse) -> u8 {
        Pcr13Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr13Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Ibe {
        Pcr13Ibe::from_bits(val)
    }
}
impl From<Pcr13Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Ibe) -> u8 {
        Pcr13Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr13Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Inv {
        Pcr13Inv::from_bits(val)
    }
}
impl From<Pcr13Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Inv) -> u8 {
        Pcr13Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr13Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Lk {
        Pcr13Lk::from_bits(val)
    }
}
impl From<Pcr13Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Lk) -> u8 {
        Pcr13Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Mux {
    #[doc = "Alternative 0 (GPIO)."]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)."]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)."]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)."]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)."]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)."]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)."]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)."]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)."]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)."]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)."]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)."]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)."]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)."]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr13Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Mux {
        Pcr13Mux::from_bits(val)
    }
}
impl From<Pcr13Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Mux) -> u8 {
        Pcr13Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr13Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Ode {
        Pcr13Ode::from_bits(val)
    }
}
impl From<Pcr13Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Ode) -> u8 {
        Pcr13Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr13Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Pe {
        Pcr13Pe::from_bits(val)
    }
}
impl From<Pcr13Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Pe) -> u8 {
        Pcr13Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr13Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Ps {
        Pcr13Ps::from_bits(val)
    }
}
impl From<Pcr13Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Ps) -> u8 {
        Pcr13Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr13Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Sre {
        Pcr13Sre::from_bits(val)
    }
}
impl From<Pcr13Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Sre) -> u8 {
        Pcr13Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr14Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Dse {
        Pcr14Dse::from_bits(val)
    }
}
impl From<Pcr14Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Dse) -> u8 {
        Pcr14Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr14Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Ibe {
        Pcr14Ibe::from_bits(val)
    }
}
impl From<Pcr14Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Ibe) -> u8 {
        Pcr14Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr14Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Inv {
        Pcr14Inv::from_bits(val)
    }
}
impl From<Pcr14Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Inv) -> u8 {
        Pcr14Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr14Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Lk {
        Pcr14Lk::from_bits(val)
    }
}
impl From<Pcr14Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Lk) -> u8 {
        Pcr14Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Mux {
    #[doc = "Alternative 0 (GPIO)."]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)."]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)."]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)."]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)."]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)."]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)."]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)."]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)."]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)."]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)."]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)."]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)."]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)."]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr14Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Mux {
        Pcr14Mux::from_bits(val)
    }
}
impl From<Pcr14Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Mux) -> u8 {
        Pcr14Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr14Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Ode {
        Pcr14Ode::from_bits(val)
    }
}
impl From<Pcr14Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Ode) -> u8 {
        Pcr14Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr14Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Pe {
        Pcr14Pe::from_bits(val)
    }
}
impl From<Pcr14Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Pe) -> u8 {
        Pcr14Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr14Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Ps {
        Pcr14Ps::from_bits(val)
    }
}
impl From<Pcr14Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Ps) -> u8 {
        Pcr14Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr14Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Sre {
        Pcr14Sre::from_bits(val)
    }
}
impl From<Pcr14Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Sre) -> u8 {
        Pcr14Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr15Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Dse {
        Pcr15Dse::from_bits(val)
    }
}
impl From<Pcr15Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Dse) -> u8 {
        Pcr15Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr15Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Ibe {
        Pcr15Ibe::from_bits(val)
    }
}
impl From<Pcr15Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Ibe) -> u8 {
        Pcr15Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr15Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Inv {
        Pcr15Inv::from_bits(val)
    }
}
impl From<Pcr15Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Inv) -> u8 {
        Pcr15Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr15Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Lk {
        Pcr15Lk::from_bits(val)
    }
}
impl From<Pcr15Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Lk) -> u8 {
        Pcr15Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Mux {
    #[doc = "Alternative 0 (GPIO)."]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)."]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)."]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)."]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)."]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)."]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)."]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)."]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)."]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)."]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)."]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)."]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)."]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)."]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr15Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Mux {
        Pcr15Mux::from_bits(val)
    }
}
impl From<Pcr15Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Mux) -> u8 {
        Pcr15Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr15Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Ode {
        Pcr15Ode::from_bits(val)
    }
}
impl From<Pcr15Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Ode) -> u8 {
        Pcr15Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr15Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Pe {
        Pcr15Pe::from_bits(val)
    }
}
impl From<Pcr15Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Pe) -> u8 {
        Pcr15Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr15Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Ps {
        Pcr15Ps::from_bits(val)
    }
}
impl From<Pcr15Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Ps) -> u8 {
        Pcr15Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr15Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Sre {
        Pcr15Sre::from_bits(val)
    }
}
impl From<Pcr15Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Sre) -> u8 {
        Pcr15Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr16Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Dse {
        Pcr16Dse::from_bits(val)
    }
}
impl From<Pcr16Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Dse) -> u8 {
        Pcr16Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr16Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Ibe {
        Pcr16Ibe::from_bits(val)
    }
}
impl From<Pcr16Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Ibe) -> u8 {
        Pcr16Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr16Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Inv {
        Pcr16Inv::from_bits(val)
    }
}
impl From<Pcr16Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Inv) -> u8 {
        Pcr16Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr16Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Lk {
        Pcr16Lk::from_bits(val)
    }
}
impl From<Pcr16Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Lk) -> u8 {
        Pcr16Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Mux {
    #[doc = "Alternative 0 (GPIO)."]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)."]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)."]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)."]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)."]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)."]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)."]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)."]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)."]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)."]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)."]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)."]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)."]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)."]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr16Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Mux {
        Pcr16Mux::from_bits(val)
    }
}
impl From<Pcr16Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Mux) -> u8 {
        Pcr16Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr16Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Ode {
        Pcr16Ode::from_bits(val)
    }
}
impl From<Pcr16Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Ode) -> u8 {
        Pcr16Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr16Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Pe {
        Pcr16Pe::from_bits(val)
    }
}
impl From<Pcr16Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Pe) -> u8 {
        Pcr16Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr16Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Ps {
        Pcr16Ps::from_bits(val)
    }
}
impl From<Pcr16Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Ps) -> u8 {
        Pcr16Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr16Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Sre {
        Pcr16Sre::from_bits(val)
    }
}
impl From<Pcr16Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Sre) -> u8 {
        Pcr16Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr17Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Dse {
        Pcr17Dse::from_bits(val)
    }
}
impl From<Pcr17Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Dse) -> u8 {
        Pcr17Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr17Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Ibe {
        Pcr17Ibe::from_bits(val)
    }
}
impl From<Pcr17Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Ibe) -> u8 {
        Pcr17Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr17Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Inv {
        Pcr17Inv::from_bits(val)
    }
}
impl From<Pcr17Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Inv) -> u8 {
        Pcr17Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr17Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Lk {
        Pcr17Lk::from_bits(val)
    }
}
impl From<Pcr17Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Lk) -> u8 {
        Pcr17Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Mux {
    #[doc = "Alternative 0 (GPIO)."]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)."]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)."]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)."]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)."]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)."]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)."]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)."]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)."]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)."]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)."]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)."]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)."]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)."]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr17Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Mux {
        Pcr17Mux::from_bits(val)
    }
}
impl From<Pcr17Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Mux) -> u8 {
        Pcr17Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr17Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Ode {
        Pcr17Ode::from_bits(val)
    }
}
impl From<Pcr17Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Ode) -> u8 {
        Pcr17Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr17Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Pe {
        Pcr17Pe::from_bits(val)
    }
}
impl From<Pcr17Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Pe) -> u8 {
        Pcr17Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr17Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Ps {
        Pcr17Ps::from_bits(val)
    }
}
impl From<Pcr17Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Ps) -> u8 {
        Pcr17Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr17Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Sre {
        Pcr17Sre::from_bits(val)
    }
}
impl From<Pcr17Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Sre) -> u8 {
        Pcr17Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr18Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Dse {
        Pcr18Dse::from_bits(val)
    }
}
impl From<Pcr18Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Dse) -> u8 {
        Pcr18Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr18Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Ibe {
        Pcr18Ibe::from_bits(val)
    }
}
impl From<Pcr18Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Ibe) -> u8 {
        Pcr18Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr18Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Inv {
        Pcr18Inv::from_bits(val)
    }
}
impl From<Pcr18Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Inv) -> u8 {
        Pcr18Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr18Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Lk {
        Pcr18Lk::from_bits(val)
    }
}
impl From<Pcr18Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Lk) -> u8 {
        Pcr18Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Mux {
    #[doc = "Alternative 0 (GPIO)."]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)."]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)."]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)."]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)."]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)."]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)."]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)."]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)."]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)."]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)."]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)."]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)."]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)."]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr18Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Mux {
        Pcr18Mux::from_bits(val)
    }
}
impl From<Pcr18Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Mux) -> u8 {
        Pcr18Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr18Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Ode {
        Pcr18Ode::from_bits(val)
    }
}
impl From<Pcr18Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Ode) -> u8 {
        Pcr18Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr18Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Pe {
        Pcr18Pe::from_bits(val)
    }
}
impl From<Pcr18Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Pe) -> u8 {
        Pcr18Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr18Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Ps {
        Pcr18Ps::from_bits(val)
    }
}
impl From<Pcr18Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Ps) -> u8 {
        Pcr18Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr18Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Sre {
        Pcr18Sre::from_bits(val)
    }
}
impl From<Pcr18Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Sre) -> u8 {
        Pcr18Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr19Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Dse {
        Pcr19Dse::from_bits(val)
    }
}
impl From<Pcr19Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Dse) -> u8 {
        Pcr19Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr19Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Ibe {
        Pcr19Ibe::from_bits(val)
    }
}
impl From<Pcr19Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Ibe) -> u8 {
        Pcr19Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr19Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Inv {
        Pcr19Inv::from_bits(val)
    }
}
impl From<Pcr19Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Inv) -> u8 {
        Pcr19Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr19Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Lk {
        Pcr19Lk::from_bits(val)
    }
}
impl From<Pcr19Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Lk) -> u8 {
        Pcr19Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Mux {
    #[doc = "Alternative 0 (GPIO)."]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)."]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)."]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)."]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)."]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)."]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)."]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)."]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)."]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)."]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)."]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)."]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)."]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)."]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr19Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Mux {
        Pcr19Mux::from_bits(val)
    }
}
impl From<Pcr19Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Mux) -> u8 {
        Pcr19Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr19Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Ode {
        Pcr19Ode::from_bits(val)
    }
}
impl From<Pcr19Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Ode) -> u8 {
        Pcr19Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr19Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Pe {
        Pcr19Pe::from_bits(val)
    }
}
impl From<Pcr19Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Pe) -> u8 {
        Pcr19Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr19Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Ps {
        Pcr19Ps::from_bits(val)
    }
}
impl From<Pcr19Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Ps) -> u8 {
        Pcr19Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr19Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Sre {
        Pcr19Sre::from_bits(val)
    }
}
impl From<Pcr19Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Sre) -> u8 {
        Pcr19Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr1Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Dse {
        Pcr1Dse::from_bits(val)
    }
}
impl From<Pcr1Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Dse) -> u8 {
        Pcr1Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr1Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Ibe {
        Pcr1Ibe::from_bits(val)
    }
}
impl From<Pcr1Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Ibe) -> u8 {
        Pcr1Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr1Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Inv {
        Pcr1Inv::from_bits(val)
    }
}
impl From<Pcr1Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Inv) -> u8 {
        Pcr1Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr1Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Lk {
        Pcr1Lk::from_bits(val)
    }
}
impl From<Pcr1Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Lk) -> u8 {
        Pcr1Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Mux {
    #[doc = "Alternative 0 (GPIO)."]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)."]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)."]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)."]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)."]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)."]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)."]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)."]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)."]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)."]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)."]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)."]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)."]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)."]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr1Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Mux {
        Pcr1Mux::from_bits(val)
    }
}
impl From<Pcr1Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Mux) -> u8 {
        Pcr1Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr1Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Ode {
        Pcr1Ode::from_bits(val)
    }
}
impl From<Pcr1Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Ode) -> u8 {
        Pcr1Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr1Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Pe {
        Pcr1Pe::from_bits(val)
    }
}
impl From<Pcr1Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Pe) -> u8 {
        Pcr1Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Pfe {
    #[doc = "Disables."]
    PFE0 = 0x0,
    #[doc = "Enables."]
    PFE1 = 0x01,
}
impl Pcr1Pfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Pfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Pfe {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Pfe {
        Pcr1Pfe::from_bits(val)
    }
}
impl From<Pcr1Pfe> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Pfe) -> u8 {
        Pcr1Pfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr1Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Ps {
        Pcr1Ps::from_bits(val)
    }
}
impl From<Pcr1Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Ps) -> u8 {
        Pcr1Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr1Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Sre {
        Pcr1Sre::from_bits(val)
    }
}
impl From<Pcr1Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Sre) -> u8 {
        Pcr1Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr20Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr20Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr20Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr20Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr20Dse {
        Pcr20Dse::from_bits(val)
    }
}
impl From<Pcr20Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr20Dse) -> u8 {
        Pcr20Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr20Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr20Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr20Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr20Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr20Ibe {
        Pcr20Ibe::from_bits(val)
    }
}
impl From<Pcr20Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr20Ibe) -> u8 {
        Pcr20Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr20Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr20Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr20Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr20Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr20Inv {
        Pcr20Inv::from_bits(val)
    }
}
impl From<Pcr20Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr20Inv) -> u8 {
        Pcr20Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr20Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr20Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr20Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr20Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr20Lk {
        Pcr20Lk::from_bits(val)
    }
}
impl From<Pcr20Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr20Lk) -> u8 {
        Pcr20Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr20Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr20Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr20Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr20Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr20Ode {
        Pcr20Ode::from_bits(val)
    }
}
impl From<Pcr20Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr20Ode) -> u8 {
        Pcr20Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr20Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr20Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr20Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr20Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr20Pe {
        Pcr20Pe::from_bits(val)
    }
}
impl From<Pcr20Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr20Pe) -> u8 {
        Pcr20Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr20Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr20Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr20Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr20Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr20Ps {
        Pcr20Ps::from_bits(val)
    }
}
impl From<Pcr20Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr20Ps) -> u8 {
        Pcr20Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr20Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr20Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr20Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr20Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr20Sre {
        Pcr20Sre::from_bits(val)
    }
}
impl From<Pcr20Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr20Sre) -> u8 {
        Pcr20Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr21Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr21Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr21Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr21Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr21Dse {
        Pcr21Dse::from_bits(val)
    }
}
impl From<Pcr21Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr21Dse) -> u8 {
        Pcr21Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr21Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr21Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr21Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr21Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr21Ibe {
        Pcr21Ibe::from_bits(val)
    }
}
impl From<Pcr21Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr21Ibe) -> u8 {
        Pcr21Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr21Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr21Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr21Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr21Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr21Inv {
        Pcr21Inv::from_bits(val)
    }
}
impl From<Pcr21Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr21Inv) -> u8 {
        Pcr21Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr21Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr21Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr21Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr21Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr21Lk {
        Pcr21Lk::from_bits(val)
    }
}
impl From<Pcr21Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr21Lk) -> u8 {
        Pcr21Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr21Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr21Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr21Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr21Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr21Ode {
        Pcr21Ode::from_bits(val)
    }
}
impl From<Pcr21Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr21Ode) -> u8 {
        Pcr21Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr21Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr21Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr21Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr21Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr21Pe {
        Pcr21Pe::from_bits(val)
    }
}
impl From<Pcr21Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr21Pe) -> u8 {
        Pcr21Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr21Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr21Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr21Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr21Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr21Ps {
        Pcr21Ps::from_bits(val)
    }
}
impl From<Pcr21Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr21Ps) -> u8 {
        Pcr21Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr21Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr21Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr21Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr21Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr21Sre {
        Pcr21Sre::from_bits(val)
    }
}
impl From<Pcr21Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr21Sre) -> u8 {
        Pcr21Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr22Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr22Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr22Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr22Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr22Dse {
        Pcr22Dse::from_bits(val)
    }
}
impl From<Pcr22Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr22Dse) -> u8 {
        Pcr22Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr22Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr22Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr22Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr22Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr22Ibe {
        Pcr22Ibe::from_bits(val)
    }
}
impl From<Pcr22Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr22Ibe) -> u8 {
        Pcr22Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr22Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr22Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr22Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr22Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr22Inv {
        Pcr22Inv::from_bits(val)
    }
}
impl From<Pcr22Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr22Inv) -> u8 {
        Pcr22Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr22Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr22Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr22Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr22Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr22Lk {
        Pcr22Lk::from_bits(val)
    }
}
impl From<Pcr22Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr22Lk) -> u8 {
        Pcr22Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr22Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr22Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr22Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr22Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr22Ode {
        Pcr22Ode::from_bits(val)
    }
}
impl From<Pcr22Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr22Ode) -> u8 {
        Pcr22Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr22Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr22Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr22Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr22Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr22Pe {
        Pcr22Pe::from_bits(val)
    }
}
impl From<Pcr22Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr22Pe) -> u8 {
        Pcr22Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr22Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr22Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr22Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr22Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr22Ps {
        Pcr22Ps::from_bits(val)
    }
}
impl From<Pcr22Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr22Ps) -> u8 {
        Pcr22Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr22Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr22Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr22Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr22Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr22Sre {
        Pcr22Sre::from_bits(val)
    }
}
impl From<Pcr22Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr22Sre) -> u8 {
        Pcr22Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr23Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr23Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr23Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr23Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr23Dse {
        Pcr23Dse::from_bits(val)
    }
}
impl From<Pcr23Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr23Dse) -> u8 {
        Pcr23Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr23Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr23Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr23Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr23Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr23Ibe {
        Pcr23Ibe::from_bits(val)
    }
}
impl From<Pcr23Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr23Ibe) -> u8 {
        Pcr23Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr23Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr23Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr23Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr23Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr23Inv {
        Pcr23Inv::from_bits(val)
    }
}
impl From<Pcr23Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr23Inv) -> u8 {
        Pcr23Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr23Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr23Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr23Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr23Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr23Lk {
        Pcr23Lk::from_bits(val)
    }
}
impl From<Pcr23Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr23Lk) -> u8 {
        Pcr23Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr23Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr23Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr23Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr23Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr23Ode {
        Pcr23Ode::from_bits(val)
    }
}
impl From<Pcr23Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr23Ode) -> u8 {
        Pcr23Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr23Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr23Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr23Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr23Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr23Pe {
        Pcr23Pe::from_bits(val)
    }
}
impl From<Pcr23Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr23Pe) -> u8 {
        Pcr23Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr23Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr23Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr23Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr23Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr23Ps {
        Pcr23Ps::from_bits(val)
    }
}
impl From<Pcr23Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr23Ps) -> u8 {
        Pcr23Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr23Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr23Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr23Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr23Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr23Sre {
        Pcr23Sre::from_bits(val)
    }
}
impl From<Pcr23Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr23Sre) -> u8 {
        Pcr23Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr24Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr24Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr24Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr24Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr24Dse {
        Pcr24Dse::from_bits(val)
    }
}
impl From<Pcr24Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr24Dse) -> u8 {
        Pcr24Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr24Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr24Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr24Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr24Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr24Ibe {
        Pcr24Ibe::from_bits(val)
    }
}
impl From<Pcr24Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr24Ibe) -> u8 {
        Pcr24Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr24Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr24Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr24Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr24Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr24Inv {
        Pcr24Inv::from_bits(val)
    }
}
impl From<Pcr24Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr24Inv) -> u8 {
        Pcr24Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr24Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr24Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr24Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr24Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr24Lk {
        Pcr24Lk::from_bits(val)
    }
}
impl From<Pcr24Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr24Lk) -> u8 {
        Pcr24Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr24Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr24Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr24Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr24Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr24Ode {
        Pcr24Ode::from_bits(val)
    }
}
impl From<Pcr24Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr24Ode) -> u8 {
        Pcr24Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr24Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr24Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr24Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr24Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr24Pe {
        Pcr24Pe::from_bits(val)
    }
}
impl From<Pcr24Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr24Pe) -> u8 {
        Pcr24Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr24Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr24Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr24Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr24Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr24Ps {
        Pcr24Ps::from_bits(val)
    }
}
impl From<Pcr24Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr24Ps) -> u8 {
        Pcr24Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr24Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr24Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr24Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr24Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr24Sre {
        Pcr24Sre::from_bits(val)
    }
}
impl From<Pcr24Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr24Sre) -> u8 {
        Pcr24Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr25Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr25Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr25Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr25Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr25Dse {
        Pcr25Dse::from_bits(val)
    }
}
impl From<Pcr25Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr25Dse) -> u8 {
        Pcr25Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr25Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr25Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr25Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr25Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr25Ibe {
        Pcr25Ibe::from_bits(val)
    }
}
impl From<Pcr25Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr25Ibe) -> u8 {
        Pcr25Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr25Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr25Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr25Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr25Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr25Inv {
        Pcr25Inv::from_bits(val)
    }
}
impl From<Pcr25Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr25Inv) -> u8 {
        Pcr25Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr25Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr25Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr25Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr25Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr25Lk {
        Pcr25Lk::from_bits(val)
    }
}
impl From<Pcr25Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr25Lk) -> u8 {
        Pcr25Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr25Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr25Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr25Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr25Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr25Ode {
        Pcr25Ode::from_bits(val)
    }
}
impl From<Pcr25Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr25Ode) -> u8 {
        Pcr25Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr25Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr25Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr25Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr25Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr25Pe {
        Pcr25Pe::from_bits(val)
    }
}
impl From<Pcr25Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr25Pe) -> u8 {
        Pcr25Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr25Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr25Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr25Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr25Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr25Ps {
        Pcr25Ps::from_bits(val)
    }
}
impl From<Pcr25Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr25Ps) -> u8 {
        Pcr25Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr25Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr25Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr25Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr25Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr25Sre {
        Pcr25Sre::from_bits(val)
    }
}
impl From<Pcr25Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr25Sre) -> u8 {
        Pcr25Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr26Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr26Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr26Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr26Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr26Dse {
        Pcr26Dse::from_bits(val)
    }
}
impl From<Pcr26Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr26Dse) -> u8 {
        Pcr26Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr26Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr26Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr26Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr26Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr26Ibe {
        Pcr26Ibe::from_bits(val)
    }
}
impl From<Pcr26Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr26Ibe) -> u8 {
        Pcr26Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr26Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr26Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr26Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr26Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr26Inv {
        Pcr26Inv::from_bits(val)
    }
}
impl From<Pcr26Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr26Inv) -> u8 {
        Pcr26Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr26Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr26Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr26Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr26Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr26Lk {
        Pcr26Lk::from_bits(val)
    }
}
impl From<Pcr26Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr26Lk) -> u8 {
        Pcr26Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr26Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr26Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr26Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr26Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr26Ode {
        Pcr26Ode::from_bits(val)
    }
}
impl From<Pcr26Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr26Ode) -> u8 {
        Pcr26Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr26Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr26Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr26Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr26Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr26Pe {
        Pcr26Pe::from_bits(val)
    }
}
impl From<Pcr26Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr26Pe) -> u8 {
        Pcr26Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr26Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr26Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr26Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr26Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr26Ps {
        Pcr26Ps::from_bits(val)
    }
}
impl From<Pcr26Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr26Ps) -> u8 {
        Pcr26Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr26Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr26Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr26Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr26Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr26Sre {
        Pcr26Sre::from_bits(val)
    }
}
impl From<Pcr26Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr26Sre) -> u8 {
        Pcr26Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr27Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr27Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr27Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr27Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr27Dse {
        Pcr27Dse::from_bits(val)
    }
}
impl From<Pcr27Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr27Dse) -> u8 {
        Pcr27Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr27Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr27Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr27Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr27Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr27Ibe {
        Pcr27Ibe::from_bits(val)
    }
}
impl From<Pcr27Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr27Ibe) -> u8 {
        Pcr27Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr27Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr27Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr27Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr27Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr27Inv {
        Pcr27Inv::from_bits(val)
    }
}
impl From<Pcr27Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr27Inv) -> u8 {
        Pcr27Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr27Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr27Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr27Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr27Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr27Lk {
        Pcr27Lk::from_bits(val)
    }
}
impl From<Pcr27Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr27Lk) -> u8 {
        Pcr27Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr27Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr27Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr27Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr27Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr27Ode {
        Pcr27Ode::from_bits(val)
    }
}
impl From<Pcr27Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr27Ode) -> u8 {
        Pcr27Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr27Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr27Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr27Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr27Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr27Pe {
        Pcr27Pe::from_bits(val)
    }
}
impl From<Pcr27Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr27Pe) -> u8 {
        Pcr27Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr27Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr27Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr27Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr27Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr27Ps {
        Pcr27Ps::from_bits(val)
    }
}
impl From<Pcr27Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr27Ps) -> u8 {
        Pcr27Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr27Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr27Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr27Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr27Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr27Sre {
        Pcr27Sre::from_bits(val)
    }
}
impl From<Pcr27Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr27Sre) -> u8 {
        Pcr27Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr28Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr28Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr28Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr28Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr28Dse {
        Pcr28Dse::from_bits(val)
    }
}
impl From<Pcr28Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr28Dse) -> u8 {
        Pcr28Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr28Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr28Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr28Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr28Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr28Ibe {
        Pcr28Ibe::from_bits(val)
    }
}
impl From<Pcr28Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr28Ibe) -> u8 {
        Pcr28Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr28Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr28Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr28Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr28Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr28Inv {
        Pcr28Inv::from_bits(val)
    }
}
impl From<Pcr28Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr28Inv) -> u8 {
        Pcr28Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr28Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr28Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr28Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr28Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr28Lk {
        Pcr28Lk::from_bits(val)
    }
}
impl From<Pcr28Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr28Lk) -> u8 {
        Pcr28Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr28Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr28Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr28Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr28Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr28Ode {
        Pcr28Ode::from_bits(val)
    }
}
impl From<Pcr28Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr28Ode) -> u8 {
        Pcr28Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr28Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr28Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr28Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr28Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr28Pe {
        Pcr28Pe::from_bits(val)
    }
}
impl From<Pcr28Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr28Pe) -> u8 {
        Pcr28Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr28Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr28Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr28Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr28Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr28Ps {
        Pcr28Ps::from_bits(val)
    }
}
impl From<Pcr28Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr28Ps) -> u8 {
        Pcr28Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr28Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr28Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr28Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr28Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr28Sre {
        Pcr28Sre::from_bits(val)
    }
}
impl From<Pcr28Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr28Sre) -> u8 {
        Pcr28Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr29Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr29Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr29Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr29Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr29Dse {
        Pcr29Dse::from_bits(val)
    }
}
impl From<Pcr29Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr29Dse) -> u8 {
        Pcr29Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr29Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr29Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr29Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr29Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr29Ibe {
        Pcr29Ibe::from_bits(val)
    }
}
impl From<Pcr29Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr29Ibe) -> u8 {
        Pcr29Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr29Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr29Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr29Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr29Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr29Inv {
        Pcr29Inv::from_bits(val)
    }
}
impl From<Pcr29Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr29Inv) -> u8 {
        Pcr29Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr29Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr29Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr29Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr29Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr29Lk {
        Pcr29Lk::from_bits(val)
    }
}
impl From<Pcr29Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr29Lk) -> u8 {
        Pcr29Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr29Mux {
    #[doc = "Alternative 0 (GPIO)."]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)."]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)."]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)."]
    MUX11 = 0x03,
}
impl Pcr29Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr29Mux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr29Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr29Mux {
        Pcr29Mux::from_bits(val)
    }
}
impl From<Pcr29Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr29Mux) -> u8 {
        Pcr29Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr29Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr29Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr29Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr29Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr29Ode {
        Pcr29Ode::from_bits(val)
    }
}
impl From<Pcr29Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr29Ode) -> u8 {
        Pcr29Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr29Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr29Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr29Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr29Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr29Pe {
        Pcr29Pe::from_bits(val)
    }
}
impl From<Pcr29Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr29Pe) -> u8 {
        Pcr29Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr29Pfe {
    #[doc = "Disables."]
    PFE0 = 0x0,
    #[doc = "Enables."]
    PFE1 = 0x01,
}
impl Pcr29Pfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr29Pfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr29Pfe {
    #[inline(always)]
    fn from(val: u8) -> Pcr29Pfe {
        Pcr29Pfe::from_bits(val)
    }
}
impl From<Pcr29Pfe> for u8 {
    #[inline(always)]
    fn from(val: Pcr29Pfe) -> u8 {
        Pcr29Pfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr29Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr29Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr29Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr29Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr29Ps {
        Pcr29Ps::from_bits(val)
    }
}
impl From<Pcr29Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr29Ps) -> u8 {
        Pcr29Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr29Pv {
    #[doc = "Low."]
    PV0 = 0x0,
    #[doc = "High."]
    PV1 = 0x01,
}
impl Pcr29Pv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr29Pv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr29Pv {
    #[inline(always)]
    fn from(val: u8) -> Pcr29Pv {
        Pcr29Pv::from_bits(val)
    }
}
impl From<Pcr29Pv> for u8 {
    #[inline(always)]
    fn from(val: Pcr29Pv) -> u8 {
        Pcr29Pv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr29Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr29Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr29Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr29Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr29Sre {
        Pcr29Sre::from_bits(val)
    }
}
impl From<Pcr29Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr29Sre) -> u8 {
        Pcr29Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr2Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Dse {
        Pcr2Dse::from_bits(val)
    }
}
impl From<Pcr2Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Dse) -> u8 {
        Pcr2Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr2Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Ibe {
        Pcr2Ibe::from_bits(val)
    }
}
impl From<Pcr2Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Ibe) -> u8 {
        Pcr2Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr2Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Inv {
        Pcr2Inv::from_bits(val)
    }
}
impl From<Pcr2Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Inv) -> u8 {
        Pcr2Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr2Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Lk {
        Pcr2Lk::from_bits(val)
    }
}
impl From<Pcr2Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Lk) -> u8 {
        Pcr2Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Mux {
    #[doc = "Alternative 0 (GPIO)."]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)."]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)."]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)."]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)."]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)."]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)."]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)."]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)."]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)."]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)."]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)."]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)."]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)."]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr2Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Mux {
        Pcr2Mux::from_bits(val)
    }
}
impl From<Pcr2Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Mux) -> u8 {
        Pcr2Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr2Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Ode {
        Pcr2Ode::from_bits(val)
    }
}
impl From<Pcr2Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Ode) -> u8 {
        Pcr2Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr2Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Pe {
        Pcr2Pe::from_bits(val)
    }
}
impl From<Pcr2Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Pe) -> u8 {
        Pcr2Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr2Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Ps {
        Pcr2Ps::from_bits(val)
    }
}
impl From<Pcr2Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Ps) -> u8 {
        Pcr2Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr2Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Sre {
        Pcr2Sre::from_bits(val)
    }
}
impl From<Pcr2Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Sre) -> u8 {
        Pcr2Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr30Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr30Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr30Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr30Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr30Dse {
        Pcr30Dse::from_bits(val)
    }
}
impl From<Pcr30Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr30Dse) -> u8 {
        Pcr30Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr30Dse1 {
    #[doc = "Normal."]
    DSE10 = 0x0,
    #[doc = "Double."]
    DSE11 = 0x01,
}
impl Pcr30Dse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr30Dse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr30Dse1 {
    #[inline(always)]
    fn from(val: u8) -> Pcr30Dse1 {
        Pcr30Dse1::from_bits(val)
    }
}
impl From<Pcr30Dse1> for u8 {
    #[inline(always)]
    fn from(val: Pcr30Dse1) -> u8 {
        Pcr30Dse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr30Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr30Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr30Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr30Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr30Ibe {
        Pcr30Ibe::from_bits(val)
    }
}
impl From<Pcr30Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr30Ibe) -> u8 {
        Pcr30Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr30Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr30Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr30Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr30Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr30Inv {
        Pcr30Inv::from_bits(val)
    }
}
impl From<Pcr30Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr30Inv) -> u8 {
        Pcr30Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr30Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr30Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr30Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr30Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr30Lk {
        Pcr30Lk::from_bits(val)
    }
}
impl From<Pcr30Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr30Lk) -> u8 {
        Pcr30Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr30Mux {
    #[doc = "Alternative 0 (GPIO)."]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)."]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)."]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)."]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)."]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)."]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)."]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)."]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)."]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)."]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)."]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)."]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)."]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)."]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr30Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr30Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr30Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr30Mux {
        Pcr30Mux::from_bits(val)
    }
}
impl From<Pcr30Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr30Mux) -> u8 {
        Pcr30Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr30Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr30Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr30Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr30Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr30Ode {
        Pcr30Ode::from_bits(val)
    }
}
impl From<Pcr30Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr30Ode) -> u8 {
        Pcr30Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr30Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr30Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr30Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr30Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr30Pe {
        Pcr30Pe::from_bits(val)
    }
}
impl From<Pcr30Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr30Pe) -> u8 {
        Pcr30Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr30Pfe {
    #[doc = "Disables."]
    PFE0 = 0x0,
    #[doc = "Enables."]
    PFE1 = 0x01,
}
impl Pcr30Pfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr30Pfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr30Pfe {
    #[inline(always)]
    fn from(val: u8) -> Pcr30Pfe {
        Pcr30Pfe::from_bits(val)
    }
}
impl From<Pcr30Pfe> for u8 {
    #[inline(always)]
    fn from(val: Pcr30Pfe) -> u8 {
        Pcr30Pfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr30Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr30Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr30Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr30Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr30Ps {
        Pcr30Ps::from_bits(val)
    }
}
impl From<Pcr30Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr30Ps) -> u8 {
        Pcr30Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr30Pv {
    #[doc = "Low."]
    PV0 = 0x0,
    #[doc = "High."]
    PV1 = 0x01,
}
impl Pcr30Pv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr30Pv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr30Pv {
    #[inline(always)]
    fn from(val: u8) -> Pcr30Pv {
        Pcr30Pv::from_bits(val)
    }
}
impl From<Pcr30Pv> for u8 {
    #[inline(always)]
    fn from(val: Pcr30Pv) -> u8 {
        Pcr30Pv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr30Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr30Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr30Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr30Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr30Sre {
        Pcr30Sre::from_bits(val)
    }
}
impl From<Pcr30Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr30Sre) -> u8 {
        Pcr30Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr31Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr31Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr31Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr31Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr31Dse {
        Pcr31Dse::from_bits(val)
    }
}
impl From<Pcr31Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr31Dse) -> u8 {
        Pcr31Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr31Dse1 {
    #[doc = "Normal."]
    DSE10 = 0x0,
    #[doc = "Double."]
    DSE11 = 0x01,
}
impl Pcr31Dse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr31Dse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr31Dse1 {
    #[inline(always)]
    fn from(val: u8) -> Pcr31Dse1 {
        Pcr31Dse1::from_bits(val)
    }
}
impl From<Pcr31Dse1> for u8 {
    #[inline(always)]
    fn from(val: Pcr31Dse1) -> u8 {
        Pcr31Dse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr31Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr31Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr31Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr31Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr31Ibe {
        Pcr31Ibe::from_bits(val)
    }
}
impl From<Pcr31Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr31Ibe) -> u8 {
        Pcr31Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr31Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr31Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr31Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr31Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr31Inv {
        Pcr31Inv::from_bits(val)
    }
}
impl From<Pcr31Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr31Inv) -> u8 {
        Pcr31Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr31Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr31Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr31Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr31Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr31Lk {
        Pcr31Lk::from_bits(val)
    }
}
impl From<Pcr31Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr31Lk) -> u8 {
        Pcr31Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr31Mux {
    #[doc = "Alternative 0 (GPIO)."]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)."]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)."]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)."]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)."]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)."]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)."]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)."]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)."]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)."]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)."]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)."]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)."]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)."]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr31Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr31Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr31Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr31Mux {
        Pcr31Mux::from_bits(val)
    }
}
impl From<Pcr31Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr31Mux) -> u8 {
        Pcr31Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr31Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr31Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr31Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr31Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr31Ode {
        Pcr31Ode::from_bits(val)
    }
}
impl From<Pcr31Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr31Ode) -> u8 {
        Pcr31Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr31Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr31Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr31Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr31Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr31Pe {
        Pcr31Pe::from_bits(val)
    }
}
impl From<Pcr31Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr31Pe) -> u8 {
        Pcr31Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr31Pfe {
    #[doc = "Disables."]
    PFE0 = 0x0,
    #[doc = "Enables."]
    PFE1 = 0x01,
}
impl Pcr31Pfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr31Pfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr31Pfe {
    #[inline(always)]
    fn from(val: u8) -> Pcr31Pfe {
        Pcr31Pfe::from_bits(val)
    }
}
impl From<Pcr31Pfe> for u8 {
    #[inline(always)]
    fn from(val: Pcr31Pfe) -> u8 {
        Pcr31Pfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr31Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr31Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr31Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr31Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr31Ps {
        Pcr31Ps::from_bits(val)
    }
}
impl From<Pcr31Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr31Ps) -> u8 {
        Pcr31Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr31Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr31Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr31Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr31Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr31Sre {
        Pcr31Sre::from_bits(val)
    }
}
impl From<Pcr31Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr31Sre) -> u8 {
        Pcr31Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr3Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Dse {
        Pcr3Dse::from_bits(val)
    }
}
impl From<Pcr3Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Dse) -> u8 {
        Pcr3Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr3Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Ibe {
        Pcr3Ibe::from_bits(val)
    }
}
impl From<Pcr3Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Ibe) -> u8 {
        Pcr3Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr3Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Inv {
        Pcr3Inv::from_bits(val)
    }
}
impl From<Pcr3Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Inv) -> u8 {
        Pcr3Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr3Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Lk {
        Pcr3Lk::from_bits(val)
    }
}
impl From<Pcr3Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Lk) -> u8 {
        Pcr3Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Mux {
    #[doc = "Alternative 0 (GPIO)."]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)."]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)."]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)."]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)."]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)."]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)."]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)."]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)."]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)."]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)."]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)."]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)."]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)."]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr3Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Mux {
        Pcr3Mux::from_bits(val)
    }
}
impl From<Pcr3Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Mux) -> u8 {
        Pcr3Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr3Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Ode {
        Pcr3Ode::from_bits(val)
    }
}
impl From<Pcr3Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Ode) -> u8 {
        Pcr3Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr3Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Pe {
        Pcr3Pe::from_bits(val)
    }
}
impl From<Pcr3Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Pe) -> u8 {
        Pcr3Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr3Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Ps {
        Pcr3Ps::from_bits(val)
    }
}
impl From<Pcr3Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Ps) -> u8 {
        Pcr3Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr3Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Sre {
        Pcr3Sre::from_bits(val)
    }
}
impl From<Pcr3Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Sre) -> u8 {
        Pcr3Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr4Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Dse {
        Pcr4Dse::from_bits(val)
    }
}
impl From<Pcr4Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Dse) -> u8 {
        Pcr4Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr4Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Ibe {
        Pcr4Ibe::from_bits(val)
    }
}
impl From<Pcr4Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Ibe) -> u8 {
        Pcr4Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr4Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Inv {
        Pcr4Inv::from_bits(val)
    }
}
impl From<Pcr4Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Inv) -> u8 {
        Pcr4Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr4Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Lk {
        Pcr4Lk::from_bits(val)
    }
}
impl From<Pcr4Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Lk) -> u8 {
        Pcr4Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Mux {
    #[doc = "Alternative 0 (GPIO)."]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)."]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)."]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)."]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)."]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)."]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)."]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)."]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)."]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)."]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)."]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)."]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)."]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)."]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr4Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Mux {
        Pcr4Mux::from_bits(val)
    }
}
impl From<Pcr4Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Mux) -> u8 {
        Pcr4Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr4Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Ode {
        Pcr4Ode::from_bits(val)
    }
}
impl From<Pcr4Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Ode) -> u8 {
        Pcr4Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr4Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Pe {
        Pcr4Pe::from_bits(val)
    }
}
impl From<Pcr4Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Pe) -> u8 {
        Pcr4Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr4Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Ps {
        Pcr4Ps::from_bits(val)
    }
}
impl From<Pcr4Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Ps) -> u8 {
        Pcr4Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr4Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Sre {
        Pcr4Sre::from_bits(val)
    }
}
impl From<Pcr4Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Sre) -> u8 {
        Pcr4Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr5Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Dse {
        Pcr5Dse::from_bits(val)
    }
}
impl From<Pcr5Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Dse) -> u8 {
        Pcr5Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr5Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Ibe {
        Pcr5Ibe::from_bits(val)
    }
}
impl From<Pcr5Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Ibe) -> u8 {
        Pcr5Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr5Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Inv {
        Pcr5Inv::from_bits(val)
    }
}
impl From<Pcr5Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Inv) -> u8 {
        Pcr5Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr5Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Lk {
        Pcr5Lk::from_bits(val)
    }
}
impl From<Pcr5Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Lk) -> u8 {
        Pcr5Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Mux {
    #[doc = "Alternative 0 (GPIO)."]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)."]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)."]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)."]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)."]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)."]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)."]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)."]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)."]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)."]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)."]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)."]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)."]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)."]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr5Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Mux {
        Pcr5Mux::from_bits(val)
    }
}
impl From<Pcr5Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Mux) -> u8 {
        Pcr5Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr5Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Ode {
        Pcr5Ode::from_bits(val)
    }
}
impl From<Pcr5Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Ode) -> u8 {
        Pcr5Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr5Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Pe {
        Pcr5Pe::from_bits(val)
    }
}
impl From<Pcr5Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Pe) -> u8 {
        Pcr5Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr5Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Ps {
        Pcr5Ps::from_bits(val)
    }
}
impl From<Pcr5Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Ps) -> u8 {
        Pcr5Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr5Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Sre {
        Pcr5Sre::from_bits(val)
    }
}
impl From<Pcr5Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Sre) -> u8 {
        Pcr5Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr6Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Dse {
        Pcr6Dse::from_bits(val)
    }
}
impl From<Pcr6Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Dse) -> u8 {
        Pcr6Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr6Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Ibe {
        Pcr6Ibe::from_bits(val)
    }
}
impl From<Pcr6Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Ibe) -> u8 {
        Pcr6Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr6Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Inv {
        Pcr6Inv::from_bits(val)
    }
}
impl From<Pcr6Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Inv) -> u8 {
        Pcr6Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr6Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Lk {
        Pcr6Lk::from_bits(val)
    }
}
impl From<Pcr6Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Lk) -> u8 {
        Pcr6Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Mux {
    #[doc = "Alternative 0 (GPIO)."]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)."]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)."]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)."]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)."]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)."]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)."]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)."]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)."]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)."]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)."]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)."]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)."]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)."]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr6Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Mux {
        Pcr6Mux::from_bits(val)
    }
}
impl From<Pcr6Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Mux) -> u8 {
        Pcr6Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr6Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Ode {
        Pcr6Ode::from_bits(val)
    }
}
impl From<Pcr6Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Ode) -> u8 {
        Pcr6Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr6Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Pe {
        Pcr6Pe::from_bits(val)
    }
}
impl From<Pcr6Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Pe) -> u8 {
        Pcr6Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr6Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Ps {
        Pcr6Ps::from_bits(val)
    }
}
impl From<Pcr6Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Ps) -> u8 {
        Pcr6Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr6Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Sre {
        Pcr6Sre::from_bits(val)
    }
}
impl From<Pcr6Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Sre) -> u8 {
        Pcr6Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr7Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Dse {
        Pcr7Dse::from_bits(val)
    }
}
impl From<Pcr7Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Dse) -> u8 {
        Pcr7Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr7Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Ibe {
        Pcr7Ibe::from_bits(val)
    }
}
impl From<Pcr7Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Ibe) -> u8 {
        Pcr7Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr7Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Inv {
        Pcr7Inv::from_bits(val)
    }
}
impl From<Pcr7Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Inv) -> u8 {
        Pcr7Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr7Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Lk {
        Pcr7Lk::from_bits(val)
    }
}
impl From<Pcr7Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Lk) -> u8 {
        Pcr7Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Mux {
    #[doc = "Alternative 0 (GPIO)."]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)."]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)."]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)."]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)."]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)."]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)."]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)."]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)."]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)."]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)."]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)."]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)."]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)."]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr7Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Mux {
        Pcr7Mux::from_bits(val)
    }
}
impl From<Pcr7Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Mux) -> u8 {
        Pcr7Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr7Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Ode {
        Pcr7Ode::from_bits(val)
    }
}
impl From<Pcr7Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Ode) -> u8 {
        Pcr7Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr7Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Pe {
        Pcr7Pe::from_bits(val)
    }
}
impl From<Pcr7Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Pe) -> u8 {
        Pcr7Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr7Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Ps {
        Pcr7Ps::from_bits(val)
    }
}
impl From<Pcr7Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Ps) -> u8 {
        Pcr7Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr7Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Sre {
        Pcr7Sre::from_bits(val)
    }
}
impl From<Pcr7Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Sre) -> u8 {
        Pcr7Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr8Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Dse {
        Pcr8Dse::from_bits(val)
    }
}
impl From<Pcr8Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Dse) -> u8 {
        Pcr8Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Dse1 {
    #[doc = "Normal."]
    DSE10 = 0x0,
    #[doc = "Double."]
    DSE11 = 0x01,
}
impl Pcr8Dse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Dse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Dse1 {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Dse1 {
        Pcr8Dse1::from_bits(val)
    }
}
impl From<Pcr8Dse1> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Dse1) -> u8 {
        Pcr8Dse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr8Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Ibe {
        Pcr8Ibe::from_bits(val)
    }
}
impl From<Pcr8Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Ibe) -> u8 {
        Pcr8Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr8Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Inv {
        Pcr8Inv::from_bits(val)
    }
}
impl From<Pcr8Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Inv) -> u8 {
        Pcr8Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr8Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Lk {
        Pcr8Lk::from_bits(val)
    }
}
impl From<Pcr8Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Lk) -> u8 {
        Pcr8Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Mux {
    #[doc = "Alternative 0 (GPIO)."]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)."]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)."]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)."]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)."]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)."]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)."]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)."]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)."]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)."]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)."]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)."]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)."]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)."]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr8Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Mux {
        Pcr8Mux::from_bits(val)
    }
}
impl From<Pcr8Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Mux) -> u8 {
        Pcr8Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr8Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Ode {
        Pcr8Ode::from_bits(val)
    }
}
impl From<Pcr8Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Ode) -> u8 {
        Pcr8Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr8Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Pe {
        Pcr8Pe::from_bits(val)
    }
}
impl From<Pcr8Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Pe) -> u8 {
        Pcr8Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Pfe {
    #[doc = "Disables."]
    PFE0 = 0x0,
    #[doc = "Enables."]
    PFE1 = 0x01,
}
impl Pcr8Pfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Pfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Pfe {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Pfe {
        Pcr8Pfe::from_bits(val)
    }
}
impl From<Pcr8Pfe> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Pfe) -> u8 {
        Pcr8Pfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr8Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Ps {
        Pcr8Ps::from_bits(val)
    }
}
impl From<Pcr8Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Ps) -> u8 {
        Pcr8Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Pv {
    #[doc = "Low."]
    PV0 = 0x0,
    #[doc = "High."]
    PV1 = 0x01,
}
impl Pcr8Pv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Pv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Pv {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Pv {
        Pcr8Pv::from_bits(val)
    }
}
impl From<Pcr8Pv> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Pv) -> u8 {
        Pcr8Pv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr8Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Sre {
        Pcr8Sre::from_bits(val)
    }
}
impl From<Pcr8Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Sre) -> u8 {
        Pcr8Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Dse {
    #[doc = "Low."]
    DSE0 = 0x0,
    #[doc = "High."]
    DSE1 = 0x01,
}
impl Pcr9Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Dse {
        Pcr9Dse::from_bits(val)
    }
}
impl From<Pcr9Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Dse) -> u8 {
        Pcr9Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Dse1 {
    #[doc = "Normal."]
    DSE10 = 0x0,
    #[doc = "Double."]
    DSE11 = 0x01,
}
impl Pcr9Dse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Dse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Dse1 {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Dse1 {
        Pcr9Dse1::from_bits(val)
    }
}
impl From<Pcr9Dse1> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Dse1) -> u8 {
        Pcr9Dse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Ibe {
    #[doc = "Disables."]
    IBE0 = 0x0,
    #[doc = "Enables."]
    IBE1 = 0x01,
}
impl Pcr9Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Ibe {
        Pcr9Ibe::from_bits(val)
    }
}
impl From<Pcr9Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Ibe) -> u8 {
        Pcr9Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Inv {
    #[doc = "Does not invert."]
    INV0 = 0x0,
    #[doc = "Inverts."]
    INV1 = 0x01,
}
impl Pcr9Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Inv {
        Pcr9Inv::from_bits(val)
    }
}
impl From<Pcr9Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Inv) -> u8 {
        Pcr9Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Lk {
    #[doc = "Does not lock."]
    LK0 = 0x0,
    #[doc = "Locks."]
    LK1 = 0x01,
}
impl Pcr9Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Lk {
        Pcr9Lk::from_bits(val)
    }
}
impl From<Pcr9Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Lk) -> u8 {
        Pcr9Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Mux {
    #[doc = "Alternative 0 (GPIO)."]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)."]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)."]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)."]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)."]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)."]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)."]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)."]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)."]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)."]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)."]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)."]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)."]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)."]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr9Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Mux {
        Pcr9Mux::from_bits(val)
    }
}
impl From<Pcr9Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Mux) -> u8 {
        Pcr9Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Ode {
    #[doc = "Disables."]
    ODE0 = 0x0,
    #[doc = "Enables."]
    ODE1 = 0x01,
}
impl Pcr9Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Ode {
        Pcr9Ode::from_bits(val)
    }
}
impl From<Pcr9Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Ode) -> u8 {
        Pcr9Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Pe {
    #[doc = "Disables."]
    PE0 = 0x0,
    #[doc = "Enables."]
    PE1 = 0x01,
}
impl Pcr9Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Pe {
        Pcr9Pe::from_bits(val)
    }
}
impl From<Pcr9Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Pe) -> u8 {
        Pcr9Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Pfe {
    #[doc = "Disables."]
    PFE0 = 0x0,
    #[doc = "Enables."]
    PFE1 = 0x01,
}
impl Pcr9Pfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Pfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Pfe {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Pfe {
        Pcr9Pfe::from_bits(val)
    }
}
impl From<Pcr9Pfe> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Pfe) -> u8 {
        Pcr9Pfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Ps {
    #[doc = "Enables internal pulldown resistor."]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor."]
    PS1 = 0x01,
}
impl Pcr9Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Ps {
        Pcr9Ps::from_bits(val)
    }
}
impl From<Pcr9Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Ps) -> u8 {
        Pcr9Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Sre {
    #[doc = "Fast."]
    SRE0 = 0x0,
    #[doc = "Slow."]
    SRE1 = 0x01,
}
impl Pcr9Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Sre {
        Pcr9Sre::from_bits(val)
    }
}
impl From<Pcr9Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Sre) -> u8 {
        Pcr9Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Range {
    #[doc = "1.71 V-3.6 V."]
    RANGE0 = 0x0,
    #[doc = "2.70 V-3.6 V."]
    RANGE1 = 0x01,
}
impl Range {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Range {
        unsafe { core::mem::transmute(val & 0x01) }
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
pub mod common {
    use core::marker::PhantomData;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct RW;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct R;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct W;
    mod sealed {
        use super::*;
        pub trait Access {}
        impl Access for R {}
        impl Access for W {}
        impl Access for RW {}
    }
    pub trait Access: sealed::Access + Copy {}
    impl Access for R {}
    impl Access for W {}
    impl Access for RW {}
    pub trait Read: Access {}
    impl Read for RW {}
    impl Read for R {}
    pub trait Write: Access {}
    impl Write for RW {}
    impl Write for W {}
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct Reg<T: Copy, A: Access> {
        ptr: *mut u8,
        phantom: PhantomData<*mut (T, A)>,
    }
    unsafe impl<T: Copy, A: Access> Send for Reg<T, A> {}
    unsafe impl<T: Copy, A: Access> Sync for Reg<T, A> {}
    impl<T: Copy, A: Access> Reg<T, A> {
        #[allow(clippy::missing_safety_doc)]
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut T) -> Self {
            Self {
                ptr: ptr as _,
                phantom: PhantomData,
            }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut T {
            self.ptr as _
        }
    }
    impl<T: Copy, A: Read> Reg<T, A> {
        #[inline(always)]
        pub fn read(&self) -> T {
            unsafe { (self.ptr as *mut T).read_volatile() }
        }
    }
    impl<T: Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write_value(&self, val: T) {
            unsafe { (self.ptr as *mut T).write_volatile(val) }
        }
    }
    impl<T: Default + Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write(&self, f: impl FnOnce(&mut T)) {
            let mut val = Default::default();
            f(&mut val);
            self.write_value(val);
        }
    }
    impl<T: Copy, A: Read + Write> Reg<T, A> {
        #[inline(always)]
        pub fn modify(&self, f: impl FnOnce(&mut T)) {
            let mut val = self.read();
            f(&mut val);
            self.write_value(val);
        }
    }
}
