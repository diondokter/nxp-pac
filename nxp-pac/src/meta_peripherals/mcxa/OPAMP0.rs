#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6a8c2aa 2026-01-27))"]
#[doc = "OPAMP."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opamp0 {
    ptr: *mut u8,
}
unsafe impl Send for Opamp0 {}
unsafe impl Sync for Opamp0 {}
impl Opamp0 {
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
    pub const fn param(self) -> crate::pac::common::Reg<u32, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "OPAMP Control."]
    #[inline(always)]
    pub const fn opamp_ctrl(self) -> crate::pac::common::Reg<OpampCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
#[doc = "OPAMP Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OpampCtrl(pub u32);
impl OpampCtrl {
    #[doc = "OPAMP Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn opa_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OPAMP Enable."]
    #[inline(always)]
    pub const fn set_opa_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Compensation capcitor config selection."]
    #[must_use]
    #[inline(always)]
    pub const fn opa_cc_sel(&self) -> OpaCcSel {
        let val = (self.0 >> 4usize) & 0x03;
        OpaCcSel::from_bits(val as u8)
    }
    #[doc = "Compensation capcitor config selection."]
    #[inline(always)]
    pub const fn set_opa_cc_sel(&mut self, val: OpaCcSel) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Bias current config selection."]
    #[must_use]
    #[inline(always)]
    pub const fn opa_bc_sel(&self) -> OpaBcSel {
        let val = (self.0 >> 6usize) & 0x03;
        OpaBcSel::from_bits(val as u8)
    }
    #[doc = "Bias current config selection."]
    #[inline(always)]
    pub const fn set_opa_bc_sel(&mut self, val: OpaBcSel) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
}
impl Default for OpampCtrl {
    #[inline(always)]
    fn default() -> OpampCtrl {
        OpampCtrl(0)
    }
}
impl core::fmt::Debug for OpampCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OpampCtrl")
            .field("opa_en", &self.opa_en())
            .field("opa_cc_sel", &self.opa_cc_sel())
            .field("opa_bc_sel", &self.opa_bc_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OpampCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OpampCtrl {{ opa_en: {=bool:?}, opa_cc_sel: {:?}, opa_bc_sel: {:?} }}",
            self.opa_en(),
            self.opa_cc_sel(),
            self.opa_bc_sel()
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
    pub const fn feature(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feature Specification Number."]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
            "Verid {{ feature: {=u16:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OpaBcSel {
    #[doc = "Default value. Keep power consumption constant."]
    TBD1 = 0x0,
    #[doc = "Reduce power consumption to 1/4."]
    TBD2 = 0x01,
    #[doc = "Reduce power consumption to 1/2."]
    TBD3 = 0x02,
    #[doc = "Double the power consumption."]
    TBD4 = 0x03,
}
impl OpaBcSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OpaBcSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OpaBcSel {
    #[inline(always)]
    fn from(val: u8) -> OpaBcSel {
        OpaBcSel::from_bits(val)
    }
}
impl From<OpaBcSel> for u8 {
    #[inline(always)]
    fn from(val: OpaBcSel) -> u8 {
        OpaBcSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OpaCcSel {
    #[doc = "Fit 2X gains."]
    TBD1 = 0x0,
    #[doc = "Fit 4X gains."]
    TBD2 = 0x01,
    #[doc = "Fit 8X gains."]
    TBD3 = 0x02,
    #[doc = "Fit 16X gains."]
    TBD4 = 0x03,
}
impl OpaCcSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OpaCcSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OpaCcSel {
    #[inline(always)]
    fn from(val: u8) -> OpaCcSel {
        OpaCcSel::from_bits(val)
    }
}
impl From<OpaCcSel> for u8 {
    #[inline(always)]
    fn from(val: OpaCcSel) -> u8 {
        OpaCcSel::to_bits(val)
    }
}
