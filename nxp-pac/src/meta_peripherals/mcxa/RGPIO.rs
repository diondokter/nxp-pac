#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6a8c2aa 2026-01-27))"]
#[doc = "GPIO."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rgpio {
    ptr: *mut u8,
}
unsafe impl Send for Rgpio {}
unsafe impl Sync for Rgpio {}
impl Rgpio {
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
    pub const fn verid(self) -> crate::pac::common::Reg<Verid, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Parameter."]
    #[inline(always)]
    pub const fn param(self) -> crate::pac::common::Reg<Param, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn pdor(self) -> crate::pac::common::Reg<Pdor, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn psor(self) -> crate::pac::common::Reg<Psor, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn pcor(self) -> crate::pac::common::Reg<Pcor, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn ptor(self) -> crate::pac::common::Reg<Ptor, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn pdir(self) -> crate::pac::common::Reg<Pdir, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn pddr(self) -> crate::pac::common::Reg<Pddr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn pidr(self) -> crate::pac::common::Reg<Pidr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Pin Data."]
    #[inline(always)]
    pub const fn pdr(self, n: usize) -> crate::pac::common::Reg<Pdr, crate::pac::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 1usize) as _)
        }
    }
    #[doc = "Interrupt Control index."]
    #[inline(always)]
    pub const fn icr(self, n: usize) -> crate::pac::common::Reg<Icr, crate::pac::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _)
        }
    }
    #[doc = "Global Interrupt Control Low."]
    #[inline(always)]
    pub const fn giclr(self) -> crate::pac::common::Reg<Giclr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Global Interrupt Control High."]
    #[inline(always)]
    pub const fn gichr(self) -> crate::pac::common::Reg<Gichr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn isfr0(self) -> crate::pac::common::Reg<Isfr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
}
#[doc = "Global Interrupt Control High."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gichr(pub u32);
impl Gichr {
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe16(&self) -> Giwe16 {
        let val = (self.0 >> 0usize) & 0x01;
        Giwe16::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe16(&mut self, val: Giwe16) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe17(&self) -> Giwe17 {
        let val = (self.0 >> 1usize) & 0x01;
        Giwe17::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe17(&mut self, val: Giwe17) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe18(&self) -> Giwe18 {
        let val = (self.0 >> 2usize) & 0x01;
        Giwe18::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe18(&mut self, val: Giwe18) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe19(&self) -> Giwe19 {
        let val = (self.0 >> 3usize) & 0x01;
        Giwe19::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe19(&mut self, val: Giwe19) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe20(&self) -> Giwe20 {
        let val = (self.0 >> 4usize) & 0x01;
        Giwe20::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe20(&mut self, val: Giwe20) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe21(&self) -> Giwe21 {
        let val = (self.0 >> 5usize) & 0x01;
        Giwe21::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe21(&mut self, val: Giwe21) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe22(&self) -> Giwe22 {
        let val = (self.0 >> 6usize) & 0x01;
        Giwe22::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe22(&mut self, val: Giwe22) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe23(&self) -> Giwe23 {
        let val = (self.0 >> 7usize) & 0x01;
        Giwe23::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe23(&mut self, val: Giwe23) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe24(&self) -> Giwe24 {
        let val = (self.0 >> 8usize) & 0x01;
        Giwe24::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe24(&mut self, val: Giwe24) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe25(&self) -> Giwe25 {
        let val = (self.0 >> 9usize) & 0x01;
        Giwe25::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe25(&mut self, val: Giwe25) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe26(&self) -> Giwe26 {
        let val = (self.0 >> 10usize) & 0x01;
        Giwe26::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe26(&mut self, val: Giwe26) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe27(&self) -> Giwe27 {
        let val = (self.0 >> 11usize) & 0x01;
        Giwe27::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe27(&mut self, val: Giwe27) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe28(&self) -> Giwe28 {
        let val = (self.0 >> 12usize) & 0x01;
        Giwe28::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe28(&mut self, val: Giwe28) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe29(&self) -> Giwe29 {
        let val = (self.0 >> 13usize) & 0x01;
        Giwe29::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe29(&mut self, val: Giwe29) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe30(&self) -> Giwe30 {
        let val = (self.0 >> 14usize) & 0x01;
        Giwe30::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe30(&mut self, val: Giwe30) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe31(&self) -> Giwe31 {
        let val = (self.0 >> 15usize) & 0x01;
        Giwe31::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe31(&mut self, val: Giwe31) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Global Interrupt Write Data."]
    #[must_use]
    #[inline(always)]
    pub const fn giwd(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Global Interrupt Write Data."]
    #[inline(always)]
    pub const fn set_giwd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Gichr {
    #[inline(always)]
    fn default() -> Gichr {
        Gichr(0)
    }
}
impl core::fmt::Debug for Gichr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gichr")
            .field("giwe16", &self.giwe16())
            .field("giwe17", &self.giwe17())
            .field("giwe18", &self.giwe18())
            .field("giwe19", &self.giwe19())
            .field("giwe20", &self.giwe20())
            .field("giwe21", &self.giwe21())
            .field("giwe22", &self.giwe22())
            .field("giwe23", &self.giwe23())
            .field("giwe24", &self.giwe24())
            .field("giwe25", &self.giwe25())
            .field("giwe26", &self.giwe26())
            .field("giwe27", &self.giwe27())
            .field("giwe28", &self.giwe28())
            .field("giwe29", &self.giwe29())
            .field("giwe30", &self.giwe30())
            .field("giwe31", &self.giwe31())
            .field("giwd", &self.giwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gichr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gichr {{ giwe16: {:?}, giwe17: {:?}, giwe18: {:?}, giwe19: {:?}, giwe20: {:?}, giwe21: {:?}, giwe22: {:?}, giwe23: {:?}, giwe24: {:?}, giwe25: {:?}, giwe26: {:?}, giwe27: {:?}, giwe28: {:?}, giwe29: {:?}, giwe30: {:?}, giwe31: {:?}, giwd: {=u16:?} }}",
            self.giwe16(),
            self.giwe17(),
            self.giwe18(),
            self.giwe19(),
            self.giwe20(),
            self.giwe21(),
            self.giwe22(),
            self.giwe23(),
            self.giwe24(),
            self.giwe25(),
            self.giwe26(),
            self.giwe27(),
            self.giwe28(),
            self.giwe29(),
            self.giwe30(),
            self.giwe31(),
            self.giwd()
        )
    }
}
#[doc = "Global Interrupt Control Low."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Giclr(pub u32);
impl Giclr {
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe0(&self) -> Giwe0 {
        let val = (self.0 >> 0usize) & 0x01;
        Giwe0::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe0(&mut self, val: Giwe0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe1(&self) -> Giwe1 {
        let val = (self.0 >> 1usize) & 0x01;
        Giwe1::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe1(&mut self, val: Giwe1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe2(&self) -> Giwe2 {
        let val = (self.0 >> 2usize) & 0x01;
        Giwe2::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe2(&mut self, val: Giwe2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe3(&self) -> Giwe3 {
        let val = (self.0 >> 3usize) & 0x01;
        Giwe3::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe3(&mut self, val: Giwe3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe4(&self) -> Giwe4 {
        let val = (self.0 >> 4usize) & 0x01;
        Giwe4::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe4(&mut self, val: Giwe4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe5(&self) -> Giwe5 {
        let val = (self.0 >> 5usize) & 0x01;
        Giwe5::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe5(&mut self, val: Giwe5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe6(&self) -> Giwe6 {
        let val = (self.0 >> 6usize) & 0x01;
        Giwe6::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe6(&mut self, val: Giwe6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe7(&self) -> Giwe7 {
        let val = (self.0 >> 7usize) & 0x01;
        Giwe7::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe7(&mut self, val: Giwe7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe8(&self) -> Giwe8 {
        let val = (self.0 >> 8usize) & 0x01;
        Giwe8::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe8(&mut self, val: Giwe8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe9(&self) -> Giwe9 {
        let val = (self.0 >> 9usize) & 0x01;
        Giwe9::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe9(&mut self, val: Giwe9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe10(&self) -> Giwe10 {
        let val = (self.0 >> 10usize) & 0x01;
        Giwe10::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe10(&mut self, val: Giwe10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe11(&self) -> Giwe11 {
        let val = (self.0 >> 11usize) & 0x01;
        Giwe11::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe11(&mut self, val: Giwe11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe12(&self) -> Giwe12 {
        let val = (self.0 >> 12usize) & 0x01;
        Giwe12::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe12(&mut self, val: Giwe12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe13(&self) -> Giwe13 {
        let val = (self.0 >> 13usize) & 0x01;
        Giwe13::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe13(&mut self, val: Giwe13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe14(&self) -> Giwe14 {
        let val = (self.0 >> 14usize) & 0x01;
        Giwe14::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe14(&mut self, val: Giwe14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe15(&self) -> Giwe15 {
        let val = (self.0 >> 15usize) & 0x01;
        Giwe15::from_bits(val as u8)
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe15(&mut self, val: Giwe15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Global Interrupt Write Data."]
    #[must_use]
    #[inline(always)]
    pub const fn giwd(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Global Interrupt Write Data."]
    #[inline(always)]
    pub const fn set_giwd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Giclr {
    #[inline(always)]
    fn default() -> Giclr {
        Giclr(0)
    }
}
impl core::fmt::Debug for Giclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Giclr")
            .field("giwe0", &self.giwe0())
            .field("giwe1", &self.giwe1())
            .field("giwe2", &self.giwe2())
            .field("giwe3", &self.giwe3())
            .field("giwe4", &self.giwe4())
            .field("giwe5", &self.giwe5())
            .field("giwe6", &self.giwe6())
            .field("giwe7", &self.giwe7())
            .field("giwe8", &self.giwe8())
            .field("giwe9", &self.giwe9())
            .field("giwe10", &self.giwe10())
            .field("giwe11", &self.giwe11())
            .field("giwe12", &self.giwe12())
            .field("giwe13", &self.giwe13())
            .field("giwe14", &self.giwe14())
            .field("giwe15", &self.giwe15())
            .field("giwd", &self.giwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Giclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Giclr {{ giwe0: {:?}, giwe1: {:?}, giwe2: {:?}, giwe3: {:?}, giwe4: {:?}, giwe5: {:?}, giwe6: {:?}, giwe7: {:?}, giwe8: {:?}, giwe9: {:?}, giwe10: {:?}, giwe11: {:?}, giwe12: {:?}, giwe13: {:?}, giwe14: {:?}, giwe15: {:?}, giwd: {=u16:?} }}",
            self.giwe0(),
            self.giwe1(),
            self.giwe2(),
            self.giwe3(),
            self.giwe4(),
            self.giwe5(),
            self.giwe6(),
            self.giwe7(),
            self.giwe8(),
            self.giwe9(),
            self.giwe10(),
            self.giwe11(),
            self.giwe12(),
            self.giwe13(),
            self.giwe14(),
            self.giwe15(),
            self.giwd()
        )
    }
}
#[doc = "Interrupt Control index."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc = "Interrupt Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration."]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> Isf {
        let val = (self.0 >> 24usize) & 0x01;
        Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        Icr(0)
    }
}
impl core::fmt::Debug for Icr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr")
            .field("irqc", &self.irqc())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr {{ irqc: {:?}, isf: {:?} }}",
            self.irqc(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Status Flag."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isfr0(pub u32);
impl Isfr0 {
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf0(&self) -> Isf0 {
        let val = (self.0 >> 0usize) & 0x01;
        Isf0::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf0(&mut self, val: Isf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf1(&self) -> Isf1 {
        let val = (self.0 >> 1usize) & 0x01;
        Isf1::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf1(&mut self, val: Isf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf2(&self) -> Isf2 {
        let val = (self.0 >> 2usize) & 0x01;
        Isf2::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf2(&mut self, val: Isf2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf3(&self) -> Isf3 {
        let val = (self.0 >> 3usize) & 0x01;
        Isf3::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf3(&mut self, val: Isf3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf4(&self) -> Isf4 {
        let val = (self.0 >> 4usize) & 0x01;
        Isf4::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf4(&mut self, val: Isf4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf5(&self) -> Isf5 {
        let val = (self.0 >> 5usize) & 0x01;
        Isf5::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf5(&mut self, val: Isf5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf6(&self) -> Isf6 {
        let val = (self.0 >> 6usize) & 0x01;
        Isf6::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf6(&mut self, val: Isf6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf7(&self) -> Isf7 {
        let val = (self.0 >> 7usize) & 0x01;
        Isf7::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf7(&mut self, val: Isf7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf8(&self) -> Isf8 {
        let val = (self.0 >> 8usize) & 0x01;
        Isf8::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf8(&mut self, val: Isf8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf9(&self) -> Isf9 {
        let val = (self.0 >> 9usize) & 0x01;
        Isf9::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf9(&mut self, val: Isf9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf10(&self) -> Isf10 {
        let val = (self.0 >> 10usize) & 0x01;
        Isf10::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf10(&mut self, val: Isf10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf11(&self) -> Isf11 {
        let val = (self.0 >> 11usize) & 0x01;
        Isf11::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf11(&mut self, val: Isf11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf12(&self) -> Isf12 {
        let val = (self.0 >> 12usize) & 0x01;
        Isf12::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf12(&mut self, val: Isf12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf13(&self) -> Isf13 {
        let val = (self.0 >> 13usize) & 0x01;
        Isf13::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf13(&mut self, val: Isf13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf14(&self) -> Isf14 {
        let val = (self.0 >> 14usize) & 0x01;
        Isf14::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf14(&mut self, val: Isf14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf15(&self) -> Isf15 {
        let val = (self.0 >> 15usize) & 0x01;
        Isf15::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf15(&mut self, val: Isf15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf16(&self) -> Isf16 {
        let val = (self.0 >> 16usize) & 0x01;
        Isf16::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf16(&mut self, val: Isf16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf17(&self) -> Isf17 {
        let val = (self.0 >> 17usize) & 0x01;
        Isf17::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf17(&mut self, val: Isf17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf18(&self) -> Isf18 {
        let val = (self.0 >> 18usize) & 0x01;
        Isf18::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf18(&mut self, val: Isf18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf19(&self) -> Isf19 {
        let val = (self.0 >> 19usize) & 0x01;
        Isf19::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf19(&mut self, val: Isf19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf20(&self) -> Isf20 {
        let val = (self.0 >> 20usize) & 0x01;
        Isf20::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf20(&mut self, val: Isf20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf21(&self) -> Isf21 {
        let val = (self.0 >> 21usize) & 0x01;
        Isf21::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf21(&mut self, val: Isf21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf22(&self) -> Isf22 {
        let val = (self.0 >> 22usize) & 0x01;
        Isf22::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf22(&mut self, val: Isf22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf23(&self) -> Isf23 {
        let val = (self.0 >> 23usize) & 0x01;
        Isf23::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf23(&mut self, val: Isf23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf24(&self) -> Isf24 {
        let val = (self.0 >> 24usize) & 0x01;
        Isf24::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf24(&mut self, val: Isf24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf25(&self) -> Isf25 {
        let val = (self.0 >> 25usize) & 0x01;
        Isf25::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf25(&mut self, val: Isf25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf26(&self) -> Isf26 {
        let val = (self.0 >> 26usize) & 0x01;
        Isf26::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf26(&mut self, val: Isf26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf27(&self) -> Isf27 {
        let val = (self.0 >> 27usize) & 0x01;
        Isf27::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf27(&mut self, val: Isf27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf28(&self) -> Isf28 {
        let val = (self.0 >> 28usize) & 0x01;
        Isf28::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf28(&mut self, val: Isf28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf29(&self) -> Isf29 {
        let val = (self.0 >> 29usize) & 0x01;
        Isf29::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf29(&mut self, val: Isf29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf30(&self) -> Isf30 {
        let val = (self.0 >> 30usize) & 0x01;
        Isf30::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf30(&mut self, val: Isf30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf31(&self) -> Isf31 {
        let val = (self.0 >> 31usize) & 0x01;
        Isf31::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf31(&mut self, val: Isf31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Isfr0 {
    #[inline(always)]
    fn default() -> Isfr0 {
        Isfr0(0)
    }
}
impl core::fmt::Debug for Isfr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isfr0")
            .field("isf0", &self.isf0())
            .field("isf1", &self.isf1())
            .field("isf2", &self.isf2())
            .field("isf3", &self.isf3())
            .field("isf4", &self.isf4())
            .field("isf5", &self.isf5())
            .field("isf6", &self.isf6())
            .field("isf7", &self.isf7())
            .field("isf8", &self.isf8())
            .field("isf9", &self.isf9())
            .field("isf10", &self.isf10())
            .field("isf11", &self.isf11())
            .field("isf12", &self.isf12())
            .field("isf13", &self.isf13())
            .field("isf14", &self.isf14())
            .field("isf15", &self.isf15())
            .field("isf16", &self.isf16())
            .field("isf17", &self.isf17())
            .field("isf18", &self.isf18())
            .field("isf19", &self.isf19())
            .field("isf20", &self.isf20())
            .field("isf21", &self.isf21())
            .field("isf22", &self.isf22())
            .field("isf23", &self.isf23())
            .field("isf24", &self.isf24())
            .field("isf25", &self.isf25())
            .field("isf26", &self.isf26())
            .field("isf27", &self.isf27())
            .field("isf28", &self.isf28())
            .field("isf29", &self.isf29())
            .field("isf30", &self.isf30())
            .field("isf31", &self.isf31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isfr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Isfr0 {{ isf0: {:?}, isf1: {:?}, isf2: {:?}, isf3: {:?}, isf4: {:?}, isf5: {:?}, isf6: {:?}, isf7: {:?}, isf8: {:?}, isf9: {:?}, isf10: {:?}, isf11: {:?}, isf12: {:?}, isf13: {:?}, isf14: {:?}, isf15: {:?}, isf16: {:?}, isf17: {:?}, isf18: {:?}, isf19: {:?}, isf20: {:?}, isf21: {:?}, isf22: {:?}, isf23: {:?}, isf24: {:?}, isf25: {:?}, isf26: {:?}, isf27: {:?}, isf28: {:?}, isf29: {:?}, isf30: {:?}, isf31: {:?} }}",
            self.isf0(),
            self.isf1(),
            self.isf2(),
            self.isf3(),
            self.isf4(),
            self.isf5(),
            self.isf6(),
            self.isf7(),
            self.isf8(),
            self.isf9(),
            self.isf10(),
            self.isf11(),
            self.isf12(),
            self.isf13(),
            self.isf14(),
            self.isf15(),
            self.isf16(),
            self.isf17(),
            self.isf18(),
            self.isf19(),
            self.isf20(),
            self.isf21(),
            self.isf22(),
            self.isf23(),
            self.isf24(),
            self.isf25(),
            self.isf26(),
            self.isf27(),
            self.isf28(),
            self.isf29(),
            self.isf30(),
            self.isf31()
        )
    }
}
#[doc = "Parameter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Interrupt Number."]
    #[must_use]
    #[inline(always)]
    pub const fn irqnum(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Interrupt Number."]
    #[inline(always)]
    pub const fn set_irqnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
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
            .field("irqnum", &self.irqnum())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Param {{ irqnum: {=u8:?} }}", self.irqnum())
    }
}
#[doc = "Port Clear Output."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcor(pub u32);
impl Pcor {
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco0(&self) -> Ptco0 {
        let val = (self.0 >> 0usize) & 0x01;
        Ptco0::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco0(&mut self, val: Ptco0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco1(&self) -> Ptco1 {
        let val = (self.0 >> 1usize) & 0x01;
        Ptco1::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco1(&mut self, val: Ptco1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco2(&self) -> Ptco2 {
        let val = (self.0 >> 2usize) & 0x01;
        Ptco2::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco2(&mut self, val: Ptco2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco3(&self) -> Ptco3 {
        let val = (self.0 >> 3usize) & 0x01;
        Ptco3::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco3(&mut self, val: Ptco3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco4(&self) -> Ptco4 {
        let val = (self.0 >> 4usize) & 0x01;
        Ptco4::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco4(&mut self, val: Ptco4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco5(&self) -> Ptco5 {
        let val = (self.0 >> 5usize) & 0x01;
        Ptco5::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco5(&mut self, val: Ptco5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco6(&self) -> Ptco6 {
        let val = (self.0 >> 6usize) & 0x01;
        Ptco6::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco6(&mut self, val: Ptco6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco7(&self) -> Ptco7 {
        let val = (self.0 >> 7usize) & 0x01;
        Ptco7::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco7(&mut self, val: Ptco7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco8(&self) -> Ptco8 {
        let val = (self.0 >> 8usize) & 0x01;
        Ptco8::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco8(&mut self, val: Ptco8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco9(&self) -> Ptco9 {
        let val = (self.0 >> 9usize) & 0x01;
        Ptco9::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco9(&mut self, val: Ptco9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco10(&self) -> Ptco10 {
        let val = (self.0 >> 10usize) & 0x01;
        Ptco10::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco10(&mut self, val: Ptco10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco11(&self) -> Ptco11 {
        let val = (self.0 >> 11usize) & 0x01;
        Ptco11::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco11(&mut self, val: Ptco11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco12(&self) -> Ptco12 {
        let val = (self.0 >> 12usize) & 0x01;
        Ptco12::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco12(&mut self, val: Ptco12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco13(&self) -> Ptco13 {
        let val = (self.0 >> 13usize) & 0x01;
        Ptco13::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco13(&mut self, val: Ptco13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco14(&self) -> Ptco14 {
        let val = (self.0 >> 14usize) & 0x01;
        Ptco14::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco14(&mut self, val: Ptco14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco15(&self) -> Ptco15 {
        let val = (self.0 >> 15usize) & 0x01;
        Ptco15::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco15(&mut self, val: Ptco15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco16(&self) -> Ptco16 {
        let val = (self.0 >> 16usize) & 0x01;
        Ptco16::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco16(&mut self, val: Ptco16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco17(&self) -> Ptco17 {
        let val = (self.0 >> 17usize) & 0x01;
        Ptco17::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco17(&mut self, val: Ptco17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco18(&self) -> Ptco18 {
        let val = (self.0 >> 18usize) & 0x01;
        Ptco18::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco18(&mut self, val: Ptco18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco19(&self) -> Ptco19 {
        let val = (self.0 >> 19usize) & 0x01;
        Ptco19::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco19(&mut self, val: Ptco19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco20(&self) -> Ptco20 {
        let val = (self.0 >> 20usize) & 0x01;
        Ptco20::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco20(&mut self, val: Ptco20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco21(&self) -> Ptco21 {
        let val = (self.0 >> 21usize) & 0x01;
        Ptco21::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco21(&mut self, val: Ptco21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco22(&self) -> Ptco22 {
        let val = (self.0 >> 22usize) & 0x01;
        Ptco22::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco22(&mut self, val: Ptco22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco23(&self) -> Ptco23 {
        let val = (self.0 >> 23usize) & 0x01;
        Ptco23::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco23(&mut self, val: Ptco23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco24(&self) -> Ptco24 {
        let val = (self.0 >> 24usize) & 0x01;
        Ptco24::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco24(&mut self, val: Ptco24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco25(&self) -> Ptco25 {
        let val = (self.0 >> 25usize) & 0x01;
        Ptco25::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco25(&mut self, val: Ptco25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco26(&self) -> Ptco26 {
        let val = (self.0 >> 26usize) & 0x01;
        Ptco26::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco26(&mut self, val: Ptco26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco27(&self) -> Ptco27 {
        let val = (self.0 >> 27usize) & 0x01;
        Ptco27::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco27(&mut self, val: Ptco27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco28(&self) -> Ptco28 {
        let val = (self.0 >> 28usize) & 0x01;
        Ptco28::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco28(&mut self, val: Ptco28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco29(&self) -> Ptco29 {
        let val = (self.0 >> 29usize) & 0x01;
        Ptco29::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco29(&mut self, val: Ptco29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco30(&self) -> Ptco30 {
        let val = (self.0 >> 30usize) & 0x01;
        Ptco30::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco30(&mut self, val: Ptco30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco31(&self) -> Ptco31 {
        let val = (self.0 >> 31usize) & 0x01;
        Ptco31::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco31(&mut self, val: Ptco31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pcor {
    #[inline(always)]
    fn default() -> Pcor {
        Pcor(0)
    }
}
impl core::fmt::Debug for Pcor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcor")
            .field("ptco0", &self.ptco0())
            .field("ptco1", &self.ptco1())
            .field("ptco2", &self.ptco2())
            .field("ptco3", &self.ptco3())
            .field("ptco4", &self.ptco4())
            .field("ptco5", &self.ptco5())
            .field("ptco6", &self.ptco6())
            .field("ptco7", &self.ptco7())
            .field("ptco8", &self.ptco8())
            .field("ptco9", &self.ptco9())
            .field("ptco10", &self.ptco10())
            .field("ptco11", &self.ptco11())
            .field("ptco12", &self.ptco12())
            .field("ptco13", &self.ptco13())
            .field("ptco14", &self.ptco14())
            .field("ptco15", &self.ptco15())
            .field("ptco16", &self.ptco16())
            .field("ptco17", &self.ptco17())
            .field("ptco18", &self.ptco18())
            .field("ptco19", &self.ptco19())
            .field("ptco20", &self.ptco20())
            .field("ptco21", &self.ptco21())
            .field("ptco22", &self.ptco22())
            .field("ptco23", &self.ptco23())
            .field("ptco24", &self.ptco24())
            .field("ptco25", &self.ptco25())
            .field("ptco26", &self.ptco26())
            .field("ptco27", &self.ptco27())
            .field("ptco28", &self.ptco28())
            .field("ptco29", &self.ptco29())
            .field("ptco30", &self.ptco30())
            .field("ptco31", &self.ptco31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcor {{ ptco0: {:?}, ptco1: {:?}, ptco2: {:?}, ptco3: {:?}, ptco4: {:?}, ptco5: {:?}, ptco6: {:?}, ptco7: {:?}, ptco8: {:?}, ptco9: {:?}, ptco10: {:?}, ptco11: {:?}, ptco12: {:?}, ptco13: {:?}, ptco14: {:?}, ptco15: {:?}, ptco16: {:?}, ptco17: {:?}, ptco18: {:?}, ptco19: {:?}, ptco20: {:?}, ptco21: {:?}, ptco22: {:?}, ptco23: {:?}, ptco24: {:?}, ptco25: {:?}, ptco26: {:?}, ptco27: {:?}, ptco28: {:?}, ptco29: {:?}, ptco30: {:?}, ptco31: {:?} }}",
            self.ptco0(),
            self.ptco1(),
            self.ptco2(),
            self.ptco3(),
            self.ptco4(),
            self.ptco5(),
            self.ptco6(),
            self.ptco7(),
            self.ptco8(),
            self.ptco9(),
            self.ptco10(),
            self.ptco11(),
            self.ptco12(),
            self.ptco13(),
            self.ptco14(),
            self.ptco15(),
            self.ptco16(),
            self.ptco17(),
            self.ptco18(),
            self.ptco19(),
            self.ptco20(),
            self.ptco21(),
            self.ptco22(),
            self.ptco23(),
            self.ptco24(),
            self.ptco25(),
            self.ptco26(),
            self.ptco27(),
            self.ptco28(),
            self.ptco29(),
            self.ptco30(),
            self.ptco31()
        )
    }
}
#[doc = "Port Data Direction."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pddr(pub u32);
impl Pddr {
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd0(&self) -> Pdd0 {
        let val = (self.0 >> 0usize) & 0x01;
        Pdd0::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd0(&mut self, val: Pdd0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd1(&self) -> Pdd1 {
        let val = (self.0 >> 1usize) & 0x01;
        Pdd1::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd1(&mut self, val: Pdd1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd2(&self) -> Pdd2 {
        let val = (self.0 >> 2usize) & 0x01;
        Pdd2::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd2(&mut self, val: Pdd2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd3(&self) -> Pdd3 {
        let val = (self.0 >> 3usize) & 0x01;
        Pdd3::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd3(&mut self, val: Pdd3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd4(&self) -> Pdd4 {
        let val = (self.0 >> 4usize) & 0x01;
        Pdd4::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd4(&mut self, val: Pdd4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd5(&self) -> Pdd5 {
        let val = (self.0 >> 5usize) & 0x01;
        Pdd5::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd5(&mut self, val: Pdd5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd6(&self) -> Pdd6 {
        let val = (self.0 >> 6usize) & 0x01;
        Pdd6::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd6(&mut self, val: Pdd6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd7(&self) -> Pdd7 {
        let val = (self.0 >> 7usize) & 0x01;
        Pdd7::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd7(&mut self, val: Pdd7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd8(&self) -> Pdd8 {
        let val = (self.0 >> 8usize) & 0x01;
        Pdd8::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd8(&mut self, val: Pdd8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd9(&self) -> Pdd9 {
        let val = (self.0 >> 9usize) & 0x01;
        Pdd9::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd9(&mut self, val: Pdd9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd10(&self) -> Pdd10 {
        let val = (self.0 >> 10usize) & 0x01;
        Pdd10::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd10(&mut self, val: Pdd10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd11(&self) -> Pdd11 {
        let val = (self.0 >> 11usize) & 0x01;
        Pdd11::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd11(&mut self, val: Pdd11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd12(&self) -> Pdd12 {
        let val = (self.0 >> 12usize) & 0x01;
        Pdd12::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd12(&mut self, val: Pdd12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd13(&self) -> Pdd13 {
        let val = (self.0 >> 13usize) & 0x01;
        Pdd13::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd13(&mut self, val: Pdd13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd14(&self) -> Pdd14 {
        let val = (self.0 >> 14usize) & 0x01;
        Pdd14::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd14(&mut self, val: Pdd14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd15(&self) -> Pdd15 {
        let val = (self.0 >> 15usize) & 0x01;
        Pdd15::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd15(&mut self, val: Pdd15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd16(&self) -> Pdd16 {
        let val = (self.0 >> 16usize) & 0x01;
        Pdd16::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd16(&mut self, val: Pdd16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd17(&self) -> Pdd17 {
        let val = (self.0 >> 17usize) & 0x01;
        Pdd17::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd17(&mut self, val: Pdd17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd18(&self) -> Pdd18 {
        let val = (self.0 >> 18usize) & 0x01;
        Pdd18::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd18(&mut self, val: Pdd18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd19(&self) -> Pdd19 {
        let val = (self.0 >> 19usize) & 0x01;
        Pdd19::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd19(&mut self, val: Pdd19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd20(&self) -> Pdd20 {
        let val = (self.0 >> 20usize) & 0x01;
        Pdd20::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd20(&mut self, val: Pdd20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd21(&self) -> Pdd21 {
        let val = (self.0 >> 21usize) & 0x01;
        Pdd21::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd21(&mut self, val: Pdd21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd22(&self) -> Pdd22 {
        let val = (self.0 >> 22usize) & 0x01;
        Pdd22::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd22(&mut self, val: Pdd22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd23(&self) -> Pdd23 {
        let val = (self.0 >> 23usize) & 0x01;
        Pdd23::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd23(&mut self, val: Pdd23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd24(&self) -> Pdd24 {
        let val = (self.0 >> 24usize) & 0x01;
        Pdd24::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd24(&mut self, val: Pdd24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd25(&self) -> Pdd25 {
        let val = (self.0 >> 25usize) & 0x01;
        Pdd25::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd25(&mut self, val: Pdd25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd26(&self) -> Pdd26 {
        let val = (self.0 >> 26usize) & 0x01;
        Pdd26::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd26(&mut self, val: Pdd26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd27(&self) -> Pdd27 {
        let val = (self.0 >> 27usize) & 0x01;
        Pdd27::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd27(&mut self, val: Pdd27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd28(&self) -> Pdd28 {
        let val = (self.0 >> 28usize) & 0x01;
        Pdd28::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd28(&mut self, val: Pdd28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd29(&self) -> Pdd29 {
        let val = (self.0 >> 29usize) & 0x01;
        Pdd29::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd29(&mut self, val: Pdd29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd30(&self) -> Pdd30 {
        let val = (self.0 >> 30usize) & 0x01;
        Pdd30::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd30(&mut self, val: Pdd30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd31(&self) -> Pdd31 {
        let val = (self.0 >> 31usize) & 0x01;
        Pdd31::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd31(&mut self, val: Pdd31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pddr {
    #[inline(always)]
    fn default() -> Pddr {
        Pddr(0)
    }
}
impl core::fmt::Debug for Pddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pddr")
            .field("pdd0", &self.pdd0())
            .field("pdd1", &self.pdd1())
            .field("pdd2", &self.pdd2())
            .field("pdd3", &self.pdd3())
            .field("pdd4", &self.pdd4())
            .field("pdd5", &self.pdd5())
            .field("pdd6", &self.pdd6())
            .field("pdd7", &self.pdd7())
            .field("pdd8", &self.pdd8())
            .field("pdd9", &self.pdd9())
            .field("pdd10", &self.pdd10())
            .field("pdd11", &self.pdd11())
            .field("pdd12", &self.pdd12())
            .field("pdd13", &self.pdd13())
            .field("pdd14", &self.pdd14())
            .field("pdd15", &self.pdd15())
            .field("pdd16", &self.pdd16())
            .field("pdd17", &self.pdd17())
            .field("pdd18", &self.pdd18())
            .field("pdd19", &self.pdd19())
            .field("pdd20", &self.pdd20())
            .field("pdd21", &self.pdd21())
            .field("pdd22", &self.pdd22())
            .field("pdd23", &self.pdd23())
            .field("pdd24", &self.pdd24())
            .field("pdd25", &self.pdd25())
            .field("pdd26", &self.pdd26())
            .field("pdd27", &self.pdd27())
            .field("pdd28", &self.pdd28())
            .field("pdd29", &self.pdd29())
            .field("pdd30", &self.pdd30())
            .field("pdd31", &self.pdd31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pddr {{ pdd0: {:?}, pdd1: {:?}, pdd2: {:?}, pdd3: {:?}, pdd4: {:?}, pdd5: {:?}, pdd6: {:?}, pdd7: {:?}, pdd8: {:?}, pdd9: {:?}, pdd10: {:?}, pdd11: {:?}, pdd12: {:?}, pdd13: {:?}, pdd14: {:?}, pdd15: {:?}, pdd16: {:?}, pdd17: {:?}, pdd18: {:?}, pdd19: {:?}, pdd20: {:?}, pdd21: {:?}, pdd22: {:?}, pdd23: {:?}, pdd24: {:?}, pdd25: {:?}, pdd26: {:?}, pdd27: {:?}, pdd28: {:?}, pdd29: {:?}, pdd30: {:?}, pdd31: {:?} }}",
            self.pdd0(),
            self.pdd1(),
            self.pdd2(),
            self.pdd3(),
            self.pdd4(),
            self.pdd5(),
            self.pdd6(),
            self.pdd7(),
            self.pdd8(),
            self.pdd9(),
            self.pdd10(),
            self.pdd11(),
            self.pdd12(),
            self.pdd13(),
            self.pdd14(),
            self.pdd15(),
            self.pdd16(),
            self.pdd17(),
            self.pdd18(),
            self.pdd19(),
            self.pdd20(),
            self.pdd21(),
            self.pdd22(),
            self.pdd23(),
            self.pdd24(),
            self.pdd25(),
            self.pdd26(),
            self.pdd27(),
            self.pdd28(),
            self.pdd29(),
            self.pdd30(),
            self.pdd31()
        )
    }
}
#[doc = "Port Data Input."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdir(pub u32);
impl Pdir {
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi0(&self) -> Pdi0 {
        let val = (self.0 >> 0usize) & 0x01;
        Pdi0::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi0(&mut self, val: Pdi0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi1(&self) -> Pdi1 {
        let val = (self.0 >> 1usize) & 0x01;
        Pdi1::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi1(&mut self, val: Pdi1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi2(&self) -> Pdi2 {
        let val = (self.0 >> 2usize) & 0x01;
        Pdi2::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi2(&mut self, val: Pdi2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi3(&self) -> Pdi3 {
        let val = (self.0 >> 3usize) & 0x01;
        Pdi3::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi3(&mut self, val: Pdi3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi4(&self) -> Pdi4 {
        let val = (self.0 >> 4usize) & 0x01;
        Pdi4::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi4(&mut self, val: Pdi4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi5(&self) -> Pdi5 {
        let val = (self.0 >> 5usize) & 0x01;
        Pdi5::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi5(&mut self, val: Pdi5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi6(&self) -> Pdi6 {
        let val = (self.0 >> 6usize) & 0x01;
        Pdi6::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi6(&mut self, val: Pdi6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi7(&self) -> Pdi7 {
        let val = (self.0 >> 7usize) & 0x01;
        Pdi7::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi7(&mut self, val: Pdi7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi8(&self) -> Pdi8 {
        let val = (self.0 >> 8usize) & 0x01;
        Pdi8::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi8(&mut self, val: Pdi8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi9(&self) -> Pdi9 {
        let val = (self.0 >> 9usize) & 0x01;
        Pdi9::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi9(&mut self, val: Pdi9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi10(&self) -> Pdi10 {
        let val = (self.0 >> 10usize) & 0x01;
        Pdi10::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi10(&mut self, val: Pdi10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi11(&self) -> Pdi11 {
        let val = (self.0 >> 11usize) & 0x01;
        Pdi11::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi11(&mut self, val: Pdi11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi12(&self) -> Pdi12 {
        let val = (self.0 >> 12usize) & 0x01;
        Pdi12::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi12(&mut self, val: Pdi12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi13(&self) -> Pdi13 {
        let val = (self.0 >> 13usize) & 0x01;
        Pdi13::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi13(&mut self, val: Pdi13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi14(&self) -> Pdi14 {
        let val = (self.0 >> 14usize) & 0x01;
        Pdi14::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi14(&mut self, val: Pdi14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi15(&self) -> Pdi15 {
        let val = (self.0 >> 15usize) & 0x01;
        Pdi15::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi15(&mut self, val: Pdi15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi16(&self) -> Pdi16 {
        let val = (self.0 >> 16usize) & 0x01;
        Pdi16::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi16(&mut self, val: Pdi16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi17(&self) -> Pdi17 {
        let val = (self.0 >> 17usize) & 0x01;
        Pdi17::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi17(&mut self, val: Pdi17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi18(&self) -> Pdi18 {
        let val = (self.0 >> 18usize) & 0x01;
        Pdi18::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi18(&mut self, val: Pdi18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi19(&self) -> Pdi19 {
        let val = (self.0 >> 19usize) & 0x01;
        Pdi19::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi19(&mut self, val: Pdi19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi20(&self) -> Pdi20 {
        let val = (self.0 >> 20usize) & 0x01;
        Pdi20::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi20(&mut self, val: Pdi20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi21(&self) -> Pdi21 {
        let val = (self.0 >> 21usize) & 0x01;
        Pdi21::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi21(&mut self, val: Pdi21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi22(&self) -> Pdi22 {
        let val = (self.0 >> 22usize) & 0x01;
        Pdi22::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi22(&mut self, val: Pdi22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi23(&self) -> Pdi23 {
        let val = (self.0 >> 23usize) & 0x01;
        Pdi23::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi23(&mut self, val: Pdi23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi24(&self) -> Pdi24 {
        let val = (self.0 >> 24usize) & 0x01;
        Pdi24::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi24(&mut self, val: Pdi24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi25(&self) -> Pdi25 {
        let val = (self.0 >> 25usize) & 0x01;
        Pdi25::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi25(&mut self, val: Pdi25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi26(&self) -> Pdi26 {
        let val = (self.0 >> 26usize) & 0x01;
        Pdi26::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi26(&mut self, val: Pdi26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi27(&self) -> Pdi27 {
        let val = (self.0 >> 27usize) & 0x01;
        Pdi27::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi27(&mut self, val: Pdi27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi28(&self) -> Pdi28 {
        let val = (self.0 >> 28usize) & 0x01;
        Pdi28::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi28(&mut self, val: Pdi28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi29(&self) -> Pdi29 {
        let val = (self.0 >> 29usize) & 0x01;
        Pdi29::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi29(&mut self, val: Pdi29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi30(&self) -> Pdi30 {
        let val = (self.0 >> 30usize) & 0x01;
        Pdi30::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi30(&mut self, val: Pdi30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi31(&self) -> Pdi31 {
        let val = (self.0 >> 31usize) & 0x01;
        Pdi31::from_bits(val as u8)
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi31(&mut self, val: Pdi31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pdir {
    #[inline(always)]
    fn default() -> Pdir {
        Pdir(0)
    }
}
impl core::fmt::Debug for Pdir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdir")
            .field("pdi0", &self.pdi0())
            .field("pdi1", &self.pdi1())
            .field("pdi2", &self.pdi2())
            .field("pdi3", &self.pdi3())
            .field("pdi4", &self.pdi4())
            .field("pdi5", &self.pdi5())
            .field("pdi6", &self.pdi6())
            .field("pdi7", &self.pdi7())
            .field("pdi8", &self.pdi8())
            .field("pdi9", &self.pdi9())
            .field("pdi10", &self.pdi10())
            .field("pdi11", &self.pdi11())
            .field("pdi12", &self.pdi12())
            .field("pdi13", &self.pdi13())
            .field("pdi14", &self.pdi14())
            .field("pdi15", &self.pdi15())
            .field("pdi16", &self.pdi16())
            .field("pdi17", &self.pdi17())
            .field("pdi18", &self.pdi18())
            .field("pdi19", &self.pdi19())
            .field("pdi20", &self.pdi20())
            .field("pdi21", &self.pdi21())
            .field("pdi22", &self.pdi22())
            .field("pdi23", &self.pdi23())
            .field("pdi24", &self.pdi24())
            .field("pdi25", &self.pdi25())
            .field("pdi26", &self.pdi26())
            .field("pdi27", &self.pdi27())
            .field("pdi28", &self.pdi28())
            .field("pdi29", &self.pdi29())
            .field("pdi30", &self.pdi30())
            .field("pdi31", &self.pdi31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdir {{ pdi0: {:?}, pdi1: {:?}, pdi2: {:?}, pdi3: {:?}, pdi4: {:?}, pdi5: {:?}, pdi6: {:?}, pdi7: {:?}, pdi8: {:?}, pdi9: {:?}, pdi10: {:?}, pdi11: {:?}, pdi12: {:?}, pdi13: {:?}, pdi14: {:?}, pdi15: {:?}, pdi16: {:?}, pdi17: {:?}, pdi18: {:?}, pdi19: {:?}, pdi20: {:?}, pdi21: {:?}, pdi22: {:?}, pdi23: {:?}, pdi24: {:?}, pdi25: {:?}, pdi26: {:?}, pdi27: {:?}, pdi28: {:?}, pdi29: {:?}, pdi30: {:?}, pdi31: {:?} }}",
            self.pdi0(),
            self.pdi1(),
            self.pdi2(),
            self.pdi3(),
            self.pdi4(),
            self.pdi5(),
            self.pdi6(),
            self.pdi7(),
            self.pdi8(),
            self.pdi9(),
            self.pdi10(),
            self.pdi11(),
            self.pdi12(),
            self.pdi13(),
            self.pdi14(),
            self.pdi15(),
            self.pdi16(),
            self.pdi17(),
            self.pdi18(),
            self.pdi19(),
            self.pdi20(),
            self.pdi21(),
            self.pdi22(),
            self.pdi23(),
            self.pdi24(),
            self.pdi25(),
            self.pdi26(),
            self.pdi27(),
            self.pdi28(),
            self.pdi29(),
            self.pdi30(),
            self.pdi31()
        )
    }
}
#[doc = "Port Data Output."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdor(pub u32);
impl Pdor {
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo0(&self) -> Pdo0 {
        let val = (self.0 >> 0usize) & 0x01;
        Pdo0::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo0(&mut self, val: Pdo0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo1(&self) -> Pdo1 {
        let val = (self.0 >> 1usize) & 0x01;
        Pdo1::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo1(&mut self, val: Pdo1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo2(&self) -> Pdo2 {
        let val = (self.0 >> 2usize) & 0x01;
        Pdo2::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo2(&mut self, val: Pdo2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo3(&self) -> Pdo3 {
        let val = (self.0 >> 3usize) & 0x01;
        Pdo3::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo3(&mut self, val: Pdo3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo4(&self) -> Pdo4 {
        let val = (self.0 >> 4usize) & 0x01;
        Pdo4::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo4(&mut self, val: Pdo4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo5(&self) -> Pdo5 {
        let val = (self.0 >> 5usize) & 0x01;
        Pdo5::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo5(&mut self, val: Pdo5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo6(&self) -> Pdo6 {
        let val = (self.0 >> 6usize) & 0x01;
        Pdo6::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo6(&mut self, val: Pdo6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo7(&self) -> Pdo7 {
        let val = (self.0 >> 7usize) & 0x01;
        Pdo7::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo7(&mut self, val: Pdo7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo8(&self) -> Pdo8 {
        let val = (self.0 >> 8usize) & 0x01;
        Pdo8::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo8(&mut self, val: Pdo8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo9(&self) -> Pdo9 {
        let val = (self.0 >> 9usize) & 0x01;
        Pdo9::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo9(&mut self, val: Pdo9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo10(&self) -> Pdo10 {
        let val = (self.0 >> 10usize) & 0x01;
        Pdo10::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo10(&mut self, val: Pdo10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo11(&self) -> Pdo11 {
        let val = (self.0 >> 11usize) & 0x01;
        Pdo11::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo11(&mut self, val: Pdo11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo12(&self) -> Pdo12 {
        let val = (self.0 >> 12usize) & 0x01;
        Pdo12::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo12(&mut self, val: Pdo12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo13(&self) -> Pdo13 {
        let val = (self.0 >> 13usize) & 0x01;
        Pdo13::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo13(&mut self, val: Pdo13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo14(&self) -> Pdo14 {
        let val = (self.0 >> 14usize) & 0x01;
        Pdo14::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo14(&mut self, val: Pdo14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo15(&self) -> Pdo15 {
        let val = (self.0 >> 15usize) & 0x01;
        Pdo15::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo15(&mut self, val: Pdo15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo16(&self) -> Pdo16 {
        let val = (self.0 >> 16usize) & 0x01;
        Pdo16::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo16(&mut self, val: Pdo16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo17(&self) -> Pdo17 {
        let val = (self.0 >> 17usize) & 0x01;
        Pdo17::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo17(&mut self, val: Pdo17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo18(&self) -> Pdo18 {
        let val = (self.0 >> 18usize) & 0x01;
        Pdo18::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo18(&mut self, val: Pdo18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo19(&self) -> Pdo19 {
        let val = (self.0 >> 19usize) & 0x01;
        Pdo19::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo19(&mut self, val: Pdo19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo20(&self) -> Pdo20 {
        let val = (self.0 >> 20usize) & 0x01;
        Pdo20::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo20(&mut self, val: Pdo20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo21(&self) -> Pdo21 {
        let val = (self.0 >> 21usize) & 0x01;
        Pdo21::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo21(&mut self, val: Pdo21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo22(&self) -> Pdo22 {
        let val = (self.0 >> 22usize) & 0x01;
        Pdo22::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo22(&mut self, val: Pdo22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo23(&self) -> Pdo23 {
        let val = (self.0 >> 23usize) & 0x01;
        Pdo23::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo23(&mut self, val: Pdo23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo24(&self) -> Pdo24 {
        let val = (self.0 >> 24usize) & 0x01;
        Pdo24::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo24(&mut self, val: Pdo24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo25(&self) -> Pdo25 {
        let val = (self.0 >> 25usize) & 0x01;
        Pdo25::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo25(&mut self, val: Pdo25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo26(&self) -> Pdo26 {
        let val = (self.0 >> 26usize) & 0x01;
        Pdo26::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo26(&mut self, val: Pdo26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo27(&self) -> Pdo27 {
        let val = (self.0 >> 27usize) & 0x01;
        Pdo27::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo27(&mut self, val: Pdo27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo28(&self) -> Pdo28 {
        let val = (self.0 >> 28usize) & 0x01;
        Pdo28::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo28(&mut self, val: Pdo28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo29(&self) -> Pdo29 {
        let val = (self.0 >> 29usize) & 0x01;
        Pdo29::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo29(&mut self, val: Pdo29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo30(&self) -> Pdo30 {
        let val = (self.0 >> 30usize) & 0x01;
        Pdo30::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo30(&mut self, val: Pdo30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo31(&self) -> Pdo31 {
        let val = (self.0 >> 31usize) & 0x01;
        Pdo31::from_bits(val as u8)
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo31(&mut self, val: Pdo31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pdor {
    #[inline(always)]
    fn default() -> Pdor {
        Pdor(0)
    }
}
impl core::fmt::Debug for Pdor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdor")
            .field("pdo0", &self.pdo0())
            .field("pdo1", &self.pdo1())
            .field("pdo2", &self.pdo2())
            .field("pdo3", &self.pdo3())
            .field("pdo4", &self.pdo4())
            .field("pdo5", &self.pdo5())
            .field("pdo6", &self.pdo6())
            .field("pdo7", &self.pdo7())
            .field("pdo8", &self.pdo8())
            .field("pdo9", &self.pdo9())
            .field("pdo10", &self.pdo10())
            .field("pdo11", &self.pdo11())
            .field("pdo12", &self.pdo12())
            .field("pdo13", &self.pdo13())
            .field("pdo14", &self.pdo14())
            .field("pdo15", &self.pdo15())
            .field("pdo16", &self.pdo16())
            .field("pdo17", &self.pdo17())
            .field("pdo18", &self.pdo18())
            .field("pdo19", &self.pdo19())
            .field("pdo20", &self.pdo20())
            .field("pdo21", &self.pdo21())
            .field("pdo22", &self.pdo22())
            .field("pdo23", &self.pdo23())
            .field("pdo24", &self.pdo24())
            .field("pdo25", &self.pdo25())
            .field("pdo26", &self.pdo26())
            .field("pdo27", &self.pdo27())
            .field("pdo28", &self.pdo28())
            .field("pdo29", &self.pdo29())
            .field("pdo30", &self.pdo30())
            .field("pdo31", &self.pdo31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdor {{ pdo0: {:?}, pdo1: {:?}, pdo2: {:?}, pdo3: {:?}, pdo4: {:?}, pdo5: {:?}, pdo6: {:?}, pdo7: {:?}, pdo8: {:?}, pdo9: {:?}, pdo10: {:?}, pdo11: {:?}, pdo12: {:?}, pdo13: {:?}, pdo14: {:?}, pdo15: {:?}, pdo16: {:?}, pdo17: {:?}, pdo18: {:?}, pdo19: {:?}, pdo20: {:?}, pdo21: {:?}, pdo22: {:?}, pdo23: {:?}, pdo24: {:?}, pdo25: {:?}, pdo26: {:?}, pdo27: {:?}, pdo28: {:?}, pdo29: {:?}, pdo30: {:?}, pdo31: {:?} }}",
            self.pdo0(),
            self.pdo1(),
            self.pdo2(),
            self.pdo3(),
            self.pdo4(),
            self.pdo5(),
            self.pdo6(),
            self.pdo7(),
            self.pdo8(),
            self.pdo9(),
            self.pdo10(),
            self.pdo11(),
            self.pdo12(),
            self.pdo13(),
            self.pdo14(),
            self.pdo15(),
            self.pdo16(),
            self.pdo17(),
            self.pdo18(),
            self.pdo19(),
            self.pdo20(),
            self.pdo21(),
            self.pdo22(),
            self.pdo23(),
            self.pdo24(),
            self.pdo25(),
            self.pdo26(),
            self.pdo27(),
            self.pdo28(),
            self.pdo29(),
            self.pdo30(),
            self.pdo31()
        )
    }
}
#[doc = "Pin Data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdr(pub u8);
impl Pdr {
    #[doc = "Pin Data (I/O)."]
    #[must_use]
    #[inline(always)]
    pub const fn pd(&self) -> Pd {
        let val = (self.0 >> 0usize) & 0x01;
        Pd::from_bits(val as u8)
    }
    #[doc = "Pin Data (I/O)."]
    #[inline(always)]
    pub const fn set_pd(&mut self, val: Pd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
}
impl Default for Pdr {
    #[inline(always)]
    fn default() -> Pdr {
        Pdr(0)
    }
}
impl core::fmt::Debug for Pdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdr").field("pd", &self.pd()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pdr {{ pd: {:?} }}", self.pd())
    }
}
#[doc = "Port Input Disable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pidr(pub u32);
impl Pidr {
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid0(&self) -> Pid0 {
        let val = (self.0 >> 0usize) & 0x01;
        Pid0::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid0(&mut self, val: Pid0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid1(&self) -> Pid1 {
        let val = (self.0 >> 1usize) & 0x01;
        Pid1::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid1(&mut self, val: Pid1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid2(&self) -> Pid2 {
        let val = (self.0 >> 2usize) & 0x01;
        Pid2::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid2(&mut self, val: Pid2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid3(&self) -> Pid3 {
        let val = (self.0 >> 3usize) & 0x01;
        Pid3::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid3(&mut self, val: Pid3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid4(&self) -> Pid4 {
        let val = (self.0 >> 4usize) & 0x01;
        Pid4::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid4(&mut self, val: Pid4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid5(&self) -> Pid5 {
        let val = (self.0 >> 5usize) & 0x01;
        Pid5::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid5(&mut self, val: Pid5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid6(&self) -> Pid6 {
        let val = (self.0 >> 6usize) & 0x01;
        Pid6::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid6(&mut self, val: Pid6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid7(&self) -> Pid7 {
        let val = (self.0 >> 7usize) & 0x01;
        Pid7::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid7(&mut self, val: Pid7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid8(&self) -> Pid8 {
        let val = (self.0 >> 8usize) & 0x01;
        Pid8::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid8(&mut self, val: Pid8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid9(&self) -> Pid9 {
        let val = (self.0 >> 9usize) & 0x01;
        Pid9::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid9(&mut self, val: Pid9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid10(&self) -> Pid10 {
        let val = (self.0 >> 10usize) & 0x01;
        Pid10::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid10(&mut self, val: Pid10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid11(&self) -> Pid11 {
        let val = (self.0 >> 11usize) & 0x01;
        Pid11::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid11(&mut self, val: Pid11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid12(&self) -> Pid12 {
        let val = (self.0 >> 12usize) & 0x01;
        Pid12::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid12(&mut self, val: Pid12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid13(&self) -> Pid13 {
        let val = (self.0 >> 13usize) & 0x01;
        Pid13::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid13(&mut self, val: Pid13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid14(&self) -> Pid14 {
        let val = (self.0 >> 14usize) & 0x01;
        Pid14::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid14(&mut self, val: Pid14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid15(&self) -> Pid15 {
        let val = (self.0 >> 15usize) & 0x01;
        Pid15::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid15(&mut self, val: Pid15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid16(&self) -> Pid16 {
        let val = (self.0 >> 16usize) & 0x01;
        Pid16::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid16(&mut self, val: Pid16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid17(&self) -> Pid17 {
        let val = (self.0 >> 17usize) & 0x01;
        Pid17::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid17(&mut self, val: Pid17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid18(&self) -> Pid18 {
        let val = (self.0 >> 18usize) & 0x01;
        Pid18::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid18(&mut self, val: Pid18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid19(&self) -> Pid19 {
        let val = (self.0 >> 19usize) & 0x01;
        Pid19::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid19(&mut self, val: Pid19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid20(&self) -> Pid20 {
        let val = (self.0 >> 20usize) & 0x01;
        Pid20::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid20(&mut self, val: Pid20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid21(&self) -> Pid21 {
        let val = (self.0 >> 21usize) & 0x01;
        Pid21::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid21(&mut self, val: Pid21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid22(&self) -> Pid22 {
        let val = (self.0 >> 22usize) & 0x01;
        Pid22::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid22(&mut self, val: Pid22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid23(&self) -> Pid23 {
        let val = (self.0 >> 23usize) & 0x01;
        Pid23::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid23(&mut self, val: Pid23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid24(&self) -> Pid24 {
        let val = (self.0 >> 24usize) & 0x01;
        Pid24::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid24(&mut self, val: Pid24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid25(&self) -> Pid25 {
        let val = (self.0 >> 25usize) & 0x01;
        Pid25::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid25(&mut self, val: Pid25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid26(&self) -> Pid26 {
        let val = (self.0 >> 26usize) & 0x01;
        Pid26::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid26(&mut self, val: Pid26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid27(&self) -> Pid27 {
        let val = (self.0 >> 27usize) & 0x01;
        Pid27::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid27(&mut self, val: Pid27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid28(&self) -> Pid28 {
        let val = (self.0 >> 28usize) & 0x01;
        Pid28::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid28(&mut self, val: Pid28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid29(&self) -> Pid29 {
        let val = (self.0 >> 29usize) & 0x01;
        Pid29::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid29(&mut self, val: Pid29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid30(&self) -> Pid30 {
        let val = (self.0 >> 30usize) & 0x01;
        Pid30::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid30(&mut self, val: Pid30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid31(&self) -> Pid31 {
        let val = (self.0 >> 31usize) & 0x01;
        Pid31::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid31(&mut self, val: Pid31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pidr {
    #[inline(always)]
    fn default() -> Pidr {
        Pidr(0)
    }
}
impl core::fmt::Debug for Pidr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pidr")
            .field("pid0", &self.pid0())
            .field("pid1", &self.pid1())
            .field("pid2", &self.pid2())
            .field("pid3", &self.pid3())
            .field("pid4", &self.pid4())
            .field("pid5", &self.pid5())
            .field("pid6", &self.pid6())
            .field("pid7", &self.pid7())
            .field("pid8", &self.pid8())
            .field("pid9", &self.pid9())
            .field("pid10", &self.pid10())
            .field("pid11", &self.pid11())
            .field("pid12", &self.pid12())
            .field("pid13", &self.pid13())
            .field("pid14", &self.pid14())
            .field("pid15", &self.pid15())
            .field("pid16", &self.pid16())
            .field("pid17", &self.pid17())
            .field("pid18", &self.pid18())
            .field("pid19", &self.pid19())
            .field("pid20", &self.pid20())
            .field("pid21", &self.pid21())
            .field("pid22", &self.pid22())
            .field("pid23", &self.pid23())
            .field("pid24", &self.pid24())
            .field("pid25", &self.pid25())
            .field("pid26", &self.pid26())
            .field("pid27", &self.pid27())
            .field("pid28", &self.pid28())
            .field("pid29", &self.pid29())
            .field("pid30", &self.pid30())
            .field("pid31", &self.pid31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pidr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pidr {{ pid0: {:?}, pid1: {:?}, pid2: {:?}, pid3: {:?}, pid4: {:?}, pid5: {:?}, pid6: {:?}, pid7: {:?}, pid8: {:?}, pid9: {:?}, pid10: {:?}, pid11: {:?}, pid12: {:?}, pid13: {:?}, pid14: {:?}, pid15: {:?}, pid16: {:?}, pid17: {:?}, pid18: {:?}, pid19: {:?}, pid20: {:?}, pid21: {:?}, pid22: {:?}, pid23: {:?}, pid24: {:?}, pid25: {:?}, pid26: {:?}, pid27: {:?}, pid28: {:?}, pid29: {:?}, pid30: {:?}, pid31: {:?} }}",
            self.pid0(),
            self.pid1(),
            self.pid2(),
            self.pid3(),
            self.pid4(),
            self.pid5(),
            self.pid6(),
            self.pid7(),
            self.pid8(),
            self.pid9(),
            self.pid10(),
            self.pid11(),
            self.pid12(),
            self.pid13(),
            self.pid14(),
            self.pid15(),
            self.pid16(),
            self.pid17(),
            self.pid18(),
            self.pid19(),
            self.pid20(),
            self.pid21(),
            self.pid22(),
            self.pid23(),
            self.pid24(),
            self.pid25(),
            self.pid26(),
            self.pid27(),
            self.pid28(),
            self.pid29(),
            self.pid30(),
            self.pid31()
        )
    }
}
#[doc = "Port Set Output."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psor(pub u32);
impl Psor {
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso0(&self) -> Ptso0 {
        let val = (self.0 >> 0usize) & 0x01;
        Ptso0::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso0(&mut self, val: Ptso0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso1(&self) -> Ptso1 {
        let val = (self.0 >> 1usize) & 0x01;
        Ptso1::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso1(&mut self, val: Ptso1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso2(&self) -> Ptso2 {
        let val = (self.0 >> 2usize) & 0x01;
        Ptso2::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso2(&mut self, val: Ptso2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso3(&self) -> Ptso3 {
        let val = (self.0 >> 3usize) & 0x01;
        Ptso3::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso3(&mut self, val: Ptso3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso4(&self) -> Ptso4 {
        let val = (self.0 >> 4usize) & 0x01;
        Ptso4::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso4(&mut self, val: Ptso4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso5(&self) -> Ptso5 {
        let val = (self.0 >> 5usize) & 0x01;
        Ptso5::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso5(&mut self, val: Ptso5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso6(&self) -> Ptso6 {
        let val = (self.0 >> 6usize) & 0x01;
        Ptso6::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso6(&mut self, val: Ptso6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso7(&self) -> Ptso7 {
        let val = (self.0 >> 7usize) & 0x01;
        Ptso7::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso7(&mut self, val: Ptso7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso8(&self) -> Ptso8 {
        let val = (self.0 >> 8usize) & 0x01;
        Ptso8::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso8(&mut self, val: Ptso8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso9(&self) -> Ptso9 {
        let val = (self.0 >> 9usize) & 0x01;
        Ptso9::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso9(&mut self, val: Ptso9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso10(&self) -> Ptso10 {
        let val = (self.0 >> 10usize) & 0x01;
        Ptso10::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso10(&mut self, val: Ptso10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso11(&self) -> Ptso11 {
        let val = (self.0 >> 11usize) & 0x01;
        Ptso11::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso11(&mut self, val: Ptso11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso12(&self) -> Ptso12 {
        let val = (self.0 >> 12usize) & 0x01;
        Ptso12::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso12(&mut self, val: Ptso12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso13(&self) -> Ptso13 {
        let val = (self.0 >> 13usize) & 0x01;
        Ptso13::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso13(&mut self, val: Ptso13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso14(&self) -> Ptso14 {
        let val = (self.0 >> 14usize) & 0x01;
        Ptso14::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso14(&mut self, val: Ptso14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso15(&self) -> Ptso15 {
        let val = (self.0 >> 15usize) & 0x01;
        Ptso15::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso15(&mut self, val: Ptso15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso16(&self) -> Ptso16 {
        let val = (self.0 >> 16usize) & 0x01;
        Ptso16::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso16(&mut self, val: Ptso16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso17(&self) -> Ptso17 {
        let val = (self.0 >> 17usize) & 0x01;
        Ptso17::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso17(&mut self, val: Ptso17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso18(&self) -> Ptso18 {
        let val = (self.0 >> 18usize) & 0x01;
        Ptso18::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso18(&mut self, val: Ptso18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso19(&self) -> Ptso19 {
        let val = (self.0 >> 19usize) & 0x01;
        Ptso19::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso19(&mut self, val: Ptso19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso20(&self) -> Ptso20 {
        let val = (self.0 >> 20usize) & 0x01;
        Ptso20::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso20(&mut self, val: Ptso20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso21(&self) -> Ptso21 {
        let val = (self.0 >> 21usize) & 0x01;
        Ptso21::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso21(&mut self, val: Ptso21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso22(&self) -> Ptso22 {
        let val = (self.0 >> 22usize) & 0x01;
        Ptso22::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso22(&mut self, val: Ptso22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso23(&self) -> Ptso23 {
        let val = (self.0 >> 23usize) & 0x01;
        Ptso23::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso23(&mut self, val: Ptso23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso24(&self) -> Ptso24 {
        let val = (self.0 >> 24usize) & 0x01;
        Ptso24::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso24(&mut self, val: Ptso24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso25(&self) -> Ptso25 {
        let val = (self.0 >> 25usize) & 0x01;
        Ptso25::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso25(&mut self, val: Ptso25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso26(&self) -> Ptso26 {
        let val = (self.0 >> 26usize) & 0x01;
        Ptso26::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso26(&mut self, val: Ptso26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso27(&self) -> Ptso27 {
        let val = (self.0 >> 27usize) & 0x01;
        Ptso27::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso27(&mut self, val: Ptso27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso28(&self) -> Ptso28 {
        let val = (self.0 >> 28usize) & 0x01;
        Ptso28::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso28(&mut self, val: Ptso28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso29(&self) -> Ptso29 {
        let val = (self.0 >> 29usize) & 0x01;
        Ptso29::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso29(&mut self, val: Ptso29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso30(&self) -> Ptso30 {
        let val = (self.0 >> 30usize) & 0x01;
        Ptso30::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso30(&mut self, val: Ptso30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso31(&self) -> Ptso31 {
        let val = (self.0 >> 31usize) & 0x01;
        Ptso31::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso31(&mut self, val: Ptso31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Psor {
    #[inline(always)]
    fn default() -> Psor {
        Psor(0)
    }
}
impl core::fmt::Debug for Psor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psor")
            .field("ptso0", &self.ptso0())
            .field("ptso1", &self.ptso1())
            .field("ptso2", &self.ptso2())
            .field("ptso3", &self.ptso3())
            .field("ptso4", &self.ptso4())
            .field("ptso5", &self.ptso5())
            .field("ptso6", &self.ptso6())
            .field("ptso7", &self.ptso7())
            .field("ptso8", &self.ptso8())
            .field("ptso9", &self.ptso9())
            .field("ptso10", &self.ptso10())
            .field("ptso11", &self.ptso11())
            .field("ptso12", &self.ptso12())
            .field("ptso13", &self.ptso13())
            .field("ptso14", &self.ptso14())
            .field("ptso15", &self.ptso15())
            .field("ptso16", &self.ptso16())
            .field("ptso17", &self.ptso17())
            .field("ptso18", &self.ptso18())
            .field("ptso19", &self.ptso19())
            .field("ptso20", &self.ptso20())
            .field("ptso21", &self.ptso21())
            .field("ptso22", &self.ptso22())
            .field("ptso23", &self.ptso23())
            .field("ptso24", &self.ptso24())
            .field("ptso25", &self.ptso25())
            .field("ptso26", &self.ptso26())
            .field("ptso27", &self.ptso27())
            .field("ptso28", &self.ptso28())
            .field("ptso29", &self.ptso29())
            .field("ptso30", &self.ptso30())
            .field("ptso31", &self.ptso31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Psor {{ ptso0: {:?}, ptso1: {:?}, ptso2: {:?}, ptso3: {:?}, ptso4: {:?}, ptso5: {:?}, ptso6: {:?}, ptso7: {:?}, ptso8: {:?}, ptso9: {:?}, ptso10: {:?}, ptso11: {:?}, ptso12: {:?}, ptso13: {:?}, ptso14: {:?}, ptso15: {:?}, ptso16: {:?}, ptso17: {:?}, ptso18: {:?}, ptso19: {:?}, ptso20: {:?}, ptso21: {:?}, ptso22: {:?}, ptso23: {:?}, ptso24: {:?}, ptso25: {:?}, ptso26: {:?}, ptso27: {:?}, ptso28: {:?}, ptso29: {:?}, ptso30: {:?}, ptso31: {:?} }}",
            self.ptso0(),
            self.ptso1(),
            self.ptso2(),
            self.ptso3(),
            self.ptso4(),
            self.ptso5(),
            self.ptso6(),
            self.ptso7(),
            self.ptso8(),
            self.ptso9(),
            self.ptso10(),
            self.ptso11(),
            self.ptso12(),
            self.ptso13(),
            self.ptso14(),
            self.ptso15(),
            self.ptso16(),
            self.ptso17(),
            self.ptso18(),
            self.ptso19(),
            self.ptso20(),
            self.ptso21(),
            self.ptso22(),
            self.ptso23(),
            self.ptso24(),
            self.ptso25(),
            self.ptso26(),
            self.ptso27(),
            self.ptso28(),
            self.ptso29(),
            self.ptso30(),
            self.ptso31()
        )
    }
}
#[doc = "Port Toggle Output."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptor(pub u32);
impl Ptor {
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto0(&self) -> Ptto0 {
        let val = (self.0 >> 0usize) & 0x01;
        Ptto0::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto0(&mut self, val: Ptto0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto1(&self) -> Ptto1 {
        let val = (self.0 >> 1usize) & 0x01;
        Ptto1::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto1(&mut self, val: Ptto1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto2(&self) -> Ptto2 {
        let val = (self.0 >> 2usize) & 0x01;
        Ptto2::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto2(&mut self, val: Ptto2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto3(&self) -> Ptto3 {
        let val = (self.0 >> 3usize) & 0x01;
        Ptto3::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto3(&mut self, val: Ptto3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto4(&self) -> Ptto4 {
        let val = (self.0 >> 4usize) & 0x01;
        Ptto4::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto4(&mut self, val: Ptto4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto5(&self) -> Ptto5 {
        let val = (self.0 >> 5usize) & 0x01;
        Ptto5::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto5(&mut self, val: Ptto5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto6(&self) -> Ptto6 {
        let val = (self.0 >> 6usize) & 0x01;
        Ptto6::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto6(&mut self, val: Ptto6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto7(&self) -> Ptto7 {
        let val = (self.0 >> 7usize) & 0x01;
        Ptto7::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto7(&mut self, val: Ptto7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto8(&self) -> Ptto8 {
        let val = (self.0 >> 8usize) & 0x01;
        Ptto8::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto8(&mut self, val: Ptto8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto9(&self) -> Ptto9 {
        let val = (self.0 >> 9usize) & 0x01;
        Ptto9::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto9(&mut self, val: Ptto9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto10(&self) -> Ptto10 {
        let val = (self.0 >> 10usize) & 0x01;
        Ptto10::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto10(&mut self, val: Ptto10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto11(&self) -> Ptto11 {
        let val = (self.0 >> 11usize) & 0x01;
        Ptto11::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto11(&mut self, val: Ptto11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto12(&self) -> Ptto12 {
        let val = (self.0 >> 12usize) & 0x01;
        Ptto12::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto12(&mut self, val: Ptto12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto13(&self) -> Ptto13 {
        let val = (self.0 >> 13usize) & 0x01;
        Ptto13::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto13(&mut self, val: Ptto13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto14(&self) -> Ptto14 {
        let val = (self.0 >> 14usize) & 0x01;
        Ptto14::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto14(&mut self, val: Ptto14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto15(&self) -> Ptto15 {
        let val = (self.0 >> 15usize) & 0x01;
        Ptto15::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto15(&mut self, val: Ptto15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto16(&self) -> Ptto16 {
        let val = (self.0 >> 16usize) & 0x01;
        Ptto16::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto16(&mut self, val: Ptto16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto17(&self) -> Ptto17 {
        let val = (self.0 >> 17usize) & 0x01;
        Ptto17::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto17(&mut self, val: Ptto17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto18(&self) -> Ptto18 {
        let val = (self.0 >> 18usize) & 0x01;
        Ptto18::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto18(&mut self, val: Ptto18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto19(&self) -> Ptto19 {
        let val = (self.0 >> 19usize) & 0x01;
        Ptto19::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto19(&mut self, val: Ptto19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto20(&self) -> Ptto20 {
        let val = (self.0 >> 20usize) & 0x01;
        Ptto20::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto20(&mut self, val: Ptto20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto21(&self) -> Ptto21 {
        let val = (self.0 >> 21usize) & 0x01;
        Ptto21::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto21(&mut self, val: Ptto21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto22(&self) -> Ptto22 {
        let val = (self.0 >> 22usize) & 0x01;
        Ptto22::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto22(&mut self, val: Ptto22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto23(&self) -> Ptto23 {
        let val = (self.0 >> 23usize) & 0x01;
        Ptto23::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto23(&mut self, val: Ptto23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto24(&self) -> Ptto24 {
        let val = (self.0 >> 24usize) & 0x01;
        Ptto24::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto24(&mut self, val: Ptto24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto25(&self) -> Ptto25 {
        let val = (self.0 >> 25usize) & 0x01;
        Ptto25::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto25(&mut self, val: Ptto25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto26(&self) -> Ptto26 {
        let val = (self.0 >> 26usize) & 0x01;
        Ptto26::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto26(&mut self, val: Ptto26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto27(&self) -> Ptto27 {
        let val = (self.0 >> 27usize) & 0x01;
        Ptto27::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto27(&mut self, val: Ptto27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto28(&self) -> Ptto28 {
        let val = (self.0 >> 28usize) & 0x01;
        Ptto28::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto28(&mut self, val: Ptto28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto29(&self) -> Ptto29 {
        let val = (self.0 >> 29usize) & 0x01;
        Ptto29::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto29(&mut self, val: Ptto29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto30(&self) -> Ptto30 {
        let val = (self.0 >> 30usize) & 0x01;
        Ptto30::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto30(&mut self, val: Ptto30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto31(&self) -> Ptto31 {
        let val = (self.0 >> 31usize) & 0x01;
        Ptto31::from_bits(val as u8)
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto31(&mut self, val: Ptto31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ptor {
    #[inline(always)]
    fn default() -> Ptor {
        Ptor(0)
    }
}
impl core::fmt::Debug for Ptor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ptor")
            .field("ptto0", &self.ptto0())
            .field("ptto1", &self.ptto1())
            .field("ptto2", &self.ptto2())
            .field("ptto3", &self.ptto3())
            .field("ptto4", &self.ptto4())
            .field("ptto5", &self.ptto5())
            .field("ptto6", &self.ptto6())
            .field("ptto7", &self.ptto7())
            .field("ptto8", &self.ptto8())
            .field("ptto9", &self.ptto9())
            .field("ptto10", &self.ptto10())
            .field("ptto11", &self.ptto11())
            .field("ptto12", &self.ptto12())
            .field("ptto13", &self.ptto13())
            .field("ptto14", &self.ptto14())
            .field("ptto15", &self.ptto15())
            .field("ptto16", &self.ptto16())
            .field("ptto17", &self.ptto17())
            .field("ptto18", &self.ptto18())
            .field("ptto19", &self.ptto19())
            .field("ptto20", &self.ptto20())
            .field("ptto21", &self.ptto21())
            .field("ptto22", &self.ptto22())
            .field("ptto23", &self.ptto23())
            .field("ptto24", &self.ptto24())
            .field("ptto25", &self.ptto25())
            .field("ptto26", &self.ptto26())
            .field("ptto27", &self.ptto27())
            .field("ptto28", &self.ptto28())
            .field("ptto29", &self.ptto29())
            .field("ptto30", &self.ptto30())
            .field("ptto31", &self.ptto31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ptor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ptor {{ ptto0: {:?}, ptto1: {:?}, ptto2: {:?}, ptto3: {:?}, ptto4: {:?}, ptto5: {:?}, ptto6: {:?}, ptto7: {:?}, ptto8: {:?}, ptto9: {:?}, ptto10: {:?}, ptto11: {:?}, ptto12: {:?}, ptto13: {:?}, ptto14: {:?}, ptto15: {:?}, ptto16: {:?}, ptto17: {:?}, ptto18: {:?}, ptto19: {:?}, ptto20: {:?}, ptto21: {:?}, ptto22: {:?}, ptto23: {:?}, ptto24: {:?}, ptto25: {:?}, ptto26: {:?}, ptto27: {:?}, ptto28: {:?}, ptto29: {:?}, ptto30: {:?}, ptto31: {:?} }}",
            self.ptto0(),
            self.ptto1(),
            self.ptto2(),
            self.ptto3(),
            self.ptto4(),
            self.ptto5(),
            self.ptto6(),
            self.ptto7(),
            self.ptto8(),
            self.ptto9(),
            self.ptto10(),
            self.ptto11(),
            self.ptto12(),
            self.ptto13(),
            self.ptto14(),
            self.ptto15(),
            self.ptto16(),
            self.ptto17(),
            self.ptto18(),
            self.ptto19(),
            self.ptto20(),
            self.ptto21(),
            self.ptto22(),
            self.ptto23(),
            self.ptto24(),
            self.ptto25(),
            self.ptto26(),
            self.ptto27(),
            self.ptto28(),
            self.ptto29(),
            self.ptto30(),
            self.ptto31()
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
    #[doc = "Protection registers implemented."]
    pub const FEATURE1: Self = Self(0x01);
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
            0x01 => f.write_str("FEATURE1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "FEATURE0"),
            0x01 => defmt::write!(f, "FEATURE1"),
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
pub enum Giwe0 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe0 {
    #[inline(always)]
    fn from(val: u8) -> Giwe0 {
        Giwe0::from_bits(val)
    }
}
impl From<Giwe0> for u8 {
    #[inline(always)]
    fn from(val: Giwe0) -> u8 {
        Giwe0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe1 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe1 {
    #[inline(always)]
    fn from(val: u8) -> Giwe1 {
        Giwe1::from_bits(val)
    }
}
impl From<Giwe1> for u8 {
    #[inline(always)]
    fn from(val: Giwe1) -> u8 {
        Giwe1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe10 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe10 {
    #[inline(always)]
    fn from(val: u8) -> Giwe10 {
        Giwe10::from_bits(val)
    }
}
impl From<Giwe10> for u8 {
    #[inline(always)]
    fn from(val: Giwe10) -> u8 {
        Giwe10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe11 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe11 {
    #[inline(always)]
    fn from(val: u8) -> Giwe11 {
        Giwe11::from_bits(val)
    }
}
impl From<Giwe11> for u8 {
    #[inline(always)]
    fn from(val: Giwe11) -> u8 {
        Giwe11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe12 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe12 {
    #[inline(always)]
    fn from(val: u8) -> Giwe12 {
        Giwe12::from_bits(val)
    }
}
impl From<Giwe12> for u8 {
    #[inline(always)]
    fn from(val: Giwe12) -> u8 {
        Giwe12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe13 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe13 {
    #[inline(always)]
    fn from(val: u8) -> Giwe13 {
        Giwe13::from_bits(val)
    }
}
impl From<Giwe13> for u8 {
    #[inline(always)]
    fn from(val: Giwe13) -> u8 {
        Giwe13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe14 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe14 {
    #[inline(always)]
    fn from(val: u8) -> Giwe14 {
        Giwe14::from_bits(val)
    }
}
impl From<Giwe14> for u8 {
    #[inline(always)]
    fn from(val: Giwe14) -> u8 {
        Giwe14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe15 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe15 {
    #[inline(always)]
    fn from(val: u8) -> Giwe15 {
        Giwe15::from_bits(val)
    }
}
impl From<Giwe15> for u8 {
    #[inline(always)]
    fn from(val: Giwe15) -> u8 {
        Giwe15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe16 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe16 {
    #[inline(always)]
    fn from(val: u8) -> Giwe16 {
        Giwe16::from_bits(val)
    }
}
impl From<Giwe16> for u8 {
    #[inline(always)]
    fn from(val: Giwe16) -> u8 {
        Giwe16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe17 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe17 {
    #[inline(always)]
    fn from(val: u8) -> Giwe17 {
        Giwe17::from_bits(val)
    }
}
impl From<Giwe17> for u8 {
    #[inline(always)]
    fn from(val: Giwe17) -> u8 {
        Giwe17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe18 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe18 {
    #[inline(always)]
    fn from(val: u8) -> Giwe18 {
        Giwe18::from_bits(val)
    }
}
impl From<Giwe18> for u8 {
    #[inline(always)]
    fn from(val: Giwe18) -> u8 {
        Giwe18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe19 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe19 {
    #[inline(always)]
    fn from(val: u8) -> Giwe19 {
        Giwe19::from_bits(val)
    }
}
impl From<Giwe19> for u8 {
    #[inline(always)]
    fn from(val: Giwe19) -> u8 {
        Giwe19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe2 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe2 {
    #[inline(always)]
    fn from(val: u8) -> Giwe2 {
        Giwe2::from_bits(val)
    }
}
impl From<Giwe2> for u8 {
    #[inline(always)]
    fn from(val: Giwe2) -> u8 {
        Giwe2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe20 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe20 {
    #[inline(always)]
    fn from(val: u8) -> Giwe20 {
        Giwe20::from_bits(val)
    }
}
impl From<Giwe20> for u8 {
    #[inline(always)]
    fn from(val: Giwe20) -> u8 {
        Giwe20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe21 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe21 {
    #[inline(always)]
    fn from(val: u8) -> Giwe21 {
        Giwe21::from_bits(val)
    }
}
impl From<Giwe21> for u8 {
    #[inline(always)]
    fn from(val: Giwe21) -> u8 {
        Giwe21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe22 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe22 {
    #[inline(always)]
    fn from(val: u8) -> Giwe22 {
        Giwe22::from_bits(val)
    }
}
impl From<Giwe22> for u8 {
    #[inline(always)]
    fn from(val: Giwe22) -> u8 {
        Giwe22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe23 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe23 {
    #[inline(always)]
    fn from(val: u8) -> Giwe23 {
        Giwe23::from_bits(val)
    }
}
impl From<Giwe23> for u8 {
    #[inline(always)]
    fn from(val: Giwe23) -> u8 {
        Giwe23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe24 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe24 {
    #[inline(always)]
    fn from(val: u8) -> Giwe24 {
        Giwe24::from_bits(val)
    }
}
impl From<Giwe24> for u8 {
    #[inline(always)]
    fn from(val: Giwe24) -> u8 {
        Giwe24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe25 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe25 {
    #[inline(always)]
    fn from(val: u8) -> Giwe25 {
        Giwe25::from_bits(val)
    }
}
impl From<Giwe25> for u8 {
    #[inline(always)]
    fn from(val: Giwe25) -> u8 {
        Giwe25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe26 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe26 {
    #[inline(always)]
    fn from(val: u8) -> Giwe26 {
        Giwe26::from_bits(val)
    }
}
impl From<Giwe26> for u8 {
    #[inline(always)]
    fn from(val: Giwe26) -> u8 {
        Giwe26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe27 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe27 {
    #[inline(always)]
    fn from(val: u8) -> Giwe27 {
        Giwe27::from_bits(val)
    }
}
impl From<Giwe27> for u8 {
    #[inline(always)]
    fn from(val: Giwe27) -> u8 {
        Giwe27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe28 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe28 {
    #[inline(always)]
    fn from(val: u8) -> Giwe28 {
        Giwe28::from_bits(val)
    }
}
impl From<Giwe28> for u8 {
    #[inline(always)]
    fn from(val: Giwe28) -> u8 {
        Giwe28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe29 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe29 {
    #[inline(always)]
    fn from(val: u8) -> Giwe29 {
        Giwe29::from_bits(val)
    }
}
impl From<Giwe29> for u8 {
    #[inline(always)]
    fn from(val: Giwe29) -> u8 {
        Giwe29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe3 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe3 {
    #[inline(always)]
    fn from(val: u8) -> Giwe3 {
        Giwe3::from_bits(val)
    }
}
impl From<Giwe3> for u8 {
    #[inline(always)]
    fn from(val: Giwe3) -> u8 {
        Giwe3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe30 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe30 {
    #[inline(always)]
    fn from(val: u8) -> Giwe30 {
        Giwe30::from_bits(val)
    }
}
impl From<Giwe30> for u8 {
    #[inline(always)]
    fn from(val: Giwe30) -> u8 {
        Giwe30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe31 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe31 {
    #[inline(always)]
    fn from(val: u8) -> Giwe31 {
        Giwe31::from_bits(val)
    }
}
impl From<Giwe31> for u8 {
    #[inline(always)]
    fn from(val: Giwe31) -> u8 {
        Giwe31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe4 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe4 {
    #[inline(always)]
    fn from(val: u8) -> Giwe4 {
        Giwe4::from_bits(val)
    }
}
impl From<Giwe4> for u8 {
    #[inline(always)]
    fn from(val: Giwe4) -> u8 {
        Giwe4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe5 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe5 {
    #[inline(always)]
    fn from(val: u8) -> Giwe5 {
        Giwe5::from_bits(val)
    }
}
impl From<Giwe5> for u8 {
    #[inline(always)]
    fn from(val: Giwe5) -> u8 {
        Giwe5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe6 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe6 {
    #[inline(always)]
    fn from(val: u8) -> Giwe6 {
        Giwe6::from_bits(val)
    }
}
impl From<Giwe6> for u8 {
    #[inline(always)]
    fn from(val: Giwe6) -> u8 {
        Giwe6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe7 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe7 {
    #[inline(always)]
    fn from(val: u8) -> Giwe7 {
        Giwe7::from_bits(val)
    }
}
impl From<Giwe7> for u8 {
    #[inline(always)]
    fn from(val: Giwe7) -> u8 {
        Giwe7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe8 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe8 {
    #[inline(always)]
    fn from(val: u8) -> Giwe8 {
        Giwe8::from_bits(val)
    }
}
impl From<Giwe8> for u8 {
    #[inline(always)]
    fn from(val: Giwe8) -> u8 {
        Giwe8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Giwe9 {
    #[doc = "Not updated."]
    GIWE0 = 0x0,
    #[doc = "Updated."]
    GIWE1 = 0x01,
}
impl Giwe9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Giwe9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Giwe9 {
    #[inline(always)]
    fn from(val: u8) -> Giwe9 {
        Giwe9::from_bits(val)
    }
}
impl From<Giwe9> for u8 {
    #[inline(always)]
    fn from(val: Giwe9) -> u8 {
        Giwe9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irqc {
    #[doc = "ISF is disabled."]
    IRQC0 = 0x0,
    #[doc = "ISF and DMA request on rising edge."]
    IRQC1 = 0x01,
    #[doc = "ISF and DMA request on falling edge."]
    IRQC2 = 0x02,
    #[doc = "ISF and DMA request on either edge."]
    IRQC3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "ISF sets on rising edge."]
    IRQC5 = 0x05,
    #[doc = "ISF sets on falling edge."]
    IRQC6 = 0x06,
    #[doc = "ISF sets on either edge."]
    IRQC7 = 0x07,
    #[doc = "ISF and interrupt when logic 0."]
    IRQC8 = 0x08,
    #[doc = "ISF and interrupt on rising edge."]
    IRQC9 = 0x09,
    #[doc = "ISF and interrupt on falling edge."]
    IRQC10 = 0x0a,
    #[doc = "ISF and Interrupt on either edge."]
    IRQC11 = 0x0b,
    #[doc = "ISF and interrupt when logic 1."]
    IRQC12 = 0x0c,
    #[doc = "Enable active-high trigger output; ISF on rising edge (pin state is ORed with other enabled triggers to generate the output trigger for use by other peripherals)."]
    IRQC13 = 0x0d,
    #[doc = "Enable active-low trigger output; ISF on falling edge (pin state is inverted and ORed with other enabled triggers to generate the output trigger for use by other peripherals)."]
    IRQC14 = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqc {
    #[inline(always)]
    fn from(val: u8) -> Irqc {
        Irqc::from_bits(val)
    }
}
impl From<Irqc> for u8 {
    #[inline(always)]
    fn from(val: Irqc) -> u8 {
        Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf {
    #[inline(always)]
    fn from(val: u8) -> Isf {
        Isf::from_bits(val)
    }
}
impl From<Isf> for u8 {
    #[inline(always)]
    fn from(val: Isf) -> u8 {
        Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf0 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf0 {
    #[inline(always)]
    fn from(val: u8) -> Isf0 {
        Isf0::from_bits(val)
    }
}
impl From<Isf0> for u8 {
    #[inline(always)]
    fn from(val: Isf0) -> u8 {
        Isf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf1 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf1 {
    #[inline(always)]
    fn from(val: u8) -> Isf1 {
        Isf1::from_bits(val)
    }
}
impl From<Isf1> for u8 {
    #[inline(always)]
    fn from(val: Isf1) -> u8 {
        Isf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf10 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf10 {
    #[inline(always)]
    fn from(val: u8) -> Isf10 {
        Isf10::from_bits(val)
    }
}
impl From<Isf10> for u8 {
    #[inline(always)]
    fn from(val: Isf10) -> u8 {
        Isf10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf11 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf11 {
    #[inline(always)]
    fn from(val: u8) -> Isf11 {
        Isf11::from_bits(val)
    }
}
impl From<Isf11> for u8 {
    #[inline(always)]
    fn from(val: Isf11) -> u8 {
        Isf11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf12 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf12 {
    #[inline(always)]
    fn from(val: u8) -> Isf12 {
        Isf12::from_bits(val)
    }
}
impl From<Isf12> for u8 {
    #[inline(always)]
    fn from(val: Isf12) -> u8 {
        Isf12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf13 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf13 {
    #[inline(always)]
    fn from(val: u8) -> Isf13 {
        Isf13::from_bits(val)
    }
}
impl From<Isf13> for u8 {
    #[inline(always)]
    fn from(val: Isf13) -> u8 {
        Isf13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf14 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf14 {
    #[inline(always)]
    fn from(val: u8) -> Isf14 {
        Isf14::from_bits(val)
    }
}
impl From<Isf14> for u8 {
    #[inline(always)]
    fn from(val: Isf14) -> u8 {
        Isf14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf15 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf15 {
    #[inline(always)]
    fn from(val: u8) -> Isf15 {
        Isf15::from_bits(val)
    }
}
impl From<Isf15> for u8 {
    #[inline(always)]
    fn from(val: Isf15) -> u8 {
        Isf15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf16 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf16 {
    #[inline(always)]
    fn from(val: u8) -> Isf16 {
        Isf16::from_bits(val)
    }
}
impl From<Isf16> for u8 {
    #[inline(always)]
    fn from(val: Isf16) -> u8 {
        Isf16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf17 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf17 {
    #[inline(always)]
    fn from(val: u8) -> Isf17 {
        Isf17::from_bits(val)
    }
}
impl From<Isf17> for u8 {
    #[inline(always)]
    fn from(val: Isf17) -> u8 {
        Isf17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf18 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf18 {
    #[inline(always)]
    fn from(val: u8) -> Isf18 {
        Isf18::from_bits(val)
    }
}
impl From<Isf18> for u8 {
    #[inline(always)]
    fn from(val: Isf18) -> u8 {
        Isf18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf19 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf19 {
    #[inline(always)]
    fn from(val: u8) -> Isf19 {
        Isf19::from_bits(val)
    }
}
impl From<Isf19> for u8 {
    #[inline(always)]
    fn from(val: Isf19) -> u8 {
        Isf19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf2 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf2 {
    #[inline(always)]
    fn from(val: u8) -> Isf2 {
        Isf2::from_bits(val)
    }
}
impl From<Isf2> for u8 {
    #[inline(always)]
    fn from(val: Isf2) -> u8 {
        Isf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf20 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf20 {
    #[inline(always)]
    fn from(val: u8) -> Isf20 {
        Isf20::from_bits(val)
    }
}
impl From<Isf20> for u8 {
    #[inline(always)]
    fn from(val: Isf20) -> u8 {
        Isf20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf21 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf21 {
    #[inline(always)]
    fn from(val: u8) -> Isf21 {
        Isf21::from_bits(val)
    }
}
impl From<Isf21> for u8 {
    #[inline(always)]
    fn from(val: Isf21) -> u8 {
        Isf21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf22 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf22 {
    #[inline(always)]
    fn from(val: u8) -> Isf22 {
        Isf22::from_bits(val)
    }
}
impl From<Isf22> for u8 {
    #[inline(always)]
    fn from(val: Isf22) -> u8 {
        Isf22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf23 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf23 {
    #[inline(always)]
    fn from(val: u8) -> Isf23 {
        Isf23::from_bits(val)
    }
}
impl From<Isf23> for u8 {
    #[inline(always)]
    fn from(val: Isf23) -> u8 {
        Isf23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf24 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf24 {
    #[inline(always)]
    fn from(val: u8) -> Isf24 {
        Isf24::from_bits(val)
    }
}
impl From<Isf24> for u8 {
    #[inline(always)]
    fn from(val: Isf24) -> u8 {
        Isf24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf25 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf25 {
    #[inline(always)]
    fn from(val: u8) -> Isf25 {
        Isf25::from_bits(val)
    }
}
impl From<Isf25> for u8 {
    #[inline(always)]
    fn from(val: Isf25) -> u8 {
        Isf25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf26 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf26 {
    #[inline(always)]
    fn from(val: u8) -> Isf26 {
        Isf26::from_bits(val)
    }
}
impl From<Isf26> for u8 {
    #[inline(always)]
    fn from(val: Isf26) -> u8 {
        Isf26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf27 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf27 {
    #[inline(always)]
    fn from(val: u8) -> Isf27 {
        Isf27::from_bits(val)
    }
}
impl From<Isf27> for u8 {
    #[inline(always)]
    fn from(val: Isf27) -> u8 {
        Isf27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf28 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf28 {
    #[inline(always)]
    fn from(val: u8) -> Isf28 {
        Isf28::from_bits(val)
    }
}
impl From<Isf28> for u8 {
    #[inline(always)]
    fn from(val: Isf28) -> u8 {
        Isf28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf29 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf29 {
    #[inline(always)]
    fn from(val: u8) -> Isf29 {
        Isf29::from_bits(val)
    }
}
impl From<Isf29> for u8 {
    #[inline(always)]
    fn from(val: Isf29) -> u8 {
        Isf29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf3 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf3 {
    #[inline(always)]
    fn from(val: u8) -> Isf3 {
        Isf3::from_bits(val)
    }
}
impl From<Isf3> for u8 {
    #[inline(always)]
    fn from(val: Isf3) -> u8 {
        Isf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf30 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf30 {
    #[inline(always)]
    fn from(val: u8) -> Isf30 {
        Isf30::from_bits(val)
    }
}
impl From<Isf30> for u8 {
    #[inline(always)]
    fn from(val: Isf30) -> u8 {
        Isf30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf31 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf31 {
    #[inline(always)]
    fn from(val: u8) -> Isf31 {
        Isf31::from_bits(val)
    }
}
impl From<Isf31> for u8 {
    #[inline(always)]
    fn from(val: Isf31) -> u8 {
        Isf31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf4 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf4 {
    #[inline(always)]
    fn from(val: u8) -> Isf4 {
        Isf4::from_bits(val)
    }
}
impl From<Isf4> for u8 {
    #[inline(always)]
    fn from(val: Isf4) -> u8 {
        Isf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf5 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf5 {
    #[inline(always)]
    fn from(val: u8) -> Isf5 {
        Isf5::from_bits(val)
    }
}
impl From<Isf5> for u8 {
    #[inline(always)]
    fn from(val: Isf5) -> u8 {
        Isf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf6 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf6 {
    #[inline(always)]
    fn from(val: u8) -> Isf6 {
        Isf6::from_bits(val)
    }
}
impl From<Isf6> for u8 {
    #[inline(always)]
    fn from(val: Isf6) -> u8 {
        Isf6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf7 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf7 {
    #[inline(always)]
    fn from(val: u8) -> Isf7 {
        Isf7::from_bits(val)
    }
}
impl From<Isf7> for u8 {
    #[inline(always)]
    fn from(val: Isf7) -> u8 {
        Isf7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf8 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf8 {
    #[inline(always)]
    fn from(val: u8) -> Isf8 {
        Isf8::from_bits(val)
    }
}
impl From<Isf8> for u8 {
    #[inline(always)]
    fn from(val: Isf8) -> u8 {
        Isf8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf9 {
    #[doc = "Not detected."]
    ISF0 = 0x0,
    #[doc = "Detected."]
    ISF1 = 0x01,
}
impl Isf9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf9 {
    #[inline(always)]
    fn from(val: u8) -> Isf9 {
        Isf9::from_bits(val)
    }
}
impl From<Isf9> for u8 {
    #[inline(always)]
    fn from(val: Isf9) -> u8 {
        Isf9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pd {
    #[doc = "Logic zero."]
    PD0 = 0x0,
    #[doc = "Logic one."]
    PD1 = 0x01,
}
impl Pd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pd {
    #[inline(always)]
    fn from(val: u8) -> Pd {
        Pd::from_bits(val)
    }
}
impl From<Pd> for u8 {
    #[inline(always)]
    fn from(val: Pd) -> u8 {
        Pd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd0 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd0 {
    #[inline(always)]
    fn from(val: u8) -> Pdd0 {
        Pdd0::from_bits(val)
    }
}
impl From<Pdd0> for u8 {
    #[inline(always)]
    fn from(val: Pdd0) -> u8 {
        Pdd0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd1 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd1 {
    #[inline(always)]
    fn from(val: u8) -> Pdd1 {
        Pdd1::from_bits(val)
    }
}
impl From<Pdd1> for u8 {
    #[inline(always)]
    fn from(val: Pdd1) -> u8 {
        Pdd1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd10 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd10 {
    #[inline(always)]
    fn from(val: u8) -> Pdd10 {
        Pdd10::from_bits(val)
    }
}
impl From<Pdd10> for u8 {
    #[inline(always)]
    fn from(val: Pdd10) -> u8 {
        Pdd10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd11 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd11 {
    #[inline(always)]
    fn from(val: u8) -> Pdd11 {
        Pdd11::from_bits(val)
    }
}
impl From<Pdd11> for u8 {
    #[inline(always)]
    fn from(val: Pdd11) -> u8 {
        Pdd11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd12 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd12 {
    #[inline(always)]
    fn from(val: u8) -> Pdd12 {
        Pdd12::from_bits(val)
    }
}
impl From<Pdd12> for u8 {
    #[inline(always)]
    fn from(val: Pdd12) -> u8 {
        Pdd12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd13 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd13 {
    #[inline(always)]
    fn from(val: u8) -> Pdd13 {
        Pdd13::from_bits(val)
    }
}
impl From<Pdd13> for u8 {
    #[inline(always)]
    fn from(val: Pdd13) -> u8 {
        Pdd13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd14 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd14 {
    #[inline(always)]
    fn from(val: u8) -> Pdd14 {
        Pdd14::from_bits(val)
    }
}
impl From<Pdd14> for u8 {
    #[inline(always)]
    fn from(val: Pdd14) -> u8 {
        Pdd14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd15 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd15 {
    #[inline(always)]
    fn from(val: u8) -> Pdd15 {
        Pdd15::from_bits(val)
    }
}
impl From<Pdd15> for u8 {
    #[inline(always)]
    fn from(val: Pdd15) -> u8 {
        Pdd15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd16 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd16 {
    #[inline(always)]
    fn from(val: u8) -> Pdd16 {
        Pdd16::from_bits(val)
    }
}
impl From<Pdd16> for u8 {
    #[inline(always)]
    fn from(val: Pdd16) -> u8 {
        Pdd16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd17 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd17 {
    #[inline(always)]
    fn from(val: u8) -> Pdd17 {
        Pdd17::from_bits(val)
    }
}
impl From<Pdd17> for u8 {
    #[inline(always)]
    fn from(val: Pdd17) -> u8 {
        Pdd17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd18 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd18 {
    #[inline(always)]
    fn from(val: u8) -> Pdd18 {
        Pdd18::from_bits(val)
    }
}
impl From<Pdd18> for u8 {
    #[inline(always)]
    fn from(val: Pdd18) -> u8 {
        Pdd18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd19 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd19 {
    #[inline(always)]
    fn from(val: u8) -> Pdd19 {
        Pdd19::from_bits(val)
    }
}
impl From<Pdd19> for u8 {
    #[inline(always)]
    fn from(val: Pdd19) -> u8 {
        Pdd19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd2 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd2 {
    #[inline(always)]
    fn from(val: u8) -> Pdd2 {
        Pdd2::from_bits(val)
    }
}
impl From<Pdd2> for u8 {
    #[inline(always)]
    fn from(val: Pdd2) -> u8 {
        Pdd2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd20 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd20 {
    #[inline(always)]
    fn from(val: u8) -> Pdd20 {
        Pdd20::from_bits(val)
    }
}
impl From<Pdd20> for u8 {
    #[inline(always)]
    fn from(val: Pdd20) -> u8 {
        Pdd20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd21 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd21 {
    #[inline(always)]
    fn from(val: u8) -> Pdd21 {
        Pdd21::from_bits(val)
    }
}
impl From<Pdd21> for u8 {
    #[inline(always)]
    fn from(val: Pdd21) -> u8 {
        Pdd21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd22 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd22 {
    #[inline(always)]
    fn from(val: u8) -> Pdd22 {
        Pdd22::from_bits(val)
    }
}
impl From<Pdd22> for u8 {
    #[inline(always)]
    fn from(val: Pdd22) -> u8 {
        Pdd22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd23 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd23 {
    #[inline(always)]
    fn from(val: u8) -> Pdd23 {
        Pdd23::from_bits(val)
    }
}
impl From<Pdd23> for u8 {
    #[inline(always)]
    fn from(val: Pdd23) -> u8 {
        Pdd23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd24 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd24 {
    #[inline(always)]
    fn from(val: u8) -> Pdd24 {
        Pdd24::from_bits(val)
    }
}
impl From<Pdd24> for u8 {
    #[inline(always)]
    fn from(val: Pdd24) -> u8 {
        Pdd24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd25 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd25 {
    #[inline(always)]
    fn from(val: u8) -> Pdd25 {
        Pdd25::from_bits(val)
    }
}
impl From<Pdd25> for u8 {
    #[inline(always)]
    fn from(val: Pdd25) -> u8 {
        Pdd25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd26 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd26 {
    #[inline(always)]
    fn from(val: u8) -> Pdd26 {
        Pdd26::from_bits(val)
    }
}
impl From<Pdd26> for u8 {
    #[inline(always)]
    fn from(val: Pdd26) -> u8 {
        Pdd26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd27 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd27 {
    #[inline(always)]
    fn from(val: u8) -> Pdd27 {
        Pdd27::from_bits(val)
    }
}
impl From<Pdd27> for u8 {
    #[inline(always)]
    fn from(val: Pdd27) -> u8 {
        Pdd27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd28 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd28 {
    #[inline(always)]
    fn from(val: u8) -> Pdd28 {
        Pdd28::from_bits(val)
    }
}
impl From<Pdd28> for u8 {
    #[inline(always)]
    fn from(val: Pdd28) -> u8 {
        Pdd28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd29 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd29 {
    #[inline(always)]
    fn from(val: u8) -> Pdd29 {
        Pdd29::from_bits(val)
    }
}
impl From<Pdd29> for u8 {
    #[inline(always)]
    fn from(val: Pdd29) -> u8 {
        Pdd29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd3 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd3 {
    #[inline(always)]
    fn from(val: u8) -> Pdd3 {
        Pdd3::from_bits(val)
    }
}
impl From<Pdd3> for u8 {
    #[inline(always)]
    fn from(val: Pdd3) -> u8 {
        Pdd3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd30 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd30 {
    #[inline(always)]
    fn from(val: u8) -> Pdd30 {
        Pdd30::from_bits(val)
    }
}
impl From<Pdd30> for u8 {
    #[inline(always)]
    fn from(val: Pdd30) -> u8 {
        Pdd30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd31 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd31 {
    #[inline(always)]
    fn from(val: u8) -> Pdd31 {
        Pdd31::from_bits(val)
    }
}
impl From<Pdd31> for u8 {
    #[inline(always)]
    fn from(val: Pdd31) -> u8 {
        Pdd31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd4 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd4 {
    #[inline(always)]
    fn from(val: u8) -> Pdd4 {
        Pdd4::from_bits(val)
    }
}
impl From<Pdd4> for u8 {
    #[inline(always)]
    fn from(val: Pdd4) -> u8 {
        Pdd4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd5 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd5 {
    #[inline(always)]
    fn from(val: u8) -> Pdd5 {
        Pdd5::from_bits(val)
    }
}
impl From<Pdd5> for u8 {
    #[inline(always)]
    fn from(val: Pdd5) -> u8 {
        Pdd5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd6 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd6 {
    #[inline(always)]
    fn from(val: u8) -> Pdd6 {
        Pdd6::from_bits(val)
    }
}
impl From<Pdd6> for u8 {
    #[inline(always)]
    fn from(val: Pdd6) -> u8 {
        Pdd6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd7 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd7 {
    #[inline(always)]
    fn from(val: u8) -> Pdd7 {
        Pdd7::from_bits(val)
    }
}
impl From<Pdd7> for u8 {
    #[inline(always)]
    fn from(val: Pdd7) -> u8 {
        Pdd7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd8 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd8 {
    #[inline(always)]
    fn from(val: u8) -> Pdd8 {
        Pdd8::from_bits(val)
    }
}
impl From<Pdd8> for u8 {
    #[inline(always)]
    fn from(val: Pdd8) -> u8 {
        Pdd8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd9 {
    #[doc = "Input."]
    PDD0 = 0x0,
    #[doc = "Output."]
    PDD1 = 0x01,
}
impl Pdd9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd9 {
    #[inline(always)]
    fn from(val: u8) -> Pdd9 {
        Pdd9::from_bits(val)
    }
}
impl From<Pdd9> for u8 {
    #[inline(always)]
    fn from(val: Pdd9) -> u8 {
        Pdd9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi0 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi0 {
    #[inline(always)]
    fn from(val: u8) -> Pdi0 {
        Pdi0::from_bits(val)
    }
}
impl From<Pdi0> for u8 {
    #[inline(always)]
    fn from(val: Pdi0) -> u8 {
        Pdi0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi1 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi1 {
    #[inline(always)]
    fn from(val: u8) -> Pdi1 {
        Pdi1::from_bits(val)
    }
}
impl From<Pdi1> for u8 {
    #[inline(always)]
    fn from(val: Pdi1) -> u8 {
        Pdi1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi10 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi10 {
    #[inline(always)]
    fn from(val: u8) -> Pdi10 {
        Pdi10::from_bits(val)
    }
}
impl From<Pdi10> for u8 {
    #[inline(always)]
    fn from(val: Pdi10) -> u8 {
        Pdi10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi11 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi11 {
    #[inline(always)]
    fn from(val: u8) -> Pdi11 {
        Pdi11::from_bits(val)
    }
}
impl From<Pdi11> for u8 {
    #[inline(always)]
    fn from(val: Pdi11) -> u8 {
        Pdi11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi12 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi12 {
    #[inline(always)]
    fn from(val: u8) -> Pdi12 {
        Pdi12::from_bits(val)
    }
}
impl From<Pdi12> for u8 {
    #[inline(always)]
    fn from(val: Pdi12) -> u8 {
        Pdi12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi13 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi13 {
    #[inline(always)]
    fn from(val: u8) -> Pdi13 {
        Pdi13::from_bits(val)
    }
}
impl From<Pdi13> for u8 {
    #[inline(always)]
    fn from(val: Pdi13) -> u8 {
        Pdi13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi14 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi14 {
    #[inline(always)]
    fn from(val: u8) -> Pdi14 {
        Pdi14::from_bits(val)
    }
}
impl From<Pdi14> for u8 {
    #[inline(always)]
    fn from(val: Pdi14) -> u8 {
        Pdi14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi15 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi15 {
    #[inline(always)]
    fn from(val: u8) -> Pdi15 {
        Pdi15::from_bits(val)
    }
}
impl From<Pdi15> for u8 {
    #[inline(always)]
    fn from(val: Pdi15) -> u8 {
        Pdi15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi16 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi16 {
    #[inline(always)]
    fn from(val: u8) -> Pdi16 {
        Pdi16::from_bits(val)
    }
}
impl From<Pdi16> for u8 {
    #[inline(always)]
    fn from(val: Pdi16) -> u8 {
        Pdi16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi17 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi17 {
    #[inline(always)]
    fn from(val: u8) -> Pdi17 {
        Pdi17::from_bits(val)
    }
}
impl From<Pdi17> for u8 {
    #[inline(always)]
    fn from(val: Pdi17) -> u8 {
        Pdi17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi18 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi18 {
    #[inline(always)]
    fn from(val: u8) -> Pdi18 {
        Pdi18::from_bits(val)
    }
}
impl From<Pdi18> for u8 {
    #[inline(always)]
    fn from(val: Pdi18) -> u8 {
        Pdi18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi19 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi19 {
    #[inline(always)]
    fn from(val: u8) -> Pdi19 {
        Pdi19::from_bits(val)
    }
}
impl From<Pdi19> for u8 {
    #[inline(always)]
    fn from(val: Pdi19) -> u8 {
        Pdi19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi2 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi2 {
    #[inline(always)]
    fn from(val: u8) -> Pdi2 {
        Pdi2::from_bits(val)
    }
}
impl From<Pdi2> for u8 {
    #[inline(always)]
    fn from(val: Pdi2) -> u8 {
        Pdi2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi20 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi20 {
    #[inline(always)]
    fn from(val: u8) -> Pdi20 {
        Pdi20::from_bits(val)
    }
}
impl From<Pdi20> for u8 {
    #[inline(always)]
    fn from(val: Pdi20) -> u8 {
        Pdi20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi21 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi21 {
    #[inline(always)]
    fn from(val: u8) -> Pdi21 {
        Pdi21::from_bits(val)
    }
}
impl From<Pdi21> for u8 {
    #[inline(always)]
    fn from(val: Pdi21) -> u8 {
        Pdi21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi22 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi22 {
    #[inline(always)]
    fn from(val: u8) -> Pdi22 {
        Pdi22::from_bits(val)
    }
}
impl From<Pdi22> for u8 {
    #[inline(always)]
    fn from(val: Pdi22) -> u8 {
        Pdi22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi23 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi23 {
    #[inline(always)]
    fn from(val: u8) -> Pdi23 {
        Pdi23::from_bits(val)
    }
}
impl From<Pdi23> for u8 {
    #[inline(always)]
    fn from(val: Pdi23) -> u8 {
        Pdi23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi24 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi24 {
    #[inline(always)]
    fn from(val: u8) -> Pdi24 {
        Pdi24::from_bits(val)
    }
}
impl From<Pdi24> for u8 {
    #[inline(always)]
    fn from(val: Pdi24) -> u8 {
        Pdi24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi25 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi25 {
    #[inline(always)]
    fn from(val: u8) -> Pdi25 {
        Pdi25::from_bits(val)
    }
}
impl From<Pdi25> for u8 {
    #[inline(always)]
    fn from(val: Pdi25) -> u8 {
        Pdi25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi26 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi26 {
    #[inline(always)]
    fn from(val: u8) -> Pdi26 {
        Pdi26::from_bits(val)
    }
}
impl From<Pdi26> for u8 {
    #[inline(always)]
    fn from(val: Pdi26) -> u8 {
        Pdi26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi27 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi27 {
    #[inline(always)]
    fn from(val: u8) -> Pdi27 {
        Pdi27::from_bits(val)
    }
}
impl From<Pdi27> for u8 {
    #[inline(always)]
    fn from(val: Pdi27) -> u8 {
        Pdi27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi28 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi28 {
    #[inline(always)]
    fn from(val: u8) -> Pdi28 {
        Pdi28::from_bits(val)
    }
}
impl From<Pdi28> for u8 {
    #[inline(always)]
    fn from(val: Pdi28) -> u8 {
        Pdi28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi29 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi29 {
    #[inline(always)]
    fn from(val: u8) -> Pdi29 {
        Pdi29::from_bits(val)
    }
}
impl From<Pdi29> for u8 {
    #[inline(always)]
    fn from(val: Pdi29) -> u8 {
        Pdi29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi3 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi3 {
    #[inline(always)]
    fn from(val: u8) -> Pdi3 {
        Pdi3::from_bits(val)
    }
}
impl From<Pdi3> for u8 {
    #[inline(always)]
    fn from(val: Pdi3) -> u8 {
        Pdi3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi30 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi30 {
    #[inline(always)]
    fn from(val: u8) -> Pdi30 {
        Pdi30::from_bits(val)
    }
}
impl From<Pdi30> for u8 {
    #[inline(always)]
    fn from(val: Pdi30) -> u8 {
        Pdi30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi31 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi31 {
    #[inline(always)]
    fn from(val: u8) -> Pdi31 {
        Pdi31::from_bits(val)
    }
}
impl From<Pdi31> for u8 {
    #[inline(always)]
    fn from(val: Pdi31) -> u8 {
        Pdi31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi4 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi4 {
    #[inline(always)]
    fn from(val: u8) -> Pdi4 {
        Pdi4::from_bits(val)
    }
}
impl From<Pdi4> for u8 {
    #[inline(always)]
    fn from(val: Pdi4) -> u8 {
        Pdi4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi5 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi5 {
    #[inline(always)]
    fn from(val: u8) -> Pdi5 {
        Pdi5::from_bits(val)
    }
}
impl From<Pdi5> for u8 {
    #[inline(always)]
    fn from(val: Pdi5) -> u8 {
        Pdi5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi6 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi6 {
    #[inline(always)]
    fn from(val: u8) -> Pdi6 {
        Pdi6::from_bits(val)
    }
}
impl From<Pdi6> for u8 {
    #[inline(always)]
    fn from(val: Pdi6) -> u8 {
        Pdi6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi7 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi7 {
    #[inline(always)]
    fn from(val: u8) -> Pdi7 {
        Pdi7::from_bits(val)
    }
}
impl From<Pdi7> for u8 {
    #[inline(always)]
    fn from(val: Pdi7) -> u8 {
        Pdi7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi8 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi8 {
    #[inline(always)]
    fn from(val: u8) -> Pdi8 {
        Pdi8::from_bits(val)
    }
}
impl From<Pdi8> for u8 {
    #[inline(always)]
    fn from(val: Pdi8) -> u8 {
        Pdi8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdi9 {
    #[doc = "Logic 0."]
    PDI0 = 0x0,
    #[doc = "Logic 1."]
    PDI1 = 0x01,
}
impl Pdi9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdi9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdi9 {
    #[inline(always)]
    fn from(val: u8) -> Pdi9 {
        Pdi9::from_bits(val)
    }
}
impl From<Pdi9> for u8 {
    #[inline(always)]
    fn from(val: Pdi9) -> u8 {
        Pdi9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo0 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo0 {
    #[inline(always)]
    fn from(val: u8) -> Pdo0 {
        Pdo0::from_bits(val)
    }
}
impl From<Pdo0> for u8 {
    #[inline(always)]
    fn from(val: Pdo0) -> u8 {
        Pdo0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo1 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo1 {
    #[inline(always)]
    fn from(val: u8) -> Pdo1 {
        Pdo1::from_bits(val)
    }
}
impl From<Pdo1> for u8 {
    #[inline(always)]
    fn from(val: Pdo1) -> u8 {
        Pdo1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo10 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo10 {
    #[inline(always)]
    fn from(val: u8) -> Pdo10 {
        Pdo10::from_bits(val)
    }
}
impl From<Pdo10> for u8 {
    #[inline(always)]
    fn from(val: Pdo10) -> u8 {
        Pdo10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo11 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo11 {
    #[inline(always)]
    fn from(val: u8) -> Pdo11 {
        Pdo11::from_bits(val)
    }
}
impl From<Pdo11> for u8 {
    #[inline(always)]
    fn from(val: Pdo11) -> u8 {
        Pdo11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo12 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo12 {
    #[inline(always)]
    fn from(val: u8) -> Pdo12 {
        Pdo12::from_bits(val)
    }
}
impl From<Pdo12> for u8 {
    #[inline(always)]
    fn from(val: Pdo12) -> u8 {
        Pdo12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo13 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo13 {
    #[inline(always)]
    fn from(val: u8) -> Pdo13 {
        Pdo13::from_bits(val)
    }
}
impl From<Pdo13> for u8 {
    #[inline(always)]
    fn from(val: Pdo13) -> u8 {
        Pdo13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo14 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo14 {
    #[inline(always)]
    fn from(val: u8) -> Pdo14 {
        Pdo14::from_bits(val)
    }
}
impl From<Pdo14> for u8 {
    #[inline(always)]
    fn from(val: Pdo14) -> u8 {
        Pdo14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo15 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo15 {
    #[inline(always)]
    fn from(val: u8) -> Pdo15 {
        Pdo15::from_bits(val)
    }
}
impl From<Pdo15> for u8 {
    #[inline(always)]
    fn from(val: Pdo15) -> u8 {
        Pdo15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo16 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo16 {
    #[inline(always)]
    fn from(val: u8) -> Pdo16 {
        Pdo16::from_bits(val)
    }
}
impl From<Pdo16> for u8 {
    #[inline(always)]
    fn from(val: Pdo16) -> u8 {
        Pdo16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo17 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo17 {
    #[inline(always)]
    fn from(val: u8) -> Pdo17 {
        Pdo17::from_bits(val)
    }
}
impl From<Pdo17> for u8 {
    #[inline(always)]
    fn from(val: Pdo17) -> u8 {
        Pdo17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo18 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo18 {
    #[inline(always)]
    fn from(val: u8) -> Pdo18 {
        Pdo18::from_bits(val)
    }
}
impl From<Pdo18> for u8 {
    #[inline(always)]
    fn from(val: Pdo18) -> u8 {
        Pdo18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo19 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo19 {
    #[inline(always)]
    fn from(val: u8) -> Pdo19 {
        Pdo19::from_bits(val)
    }
}
impl From<Pdo19> for u8 {
    #[inline(always)]
    fn from(val: Pdo19) -> u8 {
        Pdo19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo2 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo2 {
    #[inline(always)]
    fn from(val: u8) -> Pdo2 {
        Pdo2::from_bits(val)
    }
}
impl From<Pdo2> for u8 {
    #[inline(always)]
    fn from(val: Pdo2) -> u8 {
        Pdo2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo20 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo20 {
    #[inline(always)]
    fn from(val: u8) -> Pdo20 {
        Pdo20::from_bits(val)
    }
}
impl From<Pdo20> for u8 {
    #[inline(always)]
    fn from(val: Pdo20) -> u8 {
        Pdo20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo21 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo21 {
    #[inline(always)]
    fn from(val: u8) -> Pdo21 {
        Pdo21::from_bits(val)
    }
}
impl From<Pdo21> for u8 {
    #[inline(always)]
    fn from(val: Pdo21) -> u8 {
        Pdo21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo22 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo22 {
    #[inline(always)]
    fn from(val: u8) -> Pdo22 {
        Pdo22::from_bits(val)
    }
}
impl From<Pdo22> for u8 {
    #[inline(always)]
    fn from(val: Pdo22) -> u8 {
        Pdo22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo23 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo23 {
    #[inline(always)]
    fn from(val: u8) -> Pdo23 {
        Pdo23::from_bits(val)
    }
}
impl From<Pdo23> for u8 {
    #[inline(always)]
    fn from(val: Pdo23) -> u8 {
        Pdo23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo24 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo24 {
    #[inline(always)]
    fn from(val: u8) -> Pdo24 {
        Pdo24::from_bits(val)
    }
}
impl From<Pdo24> for u8 {
    #[inline(always)]
    fn from(val: Pdo24) -> u8 {
        Pdo24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo25 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo25 {
    #[inline(always)]
    fn from(val: u8) -> Pdo25 {
        Pdo25::from_bits(val)
    }
}
impl From<Pdo25> for u8 {
    #[inline(always)]
    fn from(val: Pdo25) -> u8 {
        Pdo25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo26 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo26 {
    #[inline(always)]
    fn from(val: u8) -> Pdo26 {
        Pdo26::from_bits(val)
    }
}
impl From<Pdo26> for u8 {
    #[inline(always)]
    fn from(val: Pdo26) -> u8 {
        Pdo26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo27 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo27 {
    #[inline(always)]
    fn from(val: u8) -> Pdo27 {
        Pdo27::from_bits(val)
    }
}
impl From<Pdo27> for u8 {
    #[inline(always)]
    fn from(val: Pdo27) -> u8 {
        Pdo27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo28 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo28 {
    #[inline(always)]
    fn from(val: u8) -> Pdo28 {
        Pdo28::from_bits(val)
    }
}
impl From<Pdo28> for u8 {
    #[inline(always)]
    fn from(val: Pdo28) -> u8 {
        Pdo28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo29 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo29 {
    #[inline(always)]
    fn from(val: u8) -> Pdo29 {
        Pdo29::from_bits(val)
    }
}
impl From<Pdo29> for u8 {
    #[inline(always)]
    fn from(val: Pdo29) -> u8 {
        Pdo29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo3 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo3 {
    #[inline(always)]
    fn from(val: u8) -> Pdo3 {
        Pdo3::from_bits(val)
    }
}
impl From<Pdo3> for u8 {
    #[inline(always)]
    fn from(val: Pdo3) -> u8 {
        Pdo3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo30 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo30 {
    #[inline(always)]
    fn from(val: u8) -> Pdo30 {
        Pdo30::from_bits(val)
    }
}
impl From<Pdo30> for u8 {
    #[inline(always)]
    fn from(val: Pdo30) -> u8 {
        Pdo30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo31 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo31 {
    #[inline(always)]
    fn from(val: u8) -> Pdo31 {
        Pdo31::from_bits(val)
    }
}
impl From<Pdo31> for u8 {
    #[inline(always)]
    fn from(val: Pdo31) -> u8 {
        Pdo31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo4 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo4 {
    #[inline(always)]
    fn from(val: u8) -> Pdo4 {
        Pdo4::from_bits(val)
    }
}
impl From<Pdo4> for u8 {
    #[inline(always)]
    fn from(val: Pdo4) -> u8 {
        Pdo4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo5 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo5 {
    #[inline(always)]
    fn from(val: u8) -> Pdo5 {
        Pdo5::from_bits(val)
    }
}
impl From<Pdo5> for u8 {
    #[inline(always)]
    fn from(val: Pdo5) -> u8 {
        Pdo5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo6 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo6 {
    #[inline(always)]
    fn from(val: u8) -> Pdo6 {
        Pdo6::from_bits(val)
    }
}
impl From<Pdo6> for u8 {
    #[inline(always)]
    fn from(val: Pdo6) -> u8 {
        Pdo6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo7 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo7 {
    #[inline(always)]
    fn from(val: u8) -> Pdo7 {
        Pdo7::from_bits(val)
    }
}
impl From<Pdo7> for u8 {
    #[inline(always)]
    fn from(val: Pdo7) -> u8 {
        Pdo7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo8 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo8 {
    #[inline(always)]
    fn from(val: u8) -> Pdo8 {
        Pdo8::from_bits(val)
    }
}
impl From<Pdo8> for u8 {
    #[inline(always)]
    fn from(val: Pdo8) -> u8 {
        Pdo8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo9 {
    #[doc = "Logic level 0."]
    PDO0 = 0x0,
    #[doc = "Logic level 1."]
    PDO1 = 0x01,
}
impl Pdo9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdo9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdo9 {
    #[inline(always)]
    fn from(val: u8) -> Pdo9 {
        Pdo9::from_bits(val)
    }
}
impl From<Pdo9> for u8 {
    #[inline(always)]
    fn from(val: Pdo9) -> u8 {
        Pdo9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid0 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid0 {
    #[inline(always)]
    fn from(val: u8) -> Pid0 {
        Pid0::from_bits(val)
    }
}
impl From<Pid0> for u8 {
    #[inline(always)]
    fn from(val: Pid0) -> u8 {
        Pid0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid1 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid1 {
    #[inline(always)]
    fn from(val: u8) -> Pid1 {
        Pid1::from_bits(val)
    }
}
impl From<Pid1> for u8 {
    #[inline(always)]
    fn from(val: Pid1) -> u8 {
        Pid1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid10 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid10 {
    #[inline(always)]
    fn from(val: u8) -> Pid10 {
        Pid10::from_bits(val)
    }
}
impl From<Pid10> for u8 {
    #[inline(always)]
    fn from(val: Pid10) -> u8 {
        Pid10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid11 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid11 {
    #[inline(always)]
    fn from(val: u8) -> Pid11 {
        Pid11::from_bits(val)
    }
}
impl From<Pid11> for u8 {
    #[inline(always)]
    fn from(val: Pid11) -> u8 {
        Pid11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid12 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid12 {
    #[inline(always)]
    fn from(val: u8) -> Pid12 {
        Pid12::from_bits(val)
    }
}
impl From<Pid12> for u8 {
    #[inline(always)]
    fn from(val: Pid12) -> u8 {
        Pid12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid13 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid13 {
    #[inline(always)]
    fn from(val: u8) -> Pid13 {
        Pid13::from_bits(val)
    }
}
impl From<Pid13> for u8 {
    #[inline(always)]
    fn from(val: Pid13) -> u8 {
        Pid13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid14 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid14 {
    #[inline(always)]
    fn from(val: u8) -> Pid14 {
        Pid14::from_bits(val)
    }
}
impl From<Pid14> for u8 {
    #[inline(always)]
    fn from(val: Pid14) -> u8 {
        Pid14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid15 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid15 {
    #[inline(always)]
    fn from(val: u8) -> Pid15 {
        Pid15::from_bits(val)
    }
}
impl From<Pid15> for u8 {
    #[inline(always)]
    fn from(val: Pid15) -> u8 {
        Pid15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid16 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid16 {
    #[inline(always)]
    fn from(val: u8) -> Pid16 {
        Pid16::from_bits(val)
    }
}
impl From<Pid16> for u8 {
    #[inline(always)]
    fn from(val: Pid16) -> u8 {
        Pid16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid17 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid17 {
    #[inline(always)]
    fn from(val: u8) -> Pid17 {
        Pid17::from_bits(val)
    }
}
impl From<Pid17> for u8 {
    #[inline(always)]
    fn from(val: Pid17) -> u8 {
        Pid17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid18 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid18 {
    #[inline(always)]
    fn from(val: u8) -> Pid18 {
        Pid18::from_bits(val)
    }
}
impl From<Pid18> for u8 {
    #[inline(always)]
    fn from(val: Pid18) -> u8 {
        Pid18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid19 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid19 {
    #[inline(always)]
    fn from(val: u8) -> Pid19 {
        Pid19::from_bits(val)
    }
}
impl From<Pid19> for u8 {
    #[inline(always)]
    fn from(val: Pid19) -> u8 {
        Pid19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid2 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid2 {
    #[inline(always)]
    fn from(val: u8) -> Pid2 {
        Pid2::from_bits(val)
    }
}
impl From<Pid2> for u8 {
    #[inline(always)]
    fn from(val: Pid2) -> u8 {
        Pid2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid20 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid20 {
    #[inline(always)]
    fn from(val: u8) -> Pid20 {
        Pid20::from_bits(val)
    }
}
impl From<Pid20> for u8 {
    #[inline(always)]
    fn from(val: Pid20) -> u8 {
        Pid20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid21 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid21 {
    #[inline(always)]
    fn from(val: u8) -> Pid21 {
        Pid21::from_bits(val)
    }
}
impl From<Pid21> for u8 {
    #[inline(always)]
    fn from(val: Pid21) -> u8 {
        Pid21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid22 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid22 {
    #[inline(always)]
    fn from(val: u8) -> Pid22 {
        Pid22::from_bits(val)
    }
}
impl From<Pid22> for u8 {
    #[inline(always)]
    fn from(val: Pid22) -> u8 {
        Pid22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid23 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid23 {
    #[inline(always)]
    fn from(val: u8) -> Pid23 {
        Pid23::from_bits(val)
    }
}
impl From<Pid23> for u8 {
    #[inline(always)]
    fn from(val: Pid23) -> u8 {
        Pid23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid24 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid24 {
    #[inline(always)]
    fn from(val: u8) -> Pid24 {
        Pid24::from_bits(val)
    }
}
impl From<Pid24> for u8 {
    #[inline(always)]
    fn from(val: Pid24) -> u8 {
        Pid24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid25 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid25 {
    #[inline(always)]
    fn from(val: u8) -> Pid25 {
        Pid25::from_bits(val)
    }
}
impl From<Pid25> for u8 {
    #[inline(always)]
    fn from(val: Pid25) -> u8 {
        Pid25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid26 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid26 {
    #[inline(always)]
    fn from(val: u8) -> Pid26 {
        Pid26::from_bits(val)
    }
}
impl From<Pid26> for u8 {
    #[inline(always)]
    fn from(val: Pid26) -> u8 {
        Pid26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid27 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid27 {
    #[inline(always)]
    fn from(val: u8) -> Pid27 {
        Pid27::from_bits(val)
    }
}
impl From<Pid27> for u8 {
    #[inline(always)]
    fn from(val: Pid27) -> u8 {
        Pid27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid28 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid28 {
    #[inline(always)]
    fn from(val: u8) -> Pid28 {
        Pid28::from_bits(val)
    }
}
impl From<Pid28> for u8 {
    #[inline(always)]
    fn from(val: Pid28) -> u8 {
        Pid28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid29 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid29 {
    #[inline(always)]
    fn from(val: u8) -> Pid29 {
        Pid29::from_bits(val)
    }
}
impl From<Pid29> for u8 {
    #[inline(always)]
    fn from(val: Pid29) -> u8 {
        Pid29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid3 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid3 {
    #[inline(always)]
    fn from(val: u8) -> Pid3 {
        Pid3::from_bits(val)
    }
}
impl From<Pid3> for u8 {
    #[inline(always)]
    fn from(val: Pid3) -> u8 {
        Pid3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid30 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid30 {
    #[inline(always)]
    fn from(val: u8) -> Pid30 {
        Pid30::from_bits(val)
    }
}
impl From<Pid30> for u8 {
    #[inline(always)]
    fn from(val: Pid30) -> u8 {
        Pid30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid31 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid31 {
    #[inline(always)]
    fn from(val: u8) -> Pid31 {
        Pid31::from_bits(val)
    }
}
impl From<Pid31> for u8 {
    #[inline(always)]
    fn from(val: Pid31) -> u8 {
        Pid31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid4 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid4 {
    #[inline(always)]
    fn from(val: u8) -> Pid4 {
        Pid4::from_bits(val)
    }
}
impl From<Pid4> for u8 {
    #[inline(always)]
    fn from(val: Pid4) -> u8 {
        Pid4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid5 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid5 {
    #[inline(always)]
    fn from(val: u8) -> Pid5 {
        Pid5::from_bits(val)
    }
}
impl From<Pid5> for u8 {
    #[inline(always)]
    fn from(val: Pid5) -> u8 {
        Pid5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid6 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid6 {
    #[inline(always)]
    fn from(val: u8) -> Pid6 {
        Pid6::from_bits(val)
    }
}
impl From<Pid6> for u8 {
    #[inline(always)]
    fn from(val: Pid6) -> u8 {
        Pid6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid7 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid7 {
    #[inline(always)]
    fn from(val: u8) -> Pid7 {
        Pid7::from_bits(val)
    }
}
impl From<Pid7> for u8 {
    #[inline(always)]
    fn from(val: Pid7) -> u8 {
        Pid7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid8 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid8 {
    #[inline(always)]
    fn from(val: u8) -> Pid8 {
        Pid8::from_bits(val)
    }
}
impl From<Pid8> for u8 {
    #[inline(always)]
    fn from(val: Pid8) -> u8 {
        Pid8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid9 {
    #[doc = "Configured for general-purpose input."]
    PID0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    PID1 = 0x01,
}
impl Pid9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid9 {
    #[inline(always)]
    fn from(val: u8) -> Pid9 {
        Pid9::from_bits(val)
    }
}
impl From<Pid9> for u8 {
    #[inline(always)]
    fn from(val: Pid9) -> u8 {
        Pid9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco0 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco0 {
    #[inline(always)]
    fn from(val: u8) -> Ptco0 {
        Ptco0::from_bits(val)
    }
}
impl From<Ptco0> for u8 {
    #[inline(always)]
    fn from(val: Ptco0) -> u8 {
        Ptco0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco1 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco1 {
    #[inline(always)]
    fn from(val: u8) -> Ptco1 {
        Ptco1::from_bits(val)
    }
}
impl From<Ptco1> for u8 {
    #[inline(always)]
    fn from(val: Ptco1) -> u8 {
        Ptco1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco10 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco10 {
    #[inline(always)]
    fn from(val: u8) -> Ptco10 {
        Ptco10::from_bits(val)
    }
}
impl From<Ptco10> for u8 {
    #[inline(always)]
    fn from(val: Ptco10) -> u8 {
        Ptco10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco11 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco11 {
    #[inline(always)]
    fn from(val: u8) -> Ptco11 {
        Ptco11::from_bits(val)
    }
}
impl From<Ptco11> for u8 {
    #[inline(always)]
    fn from(val: Ptco11) -> u8 {
        Ptco11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco12 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco12 {
    #[inline(always)]
    fn from(val: u8) -> Ptco12 {
        Ptco12::from_bits(val)
    }
}
impl From<Ptco12> for u8 {
    #[inline(always)]
    fn from(val: Ptco12) -> u8 {
        Ptco12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco13 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco13 {
    #[inline(always)]
    fn from(val: u8) -> Ptco13 {
        Ptco13::from_bits(val)
    }
}
impl From<Ptco13> for u8 {
    #[inline(always)]
    fn from(val: Ptco13) -> u8 {
        Ptco13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco14 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco14 {
    #[inline(always)]
    fn from(val: u8) -> Ptco14 {
        Ptco14::from_bits(val)
    }
}
impl From<Ptco14> for u8 {
    #[inline(always)]
    fn from(val: Ptco14) -> u8 {
        Ptco14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco15 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco15 {
    #[inline(always)]
    fn from(val: u8) -> Ptco15 {
        Ptco15::from_bits(val)
    }
}
impl From<Ptco15> for u8 {
    #[inline(always)]
    fn from(val: Ptco15) -> u8 {
        Ptco15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco16 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco16 {
    #[inline(always)]
    fn from(val: u8) -> Ptco16 {
        Ptco16::from_bits(val)
    }
}
impl From<Ptco16> for u8 {
    #[inline(always)]
    fn from(val: Ptco16) -> u8 {
        Ptco16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco17 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco17 {
    #[inline(always)]
    fn from(val: u8) -> Ptco17 {
        Ptco17::from_bits(val)
    }
}
impl From<Ptco17> for u8 {
    #[inline(always)]
    fn from(val: Ptco17) -> u8 {
        Ptco17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco18 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco18 {
    #[inline(always)]
    fn from(val: u8) -> Ptco18 {
        Ptco18::from_bits(val)
    }
}
impl From<Ptco18> for u8 {
    #[inline(always)]
    fn from(val: Ptco18) -> u8 {
        Ptco18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco19 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco19 {
    #[inline(always)]
    fn from(val: u8) -> Ptco19 {
        Ptco19::from_bits(val)
    }
}
impl From<Ptco19> for u8 {
    #[inline(always)]
    fn from(val: Ptco19) -> u8 {
        Ptco19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco2 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco2 {
    #[inline(always)]
    fn from(val: u8) -> Ptco2 {
        Ptco2::from_bits(val)
    }
}
impl From<Ptco2> for u8 {
    #[inline(always)]
    fn from(val: Ptco2) -> u8 {
        Ptco2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco20 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco20 {
    #[inline(always)]
    fn from(val: u8) -> Ptco20 {
        Ptco20::from_bits(val)
    }
}
impl From<Ptco20> for u8 {
    #[inline(always)]
    fn from(val: Ptco20) -> u8 {
        Ptco20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco21 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco21 {
    #[inline(always)]
    fn from(val: u8) -> Ptco21 {
        Ptco21::from_bits(val)
    }
}
impl From<Ptco21> for u8 {
    #[inline(always)]
    fn from(val: Ptco21) -> u8 {
        Ptco21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco22 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco22 {
    #[inline(always)]
    fn from(val: u8) -> Ptco22 {
        Ptco22::from_bits(val)
    }
}
impl From<Ptco22> for u8 {
    #[inline(always)]
    fn from(val: Ptco22) -> u8 {
        Ptco22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco23 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco23 {
    #[inline(always)]
    fn from(val: u8) -> Ptco23 {
        Ptco23::from_bits(val)
    }
}
impl From<Ptco23> for u8 {
    #[inline(always)]
    fn from(val: Ptco23) -> u8 {
        Ptco23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco24 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco24 {
    #[inline(always)]
    fn from(val: u8) -> Ptco24 {
        Ptco24::from_bits(val)
    }
}
impl From<Ptco24> for u8 {
    #[inline(always)]
    fn from(val: Ptco24) -> u8 {
        Ptco24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco25 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco25 {
    #[inline(always)]
    fn from(val: u8) -> Ptco25 {
        Ptco25::from_bits(val)
    }
}
impl From<Ptco25> for u8 {
    #[inline(always)]
    fn from(val: Ptco25) -> u8 {
        Ptco25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco26 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco26 {
    #[inline(always)]
    fn from(val: u8) -> Ptco26 {
        Ptco26::from_bits(val)
    }
}
impl From<Ptco26> for u8 {
    #[inline(always)]
    fn from(val: Ptco26) -> u8 {
        Ptco26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco27 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco27 {
    #[inline(always)]
    fn from(val: u8) -> Ptco27 {
        Ptco27::from_bits(val)
    }
}
impl From<Ptco27> for u8 {
    #[inline(always)]
    fn from(val: Ptco27) -> u8 {
        Ptco27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco28 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco28 {
    #[inline(always)]
    fn from(val: u8) -> Ptco28 {
        Ptco28::from_bits(val)
    }
}
impl From<Ptco28> for u8 {
    #[inline(always)]
    fn from(val: Ptco28) -> u8 {
        Ptco28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco29 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco29 {
    #[inline(always)]
    fn from(val: u8) -> Ptco29 {
        Ptco29::from_bits(val)
    }
}
impl From<Ptco29> for u8 {
    #[inline(always)]
    fn from(val: Ptco29) -> u8 {
        Ptco29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco3 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco3 {
    #[inline(always)]
    fn from(val: u8) -> Ptco3 {
        Ptco3::from_bits(val)
    }
}
impl From<Ptco3> for u8 {
    #[inline(always)]
    fn from(val: Ptco3) -> u8 {
        Ptco3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco30 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco30 {
    #[inline(always)]
    fn from(val: u8) -> Ptco30 {
        Ptco30::from_bits(val)
    }
}
impl From<Ptco30> for u8 {
    #[inline(always)]
    fn from(val: Ptco30) -> u8 {
        Ptco30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco31 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco31 {
    #[inline(always)]
    fn from(val: u8) -> Ptco31 {
        Ptco31::from_bits(val)
    }
}
impl From<Ptco31> for u8 {
    #[inline(always)]
    fn from(val: Ptco31) -> u8 {
        Ptco31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco4 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco4 {
    #[inline(always)]
    fn from(val: u8) -> Ptco4 {
        Ptco4::from_bits(val)
    }
}
impl From<Ptco4> for u8 {
    #[inline(always)]
    fn from(val: Ptco4) -> u8 {
        Ptco4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco5 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco5 {
    #[inline(always)]
    fn from(val: u8) -> Ptco5 {
        Ptco5::from_bits(val)
    }
}
impl From<Ptco5> for u8 {
    #[inline(always)]
    fn from(val: Ptco5) -> u8 {
        Ptco5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco6 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco6 {
    #[inline(always)]
    fn from(val: u8) -> Ptco6 {
        Ptco6::from_bits(val)
    }
}
impl From<Ptco6> for u8 {
    #[inline(always)]
    fn from(val: Ptco6) -> u8 {
        Ptco6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco7 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco7 {
    #[inline(always)]
    fn from(val: u8) -> Ptco7 {
        Ptco7::from_bits(val)
    }
}
impl From<Ptco7> for u8 {
    #[inline(always)]
    fn from(val: Ptco7) -> u8 {
        Ptco7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco8 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco8 {
    #[inline(always)]
    fn from(val: u8) -> Ptco8 {
        Ptco8::from_bits(val)
    }
}
impl From<Ptco8> for u8 {
    #[inline(always)]
    fn from(val: Ptco8) -> u8 {
        Ptco8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco9 {
    #[doc = "No change."]
    PTCO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    PTCO1 = 0x01,
}
impl Ptco9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco9 {
    #[inline(always)]
    fn from(val: u8) -> Ptco9 {
        Ptco9::from_bits(val)
    }
}
impl From<Ptco9> for u8 {
    #[inline(always)]
    fn from(val: Ptco9) -> u8 {
        Ptco9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso0 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso0 {
    #[inline(always)]
    fn from(val: u8) -> Ptso0 {
        Ptso0::from_bits(val)
    }
}
impl From<Ptso0> for u8 {
    #[inline(always)]
    fn from(val: Ptso0) -> u8 {
        Ptso0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso1 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso1 {
    #[inline(always)]
    fn from(val: u8) -> Ptso1 {
        Ptso1::from_bits(val)
    }
}
impl From<Ptso1> for u8 {
    #[inline(always)]
    fn from(val: Ptso1) -> u8 {
        Ptso1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso10 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso10 {
    #[inline(always)]
    fn from(val: u8) -> Ptso10 {
        Ptso10::from_bits(val)
    }
}
impl From<Ptso10> for u8 {
    #[inline(always)]
    fn from(val: Ptso10) -> u8 {
        Ptso10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso11 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso11 {
    #[inline(always)]
    fn from(val: u8) -> Ptso11 {
        Ptso11::from_bits(val)
    }
}
impl From<Ptso11> for u8 {
    #[inline(always)]
    fn from(val: Ptso11) -> u8 {
        Ptso11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso12 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso12 {
    #[inline(always)]
    fn from(val: u8) -> Ptso12 {
        Ptso12::from_bits(val)
    }
}
impl From<Ptso12> for u8 {
    #[inline(always)]
    fn from(val: Ptso12) -> u8 {
        Ptso12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso13 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso13 {
    #[inline(always)]
    fn from(val: u8) -> Ptso13 {
        Ptso13::from_bits(val)
    }
}
impl From<Ptso13> for u8 {
    #[inline(always)]
    fn from(val: Ptso13) -> u8 {
        Ptso13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso14 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso14 {
    #[inline(always)]
    fn from(val: u8) -> Ptso14 {
        Ptso14::from_bits(val)
    }
}
impl From<Ptso14> for u8 {
    #[inline(always)]
    fn from(val: Ptso14) -> u8 {
        Ptso14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso15 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso15 {
    #[inline(always)]
    fn from(val: u8) -> Ptso15 {
        Ptso15::from_bits(val)
    }
}
impl From<Ptso15> for u8 {
    #[inline(always)]
    fn from(val: Ptso15) -> u8 {
        Ptso15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso16 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso16 {
    #[inline(always)]
    fn from(val: u8) -> Ptso16 {
        Ptso16::from_bits(val)
    }
}
impl From<Ptso16> for u8 {
    #[inline(always)]
    fn from(val: Ptso16) -> u8 {
        Ptso16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso17 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso17 {
    #[inline(always)]
    fn from(val: u8) -> Ptso17 {
        Ptso17::from_bits(val)
    }
}
impl From<Ptso17> for u8 {
    #[inline(always)]
    fn from(val: Ptso17) -> u8 {
        Ptso17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso18 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso18 {
    #[inline(always)]
    fn from(val: u8) -> Ptso18 {
        Ptso18::from_bits(val)
    }
}
impl From<Ptso18> for u8 {
    #[inline(always)]
    fn from(val: Ptso18) -> u8 {
        Ptso18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso19 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso19 {
    #[inline(always)]
    fn from(val: u8) -> Ptso19 {
        Ptso19::from_bits(val)
    }
}
impl From<Ptso19> for u8 {
    #[inline(always)]
    fn from(val: Ptso19) -> u8 {
        Ptso19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso2 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso2 {
    #[inline(always)]
    fn from(val: u8) -> Ptso2 {
        Ptso2::from_bits(val)
    }
}
impl From<Ptso2> for u8 {
    #[inline(always)]
    fn from(val: Ptso2) -> u8 {
        Ptso2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso20 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso20 {
    #[inline(always)]
    fn from(val: u8) -> Ptso20 {
        Ptso20::from_bits(val)
    }
}
impl From<Ptso20> for u8 {
    #[inline(always)]
    fn from(val: Ptso20) -> u8 {
        Ptso20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso21 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso21 {
    #[inline(always)]
    fn from(val: u8) -> Ptso21 {
        Ptso21::from_bits(val)
    }
}
impl From<Ptso21> for u8 {
    #[inline(always)]
    fn from(val: Ptso21) -> u8 {
        Ptso21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso22 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso22 {
    #[inline(always)]
    fn from(val: u8) -> Ptso22 {
        Ptso22::from_bits(val)
    }
}
impl From<Ptso22> for u8 {
    #[inline(always)]
    fn from(val: Ptso22) -> u8 {
        Ptso22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso23 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso23 {
    #[inline(always)]
    fn from(val: u8) -> Ptso23 {
        Ptso23::from_bits(val)
    }
}
impl From<Ptso23> for u8 {
    #[inline(always)]
    fn from(val: Ptso23) -> u8 {
        Ptso23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso24 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso24 {
    #[inline(always)]
    fn from(val: u8) -> Ptso24 {
        Ptso24::from_bits(val)
    }
}
impl From<Ptso24> for u8 {
    #[inline(always)]
    fn from(val: Ptso24) -> u8 {
        Ptso24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso25 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso25 {
    #[inline(always)]
    fn from(val: u8) -> Ptso25 {
        Ptso25::from_bits(val)
    }
}
impl From<Ptso25> for u8 {
    #[inline(always)]
    fn from(val: Ptso25) -> u8 {
        Ptso25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso26 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso26 {
    #[inline(always)]
    fn from(val: u8) -> Ptso26 {
        Ptso26::from_bits(val)
    }
}
impl From<Ptso26> for u8 {
    #[inline(always)]
    fn from(val: Ptso26) -> u8 {
        Ptso26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso27 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso27 {
    #[inline(always)]
    fn from(val: u8) -> Ptso27 {
        Ptso27::from_bits(val)
    }
}
impl From<Ptso27> for u8 {
    #[inline(always)]
    fn from(val: Ptso27) -> u8 {
        Ptso27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso28 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso28 {
    #[inline(always)]
    fn from(val: u8) -> Ptso28 {
        Ptso28::from_bits(val)
    }
}
impl From<Ptso28> for u8 {
    #[inline(always)]
    fn from(val: Ptso28) -> u8 {
        Ptso28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso29 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso29 {
    #[inline(always)]
    fn from(val: u8) -> Ptso29 {
        Ptso29::from_bits(val)
    }
}
impl From<Ptso29> for u8 {
    #[inline(always)]
    fn from(val: Ptso29) -> u8 {
        Ptso29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso3 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso3 {
    #[inline(always)]
    fn from(val: u8) -> Ptso3 {
        Ptso3::from_bits(val)
    }
}
impl From<Ptso3> for u8 {
    #[inline(always)]
    fn from(val: Ptso3) -> u8 {
        Ptso3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso30 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso30 {
    #[inline(always)]
    fn from(val: u8) -> Ptso30 {
        Ptso30::from_bits(val)
    }
}
impl From<Ptso30> for u8 {
    #[inline(always)]
    fn from(val: Ptso30) -> u8 {
        Ptso30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso31 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso31 {
    #[inline(always)]
    fn from(val: u8) -> Ptso31 {
        Ptso31::from_bits(val)
    }
}
impl From<Ptso31> for u8 {
    #[inline(always)]
    fn from(val: Ptso31) -> u8 {
        Ptso31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso4 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso4 {
    #[inline(always)]
    fn from(val: u8) -> Ptso4 {
        Ptso4::from_bits(val)
    }
}
impl From<Ptso4> for u8 {
    #[inline(always)]
    fn from(val: Ptso4) -> u8 {
        Ptso4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso5 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso5 {
    #[inline(always)]
    fn from(val: u8) -> Ptso5 {
        Ptso5::from_bits(val)
    }
}
impl From<Ptso5> for u8 {
    #[inline(always)]
    fn from(val: Ptso5) -> u8 {
        Ptso5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso6 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso6 {
    #[inline(always)]
    fn from(val: u8) -> Ptso6 {
        Ptso6::from_bits(val)
    }
}
impl From<Ptso6> for u8 {
    #[inline(always)]
    fn from(val: Ptso6) -> u8 {
        Ptso6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso7 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso7 {
    #[inline(always)]
    fn from(val: u8) -> Ptso7 {
        Ptso7::from_bits(val)
    }
}
impl From<Ptso7> for u8 {
    #[inline(always)]
    fn from(val: Ptso7) -> u8 {
        Ptso7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso8 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso8 {
    #[inline(always)]
    fn from(val: u8) -> Ptso8 {
        Ptso8::from_bits(val)
    }
}
impl From<Ptso8> for u8 {
    #[inline(always)]
    fn from(val: Ptso8) -> u8 {
        Ptso8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso9 {
    #[doc = "No change."]
    PTSO0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    PTSO1 = 0x01,
}
impl Ptso9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso9 {
    #[inline(always)]
    fn from(val: u8) -> Ptso9 {
        Ptso9::from_bits(val)
    }
}
impl From<Ptso9> for u8 {
    #[inline(always)]
    fn from(val: Ptso9) -> u8 {
        Ptso9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto0 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto0 {
    #[inline(always)]
    fn from(val: u8) -> Ptto0 {
        Ptto0::from_bits(val)
    }
}
impl From<Ptto0> for u8 {
    #[inline(always)]
    fn from(val: Ptto0) -> u8 {
        Ptto0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto1 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto1 {
    #[inline(always)]
    fn from(val: u8) -> Ptto1 {
        Ptto1::from_bits(val)
    }
}
impl From<Ptto1> for u8 {
    #[inline(always)]
    fn from(val: Ptto1) -> u8 {
        Ptto1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto10 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto10 {
    #[inline(always)]
    fn from(val: u8) -> Ptto10 {
        Ptto10::from_bits(val)
    }
}
impl From<Ptto10> for u8 {
    #[inline(always)]
    fn from(val: Ptto10) -> u8 {
        Ptto10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto11 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto11 {
    #[inline(always)]
    fn from(val: u8) -> Ptto11 {
        Ptto11::from_bits(val)
    }
}
impl From<Ptto11> for u8 {
    #[inline(always)]
    fn from(val: Ptto11) -> u8 {
        Ptto11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto12 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto12 {
    #[inline(always)]
    fn from(val: u8) -> Ptto12 {
        Ptto12::from_bits(val)
    }
}
impl From<Ptto12> for u8 {
    #[inline(always)]
    fn from(val: Ptto12) -> u8 {
        Ptto12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto13 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto13 {
    #[inline(always)]
    fn from(val: u8) -> Ptto13 {
        Ptto13::from_bits(val)
    }
}
impl From<Ptto13> for u8 {
    #[inline(always)]
    fn from(val: Ptto13) -> u8 {
        Ptto13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto14 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto14 {
    #[inline(always)]
    fn from(val: u8) -> Ptto14 {
        Ptto14::from_bits(val)
    }
}
impl From<Ptto14> for u8 {
    #[inline(always)]
    fn from(val: Ptto14) -> u8 {
        Ptto14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto15 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto15 {
    #[inline(always)]
    fn from(val: u8) -> Ptto15 {
        Ptto15::from_bits(val)
    }
}
impl From<Ptto15> for u8 {
    #[inline(always)]
    fn from(val: Ptto15) -> u8 {
        Ptto15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto16 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto16 {
    #[inline(always)]
    fn from(val: u8) -> Ptto16 {
        Ptto16::from_bits(val)
    }
}
impl From<Ptto16> for u8 {
    #[inline(always)]
    fn from(val: Ptto16) -> u8 {
        Ptto16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto17 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto17 {
    #[inline(always)]
    fn from(val: u8) -> Ptto17 {
        Ptto17::from_bits(val)
    }
}
impl From<Ptto17> for u8 {
    #[inline(always)]
    fn from(val: Ptto17) -> u8 {
        Ptto17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto18 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto18 {
    #[inline(always)]
    fn from(val: u8) -> Ptto18 {
        Ptto18::from_bits(val)
    }
}
impl From<Ptto18> for u8 {
    #[inline(always)]
    fn from(val: Ptto18) -> u8 {
        Ptto18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto19 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto19 {
    #[inline(always)]
    fn from(val: u8) -> Ptto19 {
        Ptto19::from_bits(val)
    }
}
impl From<Ptto19> for u8 {
    #[inline(always)]
    fn from(val: Ptto19) -> u8 {
        Ptto19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto2 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto2 {
    #[inline(always)]
    fn from(val: u8) -> Ptto2 {
        Ptto2::from_bits(val)
    }
}
impl From<Ptto2> for u8 {
    #[inline(always)]
    fn from(val: Ptto2) -> u8 {
        Ptto2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto20 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto20 {
    #[inline(always)]
    fn from(val: u8) -> Ptto20 {
        Ptto20::from_bits(val)
    }
}
impl From<Ptto20> for u8 {
    #[inline(always)]
    fn from(val: Ptto20) -> u8 {
        Ptto20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto21 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto21 {
    #[inline(always)]
    fn from(val: u8) -> Ptto21 {
        Ptto21::from_bits(val)
    }
}
impl From<Ptto21> for u8 {
    #[inline(always)]
    fn from(val: Ptto21) -> u8 {
        Ptto21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto22 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto22 {
    #[inline(always)]
    fn from(val: u8) -> Ptto22 {
        Ptto22::from_bits(val)
    }
}
impl From<Ptto22> for u8 {
    #[inline(always)]
    fn from(val: Ptto22) -> u8 {
        Ptto22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto23 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto23 {
    #[inline(always)]
    fn from(val: u8) -> Ptto23 {
        Ptto23::from_bits(val)
    }
}
impl From<Ptto23> for u8 {
    #[inline(always)]
    fn from(val: Ptto23) -> u8 {
        Ptto23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto24 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto24 {
    #[inline(always)]
    fn from(val: u8) -> Ptto24 {
        Ptto24::from_bits(val)
    }
}
impl From<Ptto24> for u8 {
    #[inline(always)]
    fn from(val: Ptto24) -> u8 {
        Ptto24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto25 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto25 {
    #[inline(always)]
    fn from(val: u8) -> Ptto25 {
        Ptto25::from_bits(val)
    }
}
impl From<Ptto25> for u8 {
    #[inline(always)]
    fn from(val: Ptto25) -> u8 {
        Ptto25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto26 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto26 {
    #[inline(always)]
    fn from(val: u8) -> Ptto26 {
        Ptto26::from_bits(val)
    }
}
impl From<Ptto26> for u8 {
    #[inline(always)]
    fn from(val: Ptto26) -> u8 {
        Ptto26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto27 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto27 {
    #[inline(always)]
    fn from(val: u8) -> Ptto27 {
        Ptto27::from_bits(val)
    }
}
impl From<Ptto27> for u8 {
    #[inline(always)]
    fn from(val: Ptto27) -> u8 {
        Ptto27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto28 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto28 {
    #[inline(always)]
    fn from(val: u8) -> Ptto28 {
        Ptto28::from_bits(val)
    }
}
impl From<Ptto28> for u8 {
    #[inline(always)]
    fn from(val: Ptto28) -> u8 {
        Ptto28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto29 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto29 {
    #[inline(always)]
    fn from(val: u8) -> Ptto29 {
        Ptto29::from_bits(val)
    }
}
impl From<Ptto29> for u8 {
    #[inline(always)]
    fn from(val: Ptto29) -> u8 {
        Ptto29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto3 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto3 {
    #[inline(always)]
    fn from(val: u8) -> Ptto3 {
        Ptto3::from_bits(val)
    }
}
impl From<Ptto3> for u8 {
    #[inline(always)]
    fn from(val: Ptto3) -> u8 {
        Ptto3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto30 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto30 {
    #[inline(always)]
    fn from(val: u8) -> Ptto30 {
        Ptto30::from_bits(val)
    }
}
impl From<Ptto30> for u8 {
    #[inline(always)]
    fn from(val: Ptto30) -> u8 {
        Ptto30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto31 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto31 {
    #[inline(always)]
    fn from(val: u8) -> Ptto31 {
        Ptto31::from_bits(val)
    }
}
impl From<Ptto31> for u8 {
    #[inline(always)]
    fn from(val: Ptto31) -> u8 {
        Ptto31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto4 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto4 {
    #[inline(always)]
    fn from(val: u8) -> Ptto4 {
        Ptto4::from_bits(val)
    }
}
impl From<Ptto4> for u8 {
    #[inline(always)]
    fn from(val: Ptto4) -> u8 {
        Ptto4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto5 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto5 {
    #[inline(always)]
    fn from(val: u8) -> Ptto5 {
        Ptto5::from_bits(val)
    }
}
impl From<Ptto5> for u8 {
    #[inline(always)]
    fn from(val: Ptto5) -> u8 {
        Ptto5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto6 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto6 {
    #[inline(always)]
    fn from(val: u8) -> Ptto6 {
        Ptto6::from_bits(val)
    }
}
impl From<Ptto6> for u8 {
    #[inline(always)]
    fn from(val: Ptto6) -> u8 {
        Ptto6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto7 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto7 {
    #[inline(always)]
    fn from(val: u8) -> Ptto7 {
        Ptto7::from_bits(val)
    }
}
impl From<Ptto7> for u8 {
    #[inline(always)]
    fn from(val: Ptto7) -> u8 {
        Ptto7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto8 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto8 {
    #[inline(always)]
    fn from(val: u8) -> Ptto8 {
        Ptto8::from_bits(val)
    }
}
impl From<Ptto8> for u8 {
    #[inline(always)]
    fn from(val: Ptto8) -> u8 {
        Ptto8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptto9 {
    #[doc = "No change."]
    PTTO0 = 0x0,
    #[doc = "Set to the inverse of its current logic state."]
    PTTO1 = 0x01,
}
impl Ptto9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptto9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptto9 {
    #[inline(always)]
    fn from(val: u8) -> Ptto9 {
        Ptto9::from_bits(val)
    }
}
impl From<Ptto9> for u8 {
    #[inline(always)]
    fn from(val: Ptto9) -> u8 {
        Ptto9::to_bits(val)
    }
}
