#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6a8c2aa 2026-01-27))"]
#[doc = "System Control Block."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scb {
    ptr: *mut u8,
}
unsafe impl Send for Scb {}
unsafe impl Sync for Scb {}
impl Scb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Application Interrupt and Reset Control Register."]
    #[inline(always)]
    pub const fn aircr(self) -> crate::common::Reg<Aircr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "The SCR controls features of entry to and exit from low-power state."]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<Scr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "System Handler Control and State Register."]
    #[inline(always)]
    pub const fn shcsr(self) -> crate::common::Reg<Shcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Coprocessor Access Control Register."]
    #[inline(always)]
    pub const fn cpacr(self) -> crate::common::Reg<Cpacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Non-secure Access Control Register."]
    #[inline(always)]
    pub const fn nsacr(self) -> crate::common::Reg<Nsacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
}
#[doc = "Application Interrupt and Reset Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aircr(pub u32);
impl Aircr {
    #[doc = "Reserved for Debug use. This bit reads as 0. When writing to the register you must write 0 to this bit, otherwise behavior is UNPREDICTABLE. This bit is not banked between Security states."]
    #[must_use]
    #[inline(always)]
    pub const fn vectclractive(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved for Debug use. This bit reads as 0. When writing to the register you must write 0 to this bit, otherwise behavior is UNPREDICTABLE. This bit is not banked between Security states."]
    #[inline(always)]
    pub const fn set_vectclractive(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "System reset request. This bit allows software or a debugger to request a system reset. This bit is not banked between Security states. RW if SYSRESETREQS is 0. When SYSRESETREQS is set to 1, from Non-secure state this bit acts as RAZ/WI."]
    #[must_use]
    #[inline(always)]
    pub const fn sysresetreq(&self) -> Sysresetreq {
        let val = (self.0 >> 2usize) & 0x01;
        Sysresetreq::from_bits(val as u8)
    }
    #[doc = "System reset request. This bit allows software or a debugger to request a system reset. This bit is not banked between Security states. RW if SYSRESETREQS is 0. When SYSRESETREQS is set to 1, from Non-secure state this bit acts as RAZ/WI."]
    #[inline(always)]
    pub const fn set_sysresetreq(&mut self, val: Sysresetreq) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "System reset request, Secure state only. The value of this bit defines whether the SYSRESETREQ bit is functional for Non-secure use. This bit is not banked between Security states. RW from Secure State and RAZ/WI from Non-secure state."]
    #[must_use]
    #[inline(always)]
    pub const fn sysresetreqs(&self) -> Sysresetreqs {
        let val = (self.0 >> 3usize) & 0x01;
        Sysresetreqs::from_bits(val as u8)
    }
    #[doc = "System reset request, Secure state only. The value of this bit defines whether the SYSRESETREQ bit is functional for Non-secure use. This bit is not banked between Security states. RW from Secure State and RAZ/WI from Non-secure state."]
    #[inline(always)]
    pub const fn set_sysresetreqs(&mut self, val: Sysresetreqs) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt priority grouping field. This field determines the split of group priority from subpriority. This bit is banked between Security states."]
    #[must_use]
    #[inline(always)]
    pub const fn prigroup(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Interrupt priority grouping field. This field determines the split of group priority from subpriority. This bit is banked between Security states."]
    #[inline(always)]
    pub const fn set_prigroup(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "BusFault, HardFault, and NMI Non-secure enable. The value of this bit defines whether BusFault and NMI exceptions are Non-secure, and whether exceptions target the Non-secure HardFault exception. This bit is not banked between Security states. RW from Secure-state and RO from Non-secure state."]
    #[must_use]
    #[inline(always)]
    pub const fn bfhfnmins(&self) -> Bfhfnmins {
        let val = (self.0 >> 13usize) & 0x01;
        Bfhfnmins::from_bits(val as u8)
    }
    #[doc = "BusFault, HardFault, and NMI Non-secure enable. The value of this bit defines whether BusFault and NMI exceptions are Non-secure, and whether exceptions target the Non-secure HardFault exception. This bit is not banked between Security states. RW from Secure-state and RO from Non-secure state."]
    #[inline(always)]
    pub const fn set_bfhfnmins(&mut self, val: Bfhfnmins) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Prioritize Secure exceptions. The value of this bit defines whether Secure exception priority boosting is enabled. This bit is not banked between Security states. RW from Secure state and RAZ/WI from Non-secure state."]
    #[must_use]
    #[inline(always)]
    pub const fn pris(&self) -> Pris {
        let val = (self.0 >> 14usize) & 0x01;
        Pris::from_bits(val as u8)
    }
    #[doc = "Prioritize Secure exceptions. The value of this bit defines whether Secure exception priority boosting is enabled. This bit is not banked between Security states. RW from Secure state and RAZ/WI from Non-secure state."]
    #[inline(always)]
    pub const fn set_pris(&mut self, val: Pris) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Data endianness bit. This bit is not banked between Security states."]
    #[must_use]
    #[inline(always)]
    pub const fn endianness(&self) -> Endianness {
        let val = (self.0 >> 15usize) & 0x01;
        Endianness::from_bits(val as u8)
    }
    #[doc = "Data endianness bit. This bit is not banked between Security states."]
    #[inline(always)]
    pub const fn set_endianness(&mut self, val: Endianness) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Register key: Reads as 0xFA05. On writes, write 0x5FA to VECTKEY, otherwise the write is ignored. This Field is not banked between Security states."]
    #[must_use]
    #[inline(always)]
    pub const fn vectkey(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Register key: Reads as 0xFA05. On writes, write 0x5FA to VECTKEY, otherwise the write is ignored. This Field is not banked between Security states."]
    #[inline(always)]
    pub const fn set_vectkey(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Aircr {
    #[inline(always)]
    fn default() -> Aircr {
        Aircr(0)
    }
}
impl core::fmt::Debug for Aircr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aircr")
            .field("vectclractive", &self.vectclractive())
            .field("sysresetreq", &self.sysresetreq())
            .field("sysresetreqs", &self.sysresetreqs())
            .field("prigroup", &self.prigroup())
            .field("bfhfnmins", &self.bfhfnmins())
            .field("pris", &self.pris())
            .field("endianness", &self.endianness())
            .field("vectkey", &self.vectkey())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aircr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Aircr {{ vectclractive: {=bool:?}, sysresetreq: {:?}, sysresetreqs: {:?}, prigroup: {=u8:?}, bfhfnmins: {:?}, pris: {:?}, endianness: {:?}, vectkey: {=u16:?} }}",
            self.vectclractive(),
            self.sysresetreq(),
            self.sysresetreqs(),
            self.prigroup(),
            self.bfhfnmins(),
            self.pris(),
            self.endianness(),
            self.vectkey()
        )
    }
}
#[doc = "Coprocessor Access Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpacr(pub u32);
impl Cpacr {
    #[doc = "CP0 Privilege."]
    #[must_use]
    #[inline(always)]
    pub const fn cp0(&self) -> CpacrCp0 {
        let val = (self.0 >> 0usize) & 0x03;
        CpacrCp0::from_bits(val as u8)
    }
    #[doc = "CP0 Privilege."]
    #[inline(always)]
    pub const fn set_cp0(&mut self, val: CpacrCp0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CP1 Privilege."]
    #[must_use]
    #[inline(always)]
    pub const fn cp1(&self) -> CpacrCp1 {
        let val = (self.0 >> 2usize) & 0x03;
        CpacrCp1::from_bits(val as u8)
    }
    #[doc = "CP1 Privilege."]
    #[inline(always)]
    pub const fn set_cp1(&mut self, val: CpacrCp1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "CP2 Privilege."]
    #[must_use]
    #[inline(always)]
    pub const fn cp2(&self) -> CpacrCp2 {
        let val = (self.0 >> 4usize) & 0x03;
        CpacrCp2::from_bits(val as u8)
    }
    #[doc = "CP2 Privilege."]
    #[inline(always)]
    pub const fn set_cp2(&mut self, val: CpacrCp2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CP3 Privilege."]
    #[must_use]
    #[inline(always)]
    pub const fn cp3(&self) -> CpacrCp3 {
        let val = (self.0 >> 6usize) & 0x03;
        CpacrCp3::from_bits(val as u8)
    }
    #[doc = "CP3 Privilege."]
    #[inline(always)]
    pub const fn set_cp3(&mut self, val: CpacrCp3) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "CP4 Privilege."]
    #[must_use]
    #[inline(always)]
    pub const fn cp4(&self) -> CpacrCp4 {
        let val = (self.0 >> 8usize) & 0x03;
        CpacrCp4::from_bits(val as u8)
    }
    #[doc = "CP4 Privilege."]
    #[inline(always)]
    pub const fn set_cp4(&mut self, val: CpacrCp4) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CP5 Privilege."]
    #[must_use]
    #[inline(always)]
    pub const fn cp5(&self) -> CpacrCp5 {
        let val = (self.0 >> 10usize) & 0x03;
        CpacrCp5::from_bits(val as u8)
    }
    #[doc = "CP5 Privilege."]
    #[inline(always)]
    pub const fn set_cp5(&mut self, val: CpacrCp5) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "CP6 Privilege."]
    #[must_use]
    #[inline(always)]
    pub const fn cp6(&self) -> CpacrCp6 {
        let val = (self.0 >> 12usize) & 0x03;
        CpacrCp6::from_bits(val as u8)
    }
    #[doc = "CP6 Privilege."]
    #[inline(always)]
    pub const fn set_cp6(&mut self, val: CpacrCp6) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "CP7 Privilege."]
    #[must_use]
    #[inline(always)]
    pub const fn cp7(&self) -> CpacrCp7 {
        let val = (self.0 >> 14usize) & 0x03;
        CpacrCp7::from_bits(val as u8)
    }
    #[doc = "CP7 Privilege."]
    #[inline(always)]
    pub const fn set_cp7(&mut self, val: CpacrCp7) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "CP10 Privilege."]
    #[must_use]
    #[inline(always)]
    pub const fn cp10(&self) -> CpacrCp10 {
        let val = (self.0 >> 20usize) & 0x03;
        CpacrCp10::from_bits(val as u8)
    }
    #[doc = "CP10 Privilege."]
    #[inline(always)]
    pub const fn set_cp10(&mut self, val: CpacrCp10) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "CP11 Privilege."]
    #[must_use]
    #[inline(always)]
    pub const fn cp11(&self) -> Cp11 {
        let val = (self.0 >> 22usize) & 0x03;
        Cp11::from_bits(val as u8)
    }
    #[doc = "CP11 Privilege."]
    #[inline(always)]
    pub const fn set_cp11(&mut self, val: Cp11) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
}
impl Default for Cpacr {
    #[inline(always)]
    fn default() -> Cpacr {
        Cpacr(0)
    }
}
impl core::fmt::Debug for Cpacr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpacr")
            .field("cp0", &self.cp0())
            .field("cp1", &self.cp1())
            .field("cp2", &self.cp2())
            .field("cp3", &self.cp3())
            .field("cp4", &self.cp4())
            .field("cp5", &self.cp5())
            .field("cp6", &self.cp6())
            .field("cp7", &self.cp7())
            .field("cp10", &self.cp10())
            .field("cp11", &self.cp11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpacr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpacr {{ cp0: {:?}, cp1: {:?}, cp2: {:?}, cp3: {:?}, cp4: {:?}, cp5: {:?}, cp6: {:?}, cp7: {:?}, cp10: {:?}, cp11: {:?} }}",
            self.cp0(),
            self.cp1(),
            self.cp2(),
            self.cp3(),
            self.cp4(),
            self.cp5(),
            self.cp6(),
            self.cp7(),
            self.cp10(),
            self.cp11()
        )
    }
}
#[doc = "Non-secure Access Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nsacr(pub u32);
impl Nsacr {
    #[doc = "CP0 access."]
    #[must_use]
    #[inline(always)]
    pub const fn cp0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CP0 access."]
    #[inline(always)]
    pub const fn set_cp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CP1 access."]
    #[must_use]
    #[inline(always)]
    pub const fn cp1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CP1 access."]
    #[inline(always)]
    pub const fn set_cp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CP2 access."]
    #[must_use]
    #[inline(always)]
    pub const fn cp2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CP2 access."]
    #[inline(always)]
    pub const fn set_cp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CP3 access."]
    #[must_use]
    #[inline(always)]
    pub const fn cp3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CP3 access."]
    #[inline(always)]
    pub const fn set_cp3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CP4 access."]
    #[must_use]
    #[inline(always)]
    pub const fn cp4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CP4 access."]
    #[inline(always)]
    pub const fn set_cp4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CP5 access."]
    #[must_use]
    #[inline(always)]
    pub const fn cp5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CP5 access."]
    #[inline(always)]
    pub const fn set_cp5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CP6 access."]
    #[must_use]
    #[inline(always)]
    pub const fn cp6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CP6 access."]
    #[inline(always)]
    pub const fn set_cp6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CP7 access."]
    #[must_use]
    #[inline(always)]
    pub const fn cp7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CP7 access."]
    #[inline(always)]
    pub const fn set_cp7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "CP10 access."]
    #[must_use]
    #[inline(always)]
    pub const fn cp10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CP10 access."]
    #[inline(always)]
    pub const fn set_cp10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "CP11 access."]
    #[must_use]
    #[inline(always)]
    pub const fn cp11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CP11 access."]
    #[inline(always)]
    pub const fn set_cp11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Nsacr {
    #[inline(always)]
    fn default() -> Nsacr {
        Nsacr(0)
    }
}
impl core::fmt::Debug for Nsacr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nsacr")
            .field("cp0", &self.cp0())
            .field("cp1", &self.cp1())
            .field("cp2", &self.cp2())
            .field("cp3", &self.cp3())
            .field("cp4", &self.cp4())
            .field("cp5", &self.cp5())
            .field("cp6", &self.cp6())
            .field("cp7", &self.cp7())
            .field("cp10", &self.cp10())
            .field("cp11", &self.cp11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nsacr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Nsacr {{ cp0: {=bool:?}, cp1: {=bool:?}, cp2: {=bool:?}, cp3: {=bool:?}, cp4: {=bool:?}, cp5: {=bool:?}, cp6: {=bool:?}, cp7: {=bool:?}, cp10: {=bool:?}, cp11: {=bool:?} }}",
            self.cp0(),
            self.cp1(),
            self.cp2(),
            self.cp3(),
            self.cp4(),
            self.cp5(),
            self.cp6(),
            self.cp7(),
            self.cp10(),
            self.cp11()
        )
    }
}
#[doc = "The SCR controls features of entry to and exit from low-power state."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr(pub u32);
impl Scr {
    #[doc = "Indicates sleep-on-exit when returning from Handler mode to Thread mode. Setting this bit to 1 enables an interrupt driven application to avoid returning to an empty main application. This bit is banked between Security states."]
    #[must_use]
    #[inline(always)]
    pub const fn sleeponexit(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates sleep-on-exit when returning from Handler mode to Thread mode. Setting this bit to 1 enables an interrupt driven application to avoid returning to an empty main application. This bit is banked between Security states."]
    #[inline(always)]
    pub const fn set_sleeponexit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Controls whether the processor uses sleep or deep sleep as its low-power mode. This bit is not banked between Security states."]
    #[must_use]
    #[inline(always)]
    pub const fn sleepdeep(&self) -> Sleepdeep {
        let val = (self.0 >> 2usize) & 0x01;
        Sleepdeep::from_bits(val as u8)
    }
    #[doc = "Controls whether the processor uses sleep or deep sleep as its low-power mode. This bit is not banked between Security states."]
    #[inline(always)]
    pub const fn set_sleepdeep(&mut self, val: Sleepdeep) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Controls whether the SLEEPDEEP bit is only accessible from the Secure state. This bit in only accessible from the Secure state, and behaves as RAZ/WI when accessed from the Nonsecure state. This bit is not banked between Security states."]
    #[must_use]
    #[inline(always)]
    pub const fn sleepdeeps(&self) -> Sleepdeeps {
        let val = (self.0 >> 3usize) & 0x01;
        Sleepdeeps::from_bits(val as u8)
    }
    #[doc = "Controls whether the SLEEPDEEP bit is only accessible from the Secure state. This bit in only accessible from the Secure state, and behaves as RAZ/WI when accessed from the Nonsecure state. This bit is not banked between Security states."]
    #[inline(always)]
    pub const fn set_sleepdeeps(&mut self, val: Sleepdeeps) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Send Event on Pending bit. When an event or interrupt enters pending state, the event signal wakes up the processor from WFE. If the processor is not waiting for an event, the event is registered and affects the next WFE. The processor also wakes up on execution of an SEV instruction or an external event. This bit is banked between Security states."]
    #[must_use]
    #[inline(always)]
    pub const fn sevonpend(&self) -> Sevonpend {
        let val = (self.0 >> 4usize) & 0x01;
        Sevonpend::from_bits(val as u8)
    }
    #[doc = "Send Event on Pending bit. When an event or interrupt enters pending state, the event signal wakes up the processor from WFE. If the processor is not waiting for an event, the event is registered and affects the next WFE. The processor also wakes up on execution of an SEV instruction or an external event. This bit is banked between Security states."]
    #[inline(always)]
    pub const fn set_sevonpend(&mut self, val: Sevonpend) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for Scr {
    #[inline(always)]
    fn default() -> Scr {
        Scr(0)
    }
}
impl core::fmt::Debug for Scr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr")
            .field("sleeponexit", &self.sleeponexit())
            .field("sleepdeep", &self.sleepdeep())
            .field("sleepdeeps", &self.sleepdeeps())
            .field("sevonpend", &self.sevonpend())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr {{ sleeponexit: {=bool:?}, sleepdeep: {:?}, sleepdeeps: {:?}, sevonpend: {:?} }}",
            self.sleeponexit(),
            self.sleepdeep(),
            self.sleepdeeps(),
            self.sevonpend()
        )
    }
}
#[doc = "System Handler Control and State Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shcsr(pub u32);
impl Shcsr {
    #[doc = "MemManage exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn memfaultact(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "MemManage exception active."]
    #[inline(always)]
    pub const fn set_memfaultact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "BusFault exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn busfaultact(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BusFault exception active."]
    #[inline(always)]
    pub const fn set_busfaultact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HardFault exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn hardfaultact(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HardFault exception active."]
    #[inline(always)]
    pub const fn set_hardfaultact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "UsageFault exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn usgfaultact(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "UsageFault exception active."]
    #[inline(always)]
    pub const fn set_usgfaultact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SecureFault exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn securefaultact(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "SecureFault exception active."]
    #[inline(always)]
    pub const fn set_securefaultact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NMI exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn nmiact(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NMI exception active."]
    #[inline(always)]
    pub const fn set_nmiact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "SVCall active."]
    #[must_use]
    #[inline(always)]
    pub const fn svcallact(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SVCall active."]
    #[inline(always)]
    pub const fn set_svcallact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Debug monitor active."]
    #[must_use]
    #[inline(always)]
    pub const fn monitoract(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Debug monitor active."]
    #[inline(always)]
    pub const fn set_monitoract(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PendSV exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn pendsvact(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PendSV exception active."]
    #[inline(always)]
    pub const fn set_pendsvact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SysTick exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn systickact(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SysTick exception active."]
    #[inline(always)]
    pub const fn set_systickact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "UsageFault exception pending."]
    #[must_use]
    #[inline(always)]
    pub const fn usgfaultpended(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "UsageFault exception pending."]
    #[inline(always)]
    pub const fn set_usgfaultpended(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "MemManage exception pending."]
    #[must_use]
    #[inline(always)]
    pub const fn memfaultpended(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "MemManage exception pending."]
    #[inline(always)]
    pub const fn set_memfaultpended(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "BusFault exception pending."]
    #[must_use]
    #[inline(always)]
    pub const fn busfaultpended(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "BusFault exception pending."]
    #[inline(always)]
    pub const fn set_busfaultpended(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "SVCall pending."]
    #[must_use]
    #[inline(always)]
    pub const fn svcallpended(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "SVCall pending."]
    #[inline(always)]
    pub const fn set_svcallpended(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "MemManage enable."]
    #[must_use]
    #[inline(always)]
    pub const fn memfaultena(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "MemManage enable."]
    #[inline(always)]
    pub const fn set_memfaultena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "BusFault enable."]
    #[must_use]
    #[inline(always)]
    pub const fn busfaultena(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "BusFault enable."]
    #[inline(always)]
    pub const fn set_busfaultena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "UsageFault enable."]
    #[must_use]
    #[inline(always)]
    pub const fn usgfaultena(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "UsageFault enable."]
    #[inline(always)]
    pub const fn set_usgfaultena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SecureFault exception enable."]
    #[must_use]
    #[inline(always)]
    pub const fn securefaultena(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "SecureFault exception enable."]
    #[inline(always)]
    pub const fn set_securefaultena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "SecureFault exception pended state bit."]
    #[must_use]
    #[inline(always)]
    pub const fn securefaultpended(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "SecureFault exception pended state bit."]
    #[inline(always)]
    pub const fn set_securefaultpended(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "HardFault exception pended state."]
    #[must_use]
    #[inline(always)]
    pub const fn hardfaultpended(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "HardFault exception pended state."]
    #[inline(always)]
    pub const fn set_hardfaultpended(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Shcsr {
    #[inline(always)]
    fn default() -> Shcsr {
        Shcsr(0)
    }
}
impl core::fmt::Debug for Shcsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shcsr")
            .field("memfaultact", &self.memfaultact())
            .field("busfaultact", &self.busfaultact())
            .field("hardfaultact", &self.hardfaultact())
            .field("usgfaultact", &self.usgfaultact())
            .field("securefaultact", &self.securefaultact())
            .field("nmiact", &self.nmiact())
            .field("svcallact", &self.svcallact())
            .field("monitoract", &self.monitoract())
            .field("pendsvact", &self.pendsvact())
            .field("systickact", &self.systickact())
            .field("usgfaultpended", &self.usgfaultpended())
            .field("memfaultpended", &self.memfaultpended())
            .field("busfaultpended", &self.busfaultpended())
            .field("svcallpended", &self.svcallpended())
            .field("memfaultena", &self.memfaultena())
            .field("busfaultena", &self.busfaultena())
            .field("usgfaultena", &self.usgfaultena())
            .field("securefaultena", &self.securefaultena())
            .field("securefaultpended", &self.securefaultpended())
            .field("hardfaultpended", &self.hardfaultpended())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shcsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Shcsr {{ memfaultact: {=bool:?}, busfaultact: {=bool:?}, hardfaultact: {=bool:?}, usgfaultact: {=bool:?}, securefaultact: {=bool:?}, nmiact: {=bool:?}, svcallact: {=bool:?}, monitoract: {=bool:?}, pendsvact: {=bool:?}, systickact: {=bool:?}, usgfaultpended: {=bool:?}, memfaultpended: {=bool:?}, busfaultpended: {=bool:?}, svcallpended: {=bool:?}, memfaultena: {=bool:?}, busfaultena: {=bool:?}, usgfaultena: {=bool:?}, securefaultena: {=bool:?}, securefaultpended: {=bool:?}, hardfaultpended: {=bool:?} }}",
            self.memfaultact(),
            self.busfaultact(),
            self.hardfaultact(),
            self.usgfaultact(),
            self.securefaultact(),
            self.nmiact(),
            self.svcallact(),
            self.monitoract(),
            self.pendsvact(),
            self.systickact(),
            self.usgfaultpended(),
            self.memfaultpended(),
            self.busfaultpended(),
            self.svcallpended(),
            self.memfaultena(),
            self.busfaultena(),
            self.usgfaultena(),
            self.securefaultena(),
            self.securefaultpended(),
            self.hardfaultpended()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfhfnmins {
    #[doc = "BusFault, HardFault, and NMI are Secure."]
    SECURE = 0x0,
    #[doc = "BusFault and NMI are Non-secure and exceptions can target Non-secure HardFault."]
    NON_SECURE = 0x01,
}
impl Bfhfnmins {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfhfnmins {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfhfnmins {
    #[inline(always)]
    fn from(val: u8) -> Bfhfnmins {
        Bfhfnmins::from_bits(val)
    }
}
impl From<Bfhfnmins> for u8 {
    #[inline(always)]
    fn from(val: Bfhfnmins) -> u8 {
        Bfhfnmins::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cp11 {
    #[doc = "Access Denied. All accesses to the Floating-point Extension result in NOCP UsageFault."]
    ACCESS_DENIED = 0x0,
    #[doc = "Privileged access only. Unprivileged access to the Floatingpoint Extension result in NOCP UsageFault."]
    PRIVILEGED_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access. Full access rights to the Floatingpoint Extension."]
    FULL_ACCESS = 0x03,
}
impl Cp11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cp11 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cp11 {
    #[inline(always)]
    fn from(val: u8) -> Cp11 {
        Cp11::from_bits(val)
    }
}
impl From<Cp11> for u8 {
    #[inline(always)]
    fn from(val: Cp11) -> u8 {
        Cp11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpacrCp0 {
    #[doc = "Access Denied. Any attempted accesses to this coprocessor generates a NOCP UsageFault."]
    ACCESS_DENIED = 0x0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    PRIVILEGED_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access. Full access rights for this coprocessor."]
    FULL_ACCESS = 0x03,
}
impl CpacrCp0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpacrCp0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpacrCp0 {
    #[inline(always)]
    fn from(val: u8) -> CpacrCp0 {
        CpacrCp0::from_bits(val)
    }
}
impl From<CpacrCp0> for u8 {
    #[inline(always)]
    fn from(val: CpacrCp0) -> u8 {
        CpacrCp0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpacrCp1 {
    #[doc = "Access Denied. Any attempted accesses to this coprocessor generates a NOCP UsageFault."]
    ACCESS_DENIED = 0x0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    PRIVILEGED_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access. Full access rights for this coprocessor."]
    FULL_ACCESS = 0x03,
}
impl CpacrCp1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpacrCp1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpacrCp1 {
    #[inline(always)]
    fn from(val: u8) -> CpacrCp1 {
        CpacrCp1::from_bits(val)
    }
}
impl From<CpacrCp1> for u8 {
    #[inline(always)]
    fn from(val: CpacrCp1) -> u8 {
        CpacrCp1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpacrCp10 {
    #[doc = "Access Denied. All accesses to the Floating-point Extension result in NOCP UsageFault."]
    ACCESS_DENIED = 0x0,
    #[doc = "Privileged access only. Unprivileged access to the Floatingpoint Extension result in NOCP UsageFault."]
    PRIVILEGED_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access. Full access rights to the Floatingpoint Extension."]
    FULL_ACCESS = 0x03,
}
impl CpacrCp10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpacrCp10 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpacrCp10 {
    #[inline(always)]
    fn from(val: u8) -> CpacrCp10 {
        CpacrCp10::from_bits(val)
    }
}
impl From<CpacrCp10> for u8 {
    #[inline(always)]
    fn from(val: CpacrCp10) -> u8 {
        CpacrCp10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpacrCp2 {
    #[doc = "Access Denied. Any attempted accesses to this coprocessor generates a NOCP UsageFault."]
    ACCESS_DENIED = 0x0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    PRIVILEGED_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access. Full access rights for this coprocessor."]
    FULL_ACCESS = 0x03,
}
impl CpacrCp2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpacrCp2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpacrCp2 {
    #[inline(always)]
    fn from(val: u8) -> CpacrCp2 {
        CpacrCp2::from_bits(val)
    }
}
impl From<CpacrCp2> for u8 {
    #[inline(always)]
    fn from(val: CpacrCp2) -> u8 {
        CpacrCp2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpacrCp3 {
    #[doc = "Access Denied. Any attempted accesses to this coprocessor generates a NOCP UsageFault."]
    ACCESS_DENIED = 0x0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    PRIVILEGED_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access. Full access rights for this coprocessor."]
    FULL_ACCESS = 0x03,
}
impl CpacrCp3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpacrCp3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpacrCp3 {
    #[inline(always)]
    fn from(val: u8) -> CpacrCp3 {
        CpacrCp3::from_bits(val)
    }
}
impl From<CpacrCp3> for u8 {
    #[inline(always)]
    fn from(val: CpacrCp3) -> u8 {
        CpacrCp3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpacrCp4 {
    #[doc = "Access Denied. Any attempted accesses to this coprocessor generates a NOCP UsageFault."]
    ACCESS_DENIED = 0x0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    PRIVILEGED_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access. Full access rights for this coprocessor."]
    FULL_ACCESS = 0x03,
}
impl CpacrCp4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpacrCp4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpacrCp4 {
    #[inline(always)]
    fn from(val: u8) -> CpacrCp4 {
        CpacrCp4::from_bits(val)
    }
}
impl From<CpacrCp4> for u8 {
    #[inline(always)]
    fn from(val: CpacrCp4) -> u8 {
        CpacrCp4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpacrCp5 {
    #[doc = "Access Denied. Any attempted accesses to this coprocessor generates a NOCP UsageFault."]
    ACCESS_DENIED = 0x0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    PRIVILEGED_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access. Full access rights for this coprocessor."]
    FULL_ACCESS = 0x03,
}
impl CpacrCp5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpacrCp5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpacrCp5 {
    #[inline(always)]
    fn from(val: u8) -> CpacrCp5 {
        CpacrCp5::from_bits(val)
    }
}
impl From<CpacrCp5> for u8 {
    #[inline(always)]
    fn from(val: CpacrCp5) -> u8 {
        CpacrCp5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpacrCp6 {
    #[doc = "Access Denied. Any attempted accesses to this coprocessor generates a NOCP UsageFault."]
    ACCESS_DENIED = 0x0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    PRIVILEGED_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access. Full access rights for this coprocessor."]
    FULL_ACCESS = 0x03,
}
impl CpacrCp6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpacrCp6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpacrCp6 {
    #[inline(always)]
    fn from(val: u8) -> CpacrCp6 {
        CpacrCp6::from_bits(val)
    }
}
impl From<CpacrCp6> for u8 {
    #[inline(always)]
    fn from(val: CpacrCp6) -> u8 {
        CpacrCp6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpacrCp7 {
    #[doc = "Access Denied. Any attempted accesses to this coprocessor generates a NOCP UsageFault."]
    ACCESS_DENIED = 0x0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    PRIVILEGED_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Full access. Full access rights for this coprocessor."]
    FULL_ACCESS = 0x03,
}
impl CpacrCp7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpacrCp7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpacrCp7 {
    #[inline(always)]
    fn from(val: u8) -> CpacrCp7 {
        CpacrCp7::from_bits(val)
    }
}
impl From<CpacrCp7> for u8 {
    #[inline(always)]
    fn from(val: CpacrCp7) -> u8 {
        CpacrCp7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endianness {
    #[doc = "Little-endian."]
    LITTLE_ENDIAN = 0x0,
    #[doc = "Big-endian."]
    BIG_ENDIAN = 0x01,
}
impl Endianness {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endianness {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endianness {
    #[inline(always)]
    fn from(val: u8) -> Endianness {
        Endianness::from_bits(val)
    }
}
impl From<Endianness> for u8 {
    #[inline(always)]
    fn from(val: Endianness) -> u8 {
        Endianness::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pris {
    #[doc = "Priority ranges of Secure and Non-secure exceptions are identical."]
    SAME_PRIORITY = 0x0,
    #[doc = "Non-secure exceptions are de-prioritized."]
    SECURE_PRIORITIZED = 0x01,
}
impl Pris {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pris {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pris {
    #[inline(always)]
    fn from(val: u8) -> Pris {
        Pris::from_bits(val)
    }
}
impl From<Pris> for u8 {
    #[inline(always)]
    fn from(val: Pris) -> u8 {
        Pris::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sevonpend {
    #[doc = "Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded."]
    EXCLUDE_DISABLED_INTERRUPTS = 0x0,
    #[doc = "Enabled events and all interrupts, including disabled interrupts, can wakeup the processor."]
    INCLUDE_DISABLED_INTERRUPTS = 0x01,
}
impl Sevonpend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sevonpend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sevonpend {
    #[inline(always)]
    fn from(val: u8) -> Sevonpend {
        Sevonpend::from_bits(val)
    }
}
impl From<Sevonpend> for u8 {
    #[inline(always)]
    fn from(val: Sevonpend) -> u8 {
        Sevonpend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sleepdeep {
    #[doc = "Sleep."]
    SLEEP = 0x0,
    #[doc = "Deep sleep."]
    DEEP_SLEEP = 0x01,
}
impl Sleepdeep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sleepdeep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sleepdeep {
    #[inline(always)]
    fn from(val: u8) -> Sleepdeep {
        Sleepdeep::from_bits(val)
    }
}
impl From<Sleepdeep> for u8 {
    #[inline(always)]
    fn from(val: Sleepdeep) -> u8 {
        Sleepdeep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sleepdeeps {
    #[doc = "The SLEEPDEEP bit is accessible from both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "The SLEEPDEEP bit behaves as RAZ/WI when accessed from the Non-secure state."]
    SECURE_ONLY = 0x01,
}
impl Sleepdeeps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sleepdeeps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sleepdeeps {
    #[inline(always)]
    fn from(val: u8) -> Sleepdeeps {
        Sleepdeeps::from_bits(val)
    }
}
impl From<Sleepdeeps> for u8 {
    #[inline(always)]
    fn from(val: Sleepdeeps) -> u8 {
        Sleepdeeps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sysresetreq {
    #[doc = "Do not request a system reset."]
    NO_REQUEST = 0x0,
    #[doc = "Request a system reset."]
    REQUEST_RESET = 0x01,
}
impl Sysresetreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sysresetreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sysresetreq {
    #[inline(always)]
    fn from(val: u8) -> Sysresetreq {
        Sysresetreq::from_bits(val)
    }
}
impl From<Sysresetreq> for u8 {
    #[inline(always)]
    fn from(val: Sysresetreq) -> u8 {
        Sysresetreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sysresetreqs {
    #[doc = "SYSRESETREQ functionality is available to both Security states."]
    SECURE_AND_NON_SECURE = 0x0,
    #[doc = "SYSRESETREQ functionality is only available to Secure state."]
    SECURE_ONLY = 0x01,
}
impl Sysresetreqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sysresetreqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sysresetreqs {
    #[inline(always)]
    fn from(val: u8) -> Sysresetreqs {
        Sysresetreqs::from_bits(val)
    }
}
impl From<Sysresetreqs> for u8 {
    #[inline(always)]
    fn from(val: Sysresetreqs) -> u8 {
        Sysresetreqs::to_bits(val)
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
