#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6a8c2aa 2026-01-27))"]
#[doc = "TDET."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdet0 {
    ptr: *mut u8,
}
unsafe impl Send for Tdet0 {}
unsafe impl Sync for Tdet0 {}
impl Tdet0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn lr(self) -> crate::common::Reg<Lr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Tamper Seconds."]
    #[inline(always)]
    pub const fn tsr(self) -> crate::common::Reg<Tsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Tamper Enable."]
    #[inline(always)]
    pub const fn ter(self) -> crate::common::Reg<Ter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Pin Polarity."]
    #[inline(always)]
    pub const fn ppr(self) -> crate::common::Reg<Ppr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Pin Glitch Filter."]
    #[inline(always)]
    pub const fn pgfr(self, n: usize) -> crate::common::Reg<Pgfr, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
}
#[doc = "Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn swr(&self) -> Swr {
        let val = (self.0 >> 0usize) & 0x01;
        Swr::from_bits(val as u8)
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_swr(&mut self, val: Swr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Digital Tamper Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn den(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Digital Tamper Enable."]
    #[inline(always)]
    pub const fn set_den(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Tamper Force System Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn tfsr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Force System Reset."]
    #[inline(always)]
    pub const fn set_tfsr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Update Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn um(&self) -> Um {
        let val = (self.0 >> 3usize) & 0x01;
        Um::from_bits(val as u8)
    }
    #[doc = "Update Mode."]
    #[inline(always)]
    pub const fn set_um(&mut self, val: Um) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Disable Prescaler On Tamper."]
    #[must_use]
    #[inline(always)]
    pub const fn distam(&self) -> Distam {
        let val = (self.0 >> 8usize) & 0x01;
        Distam::from_bits(val as u8)
    }
    #[doc = "Disable Prescaler On Tamper."]
    #[inline(always)]
    pub const fn set_distam(&mut self, val: Distam) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Digital Tamper Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn dpr(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "Digital Tamper Prescaler."]
    #[inline(always)]
    pub const fn set_dpr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
impl core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr")
            .field("swr", &self.swr())
            .field("den", &self.den())
            .field("tfsr", &self.tfsr())
            .field("um", &self.um())
            .field("distam", &self.distam())
            .field("dpr", &self.dpr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr {{ swr: {:?}, den: {=bool:?}, tfsr: {=bool:?}, um: {:?}, distam: {:?}, dpr: {=u16:?} }}",
            self.swr(),
            self.den(),
            self.tfsr(),
            self.um(),
            self.distam(),
            self.dpr()
        )
    }
}
#[doc = "Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "Digital Tamper Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dtie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Digital Tamper Interrupt Enable."]
    #[inline(always)]
    pub const fn set_dtie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Tamper Input n Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tiie0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tiie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Tamper Input n Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tiie1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tiie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Tamper Input n Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tiie2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tiie2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Tamper Input n Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tiie3(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tiie3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Tamper Input n Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tiie4(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tiie4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Tamper Input n Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tiie5(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tiie5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Tamper Input n Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tiie6(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tiie6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Tamper Input n Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tiie7(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tiie7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Tamper Input n Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tiie8(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tiie8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Tamper Input n Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tiie9(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tiie9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Tamper Input n Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tiie10(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tiie10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Tamper Pin n Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tpie0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tpie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Tamper Pin n Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tpie1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tpie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Tamper Pin n Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tpie2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tpie2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Tamper Pin n Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tpie3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tpie3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Tamper Pin n Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tpie4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tpie4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Tamper Pin n Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tpie5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tpie5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Ier {
    #[inline(always)]
    fn default() -> Ier {
        Ier(0)
    }
}
impl core::fmt::Debug for Ier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ier")
            .field("dtie", &self.dtie())
            .field("tiie0", &self.tiie0())
            .field("tiie1", &self.tiie1())
            .field("tiie2", &self.tiie2())
            .field("tiie3", &self.tiie3())
            .field("tiie4", &self.tiie4())
            .field("tiie5", &self.tiie5())
            .field("tiie6", &self.tiie6())
            .field("tiie7", &self.tiie7())
            .field("tiie8", &self.tiie8())
            .field("tiie9", &self.tiie9())
            .field("tiie10", &self.tiie10())
            .field("tpie0", &self.tpie0())
            .field("tpie1", &self.tpie1())
            .field("tpie2", &self.tpie2())
            .field("tpie3", &self.tpie3())
            .field("tpie4", &self.tpie4())
            .field("tpie5", &self.tpie5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ier {{ dtie: {=bool:?}, tiie0: {=bool:?}, tiie1: {=bool:?}, tiie2: {=bool:?}, tiie3: {=bool:?}, tiie4: {=bool:?}, tiie5: {=bool:?}, tiie6: {=bool:?}, tiie7: {=bool:?}, tiie8: {=bool:?}, tiie9: {=bool:?}, tiie10: {=bool:?}, tpie0: {=bool:?}, tpie1: {=bool:?}, tpie2: {=bool:?}, tpie3: {=bool:?}, tpie4: {=bool:?}, tpie5: {=bool:?} }}",
            self.dtie(),
            self.tiie0(),
            self.tiie1(),
            self.tiie2(),
            self.tiie3(),
            self.tiie4(),
            self.tiie5(),
            self.tiie6(),
            self.tiie7(),
            self.tiie8(),
            self.tiie9(),
            self.tiie10(),
            self.tpie0(),
            self.tpie1(),
            self.tpie2(),
            self.tpie3(),
            self.tpie4(),
            self.tpie5()
        )
    }
}
#[doc = "Lock."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lr(pub u32);
impl Lr {
    #[doc = "Control Register Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn crl(&self) -> Crl {
        let val = (self.0 >> 4usize) & 0x01;
        Crl::from_bits(val as u8)
    }
    #[doc = "Control Register Lock."]
    #[inline(always)]
    pub const fn set_crl(&mut self, val: Crl) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Status Register Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn srl(&self) -> Srl {
        let val = (self.0 >> 5usize) & 0x01;
        Srl::from_bits(val as u8)
    }
    #[doc = "Status Register Lock."]
    #[inline(always)]
    pub const fn set_srl(&mut self, val: Srl) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Lock Register Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lrl(&self) -> Lrl {
        let val = (self.0 >> 6usize) & 0x01;
        Lrl::from_bits(val as u8)
    }
    #[doc = "Lock Register Lock."]
    #[inline(always)]
    pub const fn set_lrl(&mut self, val: Lrl) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt Enable Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn iel(&self) -> Iel {
        let val = (self.0 >> 7usize) & 0x01;
        Iel::from_bits(val as u8)
    }
    #[doc = "Interrupt Enable Lock."]
    #[inline(always)]
    pub const fn set_iel(&mut self, val: Iel) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Tamper Seconds Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn tsl(&self) -> Tsl {
        let val = (self.0 >> 8usize) & 0x01;
        Tsl::from_bits(val as u8)
    }
    #[doc = "Tamper Seconds Lock."]
    #[inline(always)]
    pub const fn set_tsl(&mut self, val: Tsl) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Tamper Enable Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn tel(&self) -> Tel {
        let val = (self.0 >> 9usize) & 0x01;
        Tel::from_bits(val as u8)
    }
    #[doc = "Tamper Enable Lock."]
    #[inline(always)]
    pub const fn set_tel(&mut self, val: Tel) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Pin Polarity Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn ppl(&self) -> Ppl {
        let val = (self.0 >> 11usize) & 0x01;
        Ppl::from_bits(val as u8)
    }
    #[doc = "Pin Polarity Lock."]
    #[inline(always)]
    pub const fn set_ppl(&mut self, val: Ppl) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Glitch Filter Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn gfl0(&self) -> Gfl0 {
        let val = (self.0 >> 16usize) & 0x01;
        Gfl0::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Lock."]
    #[inline(always)]
    pub const fn set_gfl0(&mut self, val: Gfl0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Glitch Filter Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn gfl1(&self) -> Gfl1 {
        let val = (self.0 >> 17usize) & 0x01;
        Gfl1::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Lock."]
    #[inline(always)]
    pub const fn set_gfl1(&mut self, val: Gfl1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Glitch Filter Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn gfl2(&self) -> Gfl2 {
        let val = (self.0 >> 18usize) & 0x01;
        Gfl2::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Lock."]
    #[inline(always)]
    pub const fn set_gfl2(&mut self, val: Gfl2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Glitch Filter Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn gfl3(&self) -> Gfl3 {
        let val = (self.0 >> 19usize) & 0x01;
        Gfl3::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Lock."]
    #[inline(always)]
    pub const fn set_gfl3(&mut self, val: Gfl3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Glitch Filter Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn gfl4(&self) -> Gfl4 {
        let val = (self.0 >> 20usize) & 0x01;
        Gfl4::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Lock."]
    #[inline(always)]
    pub const fn set_gfl4(&mut self, val: Gfl4) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Glitch Filter Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn gfl5(&self) -> Gfl5 {
        let val = (self.0 >> 21usize) & 0x01;
        Gfl5::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Lock."]
    #[inline(always)]
    pub const fn set_gfl5(&mut self, val: Gfl5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
}
impl Default for Lr {
    #[inline(always)]
    fn default() -> Lr {
        Lr(0)
    }
}
impl core::fmt::Debug for Lr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lr")
            .field("crl", &self.crl())
            .field("srl", &self.srl())
            .field("lrl", &self.lrl())
            .field("iel", &self.iel())
            .field("tsl", &self.tsl())
            .field("tel", &self.tel())
            .field("ppl", &self.ppl())
            .field("gfl0", &self.gfl0())
            .field("gfl1", &self.gfl1())
            .field("gfl2", &self.gfl2())
            .field("gfl3", &self.gfl3())
            .field("gfl4", &self.gfl4())
            .field("gfl5", &self.gfl5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lr {{ crl: {:?}, srl: {:?}, lrl: {:?}, iel: {:?}, tsl: {:?}, tel: {:?}, ppl: {:?}, gfl0: {:?}, gfl1: {:?}, gfl2: {:?}, gfl3: {:?}, gfl4: {:?}, gfl5: {:?} }}",
            self.crl(),
            self.srl(),
            self.lrl(),
            self.iel(),
            self.tsl(),
            self.tel(),
            self.ppl(),
            self.gfl0(),
            self.gfl1(),
            self.gfl2(),
            self.gfl3(),
            self.gfl4(),
            self.gfl5()
        )
    }
}
#[doc = "Pin Glitch Filter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pgfr(pub u32);
impl Pgfr {
    #[doc = "Glitch Filter Width."]
    #[must_use]
    #[inline(always)]
    pub const fn gfw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Glitch Filter Width."]
    #[inline(always)]
    pub const fn set_gfw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Glitch Filter Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn gfp(&self) -> Gfp {
        let val = (self.0 >> 6usize) & 0x01;
        Gfp::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Prescaler."]
    #[inline(always)]
    pub const fn set_gfp(&mut self, val: Gfp) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Glitch Filter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gfe(&self) -> Gfe {
        let val = (self.0 >> 7usize) & 0x01;
        Gfe::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Enable."]
    #[inline(always)]
    pub const fn set_gfe(&mut self, val: Gfe) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Tamper Pin Sample Width."]
    #[must_use]
    #[inline(always)]
    pub const fn tpsw(&self) -> Tpsw {
        let val = (self.0 >> 8usize) & 0x03;
        Tpsw::from_bits(val as u8)
    }
    #[doc = "Tamper Pin Sample Width."]
    #[inline(always)]
    pub const fn set_tpsw(&mut self, val: Tpsw) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Tamper Pin Sample Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn tpsf(&self) -> Tpsf {
        let val = (self.0 >> 10usize) & 0x03;
        Tpsf::from_bits(val as u8)
    }
    #[doc = "Tamper Pin Sample Frequency."]
    #[inline(always)]
    pub const fn set_tpsf(&mut self, val: Tpsf) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Tamper Pull Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tpe(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pull Enable."]
    #[inline(always)]
    pub const fn set_tpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Tamper Pull Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tps(&self) -> Tps {
        let val = (self.0 >> 25usize) & 0x01;
        Tps::from_bits(val as u8)
    }
    #[doc = "Tamper Pull Select."]
    #[inline(always)]
    pub const fn set_tps(&mut self, val: Tps) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Tamper Pull Value."]
    #[must_use]
    #[inline(always)]
    pub const fn tpv(&self) -> Tpv {
        let val = (self.0 >> 26usize) & 0x01;
        Tpv::from_bits(val as u8)
    }
    #[doc = "Tamper Pull Value."]
    #[inline(always)]
    pub const fn set_tpv(&mut self, val: Tpv) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Tamper Passive Filter."]
    #[must_use]
    #[inline(always)]
    pub const fn tpf(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Passive Filter."]
    #[inline(always)]
    pub const fn set_tpf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Input Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Input Buffer Enable."]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pgfr {
    #[inline(always)]
    fn default() -> Pgfr {
        Pgfr(0)
    }
}
impl core::fmt::Debug for Pgfr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pgfr")
            .field("gfw", &self.gfw())
            .field("gfp", &self.gfp())
            .field("gfe", &self.gfe())
            .field("tpsw", &self.tpsw())
            .field("tpsf", &self.tpsf())
            .field("tpe", &self.tpe())
            .field("tps", &self.tps())
            .field("tpv", &self.tpv())
            .field("tpf", &self.tpf())
            .field("ibe", &self.ibe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pgfr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pgfr {{ gfw: {=u8:?}, gfp: {:?}, gfe: {:?}, tpsw: {:?}, tpsf: {:?}, tpe: {=bool:?}, tps: {:?}, tpv: {:?}, tpf: {=bool:?}, ibe: {=bool:?} }}",
            self.gfw(),
            self.gfp(),
            self.gfe(),
            self.tpsw(),
            self.tpsf(),
            self.tpe(),
            self.tps(),
            self.tpv(),
            self.tpf(),
            self.ibe()
        )
    }
}
#[doc = "Pin Polarity."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppr(pub u32);
impl Ppr {
    #[doc = "Tamper Pin n Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tpp0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Polarity."]
    #[inline(always)]
    pub const fn set_tpp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Tamper Pin n Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tpp1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Polarity."]
    #[inline(always)]
    pub const fn set_tpp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Tamper Pin n Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tpp2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Polarity."]
    #[inline(always)]
    pub const fn set_tpp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Tamper Pin n Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tpp3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Polarity."]
    #[inline(always)]
    pub const fn set_tpp3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Tamper Pin n Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tpp4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Polarity."]
    #[inline(always)]
    pub const fn set_tpp4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Tamper Pin n Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn tpp5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Polarity."]
    #[inline(always)]
    pub const fn set_tpp5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Tamper Pin n Input Data."]
    #[must_use]
    #[inline(always)]
    pub const fn tpid0(&self) -> Tpid0 {
        let val = (self.0 >> 16usize) & 0x01;
        Tpid0::from_bits(val as u8)
    }
    #[doc = "Tamper Pin n Input Data."]
    #[inline(always)]
    pub const fn set_tpid0(&mut self, val: Tpid0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Tamper Pin n Input Data."]
    #[must_use]
    #[inline(always)]
    pub const fn tpid1(&self) -> Tpid1 {
        let val = (self.0 >> 17usize) & 0x01;
        Tpid1::from_bits(val as u8)
    }
    #[doc = "Tamper Pin n Input Data."]
    #[inline(always)]
    pub const fn set_tpid1(&mut self, val: Tpid1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Tamper Pin n Input Data."]
    #[must_use]
    #[inline(always)]
    pub const fn tpid2(&self) -> Tpid2 {
        let val = (self.0 >> 18usize) & 0x01;
        Tpid2::from_bits(val as u8)
    }
    #[doc = "Tamper Pin n Input Data."]
    #[inline(always)]
    pub const fn set_tpid2(&mut self, val: Tpid2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Tamper Pin n Input Data."]
    #[must_use]
    #[inline(always)]
    pub const fn tpid3(&self) -> Tpid3 {
        let val = (self.0 >> 19usize) & 0x01;
        Tpid3::from_bits(val as u8)
    }
    #[doc = "Tamper Pin n Input Data."]
    #[inline(always)]
    pub const fn set_tpid3(&mut self, val: Tpid3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Tamper Pin n Input Data."]
    #[must_use]
    #[inline(always)]
    pub const fn tpid4(&self) -> Tpid4 {
        let val = (self.0 >> 20usize) & 0x01;
        Tpid4::from_bits(val as u8)
    }
    #[doc = "Tamper Pin n Input Data."]
    #[inline(always)]
    pub const fn set_tpid4(&mut self, val: Tpid4) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Tamper Pin n Input Data."]
    #[must_use]
    #[inline(always)]
    pub const fn tpid5(&self) -> Tpid5 {
        let val = (self.0 >> 21usize) & 0x01;
        Tpid5::from_bits(val as u8)
    }
    #[doc = "Tamper Pin n Input Data."]
    #[inline(always)]
    pub const fn set_tpid5(&mut self, val: Tpid5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
}
impl Default for Ppr {
    #[inline(always)]
    fn default() -> Ppr {
        Ppr(0)
    }
}
impl core::fmt::Debug for Ppr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ppr")
            .field("tpp0", &self.tpp0())
            .field("tpp1", &self.tpp1())
            .field("tpp2", &self.tpp2())
            .field("tpp3", &self.tpp3())
            .field("tpp4", &self.tpp4())
            .field("tpp5", &self.tpp5())
            .field("tpid0", &self.tpid0())
            .field("tpid1", &self.tpid1())
            .field("tpid2", &self.tpid2())
            .field("tpid3", &self.tpid3())
            .field("tpid4", &self.tpid4())
            .field("tpid5", &self.tpid5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ppr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ppr {{ tpp0: {=bool:?}, tpp1: {=bool:?}, tpp2: {=bool:?}, tpp3: {=bool:?}, tpp4: {=bool:?}, tpp5: {=bool:?}, tpid0: {:?}, tpid1: {:?}, tpid2: {:?}, tpid3: {:?}, tpid4: {:?}, tpid5: {:?} }}",
            self.tpp0(),
            self.tpp1(),
            self.tpp2(),
            self.tpp3(),
            self.tpp4(),
            self.tpp5(),
            self.tpid0(),
            self.tpid1(),
            self.tpid2(),
            self.tpid3(),
            self.tpid4(),
            self.tpid5()
        )
    }
}
#[doc = "Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Digital Tamper Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dtf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Digital Tamper Flag."]
    #[inline(always)]
    pub const fn set_dtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Tamper Acknowledge Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn taf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Acknowledge Flag."]
    #[inline(always)]
    pub const fn set_taf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Tamper Input n Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tif0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Flag."]
    #[inline(always)]
    pub const fn set_tif0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Tamper Input n Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tif1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Flag."]
    #[inline(always)]
    pub const fn set_tif1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Tamper Input n Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tif2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Flag."]
    #[inline(always)]
    pub const fn set_tif2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Tamper Input n Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tif3(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Flag."]
    #[inline(always)]
    pub const fn set_tif3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Tamper Input n Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tif4(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Flag."]
    #[inline(always)]
    pub const fn set_tif4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Tamper Input n Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tif5(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Flag."]
    #[inline(always)]
    pub const fn set_tif5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Tamper Input n Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tif6(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Flag."]
    #[inline(always)]
    pub const fn set_tif6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Tamper Input n Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tif7(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Flag."]
    #[inline(always)]
    pub const fn set_tif7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Tamper Input n Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tif8(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Flag."]
    #[inline(always)]
    pub const fn set_tif8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Tamper Input n Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tif9(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Flag."]
    #[inline(always)]
    pub const fn set_tif9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Tamper Input n Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tif10(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Flag."]
    #[inline(always)]
    pub const fn set_tif10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Tamper Pin n Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tpf0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Flag."]
    #[inline(always)]
    pub const fn set_tpf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Tamper Pin n Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tpf1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Flag."]
    #[inline(always)]
    pub const fn set_tpf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Tamper Pin n Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tpf2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Flag."]
    #[inline(always)]
    pub const fn set_tpf2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Tamper Pin n Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tpf3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Flag."]
    #[inline(always)]
    pub const fn set_tpf3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Tamper Pin n Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tpf4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Flag."]
    #[inline(always)]
    pub const fn set_tpf4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Tamper Pin n Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tpf5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Flag."]
    #[inline(always)]
    pub const fn set_tpf5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
impl core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr")
            .field("dtf", &self.dtf())
            .field("taf", &self.taf())
            .field("tif0", &self.tif0())
            .field("tif1", &self.tif1())
            .field("tif2", &self.tif2())
            .field("tif3", &self.tif3())
            .field("tif4", &self.tif4())
            .field("tif5", &self.tif5())
            .field("tif6", &self.tif6())
            .field("tif7", &self.tif7())
            .field("tif8", &self.tif8())
            .field("tif9", &self.tif9())
            .field("tif10", &self.tif10())
            .field("tpf0", &self.tpf0())
            .field("tpf1", &self.tpf1())
            .field("tpf2", &self.tpf2())
            .field("tpf3", &self.tpf3())
            .field("tpf4", &self.tpf4())
            .field("tpf5", &self.tpf5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sr {{ dtf: {=bool:?}, taf: {=bool:?}, tif0: {=bool:?}, tif1: {=bool:?}, tif2: {=bool:?}, tif3: {=bool:?}, tif4: {=bool:?}, tif5: {=bool:?}, tif6: {=bool:?}, tif7: {=bool:?}, tif8: {=bool:?}, tif9: {=bool:?}, tif10: {=bool:?}, tpf0: {=bool:?}, tpf1: {=bool:?}, tpf2: {=bool:?}, tpf3: {=bool:?}, tpf4: {=bool:?}, tpf5: {=bool:?} }}",
            self.dtf(),
            self.taf(),
            self.tif0(),
            self.tif1(),
            self.tif2(),
            self.tif3(),
            self.tif4(),
            self.tif5(),
            self.tif6(),
            self.tif7(),
            self.tif8(),
            self.tif9(),
            self.tif10(),
            self.tpf0(),
            self.tpf1(),
            self.tpf2(),
            self.tpf3(),
            self.tpf4(),
            self.tpf5()
        )
    }
}
#[doc = "Tamper Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ter(pub u32);
impl Ter {
    #[doc = "Tamper Input Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tie0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input Enable."]
    #[inline(always)]
    pub const fn set_tie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Tamper Input Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tie1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input Enable."]
    #[inline(always)]
    pub const fn set_tie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Tamper Input Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tie2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input Enable."]
    #[inline(always)]
    pub const fn set_tie2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Tamper Input Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tie3(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input Enable."]
    #[inline(always)]
    pub const fn set_tie3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Tamper Input Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tie4(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input Enable."]
    #[inline(always)]
    pub const fn set_tie4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Tamper Input Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tie5(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input Enable."]
    #[inline(always)]
    pub const fn set_tie5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Tamper Input Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tie6(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input Enable."]
    #[inline(always)]
    pub const fn set_tie6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Tamper Input Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tie7(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input Enable."]
    #[inline(always)]
    pub const fn set_tie7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Tamper Input Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tie8(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input Enable."]
    #[inline(always)]
    pub const fn set_tie8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Tamper Input Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tie9(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input Enable."]
    #[inline(always)]
    pub const fn set_tie9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Tamper Input Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tie10(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input Enable."]
    #[inline(always)]
    pub const fn set_tie10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Tamper Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tpe0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin Enable."]
    #[inline(always)]
    pub const fn set_tpe0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Tamper Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tpe1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin Enable."]
    #[inline(always)]
    pub const fn set_tpe1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Tamper Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tpe2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin Enable."]
    #[inline(always)]
    pub const fn set_tpe2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Tamper Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tpe3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin Enable."]
    #[inline(always)]
    pub const fn set_tpe3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Tamper Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tpe4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin Enable."]
    #[inline(always)]
    pub const fn set_tpe4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Tamper Pin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tpe5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin Enable."]
    #[inline(always)]
    pub const fn set_tpe5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Ter {
    #[inline(always)]
    fn default() -> Ter {
        Ter(0)
    }
}
impl core::fmt::Debug for Ter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ter")
            .field("tie0", &self.tie0())
            .field("tie1", &self.tie1())
            .field("tie2", &self.tie2())
            .field("tie3", &self.tie3())
            .field("tie4", &self.tie4())
            .field("tie5", &self.tie5())
            .field("tie6", &self.tie6())
            .field("tie7", &self.tie7())
            .field("tie8", &self.tie8())
            .field("tie9", &self.tie9())
            .field("tie10", &self.tie10())
            .field("tpe0", &self.tpe0())
            .field("tpe1", &self.tpe1())
            .field("tpe2", &self.tpe2())
            .field("tpe3", &self.tpe3())
            .field("tpe4", &self.tpe4())
            .field("tpe5", &self.tpe5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ter {{ tie0: {=bool:?}, tie1: {=bool:?}, tie2: {=bool:?}, tie3: {=bool:?}, tie4: {=bool:?}, tie5: {=bool:?}, tie6: {=bool:?}, tie7: {=bool:?}, tie8: {=bool:?}, tie9: {=bool:?}, tie10: {=bool:?}, tpe0: {=bool:?}, tpe1: {=bool:?}, tpe2: {=bool:?}, tpe3: {=bool:?}, tpe4: {=bool:?}, tpe5: {=bool:?} }}",
            self.tie0(),
            self.tie1(),
            self.tie2(),
            self.tie3(),
            self.tie4(),
            self.tie5(),
            self.tie6(),
            self.tie7(),
            self.tie8(),
            self.tie9(),
            self.tie10(),
            self.tpe0(),
            self.tpe1(),
            self.tpe2(),
            self.tpe3(),
            self.tpe4(),
            self.tpe5()
        )
    }
}
#[doc = "Tamper Seconds."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsr(pub u32);
impl Tsr {
    #[doc = "Tamper Time Seconds."]
    #[must_use]
    #[inline(always)]
    pub const fn tts(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Tamper Time Seconds."]
    #[inline(always)]
    pub const fn set_tts(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tsr {
    #[inline(always)]
    fn default() -> Tsr {
        Tsr(0)
    }
}
impl core::fmt::Debug for Tsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsr").field("tts", &self.tts()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tsr {{ tts: {=u32:?} }}", self.tts())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crl {
    #[doc = "Locked and writes are ignored."]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal."]
    NOT_LOCK = 0x01,
}
impl Crl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crl {
    #[inline(always)]
    fn from(val: u8) -> Crl {
        Crl::from_bits(val)
    }
}
impl From<Crl> for u8 {
    #[inline(always)]
    fn from(val: Crl) -> u8 {
        Crl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Distam {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "Automatically disables the prescaler after tamper detection."]
    AUTO_DIS = 0x01,
}
impl Distam {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Distam {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Distam {
    #[inline(always)]
    fn from(val: u8) -> Distam {
        Distam::from_bits(val)
    }
}
impl From<Distam> for u8 {
    #[inline(always)]
    fn from(val: Distam) -> u8 {
        Distam::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfe {
    #[doc = "Bypasses."]
    BYPASS = 0x0,
    #[doc = "Enables."]
    ENABLE = 0x01,
}
impl Gfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfe {
    #[inline(always)]
    fn from(val: u8) -> Gfe {
        Gfe::from_bits(val)
    }
}
impl From<Gfe> for u8 {
    #[inline(always)]
    fn from(val: Gfe) -> u8 {
        Gfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfl0 {
    #[doc = "Locked and writes are ignored."]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal."]
    NOT_LOCK = 0x01,
}
impl Gfl0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfl0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfl0 {
    #[inline(always)]
    fn from(val: u8) -> Gfl0 {
        Gfl0::from_bits(val)
    }
}
impl From<Gfl0> for u8 {
    #[inline(always)]
    fn from(val: Gfl0) -> u8 {
        Gfl0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfl1 {
    #[doc = "Locked and writes are ignored."]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal."]
    NOT_LOCK = 0x01,
}
impl Gfl1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfl1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfl1 {
    #[inline(always)]
    fn from(val: u8) -> Gfl1 {
        Gfl1::from_bits(val)
    }
}
impl From<Gfl1> for u8 {
    #[inline(always)]
    fn from(val: Gfl1) -> u8 {
        Gfl1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfl2 {
    #[doc = "Locked and writes are ignored."]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal."]
    NOT_LOCK = 0x01,
}
impl Gfl2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfl2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfl2 {
    #[inline(always)]
    fn from(val: u8) -> Gfl2 {
        Gfl2::from_bits(val)
    }
}
impl From<Gfl2> for u8 {
    #[inline(always)]
    fn from(val: Gfl2) -> u8 {
        Gfl2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfl3 {
    #[doc = "Locked and writes are ignored."]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal."]
    NOT_LOCK = 0x01,
}
impl Gfl3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfl3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfl3 {
    #[inline(always)]
    fn from(val: u8) -> Gfl3 {
        Gfl3::from_bits(val)
    }
}
impl From<Gfl3> for u8 {
    #[inline(always)]
    fn from(val: Gfl3) -> u8 {
        Gfl3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfl4 {
    #[doc = "Locked and writes are ignored."]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal."]
    NOT_LOCK = 0x01,
}
impl Gfl4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfl4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfl4 {
    #[inline(always)]
    fn from(val: u8) -> Gfl4 {
        Gfl4::from_bits(val)
    }
}
impl From<Gfl4> for u8 {
    #[inline(always)]
    fn from(val: Gfl4) -> u8 {
        Gfl4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfl5 {
    #[doc = "Locked and writes are ignored."]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal."]
    NOT_LOCK = 0x01,
}
impl Gfl5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfl5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfl5 {
    #[inline(always)]
    fn from(val: u8) -> Gfl5 {
        Gfl5::from_bits(val)
    }
}
impl From<Gfl5> for u8 {
    #[inline(always)]
    fn from(val: Gfl5) -> u8 {
        Gfl5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfp {
    #[doc = "512 Hz prescaler clock."]
    FREQ_512_HZ = 0x0,
    #[doc = "32.768 kHz clock."]
    FREQ_32_KHZ = 0x01,
}
impl Gfp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfp {
    #[inline(always)]
    fn from(val: u8) -> Gfp {
        Gfp::from_bits(val)
    }
}
impl From<Gfp> for u8 {
    #[inline(always)]
    fn from(val: Gfp) -> u8 {
        Gfp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iel {
    #[doc = "Locked and writes are ignored."]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal."]
    NOT_LOCK = 0x01,
}
impl Iel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iel {
    #[inline(always)]
    fn from(val: u8) -> Iel {
        Iel::from_bits(val)
    }
}
impl From<Iel> for u8 {
    #[inline(always)]
    fn from(val: Iel) -> u8 {
        Iel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lrl {
    #[doc = "Locked and writes are ignored."]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal."]
    NOT_LOCK = 0x01,
}
impl Lrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lrl {
    #[inline(always)]
    fn from(val: u8) -> Lrl {
        Lrl::from_bits(val)
    }
}
impl From<Lrl> for u8 {
    #[inline(always)]
    fn from(val: Lrl) -> u8 {
        Lrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ppl {
    #[doc = "Locked and writes are ignored."]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal."]
    NOT_LOCK = 0x01,
}
impl Ppl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ppl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ppl {
    #[inline(always)]
    fn from(val: u8) -> Ppl {
        Ppl::from_bits(val)
    }
}
impl From<Ppl> for u8 {
    #[inline(always)]
    fn from(val: Ppl) -> u8 {
        Ppl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srl {
    #[doc = "Locked and writes are ignored."]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal."]
    NOT_LOCK = 0x01,
}
impl Srl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srl {
    #[inline(always)]
    fn from(val: u8) -> Srl {
        Srl::from_bits(val)
    }
}
impl From<Srl> for u8 {
    #[inline(always)]
    fn from(val: Srl) -> u8 {
        Srl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swr {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "Perform a software reset."]
    SW_RESET = 0x01,
}
impl Swr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swr {
    #[inline(always)]
    fn from(val: u8) -> Swr {
        Swr::from_bits(val)
    }
}
impl From<Swr> for u8 {
    #[inline(always)]
    fn from(val: Swr) -> u8 {
        Swr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tel {
    #[doc = "Locked and writes are ignored."]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal."]
    NOT_LOCK = 0x01,
}
impl Tel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tel {
    #[inline(always)]
    fn from(val: u8) -> Tel {
        Tel::from_bits(val)
    }
}
impl From<Tel> for u8 {
    #[inline(always)]
    fn from(val: Tel) -> u8 {
        Tel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpid0 {
    #[doc = "Zero."]
    ZERO = 0x0,
    #[doc = "One."]
    ONE = 0x01,
}
impl Tpid0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpid0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpid0 {
    #[inline(always)]
    fn from(val: u8) -> Tpid0 {
        Tpid0::from_bits(val)
    }
}
impl From<Tpid0> for u8 {
    #[inline(always)]
    fn from(val: Tpid0) -> u8 {
        Tpid0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpid1 {
    #[doc = "Zero."]
    ZERO = 0x0,
    #[doc = "One."]
    ONE = 0x01,
}
impl Tpid1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpid1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpid1 {
    #[inline(always)]
    fn from(val: u8) -> Tpid1 {
        Tpid1::from_bits(val)
    }
}
impl From<Tpid1> for u8 {
    #[inline(always)]
    fn from(val: Tpid1) -> u8 {
        Tpid1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpid2 {
    #[doc = "Zero."]
    ZERO = 0x0,
    #[doc = "One."]
    ONE = 0x01,
}
impl Tpid2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpid2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpid2 {
    #[inline(always)]
    fn from(val: u8) -> Tpid2 {
        Tpid2::from_bits(val)
    }
}
impl From<Tpid2> for u8 {
    #[inline(always)]
    fn from(val: Tpid2) -> u8 {
        Tpid2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpid3 {
    #[doc = "Zero."]
    ZERO = 0x0,
    #[doc = "One."]
    ONE = 0x01,
}
impl Tpid3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpid3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpid3 {
    #[inline(always)]
    fn from(val: u8) -> Tpid3 {
        Tpid3::from_bits(val)
    }
}
impl From<Tpid3> for u8 {
    #[inline(always)]
    fn from(val: Tpid3) -> u8 {
        Tpid3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpid4 {
    #[doc = "Zero."]
    ZERO = 0x0,
    #[doc = "One."]
    ONE = 0x01,
}
impl Tpid4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpid4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpid4 {
    #[inline(always)]
    fn from(val: u8) -> Tpid4 {
        Tpid4::from_bits(val)
    }
}
impl From<Tpid4> for u8 {
    #[inline(always)]
    fn from(val: Tpid4) -> u8 {
        Tpid4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpid5 {
    #[doc = "Zero."]
    ZERO = 0x0,
    #[doc = "One."]
    ONE = 0x01,
}
impl Tpid5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpid5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpid5 {
    #[inline(always)]
    fn from(val: u8) -> Tpid5 {
        Tpid5::from_bits(val)
    }
}
impl From<Tpid5> for u8 {
    #[inline(always)]
    fn from(val: Tpid5) -> u8 {
        Tpid5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tps {
    #[doc = "Asserts."]
    ASSERT = 0x0,
    #[doc = "Negates."]
    NEGATE = 0x01,
}
impl Tps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tps {
    #[inline(always)]
    fn from(val: u8) -> Tps {
        Tps::from_bits(val)
    }
}
impl From<Tps> for u8 {
    #[inline(always)]
    fn from(val: Tps) -> u8 {
        Tps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpsf {
    #[doc = "Every 8 cycles."]
    CYCLES_8 = 0x0,
    #[doc = "Every 32 cycles."]
    CYCLES_32 = 0x01,
    #[doc = "Every 128 cycles."]
    CYCLES_128 = 0x02,
    #[doc = "Every 512 cycles."]
    CYCLES_512 = 0x03,
}
impl Tpsf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpsf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpsf {
    #[inline(always)]
    fn from(val: u8) -> Tpsf {
        Tpsf::from_bits(val)
    }
}
impl From<Tpsf> for u8 {
    #[inline(always)]
    fn from(val: Tpsf) -> u8 {
        Tpsf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpsw {
    #[doc = "Continuous monitoring, pin sampling disabled."]
    DISABLE = 0x0,
    #[doc = "2 cycles for pull enable and 1 cycle for input buffer enable."]
    CYCLES_2 = 0x01,
    #[doc = "4 cycles for pull enable and 2 cycles for input buffer enable."]
    CYCLES_4 = 0x02,
    #[doc = "8 cycles for pull enable and 4 cycles for input buffer enable."]
    CYCLES_8 = 0x03,
}
impl Tpsw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpsw {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpsw {
    #[inline(always)]
    fn from(val: u8) -> Tpsw {
        Tpsw::from_bits(val)
    }
}
impl From<Tpsw> for u8 {
    #[inline(always)]
    fn from(val: Tpsw) -> u8 {
        Tpsw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpv {
    #[doc = "Low."]
    LOW = 0x0,
    #[doc = "High."]
    HIGH = 0x01,
}
impl Tpv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpv {
    #[inline(always)]
    fn from(val: u8) -> Tpv {
        Tpv::from_bits(val)
    }
}
impl From<Tpv> for u8 {
    #[inline(always)]
    fn from(val: Tpv) -> u8 {
        Tpv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsl {
    #[doc = "Locked and writes are ignored."]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal."]
    NOT_LOCK = 0x01,
}
impl Tsl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsl {
    #[inline(always)]
    fn from(val: u8) -> Tsl {
        Tsl::from_bits(val)
    }
}
impl From<Tsl> for u8 {
    #[inline(always)]
    fn from(val: Tsl) -> u8 {
        Tsl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Um {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "Allows the clearing of interrupts."]
    CLEAR_INTS = 0x01,
}
impl Um {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Um {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Um {
    #[inline(always)]
    fn from(val: u8) -> Um {
        Um::from_bits(val)
    }
}
impl From<Um> for u8 {
    #[inline(always)]
    fn from(val: Um) -> u8 {
        Um::to_bits(val)
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
