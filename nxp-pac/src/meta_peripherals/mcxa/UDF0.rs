#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6a8c2aa 2026-01-27))"]
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Udf0 {
    ptr: *mut u8,
}
unsafe impl Send for Udf0 {}
unsafe impl Sync for Udf0 {}
impl Udf0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register."]
    #[inline(always)]
    pub const fn udf_ctrl(self) -> crate::pac::common::Reg<UdfCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Status register."]
    #[inline(always)]
    pub const fn udf_status(self) -> crate::pac::common::Reg<UdfStatus, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Data In Register."]
    #[inline(always)]
    pub const fn udf_wr_data(self) -> crate::pac::common::Reg<UdfWrData, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Data Out Register."]
    #[inline(always)]
    pub const fn udf_rd_data(self) -> crate::pac::common::Reg<UdfRdData, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
#[doc = "Control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdfCtrl(pub u32);
impl UdfCtrl {
    #[doc = "Bits are internally XORed with i_custom."]
    #[must_use]
    #[inline(always)]
    pub const fn salt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Bits are internally XORed with i_custom."]
    #[inline(always)]
    pub const fn set_salt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Lock access to UDF."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Lock access to UDF."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "RFU."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved21(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x07;
        val as u8
    }
    #[doc = "RFU."]
    #[inline(always)]
    pub const fn set_reserved21(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
    }
    #[doc = "Enable the UDF block."]
    #[must_use]
    #[inline(always)]
    pub const fn udf_en(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x07;
        val as u8
    }
    #[doc = "Enable the UDF block."]
    #[inline(always)]
    pub const fn set_udf_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
    }
    #[doc = "RFU."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "RFU."]
    #[inline(always)]
    pub const fn set_reserved25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "RFU."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved27(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "RFU."]
    #[inline(always)]
    pub const fn set_reserved27(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Flush UDF and return to reset state."]
    #[must_use]
    #[inline(always)]
    pub const fn flush(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "Flush UDF and return to reset state."]
    #[inline(always)]
    pub const fn set_flush(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
    #[doc = "reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "reserved."]
    #[inline(always)]
    pub const fn set_reserved31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for UdfCtrl {
    #[inline(always)]
    fn default() -> UdfCtrl {
        UdfCtrl(0)
    }
}
impl core::fmt::Debug for UdfCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdfCtrl")
            .field("salt", &self.salt())
            .field("lock", &self.lock())
            .field("reserved21", &self.reserved21())
            .field("udf_en", &self.udf_en())
            .field("reserved25", &self.reserved25())
            .field("reserved27", &self.reserved27())
            .field("flush", &self.flush())
            .field("reserved31", &self.reserved31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdfCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UdfCtrl {{ salt: {=u16:?}, lock: {=u8:?}, reserved21: {=u8:?}, udf_en: {=u8:?}, reserved25: {=bool:?}, reserved27: {=u8:?}, flush: {=u8:?}, reserved31: {=bool:?} }}",
            self.salt(),
            self.lock(),
            self.reserved21(),
            self.udf_en(),
            self.reserved25(),
            self.reserved27(),
            self.flush(),
            self.reserved31()
        )
    }
}
#[doc = "Data Out Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdfRdData(pub u32);
impl UdfRdData {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn o_dat(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_o_dat(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdfRdData {
    #[inline(always)]
    fn default() -> UdfRdData {
        UdfRdData(0)
    }
}
impl core::fmt::Debug for UdfRdData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdfRdData")
            .field("o_dat", &self.o_dat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdfRdData {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdfRdData {{ o_dat: {=u32:?} }}", self.o_dat())
    }
}
#[doc = "Status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdfStatus(pub u32);
impl UdfStatus {
    #[doc = "Status bits."]
    #[must_use]
    #[inline(always)]
    pub const fn o_status(&self) -> OStatus {
        let val = (self.0 >> 0usize) & 0x1f;
        OStatus::from_bits(val as u8)
    }
    #[doc = "Status bits."]
    #[inline(always)]
    pub const fn set_o_status(&mut self, val: OStatus) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "RFU."]
    #[must_use]
    #[inline(always)]
    pub const fn rsv(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "RFU."]
    #[inline(always)]
    pub const fn set_rsv(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 5usize)) | (((val as u32) & 0x03ff_ffff) << 5usize);
    }
    #[doc = "Indicates UDF is processing data."]
    #[must_use]
    #[inline(always)]
    pub const fn o_wait(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates UDF is processing data."]
    #[inline(always)]
    pub const fn set_o_wait(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for UdfStatus {
    #[inline(always)]
    fn default() -> UdfStatus {
        UdfStatus(0)
    }
}
impl core::fmt::Debug for UdfStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdfStatus")
            .field("o_status", &self.o_status())
            .field("rsv", &self.rsv())
            .field("o_wait", &self.o_wait())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdfStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UdfStatus {{ o_status: {:?}, rsv: {=u32:?}, o_wait: {=bool:?} }}",
            self.o_status(),
            self.rsv(),
            self.o_wait()
        )
    }
}
#[doc = "Data In Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdfWrData(pub u32);
impl UdfWrData {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn i_dat(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_i_dat(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdfWrData {
    #[inline(always)]
    fn default() -> UdfWrData {
        UdfWrData(0)
    }
}
impl core::fmt::Debug for UdfWrData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdfWrData")
            .field("i_dat", &self.i_dat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdfWrData {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdfWrData {{ i_dat: {=u32:?} }}", self.i_dat())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OStatus {
    _RESERVED_0 = 0x0,
    #[doc = "5'b00001 = Reset."]
    RESET = 0x01,
    #[doc = "5'b00010 = Init."]
    INIT = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "5'b00100 = Warmup."]
    WARMUP = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "5'b01000 = Ready."]
    READY = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "5'b10000 = Error."]
    ERROR = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl OStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OStatus {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OStatus {
    #[inline(always)]
    fn from(val: u8) -> OStatus {
        OStatus::from_bits(val)
    }
}
impl From<OStatus> for u8 {
    #[inline(always)]
    fn from(val: OStatus) -> u8 {
        OStatus::to_bits(val)
    }
}
