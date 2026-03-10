#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6a8c2aa 2026-01-27))"]
#[doc = "Standard Counter or Timer."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer {
    ptr: *mut u8,
}
unsafe impl Send for Ctimer {}
unsafe impl Sync for Ctimer {}
impl Ctimer {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt."]
    #[inline(always)]
    pub const fn ir(self) -> crate::common::Reg<Ir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Timer Control."]
    #[inline(always)]
    pub const fn tcr(self) -> crate::common::Reg<Tcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Timer Counter."]
    #[inline(always)]
    pub const fn tc(self) -> crate::common::Reg<Tc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Prescale."]
    #[inline(always)]
    pub const fn pr(self) -> crate::common::Reg<Pr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Prescale Counter."]
    #[inline(always)]
    pub const fn pc(self) -> crate::common::Reg<Pc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Match Control."]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<Mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Match."]
    #[inline(always)]
    pub const fn mr(self, n: usize) -> crate::common::Reg<Mr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "Capture Control."]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Capture."]
    #[inline(always)]
    pub const fn cr(self, n: usize) -> crate::common::Reg<Cr, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize + n * 4usize) as _) }
    }
    #[doc = "External Match."]
    #[inline(always)]
    pub const fn emr(self) -> crate::common::Reg<Emr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Count Control."]
    #[inline(always)]
    pub const fn ctcr(self) -> crate::common::Reg<Ctcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "PWM Control."]
    #[inline(always)]
    pub const fn pwmc(self) -> crate::common::Reg<Pwmc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Match Shadow."]
    #[inline(always)]
    pub const fn msr(self, n: usize) -> crate::common::Reg<Msr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize + n * 4usize) as _) }
    }
}
#[doc = "Capture Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc = "Rising Edge of Capture Channel 0."]
    #[must_use]
    #[inline(always)]
    pub const fn cap0re(&self) -> Cap0re {
        let val = (self.0 >> 0usize) & 0x01;
        Cap0re::from_bits(val as u8)
    }
    #[doc = "Rising Edge of Capture Channel 0."]
    #[inline(always)]
    pub const fn set_cap0re(&mut self, val: Cap0re) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Falling Edge of Capture Channel 0."]
    #[must_use]
    #[inline(always)]
    pub const fn cap0fe(&self) -> Cap0fe {
        let val = (self.0 >> 1usize) & 0x01;
        Cap0fe::from_bits(val as u8)
    }
    #[doc = "Falling Edge of Capture Channel 0."]
    #[inline(always)]
    pub const fn set_cap0fe(&mut self, val: Cap0fe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Generate Interrupt on Channel 0 Capture Event."]
    #[must_use]
    #[inline(always)]
    pub const fn cap0i(&self) -> Cap0i {
        let val = (self.0 >> 2usize) & 0x01;
        Cap0i::from_bits(val as u8)
    }
    #[doc = "Generate Interrupt on Channel 0 Capture Event."]
    #[inline(always)]
    pub const fn set_cap0i(&mut self, val: Cap0i) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Rising Edge of Capture Channel 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cap1re(&self) -> Cap1re {
        let val = (self.0 >> 3usize) & 0x01;
        Cap1re::from_bits(val as u8)
    }
    #[doc = "Rising Edge of Capture Channel 1."]
    #[inline(always)]
    pub const fn set_cap1re(&mut self, val: Cap1re) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Falling Edge of Capture Channel 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cap1fe(&self) -> Cap1fe {
        let val = (self.0 >> 4usize) & 0x01;
        Cap1fe::from_bits(val as u8)
    }
    #[doc = "Falling Edge of Capture Channel 1."]
    #[inline(always)]
    pub const fn set_cap1fe(&mut self, val: Cap1fe) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Generate Interrupt on Channel 1 Capture Event."]
    #[must_use]
    #[inline(always)]
    pub const fn cap1i(&self) -> Cap1i {
        let val = (self.0 >> 5usize) & 0x01;
        Cap1i::from_bits(val as u8)
    }
    #[doc = "Generate Interrupt on Channel 1 Capture Event."]
    #[inline(always)]
    pub const fn set_cap1i(&mut self, val: Cap1i) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Rising Edge of Capture Channel 2."]
    #[must_use]
    #[inline(always)]
    pub const fn cap2re(&self) -> Cap2re {
        let val = (self.0 >> 6usize) & 0x01;
        Cap2re::from_bits(val as u8)
    }
    #[doc = "Rising Edge of Capture Channel 2."]
    #[inline(always)]
    pub const fn set_cap2re(&mut self, val: Cap2re) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Falling Edge of Capture Channel 2."]
    #[must_use]
    #[inline(always)]
    pub const fn cap2fe(&self) -> Cap2fe {
        let val = (self.0 >> 7usize) & 0x01;
        Cap2fe::from_bits(val as u8)
    }
    #[doc = "Falling Edge of Capture Channel 2."]
    #[inline(always)]
    pub const fn set_cap2fe(&mut self, val: Cap2fe) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Generate Interrupt on Channel 2 Capture Event."]
    #[must_use]
    #[inline(always)]
    pub const fn cap2i(&self) -> Cap2i {
        let val = (self.0 >> 8usize) & 0x01;
        Cap2i::from_bits(val as u8)
    }
    #[doc = "Generate Interrupt on Channel 2 Capture Event."]
    #[inline(always)]
    pub const fn set_cap2i(&mut self, val: Cap2i) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Rising Edge of Capture Channel 3."]
    #[must_use]
    #[inline(always)]
    pub const fn cap3re(&self) -> Cap3re {
        let val = (self.0 >> 9usize) & 0x01;
        Cap3re::from_bits(val as u8)
    }
    #[doc = "Rising Edge of Capture Channel 3."]
    #[inline(always)]
    pub const fn set_cap3re(&mut self, val: Cap3re) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Falling Edge of Capture Channel 3."]
    #[must_use]
    #[inline(always)]
    pub const fn cap3fe(&self) -> Cap3fe {
        let val = (self.0 >> 10usize) & 0x01;
        Cap3fe::from_bits(val as u8)
    }
    #[doc = "Falling Edge of Capture Channel 3."]
    #[inline(always)]
    pub const fn set_cap3fe(&mut self, val: Cap3fe) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Generate Interrupt on Channel 3 Capture Event."]
    #[must_use]
    #[inline(always)]
    pub const fn cap3i(&self) -> Cap3i {
        let val = (self.0 >> 11usize) & 0x01;
        Cap3i::from_bits(val as u8)
    }
    #[doc = "Generate Interrupt on Channel 3 Capture Event."]
    #[inline(always)]
    pub const fn set_cap3i(&mut self, val: Cap3i) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
}
impl Default for Ccr {
    #[inline(always)]
    fn default() -> Ccr {
        Ccr(0)
    }
}
impl core::fmt::Debug for Ccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr")
            .field("cap0re", &self.cap0re())
            .field("cap0fe", &self.cap0fe())
            .field("cap0i", &self.cap0i())
            .field("cap1re", &self.cap1re())
            .field("cap1fe", &self.cap1fe())
            .field("cap1i", &self.cap1i())
            .field("cap2re", &self.cap2re())
            .field("cap2fe", &self.cap2fe())
            .field("cap2i", &self.cap2i())
            .field("cap3re", &self.cap3re())
            .field("cap3fe", &self.cap3fe())
            .field("cap3i", &self.cap3i())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccr {{ cap0re: {:?}, cap0fe: {:?}, cap0i: {:?}, cap1re: {:?}, cap1fe: {:?}, cap1i: {:?}, cap2re: {:?}, cap2fe: {:?}, cap2i: {:?}, cap3re: {:?}, cap3fe: {:?}, cap3i: {:?} }}",
            self.cap0re(),
            self.cap0fe(),
            self.cap0i(),
            self.cap1re(),
            self.cap1fe(),
            self.cap1i(),
            self.cap2re(),
            self.cap2fe(),
            self.cap2i(),
            self.cap3re(),
            self.cap3fe(),
            self.cap3i()
        )
    }
}
#[doc = "Capture."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Timer Counter Capture Value."]
    #[must_use]
    #[inline(always)]
    pub const fn cap(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timer Counter Capture Value."]
    #[inline(always)]
    pub const fn set_cap(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
        f.debug_struct("Cr").field("cap", &self.cap()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cr {{ cap: {=u32:?} }}", self.cap())
    }
}
#[doc = "Count Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctcr(pub u32);
impl Ctcr {
    #[doc = "Counter Timer Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmode(&self) -> Ctmode {
        let val = (self.0 >> 0usize) & 0x03;
        Ctmode::from_bits(val as u8)
    }
    #[doc = "Counter Timer Mode."]
    #[inline(always)]
    pub const fn set_ctmode(&mut self, val: Ctmode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Count Input Select."]
    #[must_use]
    #[inline(always)]
    pub const fn cinsel(&self) -> Cinsel {
        let val = (self.0 >> 2usize) & 0x03;
        Cinsel::from_bits(val as u8)
    }
    #[doc = "Count Input Select."]
    #[inline(always)]
    pub const fn set_cinsel(&mut self, val: Cinsel) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Capture Channel Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn encc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Channel Enable."]
    #[inline(always)]
    pub const fn set_encc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Edge Select."]
    #[must_use]
    #[inline(always)]
    pub const fn selcc(&self) -> Selcc {
        let val = (self.0 >> 5usize) & 0x07;
        Selcc::from_bits(val as u8)
    }
    #[doc = "Edge Select."]
    #[inline(always)]
    pub const fn set_selcc(&mut self, val: Selcc) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
    }
}
impl Default for Ctcr {
    #[inline(always)]
    fn default() -> Ctcr {
        Ctcr(0)
    }
}
impl core::fmt::Debug for Ctcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctcr")
            .field("ctmode", &self.ctmode())
            .field("cinsel", &self.cinsel())
            .field("encc", &self.encc())
            .field("selcc", &self.selcc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctcr {{ ctmode: {:?}, cinsel: {:?}, encc: {=bool:?}, selcc: {:?} }}",
            self.ctmode(),
            self.cinsel(),
            self.encc(),
            self.selcc()
        )
    }
}
#[doc = "External Match."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emr(pub u32);
impl Emr {
    #[doc = "External Match 0."]
    #[must_use]
    #[inline(always)]
    pub const fn em0(&self) -> Em0 {
        let val = (self.0 >> 0usize) & 0x01;
        Em0::from_bits(val as u8)
    }
    #[doc = "External Match 0."]
    #[inline(always)]
    pub const fn set_em0(&mut self, val: Em0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "External Match 1."]
    #[must_use]
    #[inline(always)]
    pub const fn em1(&self) -> Em1 {
        let val = (self.0 >> 1usize) & 0x01;
        Em1::from_bits(val as u8)
    }
    #[doc = "External Match 1."]
    #[inline(always)]
    pub const fn set_em1(&mut self, val: Em1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "External Match 2."]
    #[must_use]
    #[inline(always)]
    pub const fn em2(&self) -> Em2 {
        let val = (self.0 >> 2usize) & 0x01;
        Em2::from_bits(val as u8)
    }
    #[doc = "External Match 2."]
    #[inline(always)]
    pub const fn set_em2(&mut self, val: Em2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "External Match 3."]
    #[must_use]
    #[inline(always)]
    pub const fn em3(&self) -> Em3 {
        let val = (self.0 >> 3usize) & 0x01;
        Em3::from_bits(val as u8)
    }
    #[doc = "External Match 3."]
    #[inline(always)]
    pub const fn set_em3(&mut self, val: Em3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "External Match Control 0."]
    #[must_use]
    #[inline(always)]
    pub const fn emc0(&self) -> Emc0 {
        let val = (self.0 >> 4usize) & 0x03;
        Emc0::from_bits(val as u8)
    }
    #[doc = "External Match Control 0."]
    #[inline(always)]
    pub const fn set_emc0(&mut self, val: Emc0) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "External Match Control 1."]
    #[must_use]
    #[inline(always)]
    pub const fn emc1(&self) -> Emc1 {
        let val = (self.0 >> 6usize) & 0x03;
        Emc1::from_bits(val as u8)
    }
    #[doc = "External Match Control 1."]
    #[inline(always)]
    pub const fn set_emc1(&mut self, val: Emc1) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "External Match Control 2."]
    #[must_use]
    #[inline(always)]
    pub const fn emc2(&self) -> Emc2 {
        let val = (self.0 >> 8usize) & 0x03;
        Emc2::from_bits(val as u8)
    }
    #[doc = "External Match Control 2."]
    #[inline(always)]
    pub const fn set_emc2(&mut self, val: Emc2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "External Match Control 3."]
    #[must_use]
    #[inline(always)]
    pub const fn emc3(&self) -> Emc3 {
        let val = (self.0 >> 10usize) & 0x03;
        Emc3::from_bits(val as u8)
    }
    #[doc = "External Match Control 3."]
    #[inline(always)]
    pub const fn set_emc3(&mut self, val: Emc3) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
}
impl Default for Emr {
    #[inline(always)]
    fn default() -> Emr {
        Emr(0)
    }
}
impl core::fmt::Debug for Emr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Emr")
            .field("em0", &self.em0())
            .field("em1", &self.em1())
            .field("em2", &self.em2())
            .field("em3", &self.em3())
            .field("emc0", &self.emc0())
            .field("emc1", &self.emc1())
            .field("emc2", &self.emc2())
            .field("emc3", &self.emc3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Emr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Emr {{ em0: {:?}, em1: {:?}, em2: {:?}, em3: {:?}, emc0: {:?}, emc1: {:?}, emc2: {:?}, emc3: {:?} }}",
            self.em0(),
            self.em1(),
            self.em2(),
            self.em3(),
            self.emc0(),
            self.emc1(),
            self.emc2(),
            self.emc3()
        )
    }
}
#[doc = "Interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ir(pub u32);
impl Ir {
    #[doc = "Interrupt Flag for Match Channel 0 Event."]
    #[must_use]
    #[inline(always)]
    pub const fn mr0int(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag for Match Channel 0 Event."]
    #[inline(always)]
    pub const fn set_mr0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Flag for Match Channel 1 Event."]
    #[must_use]
    #[inline(always)]
    pub const fn mr1int(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag for Match Channel 1 Event."]
    #[inline(always)]
    pub const fn set_mr1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt Flag for Match Channel 2 Event."]
    #[must_use]
    #[inline(always)]
    pub const fn mr2int(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag for Match Channel 2 Event."]
    #[inline(always)]
    pub const fn set_mr2int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt Flag for Match Channel 3 Event."]
    #[must_use]
    #[inline(always)]
    pub const fn mr3int(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag for Match Channel 3 Event."]
    #[inline(always)]
    pub const fn set_mr3int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt Flag for Capture Channel 0 Event."]
    #[must_use]
    #[inline(always)]
    pub const fn cr0int(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag for Capture Channel 0 Event."]
    #[inline(always)]
    pub const fn set_cr0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt Flag for Capture Channel 1 Event."]
    #[must_use]
    #[inline(always)]
    pub const fn cr1int(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag for Capture Channel 1 Event."]
    #[inline(always)]
    pub const fn set_cr1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt Flag for Capture Channel 2 Event."]
    #[must_use]
    #[inline(always)]
    pub const fn cr2int(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag for Capture Channel 2 Event."]
    #[inline(always)]
    pub const fn set_cr2int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt Flag for Capture Channel 3 Event."]
    #[must_use]
    #[inline(always)]
    pub const fn cr3int(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag for Capture Channel 3 Event."]
    #[inline(always)]
    pub const fn set_cr3int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Ir {
    #[inline(always)]
    fn default() -> Ir {
        Ir(0)
    }
}
impl core::fmt::Debug for Ir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ir")
            .field("mr0int", &self.mr0int())
            .field("mr1int", &self.mr1int())
            .field("mr2int", &self.mr2int())
            .field("mr3int", &self.mr3int())
            .field("cr0int", &self.cr0int())
            .field("cr1int", &self.cr1int())
            .field("cr2int", &self.cr2int())
            .field("cr3int", &self.cr3int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ir {{ mr0int: {=bool:?}, mr1int: {=bool:?}, mr2int: {=bool:?}, mr3int: {=bool:?}, cr0int: {=bool:?}, cr1int: {=bool:?}, cr2int: {=bool:?}, cr3int: {=bool:?} }}",
            self.mr0int(),
            self.mr1int(),
            self.mr2int(),
            self.mr3int(),
            self.cr0int(),
            self.cr1int(),
            self.cr2int(),
            self.cr3int()
        )
    }
}
#[doc = "Match Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc = "Interrupt on MR0."]
    #[must_use]
    #[inline(always)]
    pub const fn mr0i(&self) -> Mr0i {
        let val = (self.0 >> 0usize) & 0x01;
        Mr0i::from_bits(val as u8)
    }
    #[doc = "Interrupt on MR0."]
    #[inline(always)]
    pub const fn set_mr0i(&mut self, val: Mr0i) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Reset on MR0."]
    #[must_use]
    #[inline(always)]
    pub const fn mr0r(&self) -> Mr0r {
        let val = (self.0 >> 1usize) & 0x01;
        Mr0r::from_bits(val as u8)
    }
    #[doc = "Reset on MR0."]
    #[inline(always)]
    pub const fn set_mr0r(&mut self, val: Mr0r) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Stop on MR0."]
    #[must_use]
    #[inline(always)]
    pub const fn mr0s(&self) -> Mr0s {
        let val = (self.0 >> 2usize) & 0x01;
        Mr0s::from_bits(val as u8)
    }
    #[doc = "Stop on MR0."]
    #[inline(always)]
    pub const fn set_mr0s(&mut self, val: Mr0s) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt on MR1."]
    #[must_use]
    #[inline(always)]
    pub const fn mr1i(&self) -> Mr1i {
        let val = (self.0 >> 3usize) & 0x01;
        Mr1i::from_bits(val as u8)
    }
    #[doc = "Interrupt on MR1."]
    #[inline(always)]
    pub const fn set_mr1i(&mut self, val: Mr1i) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Reset on MR1."]
    #[must_use]
    #[inline(always)]
    pub const fn mr1r(&self) -> Mr1r {
        let val = (self.0 >> 4usize) & 0x01;
        Mr1r::from_bits(val as u8)
    }
    #[doc = "Reset on MR1."]
    #[inline(always)]
    pub const fn set_mr1r(&mut self, val: Mr1r) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Stop on MR1."]
    #[must_use]
    #[inline(always)]
    pub const fn mr1s(&self) -> Mr1s {
        let val = (self.0 >> 5usize) & 0x01;
        Mr1s::from_bits(val as u8)
    }
    #[doc = "Stop on MR1."]
    #[inline(always)]
    pub const fn set_mr1s(&mut self, val: Mr1s) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt on MR2."]
    #[must_use]
    #[inline(always)]
    pub const fn mr2i(&self) -> Mr2i {
        let val = (self.0 >> 6usize) & 0x01;
        Mr2i::from_bits(val as u8)
    }
    #[doc = "Interrupt on MR2."]
    #[inline(always)]
    pub const fn set_mr2i(&mut self, val: Mr2i) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Reset on MR2."]
    #[must_use]
    #[inline(always)]
    pub const fn mr2r(&self) -> Mr2r {
        let val = (self.0 >> 7usize) & 0x01;
        Mr2r::from_bits(val as u8)
    }
    #[doc = "Reset on MR2."]
    #[inline(always)]
    pub const fn set_mr2r(&mut self, val: Mr2r) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Stop on MR2."]
    #[must_use]
    #[inline(always)]
    pub const fn mr2s(&self) -> Mr2s {
        let val = (self.0 >> 8usize) & 0x01;
        Mr2s::from_bits(val as u8)
    }
    #[doc = "Stop on MR2."]
    #[inline(always)]
    pub const fn set_mr2s(&mut self, val: Mr2s) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt on MR3."]
    #[must_use]
    #[inline(always)]
    pub const fn mr3i(&self) -> Mr3i {
        let val = (self.0 >> 9usize) & 0x01;
        Mr3i::from_bits(val as u8)
    }
    #[doc = "Interrupt on MR3."]
    #[inline(always)]
    pub const fn set_mr3i(&mut self, val: Mr3i) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Reset on MR3."]
    #[must_use]
    #[inline(always)]
    pub const fn mr3r(&self) -> Mr3r {
        let val = (self.0 >> 10usize) & 0x01;
        Mr3r::from_bits(val as u8)
    }
    #[doc = "Reset on MR3."]
    #[inline(always)]
    pub const fn set_mr3r(&mut self, val: Mr3r) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Stop on MR3."]
    #[must_use]
    #[inline(always)]
    pub const fn mr3s(&self) -> Mr3s {
        let val = (self.0 >> 11usize) & 0x01;
        Mr3s::from_bits(val as u8)
    }
    #[doc = "Stop on MR3."]
    #[inline(always)]
    pub const fn set_mr3s(&mut self, val: Mr3s) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Reload MR."]
    #[must_use]
    #[inline(always)]
    pub const fn mr0rl(&self) -> Mr0rl {
        let val = (self.0 >> 24usize) & 0x01;
        Mr0rl::from_bits(val as u8)
    }
    #[doc = "Reload MR."]
    #[inline(always)]
    pub const fn set_mr0rl(&mut self, val: Mr0rl) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Reload MR."]
    #[must_use]
    #[inline(always)]
    pub const fn mr1rl(&self) -> Mr1rl {
        let val = (self.0 >> 25usize) & 0x01;
        Mr1rl::from_bits(val as u8)
    }
    #[doc = "Reload MR."]
    #[inline(always)]
    pub const fn set_mr1rl(&mut self, val: Mr1rl) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Reload MR."]
    #[must_use]
    #[inline(always)]
    pub const fn mr2rl(&self) -> Mr2rl {
        let val = (self.0 >> 26usize) & 0x01;
        Mr2rl::from_bits(val as u8)
    }
    #[doc = "Reload MR."]
    #[inline(always)]
    pub const fn set_mr2rl(&mut self, val: Mr2rl) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Reload MR."]
    #[must_use]
    #[inline(always)]
    pub const fn mr3rl(&self) -> Mr3rl {
        let val = (self.0 >> 27usize) & 0x01;
        Mr3rl::from_bits(val as u8)
    }
    #[doc = "Reload MR."]
    #[inline(always)]
    pub const fn set_mr3rl(&mut self, val: Mr3rl) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Mcr {
    #[inline(always)]
    fn default() -> Mcr {
        Mcr(0)
    }
}
impl core::fmt::Debug for Mcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mcr")
            .field("mr0i", &self.mr0i())
            .field("mr0r", &self.mr0r())
            .field("mr0s", &self.mr0s())
            .field("mr1i", &self.mr1i())
            .field("mr1r", &self.mr1r())
            .field("mr1s", &self.mr1s())
            .field("mr2i", &self.mr2i())
            .field("mr2r", &self.mr2r())
            .field("mr2s", &self.mr2s())
            .field("mr3i", &self.mr3i())
            .field("mr3r", &self.mr3r())
            .field("mr3s", &self.mr3s())
            .field("mr0rl", &self.mr0rl())
            .field("mr1rl", &self.mr1rl())
            .field("mr2rl", &self.mr2rl())
            .field("mr3rl", &self.mr3rl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mcr {{ mr0i: {:?}, mr0r: {:?}, mr0s: {:?}, mr1i: {:?}, mr1r: {:?}, mr1s: {:?}, mr2i: {:?}, mr2r: {:?}, mr2s: {:?}, mr3i: {:?}, mr3r: {:?}, mr3s: {:?}, mr0rl: {:?}, mr1rl: {:?}, mr2rl: {:?}, mr3rl: {:?} }}",
            self.mr0i(),
            self.mr0r(),
            self.mr0s(),
            self.mr1i(),
            self.mr1r(),
            self.mr1s(),
            self.mr2i(),
            self.mr2r(),
            self.mr2s(),
            self.mr3i(),
            self.mr3r(),
            self.mr3s(),
            self.mr0rl(),
            self.mr1rl(),
            self.mr2rl(),
            self.mr3rl()
        )
    }
}
#[doc = "Match."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mr(pub u32);
impl Mr {
    #[doc = "Timer Counter Match Value."]
    #[must_use]
    #[inline(always)]
    pub const fn match_(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timer Counter Match Value."]
    #[inline(always)]
    pub const fn set_match_(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mr {
    #[inline(always)]
    fn default() -> Mr {
        Mr(0)
    }
}
impl core::fmt::Debug for Mr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mr")
            .field("match_", &self.match_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mr {{ match_: {=u32:?} }}", self.match_())
    }
}
#[doc = "Match Shadow."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msr(pub u32);
impl Msr {
    #[doc = "Timer Counter Match Shadow Value."]
    #[must_use]
    #[inline(always)]
    pub const fn match_shadow(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timer Counter Match Shadow Value."]
    #[inline(always)]
    pub const fn set_match_shadow(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Msr {
    #[inline(always)]
    fn default() -> Msr {
        Msr(0)
    }
}
impl core::fmt::Debug for Msr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Msr")
            .field("match_shadow", &self.match_shadow())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Msr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Msr {{ match_shadow: {=u32:?} }}", self.match_shadow())
    }
}
#[doc = "Prescale Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pc(pub u32);
impl Pc {
    #[doc = "Prescale Counter Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pcval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Prescale Counter Value."]
    #[inline(always)]
    pub const fn set_pcval(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pc {
    #[inline(always)]
    fn default() -> Pc {
        Pc(0)
    }
}
impl core::fmt::Debug for Pc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pc").field("pcval", &self.pcval()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pc {{ pcval: {=u32:?} }}", self.pcval())
    }
}
#[doc = "Prescale."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pr(pub u32);
impl Pr {
    #[doc = "Prescale Reload Value."]
    #[must_use]
    #[inline(always)]
    pub const fn prval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Prescale Reload Value."]
    #[inline(always)]
    pub const fn set_prval(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pr {
    #[inline(always)]
    fn default() -> Pr {
        Pr(0)
    }
}
impl core::fmt::Debug for Pr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pr").field("prval", &self.prval()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pr {{ prval: {=u32:?} }}", self.prval())
    }
}
#[doc = "PWM Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwmc(pub u32);
impl Pwmc {
    #[doc = "PWM Mode Enable for Channel 0."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmen0(&self) -> Pwmen0 {
        let val = (self.0 >> 0usize) & 0x01;
        Pwmen0::from_bits(val as u8)
    }
    #[doc = "PWM Mode Enable for Channel 0."]
    #[inline(always)]
    pub const fn set_pwmen0(&mut self, val: Pwmen0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "PWM Mode Enable for Channel 1."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmen1(&self) -> Pwmen1 {
        let val = (self.0 >> 1usize) & 0x01;
        Pwmen1::from_bits(val as u8)
    }
    #[doc = "PWM Mode Enable for Channel 1."]
    #[inline(always)]
    pub const fn set_pwmen1(&mut self, val: Pwmen1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "PWM Mode Enable for Channel 2."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmen2(&self) -> Pwmen2 {
        let val = (self.0 >> 2usize) & 0x01;
        Pwmen2::from_bits(val as u8)
    }
    #[doc = "PWM Mode Enable for Channel 2."]
    #[inline(always)]
    pub const fn set_pwmen2(&mut self, val: Pwmen2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "PWM Mode Enable for Channel 3."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmen3(&self) -> Pwmen3 {
        let val = (self.0 >> 3usize) & 0x01;
        Pwmen3::from_bits(val as u8)
    }
    #[doc = "PWM Mode Enable for Channel 3."]
    #[inline(always)]
    pub const fn set_pwmen3(&mut self, val: Pwmen3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Pwmc {
    #[inline(always)]
    fn default() -> Pwmc {
        Pwmc(0)
    }
}
impl core::fmt::Debug for Pwmc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwmc")
            .field("pwmen0", &self.pwmen0())
            .field("pwmen1", &self.pwmen1())
            .field("pwmen2", &self.pwmen2())
            .field("pwmen3", &self.pwmen3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwmc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pwmc {{ pwmen0: {:?}, pwmen1: {:?}, pwmen2: {:?}, pwmen3: {:?} }}",
            self.pwmen0(),
            self.pwmen1(),
            self.pwmen2(),
            self.pwmen3()
        )
    }
}
#[doc = "Timer Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tc(pub u32);
impl Tc {
    #[doc = "Timer Counter Value."]
    #[must_use]
    #[inline(always)]
    pub const fn tcval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timer Counter Value."]
    #[inline(always)]
    pub const fn set_tcval(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tc {
    #[inline(always)]
    fn default() -> Tc {
        Tc(0)
    }
}
impl core::fmt::Debug for Tc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tc").field("tcval", &self.tcval()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tc {{ tcval: {=u32:?} }}", self.tcval())
    }
}
#[doc = "Timer Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr(pub u32);
impl Tcr {
    #[doc = "Counter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Counter Enable."]
    #[inline(always)]
    pub const fn set_cen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Counter Reset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn crst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Counter Reset Enable."]
    #[inline(always)]
    pub const fn set_crst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Allow Global Count Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn agcen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Global Count Enable."]
    #[inline(always)]
    pub const fn set_agcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Allow Trigger Count Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn atcen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Trigger Count Enable."]
    #[inline(always)]
    pub const fn set_atcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
            .field("cen", &self.cen())
            .field("crst", &self.crst())
            .field("agcen", &self.agcen())
            .field("atcen", &self.atcen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcr {{ cen: {=bool:?}, crst: {=bool:?}, agcen: {=bool:?}, atcen: {=bool:?} }}",
            self.cen(),
            self.crst(),
            self.agcen(),
            self.atcen()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cap0fe {
    #[doc = "Does not load."]
    CAP0FE_0 = 0x0,
    #[doc = "Loads."]
    CAPOFE_1 = 0x01,
}
impl Cap0fe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cap0fe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cap0fe {
    #[inline(always)]
    fn from(val: u8) -> Cap0fe {
        Cap0fe::from_bits(val)
    }
}
impl From<Cap0fe> for u8 {
    #[inline(always)]
    fn from(val: Cap0fe) -> u8 {
        Cap0fe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cap0i {
    #[doc = "Does not generate."]
    CAP0I_0 = 0x0,
    #[doc = "Generates."]
    CAPOI_1 = 0x01,
}
impl Cap0i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cap0i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cap0i {
    #[inline(always)]
    fn from(val: u8) -> Cap0i {
        Cap0i::from_bits(val)
    }
}
impl From<Cap0i> for u8 {
    #[inline(always)]
    fn from(val: Cap0i) -> u8 {
        Cap0i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cap0re {
    #[doc = "Does not load."]
    CAP0RE_0 = 0x0,
    #[doc = "Loads."]
    CAPORE_1 = 0x01,
}
impl Cap0re {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cap0re {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cap0re {
    #[inline(always)]
    fn from(val: u8) -> Cap0re {
        Cap0re::from_bits(val)
    }
}
impl From<Cap0re> for u8 {
    #[inline(always)]
    fn from(val: Cap0re) -> u8 {
        Cap0re::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cap1fe {
    #[doc = "Does not load."]
    CAP1FE_0 = 0x0,
    #[doc = "Loads."]
    CAP1FE_1 = 0x01,
}
impl Cap1fe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cap1fe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cap1fe {
    #[inline(always)]
    fn from(val: u8) -> Cap1fe {
        Cap1fe::from_bits(val)
    }
}
impl From<Cap1fe> for u8 {
    #[inline(always)]
    fn from(val: Cap1fe) -> u8 {
        Cap1fe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cap1i {
    #[doc = "Does not generates."]
    CAP1I_0 = 0x0,
    #[doc = "Generates."]
    CAP1I_1 = 0x01,
}
impl Cap1i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cap1i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cap1i {
    #[inline(always)]
    fn from(val: u8) -> Cap1i {
        Cap1i::from_bits(val)
    }
}
impl From<Cap1i> for u8 {
    #[inline(always)]
    fn from(val: Cap1i) -> u8 {
        Cap1i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cap1re {
    #[doc = "Does not load."]
    CAP1RE_0 = 0x0,
    #[doc = "Loads."]
    CAP1RE_1 = 0x01,
}
impl Cap1re {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cap1re {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cap1re {
    #[inline(always)]
    fn from(val: u8) -> Cap1re {
        Cap1re::from_bits(val)
    }
}
impl From<Cap1re> for u8 {
    #[inline(always)]
    fn from(val: Cap1re) -> u8 {
        Cap1re::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cap2fe {
    #[doc = "Does not load."]
    CAP2FE_0 = 0x0,
    #[doc = "Loads."]
    CAP2FE_1 = 0x01,
}
impl Cap2fe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cap2fe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cap2fe {
    #[inline(always)]
    fn from(val: u8) -> Cap2fe {
        Cap2fe::from_bits(val)
    }
}
impl From<Cap2fe> for u8 {
    #[inline(always)]
    fn from(val: Cap2fe) -> u8 {
        Cap2fe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cap2i {
    #[doc = "Does not generate."]
    CAP2I_0 = 0x0,
    #[doc = "Generates."]
    CAP2I_1 = 0x01,
}
impl Cap2i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cap2i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cap2i {
    #[inline(always)]
    fn from(val: u8) -> Cap2i {
        Cap2i::from_bits(val)
    }
}
impl From<Cap2i> for u8 {
    #[inline(always)]
    fn from(val: Cap2i) -> u8 {
        Cap2i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cap2re {
    #[doc = "Does not load."]
    CAP2RE_0 = 0x0,
    #[doc = "Loads."]
    CAP2RE_1 = 0x01,
}
impl Cap2re {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cap2re {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cap2re {
    #[inline(always)]
    fn from(val: u8) -> Cap2re {
        Cap2re::from_bits(val)
    }
}
impl From<Cap2re> for u8 {
    #[inline(always)]
    fn from(val: Cap2re) -> u8 {
        Cap2re::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cap3fe {
    #[doc = "Does not load."]
    CAP3FE_0 = 0x0,
    #[doc = "Loads."]
    CAP3FE_1 = 0x01,
}
impl Cap3fe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cap3fe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cap3fe {
    #[inline(always)]
    fn from(val: u8) -> Cap3fe {
        Cap3fe::from_bits(val)
    }
}
impl From<Cap3fe> for u8 {
    #[inline(always)]
    fn from(val: Cap3fe) -> u8 {
        Cap3fe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cap3i {
    #[doc = "Does not generate."]
    CAP3I_0 = 0x0,
    #[doc = "Generates."]
    CAP3I_1 = 0x01,
}
impl Cap3i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cap3i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cap3i {
    #[inline(always)]
    fn from(val: u8) -> Cap3i {
        Cap3i::from_bits(val)
    }
}
impl From<Cap3i> for u8 {
    #[inline(always)]
    fn from(val: Cap3i) -> u8 {
        Cap3i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cap3re {
    #[doc = "Does not load."]
    CAP3RE_0 = 0x0,
    #[doc = "Loads."]
    CAP3RE_1 = 0x01,
}
impl Cap3re {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cap3re {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cap3re {
    #[inline(always)]
    fn from(val: u8) -> Cap3re {
        Cap3re::from_bits(val)
    }
}
impl From<Cap3re> for u8 {
    #[inline(always)]
    fn from(val: Cap3re) -> u8 {
        Cap3re::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cinsel {
    #[doc = "Channel 0, CAPn\\[0\\] for CTIMERn."]
    CHANNEL_0 = 0x0,
    #[doc = "Channel 1, CAPn\\[1\\] for CTIMERn."]
    CHANNEL_1 = 0x01,
    #[doc = "Channel 2, CAPn\\[2\\] for CTIMERn."]
    CHANNEL_2 = 0x02,
    #[doc = "Channel 3, CAPn\\[3\\] for CTIMERn."]
    CHANNEL_3 = 0x03,
}
impl Cinsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cinsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cinsel {
    #[inline(always)]
    fn from(val: u8) -> Cinsel {
        Cinsel::from_bits(val)
    }
}
impl From<Cinsel> for u8 {
    #[inline(always)]
    fn from(val: Cinsel) -> u8 {
        Cinsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctmode {
    #[doc = "Timer mode."]
    TIMER = 0x0,
    #[doc = "Counter mode rising edge."]
    COUNTER_RISING_EDGE = 0x01,
    #[doc = "Counter mode falling edge."]
    COUNTER_FALLING_EDGE = 0x02,
    #[doc = "Counter mode dual edge."]
    COUNTER_DUAL_EDGE = 0x03,
}
impl Ctmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctmode {
    #[inline(always)]
    fn from(val: u8) -> Ctmode {
        Ctmode::from_bits(val)
    }
}
impl From<Ctmode> for u8 {
    #[inline(always)]
    fn from(val: Ctmode) -> u8 {
        Ctmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Em0 {
    #[doc = "Low."]
    CLEAR = 0x0,
    #[doc = "High."]
    SET = 0x01,
}
impl Em0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Em0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Em0 {
    #[inline(always)]
    fn from(val: u8) -> Em0 {
        Em0::from_bits(val)
    }
}
impl From<Em0> for u8 {
    #[inline(always)]
    fn from(val: Em0) -> u8 {
        Em0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Em1 {
    #[doc = "Low."]
    CLEAR = 0x0,
    #[doc = "High."]
    SET = 0x01,
}
impl Em1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Em1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Em1 {
    #[inline(always)]
    fn from(val: u8) -> Em1 {
        Em1::from_bits(val)
    }
}
impl From<Em1> for u8 {
    #[inline(always)]
    fn from(val: Em1) -> u8 {
        Em1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Em2 {
    #[doc = "Low."]
    CLEAR = 0x0,
    #[doc = "High."]
    SET = 0x01,
}
impl Em2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Em2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Em2 {
    #[inline(always)]
    fn from(val: u8) -> Em2 {
        Em2::from_bits(val)
    }
}
impl From<Em2> for u8 {
    #[inline(always)]
    fn from(val: Em2) -> u8 {
        Em2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Em3 {
    #[doc = "Low."]
    CLEAR = 0x0,
    #[doc = "High."]
    SET = 0x01,
}
impl Em3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Em3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Em3 {
    #[inline(always)]
    fn from(val: u8) -> Em3 {
        Em3::from_bits(val)
    }
}
impl From<Em3> for u8 {
    #[inline(always)]
    fn from(val: Em3) -> u8 {
        Em3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emc0 {
    #[doc = "Does nothing."]
    DO_NOTHING = 0x0,
    #[doc = "Goes low."]
    CLEAR = 0x01,
    #[doc = "Goes high."]
    SET = 0x02,
    #[doc = "Toggles."]
    TOGGLE = 0x03,
}
impl Emc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emc0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emc0 {
    #[inline(always)]
    fn from(val: u8) -> Emc0 {
        Emc0::from_bits(val)
    }
}
impl From<Emc0> for u8 {
    #[inline(always)]
    fn from(val: Emc0) -> u8 {
        Emc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emc1 {
    #[doc = "Does nothing."]
    DO_NOTHING = 0x0,
    #[doc = "Goes low."]
    CLEAR = 0x01,
    #[doc = "Goes high."]
    SET = 0x02,
    #[doc = "Toggles."]
    TOGGLE = 0x03,
}
impl Emc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emc1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emc1 {
    #[inline(always)]
    fn from(val: u8) -> Emc1 {
        Emc1::from_bits(val)
    }
}
impl From<Emc1> for u8 {
    #[inline(always)]
    fn from(val: Emc1) -> u8 {
        Emc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emc2 {
    #[doc = "Does nothing."]
    DO_NOTHING = 0x0,
    #[doc = "Goes low."]
    CLEAR = 0x01,
    #[doc = "Goes high."]
    SET = 0x02,
    #[doc = "Toggles."]
    TOGGLE = 0x03,
}
impl Emc2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emc2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emc2 {
    #[inline(always)]
    fn from(val: u8) -> Emc2 {
        Emc2::from_bits(val)
    }
}
impl From<Emc2> for u8 {
    #[inline(always)]
    fn from(val: Emc2) -> u8 {
        Emc2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emc3 {
    #[doc = "Does nothing."]
    DO_NOTHING = 0x0,
    #[doc = "Goes low."]
    CLEAR = 0x01,
    #[doc = "Goes high."]
    SET = 0x02,
    #[doc = "Toggles."]
    TOGGLE = 0x03,
}
impl Emc3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emc3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emc3 {
    #[inline(always)]
    fn from(val: u8) -> Emc3 {
        Emc3::from_bits(val)
    }
}
impl From<Emc3> for u8 {
    #[inline(always)]
    fn from(val: Emc3) -> u8 {
        Emc3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr0i {
    #[doc = "Does not generate."]
    MR0I_0 = 0x0,
    #[doc = "Generates."]
    MR0I_1 = 0x01,
}
impl Mr0i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr0i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr0i {
    #[inline(always)]
    fn from(val: u8) -> Mr0i {
        Mr0i::from_bits(val)
    }
}
impl From<Mr0i> for u8 {
    #[inline(always)]
    fn from(val: Mr0i) -> u8 {
        Mr0i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr0r {
    #[doc = "Does not reset."]
    MR0R_0 = 0x0,
    #[doc = "Resets."]
    MR0R_1 = 0x01,
}
impl Mr0r {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr0r {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr0r {
    #[inline(always)]
    fn from(val: u8) -> Mr0r {
        Mr0r::from_bits(val)
    }
}
impl From<Mr0r> for u8 {
    #[inline(always)]
    fn from(val: Mr0r) -> u8 {
        Mr0r::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr0rl {
    #[doc = "Does not reload."]
    MR0RL_0 = 0x0,
    #[doc = "Reloads."]
    MR0RL_1 = 0x01,
}
impl Mr0rl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr0rl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr0rl {
    #[inline(always)]
    fn from(val: u8) -> Mr0rl {
        Mr0rl::from_bits(val)
    }
}
impl From<Mr0rl> for u8 {
    #[inline(always)]
    fn from(val: Mr0rl) -> u8 {
        Mr0rl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr0s {
    #[doc = "Does not stop."]
    MR0S_0 = 0x0,
    #[doc = "Stops."]
    MR0S_1 = 0x01,
}
impl Mr0s {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr0s {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr0s {
    #[inline(always)]
    fn from(val: u8) -> Mr0s {
        Mr0s::from_bits(val)
    }
}
impl From<Mr0s> for u8 {
    #[inline(always)]
    fn from(val: Mr0s) -> u8 {
        Mr0s::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr1i {
    #[doc = "Does not generate."]
    MR1I_0 = 0x0,
    #[doc = "Generates."]
    MR1I_1 = 0x01,
}
impl Mr1i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr1i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr1i {
    #[inline(always)]
    fn from(val: u8) -> Mr1i {
        Mr1i::from_bits(val)
    }
}
impl From<Mr1i> for u8 {
    #[inline(always)]
    fn from(val: Mr1i) -> u8 {
        Mr1i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr1r {
    #[doc = "Does not reset."]
    MR1R_0 = 0x0,
    #[doc = "Resets."]
    MR1R_1 = 0x01,
}
impl Mr1r {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr1r {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr1r {
    #[inline(always)]
    fn from(val: u8) -> Mr1r {
        Mr1r::from_bits(val)
    }
}
impl From<Mr1r> for u8 {
    #[inline(always)]
    fn from(val: Mr1r) -> u8 {
        Mr1r::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr1rl {
    #[doc = "Does not reload."]
    MR1RL_0 = 0x0,
    #[doc = "Reloads."]
    MR1RL_1 = 0x01,
}
impl Mr1rl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr1rl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr1rl {
    #[inline(always)]
    fn from(val: u8) -> Mr1rl {
        Mr1rl::from_bits(val)
    }
}
impl From<Mr1rl> for u8 {
    #[inline(always)]
    fn from(val: Mr1rl) -> u8 {
        Mr1rl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr1s {
    #[doc = "Does not stop."]
    MRIS_0 = 0x0,
    #[doc = "Stops."]
    MRIS_1 = 0x01,
}
impl Mr1s {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr1s {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr1s {
    #[inline(always)]
    fn from(val: u8) -> Mr1s {
        Mr1s::from_bits(val)
    }
}
impl From<Mr1s> for u8 {
    #[inline(always)]
    fn from(val: Mr1s) -> u8 {
        Mr1s::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr2i {
    #[doc = "Does not generate."]
    MR2I_0 = 0x0,
    #[doc = "Generates."]
    MR2I_1 = 0x01,
}
impl Mr2i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr2i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr2i {
    #[inline(always)]
    fn from(val: u8) -> Mr2i {
        Mr2i::from_bits(val)
    }
}
impl From<Mr2i> for u8 {
    #[inline(always)]
    fn from(val: Mr2i) -> u8 {
        Mr2i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr2r {
    #[doc = "Does not reset."]
    MR2R_0 = 0x0,
    #[doc = "Resets."]
    MR2R_1 = 0x01,
}
impl Mr2r {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr2r {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr2r {
    #[inline(always)]
    fn from(val: u8) -> Mr2r {
        Mr2r::from_bits(val)
    }
}
impl From<Mr2r> for u8 {
    #[inline(always)]
    fn from(val: Mr2r) -> u8 {
        Mr2r::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr2rl {
    #[doc = "Does not reload."]
    MR2RL_0 = 0x0,
    #[doc = "Reloads."]
    MR2RL_1 = 0x01,
}
impl Mr2rl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr2rl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr2rl {
    #[inline(always)]
    fn from(val: u8) -> Mr2rl {
        Mr2rl::from_bits(val)
    }
}
impl From<Mr2rl> for u8 {
    #[inline(always)]
    fn from(val: Mr2rl) -> u8 {
        Mr2rl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr2s {
    #[doc = "Does not stop."]
    MR2S_0 = 0x0,
    #[doc = "Stops."]
    MR2S_1 = 0x01,
}
impl Mr2s {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr2s {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr2s {
    #[inline(always)]
    fn from(val: u8) -> Mr2s {
        Mr2s::from_bits(val)
    }
}
impl From<Mr2s> for u8 {
    #[inline(always)]
    fn from(val: Mr2s) -> u8 {
        Mr2s::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr3i {
    #[doc = "Does not generate."]
    MR3I_0 = 0x0,
    #[doc = "Generates."]
    MR3I_1 = 0x01,
}
impl Mr3i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr3i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr3i {
    #[inline(always)]
    fn from(val: u8) -> Mr3i {
        Mr3i::from_bits(val)
    }
}
impl From<Mr3i> for u8 {
    #[inline(always)]
    fn from(val: Mr3i) -> u8 {
        Mr3i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr3r {
    #[doc = "Does not reset."]
    MR3R_0 = 0x0,
    #[doc = "Resets."]
    MR3R_1 = 0x01,
}
impl Mr3r {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr3r {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr3r {
    #[inline(always)]
    fn from(val: u8) -> Mr3r {
        Mr3r::from_bits(val)
    }
}
impl From<Mr3r> for u8 {
    #[inline(always)]
    fn from(val: Mr3r) -> u8 {
        Mr3r::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr3rl {
    #[doc = "Does not reload."]
    MR3RL_0 = 0x0,
    #[doc = "Reloads."]
    MR3RL_1 = 0x01,
}
impl Mr3rl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr3rl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr3rl {
    #[inline(always)]
    fn from(val: u8) -> Mr3rl {
        Mr3rl::from_bits(val)
    }
}
impl From<Mr3rl> for u8 {
    #[inline(always)]
    fn from(val: Mr3rl) -> u8 {
        Mr3rl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr3s {
    #[doc = "Does not stop."]
    MR3S_0 = 0x0,
    #[doc = "Stops."]
    MR3S_1 = 0x01,
}
impl Mr3s {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr3s {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr3s {
    #[inline(always)]
    fn from(val: u8) -> Mr3s {
        Mr3s::from_bits(val)
    }
}
impl From<Mr3s> for u8 {
    #[inline(always)]
    fn from(val: Mr3s) -> u8 {
        Mr3s::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwmen0 {
    #[doc = "Disable."]
    MATCH = 0x0,
    #[doc = "Enable."]
    PWM = 0x01,
}
impl Pwmen0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwmen0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwmen0 {
    #[inline(always)]
    fn from(val: u8) -> Pwmen0 {
        Pwmen0::from_bits(val)
    }
}
impl From<Pwmen0> for u8 {
    #[inline(always)]
    fn from(val: Pwmen0) -> u8 {
        Pwmen0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwmen1 {
    #[doc = "Disable."]
    MATCH = 0x0,
    #[doc = "Enable."]
    PWM = 0x01,
}
impl Pwmen1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwmen1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwmen1 {
    #[inline(always)]
    fn from(val: u8) -> Pwmen1 {
        Pwmen1::from_bits(val)
    }
}
impl From<Pwmen1> for u8 {
    #[inline(always)]
    fn from(val: Pwmen1) -> u8 {
        Pwmen1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwmen2 {
    #[doc = "Disable."]
    MATCH = 0x0,
    #[doc = "Enable."]
    PWM = 0x01,
}
impl Pwmen2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwmen2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwmen2 {
    #[inline(always)]
    fn from(val: u8) -> Pwmen2 {
        Pwmen2::from_bits(val)
    }
}
impl From<Pwmen2> for u8 {
    #[inline(always)]
    fn from(val: Pwmen2) -> u8 {
        Pwmen2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwmen3 {
    #[doc = "Disable."]
    MATCH = 0x0,
    #[doc = "Enable."]
    PWM = 0x01,
}
impl Pwmen3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwmen3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwmen3 {
    #[inline(always)]
    fn from(val: u8) -> Pwmen3 {
        Pwmen3::from_bits(val)
    }
}
impl From<Pwmen3> for u8 {
    #[inline(always)]
    fn from(val: Pwmen3) -> u8 {
        Pwmen3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Selcc {
    #[doc = "Capture channel 0 rising edge."]
    CHANNEL_0_RISING = 0x0,
    #[doc = "Capture channel 0 falling edge."]
    CHANNEL_0_FALLING = 0x01,
    #[doc = "Capture channel 1 rising edge."]
    CHANNEL_1_RISING = 0x02,
    #[doc = "Capture channel 1 falling edge."]
    CHANNEL_1_FALLING = 0x03,
    #[doc = "Capture channel 2 rising edge."]
    CHANNEL_2_RISING = 0x04,
    #[doc = "Capture channel 2 falling edge."]
    CHANNEL_2_FALLING = 0x05,
    #[doc = "Capture channel 3 rising edge."]
    CHANNEL_3_RISING = 0x06,
    #[doc = "Capture channel 3 falling edge."]
    CHANNEL_3_FALLING = 0x07,
}
impl Selcc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Selcc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Selcc {
    #[inline(always)]
    fn from(val: u8) -> Selcc {
        Selcc::from_bits(val)
    }
}
impl From<Selcc> for u8 {
    #[inline(always)]
    fn from(val: Selcc) -> u8 {
        Selcc::to_bits(val)
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
