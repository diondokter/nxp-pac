#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6a8c2aa 2026-01-27))"]
#[doc = "M33 Systick module."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysTick {
    ptr: *mut u8,
}
unsafe impl Send for SysTick {}
unsafe impl Sync for SysTick {}
impl SysTick {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SysTick Control and Status Register."]
    #[inline(always)]
    pub const fn syst_csr(self) -> crate::pac::common::Reg<SystCsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "SysTick Reload Value Register."]
    #[inline(always)]
    pub const fn syst_rvr(self) -> crate::pac::common::Reg<SystRvr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "SysTick Current Value Register."]
    #[inline(always)]
    pub const fn syst_cvr(self) -> crate::pac::common::Reg<SystCvr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "SysTick Calibration Value Register."]
    #[inline(always)]
    pub const fn syst_calib(self) -> crate::pac::common::Reg<SystCalib, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
#[doc = "SysTick Calibration Value Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SystCalib(pub u32);
impl SystCalib {
    #[doc = "Reload value to use for 10ms timing."]
    #[must_use]
    #[inline(always)]
    pub const fn tenms(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Reload value to use for 10ms timing."]
    #[inline(always)]
    pub const fn set_tenms(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Indicates whether the TENMS value is exact."]
    #[must_use]
    #[inline(always)]
    pub const fn skew(&self) -> Skew {
        let val = (self.0 >> 30usize) & 0x01;
        Skew::from_bits(val as u8)
    }
    #[doc = "Indicates whether the TENMS value is exact."]
    #[inline(always)]
    pub const fn set_skew(&mut self, val: Skew) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Indicates whether the device provides an alternative reference clock."]
    #[must_use]
    #[inline(always)]
    pub const fn noref(&self) -> Noref {
        let val = (self.0 >> 31usize) & 0x01;
        Noref::from_bits(val as u8)
    }
    #[doc = "Indicates whether the device provides an alternative reference clock."]
    #[inline(always)]
    pub const fn set_noref(&mut self, val: Noref) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SystCalib {
    #[inline(always)]
    fn default() -> SystCalib {
        SystCalib(0)
    }
}
impl core::fmt::Debug for SystCalib {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SystCalib")
            .field("tenms", &self.tenms())
            .field("skew", &self.skew())
            .field("noref", &self.noref())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SystCalib {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SystCalib {{ tenms: {=u32:?}, skew: {:?}, noref: {:?} }}",
            self.tenms(),
            self.skew(),
            self.noref()
        )
    }
}
#[doc = "SysTick Control and Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SystCsr(pub u32);
impl SystCsr {
    #[doc = "Enable/disable systick counter."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> Enable {
        let val = (self.0 >> 0usize) & 0x01;
        Enable::from_bits(val as u8)
    }
    #[doc = "Enable/disable systick counter."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: Enable) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable Systick interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn tickint(&self) -> Tickint {
        let val = (self.0 >> 1usize) & 0x01;
        Tickint::from_bits(val as u8)
    }
    #[doc = "Enable Systick interrupt."]
    #[inline(always)]
    pub const fn set_tickint(&mut self, val: Tickint) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Clock source selection."]
    #[must_use]
    #[inline(always)]
    pub const fn clksource(&self) -> Clksource {
        let val = (self.0 >> 2usize) & 0x01;
        Clksource::from_bits(val as u8)
    }
    #[doc = "Clock source selection."]
    #[inline(always)]
    pub const fn set_clksource(&mut self, val: Clksource) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Returns 1 if timer counted to 0 since the last read of this register."]
    #[must_use]
    #[inline(always)]
    pub const fn countflag(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Returns 1 if timer counted to 0 since the last read of this register."]
    #[inline(always)]
    pub const fn set_countflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for SystCsr {
    #[inline(always)]
    fn default() -> SystCsr {
        SystCsr(0)
    }
}
impl core::fmt::Debug for SystCsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SystCsr")
            .field("enable", &self.enable())
            .field("tickint", &self.tickint())
            .field("clksource", &self.clksource())
            .field("countflag", &self.countflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SystCsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SystCsr {{ enable: {:?}, tickint: {:?}, clksource: {:?}, countflag: {=bool:?} }}",
            self.enable(),
            self.tickint(),
            self.clksource(),
            self.countflag()
        )
    }
}
#[doc = "SysTick Current Value Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SystCvr(pub u32);
impl SystCvr {
    #[doc = "Reads current counter value at the time the register is accessed. Any write to the register clears the register to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn current(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Reads current counter value at the time the register is accessed. Any write to the register clears the register to 0."]
    #[inline(always)]
    pub const fn set_current(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for SystCvr {
    #[inline(always)]
    fn default() -> SystCvr {
        SystCvr(0)
    }
}
impl core::fmt::Debug for SystCvr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SystCvr")
            .field("current", &self.current())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SystCvr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SystCvr {{ current: {=u32:?} }}", self.current())
    }
}
#[doc = "SysTick Reload Value Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SystRvr(pub u32);
impl SystRvr {
    #[doc = "Value to load into the SysTick Current Value Register when the counter reaches 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reload(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Value to load into the SysTick Current Value Register when the counter reaches 0."]
    #[inline(always)]
    pub const fn set_reload(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for SystRvr {
    #[inline(always)]
    fn default() -> SystRvr {
        SystRvr(0)
    }
}
impl core::fmt::Debug for SystRvr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SystRvr")
            .field("reload", &self.reload())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SystRvr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SystRvr {{ reload: {=u32:?} }}", self.reload())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clksource {
    #[doc = "external clock."]
    EXTERNAL_CLOCK = 0x0,
    #[doc = "processor clock."]
    PROCESSOR_CLOCK = 0x01,
}
impl Clksource {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clksource {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clksource {
    #[inline(always)]
    fn from(val: u8) -> Clksource {
        Clksource::from_bits(val)
    }
}
impl From<Clksource> for u8 {
    #[inline(always)]
    fn from(val: Clksource) -> u8 {
        Clksource::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enable {
    #[doc = "counter disabled."]
    COUNTER_DISABLED = 0x0,
    #[doc = "counter enabled."]
    COUNTER_ENABLED = 0x01,
}
impl Enable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enable {
    #[inline(always)]
    fn from(val: u8) -> Enable {
        Enable::from_bits(val)
    }
}
impl From<Enable> for u8 {
    #[inline(always)]
    fn from(val: Enable) -> u8 {
        Enable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Noref {
    #[doc = "The alternative reference clock is provided."]
    CLOCK_PROVIDED = 0x0,
    #[doc = "The alternative reference clock is not provided."]
    CLOCK_DISABLED = 0x01,
}
impl Noref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Noref {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Noref {
    #[inline(always)]
    fn from(val: u8) -> Noref {
        Noref::from_bits(val)
    }
}
impl From<Noref> for u8 {
    #[inline(always)]
    fn from(val: Noref) -> u8 {
        Noref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Skew {
    #[doc = "10ms calibration value is exact."]
    EXACT_VALUE = 0x0,
    #[doc = "10ms calibration value is inexact, because of the clock frequency."]
    INEXACT_VALUE = 0x01,
}
impl Skew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Skew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Skew {
    #[inline(always)]
    fn from(val: u8) -> Skew {
        Skew::from_bits(val)
    }
}
impl From<Skew> for u8 {
    #[inline(always)]
    fn from(val: Skew) -> u8 {
        Skew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tickint {
    #[doc = "counting down to 0 does not assert the SysTick exception request."]
    INTERRUPT_DISABLED = 0x0,
    #[doc = "counting down to 0 asserts the SysTick exception request."]
    INTERRUPT_ENABLED = 0x01,
}
impl Tickint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tickint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tickint {
    #[inline(always)]
    fn from(val: u8) -> Tickint {
        Tickint::from_bits(val)
    }
}
impl From<Tickint> for u8 {
    #[inline(always)]
    fn from(val: Tickint) -> u8 {
        Tickint::to_bits(val)
    }
}
