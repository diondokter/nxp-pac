#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6a8c2aa 2026-01-27))"]
#[doc = "Security Attribution Unit."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sau {
    ptr: *mut u8,
}
unsafe impl Send for Sau {}
unsafe impl Sync for Sau {}
impl Sau {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Security Attribution Unit Control Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Security Attribution Unit Type Register."]
    #[inline(always)]
    pub const fn type_(self) -> crate::common::Reg<Type, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Security Attribution Unit Region Number Register."]
    #[inline(always)]
    pub const fn rnr(self) -> crate::common::Reg<Rnr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Security Attribution Unit Region Base Address Register."]
    #[inline(always)]
    pub const fn rbar(self) -> crate::common::Reg<Rbar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Security Attribution Unit Region Limit Address Register."]
    #[inline(always)]
    pub const fn rlar(self) -> crate::common::Reg<Rlar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Secure Fault Status Register."]
    #[inline(always)]
    pub const fn sfsr(self) -> crate::common::Reg<Sfsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Secure Fault Address Register."]
    #[inline(always)]
    pub const fn sfar(self) -> crate::common::Reg<Sfar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
}
#[doc = "Security Attribution Unit Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Enable. Enables the SAU. This bit is RAZ/WI when the Security Extension is implemented without an SAU region."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable. Enables the SAU. This bit is RAZ/WI when the Security Extension is implemented without an SAU region."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "All Non-secure."]
    #[must_use]
    #[inline(always)]
    pub const fn allns(&self) -> Allns {
        let val = (self.0 >> 1usize) & 0x01;
        Allns::from_bits(val as u8)
    }
    #[doc = "All Non-secure."]
    #[inline(always)]
    pub const fn set_allns(&mut self, val: Allns) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("enable", &self.enable())
            .field("allns", &self.allns())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ enable: {=bool:?}, allns: {:?} }}",
            self.enable(),
            self.allns()
        )
    }
}
#[doc = "Security Attribution Unit Region Base Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rbar(pub u32);
impl Rbar {
    #[doc = "Base address. Holds bits\\[31:5\\] of the base address for the selected SAU region. Bits\\[4:0\\] of the base address are defined as 0x00."]
    #[must_use]
    #[inline(always)]
    pub const fn baddr(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Base address. Holds bits\\[31:5\\] of the base address for the selected SAU region. Bits\\[4:0\\] of the base address are defined as 0x00."]
    #[inline(always)]
    pub const fn set_baddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for Rbar {
    #[inline(always)]
    fn default() -> Rbar {
        Rbar(0)
    }
}
impl core::fmt::Debug for Rbar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rbar")
            .field("baddr", &self.baddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rbar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rbar {{ baddr: {=u32:?} }}", self.baddr())
    }
}
#[doc = "Security Attribution Unit Region Limit Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rlar(pub u32);
impl Rlar {
    #[doc = "Enable. SAU region enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> RlarEnable {
        let val = (self.0 >> 0usize) & 0x01;
        RlarEnable::from_bits(val as u8)
    }
    #[doc = "Enable. SAU region enable."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: RlarEnable) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Non-secure callable. Controls whether Non-secure state is permitted to execute an SG instruction from this region."]
    #[must_use]
    #[inline(always)]
    pub const fn nsc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Non-secure callable. Controls whether Non-secure state is permitted to execute an SG instruction from this region."]
    #[inline(always)]
    pub const fn set_nsc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Limit address. Holds bits\\[31:5\\] of the limit address for the selected SAU region. Bits\\[4:0\\] of the limit address are defined as 0x1F."]
    #[must_use]
    #[inline(always)]
    pub const fn laddr(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Limit address. Holds bits\\[31:5\\] of the limit address for the selected SAU region. Bits\\[4:0\\] of the limit address are defined as 0x1F."]
    #[inline(always)]
    pub const fn set_laddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for Rlar {
    #[inline(always)]
    fn default() -> Rlar {
        Rlar(0)
    }
}
impl core::fmt::Debug for Rlar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rlar")
            .field("enable", &self.enable())
            .field("nsc", &self.nsc())
            .field("laddr", &self.laddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rlar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rlar {{ enable: {:?}, nsc: {=bool:?}, laddr: {=u32:?} }}",
            self.enable(),
            self.nsc(),
            self.laddr()
        )
    }
}
#[doc = "Security Attribution Unit Region Number Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rnr(pub u32);
impl Rnr {
    #[doc = "Region number."]
    #[must_use]
    #[inline(always)]
    pub const fn region(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Region number."]
    #[inline(always)]
    pub const fn set_region(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rnr {
    #[inline(always)]
    fn default() -> Rnr {
        Rnr(0)
    }
}
impl core::fmt::Debug for Rnr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rnr")
            .field("region", &self.region())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rnr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rnr {{ region: {=u8:?} }}", self.region())
    }
}
#[doc = "Secure Fault Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sfar(pub u32);
impl Sfar {
    #[doc = "When the SFARVALID bit of the SFSR is set to 1, this field holds the address of an access that caused an SAU violation."]
    #[must_use]
    #[inline(always)]
    pub const fn address(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "When the SFARVALID bit of the SFSR is set to 1, this field holds the address of an access that caused an SAU violation."]
    #[inline(always)]
    pub const fn set_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Sfar {
    #[inline(always)]
    fn default() -> Sfar {
        Sfar(0)
    }
}
impl core::fmt::Debug for Sfar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sfar")
            .field("address", &self.address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sfar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sfar {{ address: {=u32:?} }}", self.address())
    }
}
#[doc = "Secure Fault Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sfsr(pub u32);
impl Sfsr {
    #[doc = "Invalid entry point."]
    #[must_use]
    #[inline(always)]
    pub const fn invep(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Invalid entry point."]
    #[inline(always)]
    pub const fn set_invep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Invalid integrity signature flag."]
    #[must_use]
    #[inline(always)]
    pub const fn invis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Invalid integrity signature flag."]
    #[inline(always)]
    pub const fn set_invis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Invalid exception return flag."]
    #[must_use]
    #[inline(always)]
    pub const fn inver(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Invalid exception return flag."]
    #[inline(always)]
    pub const fn set_inver(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Attribution unit violation flag."]
    #[must_use]
    #[inline(always)]
    pub const fn auviol(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Attribution unit violation flag."]
    #[inline(always)]
    pub const fn set_auviol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Invalid transition flag."]
    #[must_use]
    #[inline(always)]
    pub const fn invtran(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Invalid transition flag."]
    #[inline(always)]
    pub const fn set_invtran(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Lazy state preservation error flag."]
    #[must_use]
    #[inline(always)]
    pub const fn lsperr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Lazy state preservation error flag."]
    #[inline(always)]
    pub const fn set_lsperr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Secure fault address valid."]
    #[must_use]
    #[inline(always)]
    pub const fn sfarvalid(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Secure fault address valid."]
    #[inline(always)]
    pub const fn set_sfarvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Lazy state error flag."]
    #[must_use]
    #[inline(always)]
    pub const fn lserr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Lazy state error flag."]
    #[inline(always)]
    pub const fn set_lserr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Sfsr {
    #[inline(always)]
    fn default() -> Sfsr {
        Sfsr(0)
    }
}
impl core::fmt::Debug for Sfsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sfsr")
            .field("invep", &self.invep())
            .field("invis", &self.invis())
            .field("inver", &self.inver())
            .field("auviol", &self.auviol())
            .field("invtran", &self.invtran())
            .field("lsperr", &self.lsperr())
            .field("sfarvalid", &self.sfarvalid())
            .field("lserr", &self.lserr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sfsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sfsr {{ invep: {=bool:?}, invis: {=bool:?}, inver: {=bool:?}, auviol: {=bool:?}, invtran: {=bool:?}, lsperr: {=bool:?}, sfarvalid: {=bool:?}, lserr: {=bool:?} }}",
            self.invep(),
            self.invis(),
            self.inver(),
            self.auviol(),
            self.invtran(),
            self.lsperr(),
            self.sfarvalid(),
            self.lserr()
        )
    }
}
#[doc = "Security Attribution Unit Type Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Type(pub u32);
impl Type {
    #[doc = "SAU regions. The number of implemented SAU regions."]
    #[must_use]
    #[inline(always)]
    pub const fn sregion(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SAU regions. The number of implemented SAU regions."]
    #[inline(always)]
    pub const fn set_sregion(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Type {
    #[inline(always)]
    fn default() -> Type {
        Type(0)
    }
}
impl core::fmt::Debug for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Type")
            .field("sregion", &self.sregion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Type {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Type {{ sregion: {=u8:?} }}", self.sregion())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Allns {
    #[doc = "Memory is marked as Secure and is not Non-secure callable."]
    SECURED_MEMORY = 0x0,
    #[doc = "Memory is marked as Non-secure."]
    NON_SECURED_MEMORY = 0x01,
}
impl Allns {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Allns {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Allns {
    #[inline(always)]
    fn from(val: u8) -> Allns {
        Allns::from_bits(val)
    }
}
impl From<Allns> for u8 {
    #[inline(always)]
    fn from(val: Allns) -> u8 {
        Allns::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RlarEnable {
    #[doc = "SAU region is enabled."]
    ENABLED = 0x0,
    #[doc = "SAU region is disabled."]
    DISABLED = 0x01,
}
impl RlarEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RlarEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RlarEnable {
    #[inline(always)]
    fn from(val: u8) -> RlarEnable {
        RlarEnable::from_bits(val)
    }
}
impl From<RlarEnable> for u8 {
    #[inline(always)]
    fn from(val: RlarEnable) -> u8 {
        RlarEnable::to_bits(val)
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
