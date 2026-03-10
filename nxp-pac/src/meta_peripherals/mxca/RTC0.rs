#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6a8c2aa 2026-01-27))"]
#[doc = "RTC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc0 {
    ptr: *mut u8,
}
unsafe impl Send for Rtc0 {}
unsafe impl Sync for Rtc0 {}
impl Rtc0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RTC Time Seconds."]
    #[inline(always)]
    pub const fn tsr(self) -> crate::common::Reg<Tsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RTC Time Prescaler."]
    #[inline(always)]
    pub const fn tpr(self) -> crate::common::Reg<Tpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RTC Time Alarm."]
    #[inline(always)]
    pub const fn tar(self) -> crate::common::Reg<Tar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "RTC Time Compensation."]
    #[inline(always)]
    pub const fn tcr(self) -> crate::common::Reg<Tcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "RTC Control."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "RTC Status."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "RTC Lock."]
    #[inline(always)]
    pub const fn lr(self) -> crate::common::Reg<Lr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "RTC Interrupt Enable."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
#[doc = "RTC Control."]
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
    #[doc = "LPO Select."]
    #[must_use]
    #[inline(always)]
    pub const fn lpos(&self) -> Lpos {
        let val = (self.0 >> 7usize) & 0x01;
        Lpos::from_bits(val as u8)
    }
    #[doc = "LPO Select."]
    #[inline(always)]
    pub const fn set_lpos(&mut self, val: Lpos) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
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
            .field("um", &self.um())
            .field("lpos", &self.lpos())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr {{ swr: {:?}, um: {:?}, lpos: {:?} }}",
            self.swr(),
            self.um(),
            self.lpos()
        )
    }
}
#[doc = "RTC Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "Time Invalid Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tiie(&self) -> Tiie {
        let val = (self.0 >> 0usize) & 0x01;
        Tiie::from_bits(val as u8)
    }
    #[doc = "Time Invalid Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tiie(&mut self, val: Tiie) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Time Overflow Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn toie(&self) -> Toie {
        let val = (self.0 >> 1usize) & 0x01;
        Toie::from_bits(val as u8)
    }
    #[doc = "Time Overflow Interrupt Enable."]
    #[inline(always)]
    pub const fn set_toie(&mut self, val: Toie) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Time Alarm Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn taie(&self) -> Taie {
        let val = (self.0 >> 2usize) & 0x01;
        Taie::from_bits(val as u8)
    }
    #[doc = "Time Alarm Interrupt Enable."]
    #[inline(always)]
    pub const fn set_taie(&mut self, val: Taie) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Time Seconds Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tsie(&self) -> Tsie {
        let val = (self.0 >> 4usize) & 0x01;
        Tsie::from_bits(val as u8)
    }
    #[doc = "Time Seconds Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tsie(&mut self, val: Tsie) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Timer Seconds Interrupt Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn tsic(&self) -> Tsic {
        let val = (self.0 >> 16usize) & 0x07;
        Tsic::from_bits(val as u8)
    }
    #[doc = "Timer Seconds Interrupt Configuration."]
    #[inline(always)]
    pub const fn set_tsic(&mut self, val: Tsic) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
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
            .field("tiie", &self.tiie())
            .field("toie", &self.toie())
            .field("taie", &self.taie())
            .field("tsie", &self.tsie())
            .field("tsic", &self.tsic())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ier {{ tiie: {:?}, toie: {:?}, taie: {:?}, tsie: {:?}, tsic: {:?} }}",
            self.tiie(),
            self.toie(),
            self.taie(),
            self.tsie(),
            self.tsic()
        )
    }
}
#[doc = "RTC Lock."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lr(pub u32);
impl Lr {
    #[doc = "Time Compensation Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn tcl(&self) -> Tcl {
        let val = (self.0 >> 3usize) & 0x01;
        Tcl::from_bits(val as u8)
    }
    #[doc = "Time Compensation Lock."]
    #[inline(always)]
    pub const fn set_tcl(&mut self, val: Tcl) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
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
            .field("tcl", &self.tcl())
            .field("crl", &self.crl())
            .field("srl", &self.srl())
            .field("lrl", &self.lrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lr {{ tcl: {:?}, crl: {:?}, srl: {:?}, lrl: {:?} }}",
            self.tcl(),
            self.crl(),
            self.srl(),
            self.lrl()
        )
    }
}
#[doc = "RTC Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Time Invalid Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tif(&self) -> Tif {
        let val = (self.0 >> 0usize) & 0x01;
        Tif::from_bits(val as u8)
    }
    #[doc = "Time Invalid Flag."]
    #[inline(always)]
    pub const fn set_tif(&mut self, val: Tif) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Time Overflow Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tof(&self) -> Tof {
        let val = (self.0 >> 1usize) & 0x01;
        Tof::from_bits(val as u8)
    }
    #[doc = "Time Overflow Flag."]
    #[inline(always)]
    pub const fn set_tof(&mut self, val: Tof) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Time Alarm Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn taf(&self) -> Taf {
        let val = (self.0 >> 2usize) & 0x01;
        Taf::from_bits(val as u8)
    }
    #[doc = "Time Alarm Flag."]
    #[inline(always)]
    pub const fn set_taf(&mut self, val: Taf) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Time Counter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tce(&self) -> Tce {
        let val = (self.0 >> 4usize) & 0x01;
        Tce::from_bits(val as u8)
    }
    #[doc = "Time Counter Enable."]
    #[inline(always)]
    pub const fn set_tce(&mut self, val: Tce) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
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
            .field("tif", &self.tif())
            .field("tof", &self.tof())
            .field("taf", &self.taf())
            .field("tce", &self.tce())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sr {{ tif: {:?}, tof: {:?}, taf: {:?}, tce: {:?} }}",
            self.tif(),
            self.tof(),
            self.taf(),
            self.tce()
        )
    }
}
#[doc = "RTC Time Alarm."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tar(pub u32);
impl Tar {
    #[doc = "Time Alarm Register."]
    #[must_use]
    #[inline(always)]
    pub const fn tar(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Time Alarm Register."]
    #[inline(always)]
    pub const fn set_tar(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tar {
    #[inline(always)]
    fn default() -> Tar {
        Tar(0)
    }
}
impl core::fmt::Debug for Tar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tar").field("tar", &self.tar()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tar {{ tar: {=u32:?} }}", self.tar())
    }
}
#[doc = "RTC Time Compensation."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr(pub u32);
impl Tcr {
    #[doc = "Time Compensation Register."]
    #[must_use]
    #[inline(always)]
    pub const fn tcr(&self) -> Tcr {
        let val = (self.0 >> 0usize) & 0xff;
        Tcr::from_bits(val as u8)
    }
    #[doc = "Time Compensation Register."]
    #[inline(always)]
    pub const fn set_tcr(&mut self, val: Tcr) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Compensation Interval Register."]
    #[must_use]
    #[inline(always)]
    pub const fn cir(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Compensation Interval Register."]
    #[inline(always)]
    pub const fn set_cir(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Time Compensation Value."]
    #[must_use]
    #[inline(always)]
    pub const fn tcv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Time Compensation Value."]
    #[inline(always)]
    pub const fn set_tcv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Compensation Interval Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn cic(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Compensation Interval Counter."]
    #[inline(always)]
    pub const fn set_cic(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Tcr {
    #[inline(always)]
    fn default() -> Tcr {
        Tcr(0)
    }
}
impl core::fmt::Debug for Tcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr")
            .field("tcr", &self.tcr())
            .field("cir", &self.cir())
            .field("tcv", &self.tcv())
            .field("cic", &self.cic())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcr {{ tcr: {:?}, cir: {=u8:?}, tcv: {=u8:?}, cic: {=u8:?} }}",
            self.tcr(),
            self.cir(),
            self.tcv(),
            self.cic()
        )
    }
}
#[doc = "RTC Time Prescaler."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tpr(pub u32);
impl Tpr {
    #[doc = "Time Prescaler Register."]
    #[must_use]
    #[inline(always)]
    pub const fn tpr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Time Prescaler Register."]
    #[inline(always)]
    pub const fn set_tpr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tpr {
    #[inline(always)]
    fn default() -> Tpr {
        Tpr(0)
    }
}
impl core::fmt::Debug for Tpr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tpr").field("tpr", &self.tpr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tpr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tpr {{ tpr: {=u16:?} }}", self.tpr())
    }
}
#[doc = "RTC Time Seconds."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsr(pub u32);
impl Tsr {
    #[doc = "Time Seconds Register."]
    #[must_use]
    #[inline(always)]
    pub const fn tsr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Time Seconds Register."]
    #[inline(always)]
    pub const fn set_tsr(&mut self, val: u32) {
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
        f.debug_struct("Tsr").field("tsr", &self.tsr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tsr {{ tsr: {=u32:?} }}", self.tsr())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crl {
    #[doc = "Control Register is locked and writes are ignored."]
    CRL_0 = 0x0,
    #[doc = "Control Register is not locked and writes complete as normal."]
    CRL_1 = 0x01,
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
pub enum Lpos {
    #[doc = "RTC prescaler increments using 32.768 kHz clock."]
    LPOS_0 = 0x0,
    #[doc = "RTC prescaler increments using 1 kHz LPO, bits \\[4:0\\] of the prescaler are ignored."]
    LPOS_1 = 0x01,
}
impl Lpos {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpos {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpos {
    #[inline(always)]
    fn from(val: u8) -> Lpos {
        Lpos::from_bits(val)
    }
}
impl From<Lpos> for u8 {
    #[inline(always)]
    fn from(val: Lpos) -> u8 {
        Lpos::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lrl {
    #[doc = "Lock Register is locked and writes are ignored."]
    LRL_0 = 0x0,
    #[doc = "Lock Register is not locked and writes complete as normal."]
    LRL_1 = 0x01,
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
pub enum Srl {
    #[doc = "Status Register is locked and writes are ignored."]
    SRL_0 = 0x0,
    #[doc = "Status Register is not locked and writes complete as normal."]
    SRL_1 = 0x01,
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
    SWR_0 = 0x0,
    #[doc = "Resets all RTC registers except for the SWR bit . The SWR bit is cleared by POR and by software explicitly clearing it."]
    SWR_1 = 0x01,
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
pub enum Taf {
    #[doc = "Time alarm has not occurred."]
    TAF_0 = 0x0,
    #[doc = "Time alarm has occurred."]
    TAF_1 = 0x01,
}
impl Taf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Taf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Taf {
    #[inline(always)]
    fn from(val: u8) -> Taf {
        Taf::from_bits(val)
    }
}
impl From<Taf> for u8 {
    #[inline(always)]
    fn from(val: Taf) -> u8 {
        Taf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Taie {
    #[doc = "No interrupt is generated."]
    TAIE_0 = 0x0,
    #[doc = "An interrupt is generated."]
    TAIE_1 = 0x01,
}
impl Taie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Taie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Taie {
    #[inline(always)]
    fn from(val: u8) -> Taie {
        Taie::from_bits(val)
    }
}
impl From<Taie> for u8 {
    #[inline(always)]
    fn from(val: Taie) -> u8 {
        Taie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tce {
    #[doc = "Disables."]
    TCE_0 = 0x0,
    #[doc = "Enables."]
    TCE_1 = 0x01,
}
impl Tce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tce {
    #[inline(always)]
    fn from(val: u8) -> Tce {
        Tce::from_bits(val)
    }
}
impl From<Tce> for u8 {
    #[inline(always)]
    fn from(val: Tce) -> u8 {
        Tce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcl {
    #[doc = "Time Compensation Register is locked and writes are ignored."]
    TCL_0 = 0x0,
    #[doc = "Time Compensation Register is not locked and writes complete as normal."]
    TCL_1 = 0x01,
}
impl Tcl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcl {
    #[inline(always)]
    fn from(val: u8) -> Tcl {
        Tcl::from_bits(val)
    }
}
impl From<Tcl> for u8 {
    #[inline(always)]
    fn from(val: Tcl) -> u8 {
        Tcl::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tcr(u8);
impl Tcr {
    #[doc = "Time Prescaler Register overflows every 32768 clock cycles."]
    pub const TCR_0: Self = Self(0x0);
    #[doc = "Time Prescaler Register overflows every 32767 clock cycles."]
    pub const TCR_1: Self = Self(0x01);
    #[doc = "Time Prescaler Register overflows every 32642 clock cycles."]
    pub const TCR_126: Self = Self(0x7e);
    #[doc = "Time Prescaler Register overflows every 32641 clock cycles."]
    pub const TCR_127: Self = Self(0x7f);
    #[doc = "Time Prescaler Register overflows every 32896 clock cycles."]
    pub const TCR_128: Self = Self(0x80);
    #[doc = "Time Prescaler Register overflows every 32895 clock cycles."]
    pub const TCR_129: Self = Self(0x81);
    #[doc = "Time Prescaler Register overflows every 32769 clock cycles."]
    pub const TCR_255: Self = Self(0xff);
}
impl Tcr {
    pub const fn from_bits(val: u8) -> Tcr {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Tcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("TCR_0"),
            0x01 => f.write_str("TCR_1"),
            0x7e => f.write_str("TCR_126"),
            0x7f => f.write_str("TCR_127"),
            0x80 => f.write_str("TCR_128"),
            0x81 => f.write_str("TCR_129"),
            0xff => f.write_str("TCR_255"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "TCR_0"),
            0x01 => defmt::write!(f, "TCR_1"),
            0x7e => defmt::write!(f, "TCR_126"),
            0x7f => defmt::write!(f, "TCR_127"),
            0x80 => defmt::write!(f, "TCR_128"),
            0x81 => defmt::write!(f, "TCR_129"),
            0xff => defmt::write!(f, "TCR_255"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Tcr {
    #[inline(always)]
    fn from(val: u8) -> Tcr {
        Tcr::from_bits(val)
    }
}
impl From<Tcr> for u8 {
    #[inline(always)]
    fn from(val: Tcr) -> u8 {
        Tcr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tif {
    #[doc = "Time is valid."]
    TIF_0 = 0x0,
    #[doc = "Time is invalid and time counter is read as zero."]
    TIF_1 = 0x01,
}
impl Tif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tif {
    #[inline(always)]
    fn from(val: u8) -> Tif {
        Tif::from_bits(val)
    }
}
impl From<Tif> for u8 {
    #[inline(always)]
    fn from(val: Tif) -> u8 {
        Tif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tiie {
    #[doc = "No interrupt is generated."]
    TIIE_0 = 0x0,
    #[doc = "An interrupt is generated."]
    TIIE_1 = 0x01,
}
impl Tiie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tiie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tiie {
    #[inline(always)]
    fn from(val: u8) -> Tiie {
        Tiie::from_bits(val)
    }
}
impl From<Tiie> for u8 {
    #[inline(always)]
    fn from(val: Tiie) -> u8 {
        Tiie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tof {
    #[doc = "Time overflow has not occurred."]
    TOF_0 = 0x0,
    #[doc = "Time overflow has occurred and time counter reads as zero."]
    TOF_1 = 0x01,
}
impl Tof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tof {
    #[inline(always)]
    fn from(val: u8) -> Tof {
        Tof::from_bits(val)
    }
}
impl From<Tof> for u8 {
    #[inline(always)]
    fn from(val: Tof) -> u8 {
        Tof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Toie {
    #[doc = "No interrupt is generated."]
    TOIE_0 = 0x0,
    #[doc = "An interrupt is generated."]
    TOIE_1 = 0x01,
}
impl Toie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Toie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Toie {
    #[inline(always)]
    fn from(val: u8) -> Toie {
        Toie::from_bits(val)
    }
}
impl From<Toie> for u8 {
    #[inline(always)]
    fn from(val: Toie) -> u8 {
        Toie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsic {
    #[doc = "1 Hz."]
    TSIC_0 = 0x0,
    #[doc = "2 Hz."]
    TSIC_1 = 0x01,
    #[doc = "4 Hz."]
    TSIC_2 = 0x02,
    #[doc = "8 Hz."]
    TSIC_3 = 0x03,
    #[doc = "16 Hz."]
    TSIC_4 = 0x04,
    #[doc = "32 Hz."]
    TSIC_5 = 0x05,
    #[doc = "64 Hz."]
    TSIC_6 = 0x06,
    #[doc = "128 Hz."]
    TSIC_7 = 0x07,
}
impl Tsic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsic {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsic {
    #[inline(always)]
    fn from(val: u8) -> Tsic {
        Tsic::from_bits(val)
    }
}
impl From<Tsic> for u8 {
    #[inline(always)]
    fn from(val: Tsic) -> u8 {
        Tsic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsie {
    #[doc = "Disables."]
    TSIE_0 = 0x0,
    #[doc = "Enables."]
    TSIE_1 = 0x01,
}
impl Tsie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsie {
    #[inline(always)]
    fn from(val: u8) -> Tsie {
        Tsie::from_bits(val)
    }
}
impl From<Tsie> for u8 {
    #[inline(always)]
    fn from(val: Tsie) -> u8 {
        Tsie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Um {
    #[doc = "Registers cannot be written when locked."]
    UM_0 = 0x0,
    #[doc = "Registers can be written when locked under limited conditions."]
    UM_1 = 0x01,
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
