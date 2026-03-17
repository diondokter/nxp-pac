#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6a8c2aa 2026-01-27))"]
#[doc = "System Control not in System Control Block."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ScnScb {
    ptr: *mut u8,
}
unsafe impl Send for ScnScb {}
unsafe impl Sync for ScnScb {}
impl ScnScb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Coprocessor Power Control Register."]
    #[inline(always)]
    pub const fn cppwr(self) -> crate::pac::common::Reg<Cppwr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
#[doc = "Coprocessor Power Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cppwr(pub u32);
impl Cppwr {
    #[doc = "State UNKNOWN 0."]
    #[must_use]
    #[inline(always)]
    pub const fn su0(&self) -> Su0 {
        let val = (self.0 >> 0usize) & 0x01;
        Su0::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 0."]
    #[inline(always)]
    pub const fn set_su0(&mut self, val: Su0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "State UNKNOWN Secure only 0."]
    #[must_use]
    #[inline(always)]
    pub const fn sus0(&self) -> Sus0 {
        let val = (self.0 >> 1usize) & 0x01;
        Sus0::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 0."]
    #[inline(always)]
    pub const fn set_sus0(&mut self, val: Sus0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "State UNKNOWN 1."]
    #[must_use]
    #[inline(always)]
    pub const fn su1(&self) -> Su1 {
        let val = (self.0 >> 2usize) & 0x01;
        Su1::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 1."]
    #[inline(always)]
    pub const fn set_su1(&mut self, val: Su1) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "State UNKNOWN Secure only 1."]
    #[must_use]
    #[inline(always)]
    pub const fn sus1(&self) -> Sus1 {
        let val = (self.0 >> 3usize) & 0x01;
        Sus1::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 1."]
    #[inline(always)]
    pub const fn set_sus1(&mut self, val: Sus1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "State UNKNOWN 2."]
    #[must_use]
    #[inline(always)]
    pub const fn su2(&self) -> Su2 {
        let val = (self.0 >> 4usize) & 0x01;
        Su2::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 2."]
    #[inline(always)]
    pub const fn set_su2(&mut self, val: Su2) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "State UNKNOWN Secure only 2."]
    #[must_use]
    #[inline(always)]
    pub const fn sus2(&self) -> Sus2 {
        let val = (self.0 >> 5usize) & 0x01;
        Sus2::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 2."]
    #[inline(always)]
    pub const fn set_sus2(&mut self, val: Sus2) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "State UNKNOWN 3."]
    #[must_use]
    #[inline(always)]
    pub const fn su3(&self) -> Su3 {
        let val = (self.0 >> 6usize) & 0x01;
        Su3::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 3."]
    #[inline(always)]
    pub const fn set_su3(&mut self, val: Su3) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "State UNKNOWN Secure only 3."]
    #[must_use]
    #[inline(always)]
    pub const fn sus3(&self) -> Sus3 {
        let val = (self.0 >> 7usize) & 0x01;
        Sus3::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 3."]
    #[inline(always)]
    pub const fn set_sus3(&mut self, val: Sus3) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "State UNKNOWN 4."]
    #[must_use]
    #[inline(always)]
    pub const fn su4(&self) -> Su4 {
        let val = (self.0 >> 8usize) & 0x01;
        Su4::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 4."]
    #[inline(always)]
    pub const fn set_su4(&mut self, val: Su4) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "State UNKNOWN Secure only 4."]
    #[must_use]
    #[inline(always)]
    pub const fn sus4(&self) -> Sus4 {
        let val = (self.0 >> 9usize) & 0x01;
        Sus4::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 4."]
    #[inline(always)]
    pub const fn set_sus4(&mut self, val: Sus4) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "State UNKNOWN 5."]
    #[must_use]
    #[inline(always)]
    pub const fn su5(&self) -> Su5 {
        let val = (self.0 >> 10usize) & 0x01;
        Su5::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 5."]
    #[inline(always)]
    pub const fn set_su5(&mut self, val: Su5) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "State UNKNOWN Secure only 5."]
    #[must_use]
    #[inline(always)]
    pub const fn sus5(&self) -> Sus5 {
        let val = (self.0 >> 11usize) & 0x01;
        Sus5::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 5."]
    #[inline(always)]
    pub const fn set_sus5(&mut self, val: Sus5) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "State UNKNOWN 6."]
    #[must_use]
    #[inline(always)]
    pub const fn su6(&self) -> Su6 {
        let val = (self.0 >> 12usize) & 0x01;
        Su6::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 6."]
    #[inline(always)]
    pub const fn set_su6(&mut self, val: Su6) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "State UNKNOWN Secure only 6."]
    #[must_use]
    #[inline(always)]
    pub const fn sus6(&self) -> Sus6 {
        let val = (self.0 >> 13usize) & 0x01;
        Sus6::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 6."]
    #[inline(always)]
    pub const fn set_sus6(&mut self, val: Sus6) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "State UNKNOWN 7."]
    #[must_use]
    #[inline(always)]
    pub const fn su7(&self) -> Su7 {
        let val = (self.0 >> 14usize) & 0x01;
        Su7::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 7."]
    #[inline(always)]
    pub const fn set_su7(&mut self, val: Su7) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "State UNKNOWN Secure only 7."]
    #[must_use]
    #[inline(always)]
    pub const fn sus7(&self) -> Sus7 {
        let val = (self.0 >> 15usize) & 0x01;
        Sus7::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 7."]
    #[inline(always)]
    pub const fn set_sus7(&mut self, val: Sus7) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "State UNKNOWN 10."]
    #[must_use]
    #[inline(always)]
    pub const fn su10(&self) -> Su10 {
        let val = (self.0 >> 20usize) & 0x01;
        Su10::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN 10."]
    #[inline(always)]
    pub const fn set_su10(&mut self, val: Su10) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "State UNKNOWN Secure only 10."]
    #[must_use]
    #[inline(always)]
    pub const fn sus10(&self) -> Sus10 {
        let val = (self.0 >> 21usize) & 0x01;
        Sus10::from_bits(val as u8)
    }
    #[doc = "State UNKNOWN Secure only 10."]
    #[inline(always)]
    pub const fn set_sus10(&mut self, val: Sus10) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "State UNKNOWN 11."]
    #[must_use]
    #[inline(always)]
    pub const fn su11(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "State UNKNOWN 11."]
    #[inline(always)]
    pub const fn set_su11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "State UNKNOWN Secure only 11."]
    #[must_use]
    #[inline(always)]
    pub const fn sus11(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "State UNKNOWN Secure only 11."]
    #[inline(always)]
    pub const fn set_sus11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Cppwr {
    #[inline(always)]
    fn default() -> Cppwr {
        Cppwr(0)
    }
}
impl core::fmt::Debug for Cppwr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cppwr")
            .field("su0", &self.su0())
            .field("sus0", &self.sus0())
            .field("su1", &self.su1())
            .field("sus1", &self.sus1())
            .field("su2", &self.su2())
            .field("sus2", &self.sus2())
            .field("su3", &self.su3())
            .field("sus3", &self.sus3())
            .field("su4", &self.su4())
            .field("sus4", &self.sus4())
            .field("su5", &self.su5())
            .field("sus5", &self.sus5())
            .field("su6", &self.su6())
            .field("sus6", &self.sus6())
            .field("su7", &self.su7())
            .field("sus7", &self.sus7())
            .field("su10", &self.su10())
            .field("sus10", &self.sus10())
            .field("su11", &self.su11())
            .field("sus11", &self.sus11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cppwr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cppwr {{ su0: {:?}, sus0: {:?}, su1: {:?}, sus1: {:?}, su2: {:?}, sus2: {:?}, su3: {:?}, sus3: {:?}, su4: {:?}, sus4: {:?}, su5: {:?}, sus5: {:?}, su6: {:?}, sus6: {:?}, su7: {:?}, sus7: {:?}, su10: {:?}, sus10: {:?}, su11: {=bool:?}, sus11: {=bool:?} }}",
            self.su0(),
            self.sus0(),
            self.su1(),
            self.sus1(),
            self.su2(),
            self.sus2(),
            self.su3(),
            self.sus3(),
            self.su4(),
            self.sus4(),
            self.su5(),
            self.sus5(),
            self.su6(),
            self.sus6(),
            self.su7(),
            self.sus7(),
            self.su10(),
            self.sus10(),
            self.su11(),
            self.sus11()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Su0 {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl Su0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Su0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Su0 {
    #[inline(always)]
    fn from(val: u8) -> Su0 {
        Su0::from_bits(val)
    }
}
impl From<Su0> for u8 {
    #[inline(always)]
    fn from(val: Su0) -> u8 {
        Su0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Su1 {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl Su1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Su1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Su1 {
    #[inline(always)]
    fn from(val: u8) -> Su1 {
        Su1::from_bits(val)
    }
}
impl From<Su1> for u8 {
    #[inline(always)]
    fn from(val: Su1) -> u8 {
        Su1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Su10 {
    #[doc = "The floating-point state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The floating-point state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl Su10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Su10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Su10 {
    #[inline(always)]
    fn from(val: u8) -> Su10 {
        Su10::from_bits(val)
    }
}
impl From<Su10> for u8 {
    #[inline(always)]
    fn from(val: Su10) -> u8 {
        Su10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Su2 {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl Su2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Su2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Su2 {
    #[inline(always)]
    fn from(val: u8) -> Su2 {
        Su2::from_bits(val)
    }
}
impl From<Su2> for u8 {
    #[inline(always)]
    fn from(val: Su2) -> u8 {
        Su2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Su3 {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl Su3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Su3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Su3 {
    #[inline(always)]
    fn from(val: u8) -> Su3 {
        Su3::from_bits(val)
    }
}
impl From<Su3> for u8 {
    #[inline(always)]
    fn from(val: Su3) -> u8 {
        Su3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Su4 {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl Su4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Su4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Su4 {
    #[inline(always)]
    fn from(val: u8) -> Su4 {
        Su4::from_bits(val)
    }
}
impl From<Su4> for u8 {
    #[inline(always)]
    fn from(val: Su4) -> u8 {
        Su4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Su5 {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl Su5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Su5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Su5 {
    #[inline(always)]
    fn from(val: u8) -> Su5 {
        Su5::from_bits(val)
    }
}
impl From<Su5> for u8 {
    #[inline(always)]
    fn from(val: Su5) -> u8 {
        Su5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Su6 {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl Su6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Su6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Su6 {
    #[inline(always)]
    fn from(val: u8) -> Su6 {
        Su6::from_bits(val)
    }
}
impl From<Su6> for u8 {
    #[inline(always)]
    fn from(val: Su6) -> u8 {
        Su6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Su7 {
    #[doc = "The coprocessor state is not permitted to become UNKNOWN."]
    UNKNOWN_NOT_PERMITTED = 0x0,
    #[doc = "The coprocessor state is permitted to become UNKNOWN."]
    UNKNOWN_PERMITTED = 0x01,
}
impl Su7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Su7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Su7 {
    #[inline(always)]
    fn from(val: u8) -> Su7 {
        Su7::from_bits(val)
    }
}
impl From<Su7> for u8 {
    #[inline(always)]
    fn from(val: Su7) -> u8 {
        Su7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sus0 {
    #[doc = "The SU0 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU0 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl Sus0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sus0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sus0 {
    #[inline(always)]
    fn from(val: u8) -> Sus0 {
        Sus0::from_bits(val)
    }
}
impl From<Sus0> for u8 {
    #[inline(always)]
    fn from(val: Sus0) -> u8 {
        Sus0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sus1 {
    #[doc = "The SU7 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU7 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl Sus1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sus1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sus1 {
    #[inline(always)]
    fn from(val: u8) -> Sus1 {
        Sus1::from_bits(val)
    }
}
impl From<Sus1> for u8 {
    #[inline(always)]
    fn from(val: Sus1) -> u8 {
        Sus1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sus10 {
    #[doc = "The SU10 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU10 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl Sus10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sus10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sus10 {
    #[inline(always)]
    fn from(val: u8) -> Sus10 {
        Sus10::from_bits(val)
    }
}
impl From<Sus10> for u8 {
    #[inline(always)]
    fn from(val: Sus10) -> u8 {
        Sus10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sus2 {
    #[doc = "The SU2 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU2 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl Sus2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sus2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sus2 {
    #[inline(always)]
    fn from(val: u8) -> Sus2 {
        Sus2::from_bits(val)
    }
}
impl From<Sus2> for u8 {
    #[inline(always)]
    fn from(val: Sus2) -> u8 {
        Sus2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sus3 {
    #[doc = "The SU3 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU3 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl Sus3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sus3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sus3 {
    #[inline(always)]
    fn from(val: u8) -> Sus3 {
        Sus3::from_bits(val)
    }
}
impl From<Sus3> for u8 {
    #[inline(always)]
    fn from(val: Sus3) -> u8 {
        Sus3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sus4 {
    #[doc = "The SU4 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU4 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl Sus4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sus4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sus4 {
    #[inline(always)]
    fn from(val: u8) -> Sus4 {
        Sus4::from_bits(val)
    }
}
impl From<Sus4> for u8 {
    #[inline(always)]
    fn from(val: Sus4) -> u8 {
        Sus4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sus5 {
    #[doc = "The SU5 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU5 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl Sus5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sus5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sus5 {
    #[inline(always)]
    fn from(val: u8) -> Sus5 {
        Sus5::from_bits(val)
    }
}
impl From<Sus5> for u8 {
    #[inline(always)]
    fn from(val: Sus5) -> u8 {
        Sus5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sus6 {
    #[doc = "The SU6 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU6 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl Sus6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sus6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sus6 {
    #[inline(always)]
    fn from(val: u8) -> Sus6 {
        Sus6::from_bits(val)
    }
}
impl From<Sus6> for u8 {
    #[inline(always)]
    fn from(val: Sus6) -> u8 {
        Sus6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sus7 {
    #[doc = "The SU7 field is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SU7 field is only accessible from the Secure state."]
    SECURE_ONLY = 0x01,
}
impl Sus7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sus7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sus7 {
    #[inline(always)]
    fn from(val: u8) -> Sus7 {
        Sus7::from_bits(val)
    }
}
impl From<Sus7> for u8 {
    #[inline(always)]
    fn from(val: Sus7) -> u8 {
        Sus7::to_bits(val)
    }
}
