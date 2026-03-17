#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6a8c2aa 2026-01-27))"]
#[doc = "pd_main.trng0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trng0 {
    ptr: *mut u8,
}
unsafe impl Send for Trng0 {}
unsafe impl Sync for Trng0 {}
impl Trng0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Miscellaneous Control Register."]
    #[inline(always)]
    pub const fn mctl(self) -> crate::pac::common::Reg<Mctl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Statistical Check Miscellaneous Register."]
    #[inline(always)]
    pub const fn scmisc(self) -> crate::pac::common::Reg<Scmisc, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Seed Control Register."]
    #[inline(always)]
    pub const fn sdctl(self) -> crate::pac::common::Reg<Sdctl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Frequency Count Minimum Limit Register."]
    #[inline(always)]
    pub const fn frqmin(self) -> crate::pac::common::Reg<Frqmin, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Oscillator-2 Frequency Count Register."]
    #[inline(always)]
    pub const fn osc2_frqcnt(self) -> crate::pac::common::Reg<Osc2Frqcnt, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Frequency Count Register."]
    #[inline(always)]
    pub const fn frqcnt(self) -> crate::pac::common::Reg<Frqcnt, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Frequency Count Maximum Limit Register."]
    #[inline(always)]
    pub const fn frqmax(self) -> crate::pac::common::Reg<Frqmax, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Statistical Check Monobit Count Register."]
    #[inline(always)]
    pub const fn scmc(self) -> crate::pac::common::Reg<Scmc, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Statistical Check Monobit Limit Register."]
    #[inline(always)]
    pub const fn scml(self) -> crate::pac::common::Reg<Scml, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Statistical Check Run Length 1 Count Register."]
    #[inline(always)]
    pub const fn scr1c(self) -> crate::pac::common::Reg<Scr1c, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Statistical Check Run Length 1 Limit Register."]
    #[inline(always)]
    pub const fn scr1l(self) -> crate::pac::common::Reg<Scr1l, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Statistical Check Run Length 2 Count Register."]
    #[inline(always)]
    pub const fn scr2c(self) -> crate::pac::common::Reg<Scr2c, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Statistical Check Run Length 2 Limit Register."]
    #[inline(always)]
    pub const fn scr2l(self) -> crate::pac::common::Reg<Scr2l, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Statistical Check Run Length 3 Count Register."]
    #[inline(always)]
    pub const fn scr3c(self) -> crate::pac::common::Reg<Scr3c, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Statistical Check Run Length 3 Limit Register."]
    #[inline(always)]
    pub const fn scr3l(self) -> crate::pac::common::Reg<Scr3l, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn status(self) -> crate::pac::common::Reg<Status, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Entropy Read Register."]
    #[inline(always)]
    pub const fn ent(self, n: usize) -> crate::pac::common::Reg<Ent, crate::pac::common::R> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _)
        }
    }
    #[doc = "Security Configuration Register."]
    #[inline(always)]
    pub const fn sec_cfg(self) -> crate::pac::common::Reg<SecCfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Interrupt Control Register."]
    #[inline(always)]
    pub const fn int_ctrl(self) -> crate::pac::common::Reg<IntCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Mask Register."]
    #[inline(always)]
    pub const fn int_mask(self) -> crate::pac::common::Reg<IntMask, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Interrupt Status Register."]
    #[inline(always)]
    pub const fn int_status(self) -> crate::pac::common::Reg<IntStatus, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "TRNG Oscillator 2 Control Register."]
    #[inline(always)]
    pub const fn osc2_ctl(self) -> crate::pac::common::Reg<Osc2Ctl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Version ID Register (MS)."]
    #[inline(always)]
    pub const fn vid1(self) -> crate::pac::common::Reg<Vid1, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Version ID Register (LS)."]
    #[inline(always)]
    pub const fn vid2(self) -> crate::pac::common::Reg<Vid2, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
}
#[doc = "Entropy Read Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ent(pub u32);
impl Ent {
    #[doc = "Entropy Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ent(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Entropy Value."]
    #[inline(always)]
    pub const fn set_ent(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ent {
    #[inline(always)]
    fn default() -> Ent {
        Ent(0)
    }
}
impl core::fmt::Debug for Ent {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ent").field("ent", &self.ent()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ent {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ent {{ ent: {=u32:?} }}", self.ent())
    }
}
#[doc = "Frequency Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frqcnt(pub u32);
impl Frqcnt {
    #[doc = "Frequency Count."]
    #[must_use]
    #[inline(always)]
    pub const fn frq_ct(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Frequency Count."]
    #[inline(always)]
    pub const fn set_frq_ct(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for Frqcnt {
    #[inline(always)]
    fn default() -> Frqcnt {
        Frqcnt(0)
    }
}
impl core::fmt::Debug for Frqcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frqcnt")
            .field("frq_ct", &self.frq_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frqcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frqcnt {{ frq_ct: {=u32:?} }}", self.frq_ct())
    }
}
#[doc = "Frequency Count Maximum Limit Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frqmax(pub u32);
impl Frqmax {
    #[doc = "Frequency Counter Maximum Limit."]
    #[must_use]
    #[inline(always)]
    pub const fn frq_max(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Frequency Counter Maximum Limit."]
    #[inline(always)]
    pub const fn set_frq_max(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for Frqmax {
    #[inline(always)]
    fn default() -> Frqmax {
        Frqmax(0)
    }
}
impl core::fmt::Debug for Frqmax {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frqmax")
            .field("frq_max", &self.frq_max())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frqmax {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frqmax {{ frq_max: {=u32:?} }}", self.frq_max())
    }
}
#[doc = "Frequency Count Minimum Limit Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frqmin(pub u32);
impl Frqmin {
    #[doc = "Frequency Count Minimum Limit."]
    #[must_use]
    #[inline(always)]
    pub const fn frq_min(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Frequency Count Minimum Limit."]
    #[inline(always)]
    pub const fn set_frq_min(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for Frqmin {
    #[inline(always)]
    fn default() -> Frqmin {
        Frqmin(0)
    }
}
impl core::fmt::Debug for Frqmin {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frqmin")
            .field("frq_min", &self.frq_min())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frqmin {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frqmin {{ frq_min: {=u32:?} }}", self.frq_min())
    }
}
#[doc = "Interrupt Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntCtrl(pub u32);
impl IntCtrl {
    #[doc = "Clear the HW_ERR interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn hw_err(&self) -> IntCtrlHwErr {
        let val = (self.0 >> 0usize) & 0x01;
        IntCtrlHwErr::from_bits(val as u8)
    }
    #[doc = "Clear the HW_ERR interrupt."]
    #[inline(always)]
    pub const fn set_hw_err(&mut self, val: IntCtrlHwErr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Clear the ENT_VAL interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ent_val(&self) -> IntCtrlEntVal {
        let val = (self.0 >> 1usize) & 0x01;
        IntCtrlEntVal::from_bits(val as u8)
    }
    #[doc = "Clear the ENT_VAL interrupt."]
    #[inline(always)]
    pub const fn set_ent_val(&mut self, val: IntCtrlEntVal) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Clear the FRQ_CT_FAIL interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn frq_ct_fail(&self) -> IntCtrlFrqCtFail {
        let val = (self.0 >> 2usize) & 0x01;
        IntCtrlFrqCtFail::from_bits(val as u8)
    }
    #[doc = "Clear the FRQ_CT_FAIL interrupt."]
    #[inline(always)]
    pub const fn set_frq_ct_fail(&mut self, val: IntCtrlFrqCtFail) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Clear the INTG_FLT interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn intg_flt(&self) -> IntCtrlIntgFlt {
        let val = (self.0 >> 3usize) & 0x01;
        IntCtrlIntgFlt::from_bits(val as u8)
    }
    #[doc = "Clear the INTG_FLT interrupt."]
    #[inline(always)]
    pub const fn set_intg_flt(&mut self, val: IntCtrlIntgFlt) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for IntCtrl {
    #[inline(always)]
    fn default() -> IntCtrl {
        IntCtrl(0)
    }
}
impl core::fmt::Debug for IntCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntCtrl")
            .field("hw_err", &self.hw_err())
            .field("ent_val", &self.ent_val())
            .field("frq_ct_fail", &self.frq_ct_fail())
            .field("intg_flt", &self.intg_flt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntCtrl {{ hw_err: {:?}, ent_val: {:?}, frq_ct_fail: {:?}, intg_flt: {:?} }}",
            self.hw_err(),
            self.ent_val(),
            self.frq_ct_fail(),
            self.intg_flt()
        )
    }
}
#[doc = "Mask Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntMask(pub u32);
impl IntMask {
    #[doc = "Mask the HW_ERR interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn hw_err(&self) -> IntMaskHwErr {
        let val = (self.0 >> 0usize) & 0x01;
        IntMaskHwErr::from_bits(val as u8)
    }
    #[doc = "Mask the HW_ERR interrupt."]
    #[inline(always)]
    pub const fn set_hw_err(&mut self, val: IntMaskHwErr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask the ENT_VAL interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ent_val(&self) -> IntMaskEntVal {
        let val = (self.0 >> 1usize) & 0x01;
        IntMaskEntVal::from_bits(val as u8)
    }
    #[doc = "Mask the ENT_VAL interrupt."]
    #[inline(always)]
    pub const fn set_ent_val(&mut self, val: IntMaskEntVal) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Mask the FRQ_CT_FAIL interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn frq_ct_fail(&self) -> IntMaskFrqCtFail {
        let val = (self.0 >> 2usize) & 0x01;
        IntMaskFrqCtFail::from_bits(val as u8)
    }
    #[doc = "Mask the FRQ_CT_FAIL interrupt."]
    #[inline(always)]
    pub const fn set_frq_ct_fail(&mut self, val: IntMaskFrqCtFail) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Mask the INTG_FLT interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn intg_flt(&self) -> IntMaskIntgFlt {
        let val = (self.0 >> 3usize) & 0x01;
        IntMaskIntgFlt::from_bits(val as u8)
    }
    #[doc = "Mask the INTG_FLT interrupt."]
    #[inline(always)]
    pub const fn set_intg_flt(&mut self, val: IntMaskIntgFlt) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for IntMask {
    #[inline(always)]
    fn default() -> IntMask {
        IntMask(0)
    }
}
impl core::fmt::Debug for IntMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntMask")
            .field("hw_err", &self.hw_err())
            .field("ent_val", &self.ent_val())
            .field("frq_ct_fail", &self.frq_ct_fail())
            .field("intg_flt", &self.intg_flt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntMask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntMask {{ hw_err: {:?}, ent_val: {:?}, frq_ct_fail: {:?}, intg_flt: {:?} }}",
            self.hw_err(),
            self.ent_val(),
            self.frq_ct_fail(),
            self.intg_flt()
        )
    }
}
#[doc = "Interrupt Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntStatus(pub u32);
impl IntStatus {
    #[doc = "Read: TRNG Error. Any error in the TRNG will trigger this interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn hw_err(&self) -> IntStatusHwErr {
        let val = (self.0 >> 0usize) & 0x01;
        IntStatusHwErr::from_bits(val as u8)
    }
    #[doc = "Read: TRNG Error. Any error in the TRNG will trigger this interrupt."]
    #[inline(always)]
    pub const fn set_hw_err(&mut self, val: IntStatusHwErr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Entropy Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn ent_val(&self) -> IntStatusEntVal {
        let val = (self.0 >> 1usize) & 0x01;
        IntStatusEntVal::from_bits(val as u8)
    }
    #[doc = "Entropy Valid."]
    #[inline(always)]
    pub const fn set_ent_val(&mut self, val: IntStatusEntVal) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Frequency Count Fail."]
    #[must_use]
    #[inline(always)]
    pub const fn frq_ct_fail(&self) -> IntStatusFrqCtFail {
        let val = (self.0 >> 2usize) & 0x01;
        IntStatusFrqCtFail::from_bits(val as u8)
    }
    #[doc = "Frequency Count Fail."]
    #[inline(always)]
    pub const fn set_frq_ct_fail(&mut self, val: IntStatusFrqCtFail) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Integrity Fault. An internal fault has occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn intg_flt(&self) -> IntStatusIntgFlt {
        let val = (self.0 >> 3usize) & 0x01;
        IntStatusIntgFlt::from_bits(val as u8)
    }
    #[doc = "Integrity Fault. An internal fault has occurred."]
    #[inline(always)]
    pub const fn set_intg_flt(&mut self, val: IntStatusIntgFlt) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for IntStatus {
    #[inline(always)]
    fn default() -> IntStatus {
        IntStatus(0)
    }
}
impl core::fmt::Debug for IntStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntStatus")
            .field("hw_err", &self.hw_err())
            .field("ent_val", &self.ent_val())
            .field("frq_ct_fail", &self.frq_ct_fail())
            .field("intg_flt", &self.intg_flt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntStatus {{ hw_err: {:?}, ent_val: {:?}, frq_ct_fail: {:?}, intg_flt: {:?} }}",
            self.hw_err(),
            self.ent_val(),
            self.frq_ct_fail(),
            self.intg_flt()
        )
    }
}
#[doc = "Miscellaneous Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mctl(pub u32);
impl Mctl {
    #[doc = "Oscillator1 Divide."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_div(&self) -> OscDiv {
        let val = (self.0 >> 2usize) & 0x03;
        OscDiv::from_bits(val as u8)
    }
    #[doc = "Oscillator1 Divide."]
    #[inline(always)]
    pub const fn set_osc_div(&mut self, val: OscDiv) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Clock Output Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_out_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Output Enable."]
    #[inline(always)]
    pub const fn set_clk_out_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "TRNG Access Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn trng_acc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG Access Mode."]
    #[inline(always)]
    pub const fn set_trng_acc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Reset Defaults."]
    #[must_use]
    #[inline(always)]
    pub const fn rst_def(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Defaults."]
    #[inline(always)]
    pub const fn set_rst_def(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Frequency Count Fail."]
    #[must_use]
    #[inline(always)]
    pub const fn fct_fail(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Frequency Count Fail."]
    #[inline(always)]
    pub const fn set_fct_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Frequency Count Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn fct_val(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Frequency Count Valid."]
    #[inline(always)]
    pub const fn set_fct_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Entropy Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn ent_val(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Entropy Valid."]
    #[inline(always)]
    pub const fn set_ent_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Error Status."]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Error Status."]
    #[inline(always)]
    pub const fn set_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "TRNG is ok to stop."]
    #[must_use]
    #[inline(always)]
    pub const fn tstop_ok(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG is ok to stop."]
    #[inline(always)]
    pub const fn set_tstop_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Oscillator 2 Failure."]
    #[must_use]
    #[inline(always)]
    pub const fn osc2_fail(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Oscillator 2 Failure."]
    #[inline(always)]
    pub const fn set_osc2_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Program Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn prgm(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Program Mode."]
    #[inline(always)]
    pub const fn set_prgm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Mctl {
    #[inline(always)]
    fn default() -> Mctl {
        Mctl(0)
    }
}
impl core::fmt::Debug for Mctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mctl")
            .field("osc_div", &self.osc_div())
            .field("clk_out_en", &self.clk_out_en())
            .field("trng_acc", &self.trng_acc())
            .field("rst_def", &self.rst_def())
            .field("fct_fail", &self.fct_fail())
            .field("fct_val", &self.fct_val())
            .field("ent_val", &self.ent_val())
            .field("err", &self.err())
            .field("tstop_ok", &self.tstop_ok())
            .field("osc2_fail", &self.osc2_fail())
            .field("prgm", &self.prgm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mctl {{ osc_div: {:?}, clk_out_en: {=bool:?}, trng_acc: {=bool:?}, rst_def: {=bool:?}, fct_fail: {=bool:?}, fct_val: {=bool:?}, ent_val: {=bool:?}, err: {=bool:?}, tstop_ok: {=bool:?}, osc2_fail: {=bool:?}, prgm: {=bool:?} }}",
            self.osc_div(),
            self.clk_out_en(),
            self.trng_acc(),
            self.rst_def(),
            self.fct_fail(),
            self.fct_val(),
            self.ent_val(),
            self.err(),
            self.tstop_ok(),
            self.osc2_fail(),
            self.prgm()
        )
    }
}
#[doc = "TRNG Oscillator 2 Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osc2Ctl(pub u32);
impl Osc2Ctl {
    #[doc = "TRNG entropy generation control."]
    #[must_use]
    #[inline(always)]
    pub const fn trng_ent_ctl(&self) -> TrngEntCtl {
        let val = (self.0 >> 0usize) & 0x03;
        TrngEntCtl::from_bits(val as u8)
    }
    #[doc = "TRNG entropy generation control."]
    #[inline(always)]
    pub const fn set_trng_ent_ctl(&mut self, val: TrngEntCtl) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Oscillator 2 Divide."]
    #[must_use]
    #[inline(always)]
    pub const fn osc2_div(&self) -> Osc2Div {
        let val = (self.0 >> 2usize) & 0x03;
        Osc2Div::from_bits(val as u8)
    }
    #[doc = "Oscillator 2 Divide."]
    #[inline(always)]
    pub const fn set_osc2_div(&mut self, val: Osc2Div) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Oscillator 2 Clock Output Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn osc2_out_en(&self) -> Osc2OutEn {
        let val = (self.0 >> 4usize) & 0x01;
        Osc2OutEn::from_bits(val as u8)
    }
    #[doc = "Oscillator 2 Clock Output Enable."]
    #[inline(always)]
    pub const fn set_osc2_out_en(&mut self, val: Osc2OutEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "TRNG Oscillator 2 Frequency Count Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn osc2_fct_val(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG Oscillator 2 Frequency Count Valid."]
    #[inline(always)]
    pub const fn set_osc2_fct_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Oscillator fail safe limit."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_failsafe_lmt(&self) -> OscFailsafeLmt {
        let val = (self.0 >> 12usize) & 0x03;
        OscFailsafeLmt::from_bits(val as u8)
    }
    #[doc = "Oscillator fail safe limit."]
    #[inline(always)]
    pub const fn set_osc_failsafe_lmt(&mut self, val: OscFailsafeLmt) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Oscillator fail safe test."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_failsafe_test(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Oscillator fail safe test."]
    #[inline(always)]
    pub const fn set_osc_failsafe_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Osc2Ctl {
    #[inline(always)]
    fn default() -> Osc2Ctl {
        Osc2Ctl(0)
    }
}
impl core::fmt::Debug for Osc2Ctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Osc2Ctl")
            .field("trng_ent_ctl", &self.trng_ent_ctl())
            .field("osc2_div", &self.osc2_div())
            .field("osc2_out_en", &self.osc2_out_en())
            .field("osc2_fct_val", &self.osc2_fct_val())
            .field("osc_failsafe_lmt", &self.osc_failsafe_lmt())
            .field("osc_failsafe_test", &self.osc_failsafe_test())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Osc2Ctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Osc2Ctl {{ trng_ent_ctl: {:?}, osc2_div: {:?}, osc2_out_en: {:?}, osc2_fct_val: {=bool:?}, osc_failsafe_lmt: {:?}, osc_failsafe_test: {=bool:?} }}",
            self.trng_ent_ctl(),
            self.osc2_div(),
            self.osc2_out_en(),
            self.osc2_fct_val(),
            self.osc_failsafe_lmt(),
            self.osc_failsafe_test()
        )
    }
}
#[doc = "Oscillator-2 Frequency Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osc2Frqcnt(pub u32);
impl Osc2Frqcnt {
    #[doc = "Frequency Count."]
    #[must_use]
    #[inline(always)]
    pub const fn osc2_frq_ct(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Frequency Count."]
    #[inline(always)]
    pub const fn set_osc2_frq_ct(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for Osc2Frqcnt {
    #[inline(always)]
    fn default() -> Osc2Frqcnt {
        Osc2Frqcnt(0)
    }
}
impl core::fmt::Debug for Osc2Frqcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Osc2Frqcnt")
            .field("osc2_frq_ct", &self.osc2_frq_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Osc2Frqcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Osc2Frqcnt {{ osc2_frq_ct: {=u32:?} }}",
            self.osc2_frq_ct()
        )
    }
}
#[doc = "Statistical Check Monobit Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scmc(pub u32);
impl Scmc {
    #[doc = "Monobit Count."]
    #[must_use]
    #[inline(always)]
    pub const fn mono_ct(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Monobit Count."]
    #[inline(always)]
    pub const fn set_mono_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Scmc {
    #[inline(always)]
    fn default() -> Scmc {
        Scmc(0)
    }
}
impl core::fmt::Debug for Scmc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scmc")
            .field("mono_ct", &self.mono_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scmc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Scmc {{ mono_ct: {=u16:?} }}", self.mono_ct())
    }
}
#[doc = "Statistical Check Miscellaneous Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scmisc(pub u32);
impl Scmisc {
    #[doc = "Long run max limit."]
    #[must_use]
    #[inline(always)]
    pub const fn lrun_max(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Long run max limit."]
    #[inline(always)]
    pub const fn set_lrun_max(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Retry count."]
    #[must_use]
    #[inline(always)]
    pub const fn rty_ct(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Retry count."]
    #[inline(always)]
    pub const fn set_rty_ct(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Scmisc {
    #[inline(always)]
    fn default() -> Scmisc {
        Scmisc(0)
    }
}
impl core::fmt::Debug for Scmisc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scmisc")
            .field("lrun_max", &self.lrun_max())
            .field("rty_ct", &self.rty_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scmisc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scmisc {{ lrun_max: {=u8:?}, rty_ct: {=u8:?} }}",
            self.lrun_max(),
            self.rty_ct()
        )
    }
}
#[doc = "Statistical Check Monobit Limit Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scml(pub u32);
impl Scml {
    #[doc = "Monobit Maximum Limit."]
    #[must_use]
    #[inline(always)]
    pub const fn mono_max(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Monobit Maximum Limit."]
    #[inline(always)]
    pub const fn set_mono_max(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Monobit Range."]
    #[must_use]
    #[inline(always)]
    pub const fn mono_rng(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Monobit Range."]
    #[inline(always)]
    pub const fn set_mono_rng(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Scml {
    #[inline(always)]
    fn default() -> Scml {
        Scml(0)
    }
}
impl core::fmt::Debug for Scml {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scml")
            .field("mono_max", &self.mono_max())
            .field("mono_rng", &self.mono_rng())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scml {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scml {{ mono_max: {=u16:?}, mono_rng: {=u16:?} }}",
            self.mono_max(),
            self.mono_rng()
        )
    }
}
#[doc = "Statistical Check Run Length 1 Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr1c(pub u32);
impl Scr1c {
    #[doc = "Runs of Zero, Length 1 Count."]
    #[must_use]
    #[inline(always)]
    pub const fn r1_0_ct(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Runs of Zero, Length 1 Count."]
    #[inline(always)]
    pub const fn set_r1_0_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "Runs of One, Length 1 Count."]
    #[must_use]
    #[inline(always)]
    pub const fn r1_1_ct(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x7fff;
        val as u16
    }
    #[doc = "Runs of One, Length 1 Count."]
    #[inline(always)]
    pub const fn set_r1_1_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
    }
}
impl Default for Scr1c {
    #[inline(always)]
    fn default() -> Scr1c {
        Scr1c(0)
    }
}
impl core::fmt::Debug for Scr1c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr1c")
            .field("r1_0_ct", &self.r1_0_ct())
            .field("r1_1_ct", &self.r1_1_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr1c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr1c {{ r1_0_ct: {=u16:?}, r1_1_ct: {=u16:?} }}",
            self.r1_0_ct(),
            self.r1_1_ct()
        )
    }
}
#[doc = "Statistical Check Run Length 1 Limit Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr1l(pub u32);
impl Scr1l {
    #[doc = "Run Length 1 Maximum Limit."]
    #[must_use]
    #[inline(always)]
    pub const fn run1_max(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Run Length 1 Maximum Limit."]
    #[inline(always)]
    pub const fn set_run1_max(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "Run Length 1 Range."]
    #[must_use]
    #[inline(always)]
    pub const fn run1_rng(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x7fff;
        val as u16
    }
    #[doc = "Run Length 1 Range."]
    #[inline(always)]
    pub const fn set_run1_rng(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
    }
}
impl Default for Scr1l {
    #[inline(always)]
    fn default() -> Scr1l {
        Scr1l(0)
    }
}
impl core::fmt::Debug for Scr1l {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr1l")
            .field("run1_max", &self.run1_max())
            .field("run1_rng", &self.run1_rng())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr1l {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr1l {{ run1_max: {=u16:?}, run1_rng: {=u16:?} }}",
            self.run1_max(),
            self.run1_rng()
        )
    }
}
#[doc = "Statistical Check Run Length 2 Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr2c(pub u32);
impl Scr2c {
    #[doc = "Runs of Zero, Length 2 Count."]
    #[must_use]
    #[inline(always)]
    pub const fn r2_0_ct(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Runs of Zero, Length 2 Count."]
    #[inline(always)]
    pub const fn set_r2_0_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "Runs of One, Length 2 Count."]
    #[must_use]
    #[inline(always)]
    pub const fn r2_1_ct(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x3fff;
        val as u16
    }
    #[doc = "Runs of One, Length 2 Count."]
    #[inline(always)]
    pub const fn set_r2_1_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
    }
}
impl Default for Scr2c {
    #[inline(always)]
    fn default() -> Scr2c {
        Scr2c(0)
    }
}
impl core::fmt::Debug for Scr2c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr2c")
            .field("r2_0_ct", &self.r2_0_ct())
            .field("r2_1_ct", &self.r2_1_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr2c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr2c {{ r2_0_ct: {=u16:?}, r2_1_ct: {=u16:?} }}",
            self.r2_0_ct(),
            self.r2_1_ct()
        )
    }
}
#[doc = "Statistical Check Run Length 2 Limit Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr2l(pub u32);
impl Scr2l {
    #[doc = "Run Length 2 Maximum Limit."]
    #[must_use]
    #[inline(always)]
    pub const fn run2_max(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Run Length 2 Maximum Limit."]
    #[inline(always)]
    pub const fn set_run2_max(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "Run Length 2 Range."]
    #[must_use]
    #[inline(always)]
    pub const fn run2_rng(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x3fff;
        val as u16
    }
    #[doc = "Run Length 2 Range."]
    #[inline(always)]
    pub const fn set_run2_rng(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
    }
}
impl Default for Scr2l {
    #[inline(always)]
    fn default() -> Scr2l {
        Scr2l(0)
    }
}
impl core::fmt::Debug for Scr2l {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr2l")
            .field("run2_max", &self.run2_max())
            .field("run2_rng", &self.run2_rng())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr2l {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr2l {{ run2_max: {=u16:?}, run2_rng: {=u16:?} }}",
            self.run2_max(),
            self.run2_rng()
        )
    }
}
#[doc = "Statistical Check Run Length 3 Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr3c(pub u32);
impl Scr3c {
    #[doc = "Runs of Zeroes, Length 3 Count."]
    #[must_use]
    #[inline(always)]
    pub const fn r3_0_ct(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "Runs of Zeroes, Length 3 Count."]
    #[inline(always)]
    pub const fn set_r3_0_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "Runs of Ones, Length 3 Count."]
    #[must_use]
    #[inline(always)]
    pub const fn r3_1_ct(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x1fff;
        val as u16
    }
    #[doc = "Runs of Ones, Length 3 Count."]
    #[inline(always)]
    pub const fn set_r3_1_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
    }
}
impl Default for Scr3c {
    #[inline(always)]
    fn default() -> Scr3c {
        Scr3c(0)
    }
}
impl core::fmt::Debug for Scr3c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr3c")
            .field("r3_0_ct", &self.r3_0_ct())
            .field("r3_1_ct", &self.r3_1_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr3c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr3c {{ r3_0_ct: {=u16:?}, r3_1_ct: {=u16:?} }}",
            self.r3_0_ct(),
            self.r3_1_ct()
        )
    }
}
#[doc = "Statistical Check Run Length 3 Limit Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr3l(pub u32);
impl Scr3l {
    #[doc = "Run Length 3 Maximum Limit."]
    #[must_use]
    #[inline(always)]
    pub const fn run3_max(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "Run Length 3 Maximum Limit."]
    #[inline(always)]
    pub const fn set_run3_max(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "Run Length 3 Range."]
    #[must_use]
    #[inline(always)]
    pub const fn run3_rng(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x1fff;
        val as u16
    }
    #[doc = "Run Length 3 Range."]
    #[inline(always)]
    pub const fn set_run3_rng(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
    }
}
impl Default for Scr3l {
    #[inline(always)]
    fn default() -> Scr3l {
        Scr3l(0)
    }
}
impl core::fmt::Debug for Scr3l {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr3l")
            .field("run3_max", &self.run3_max())
            .field("run3_rng", &self.run3_rng())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr3l {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr3l {{ run3_max: {=u16:?}, run3_rng: {=u16:?} }}",
            self.run3_max(),
            self.run3_rng()
        )
    }
}
#[doc = "Seed Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdctl(pub u32);
impl Sdctl {
    #[doc = "Sample Size."]
    #[must_use]
    #[inline(always)]
    pub const fn samp_size(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Sample Size."]
    #[inline(always)]
    pub const fn set_samp_size(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Entropy Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn ent_dly(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Entropy Delay."]
    #[inline(always)]
    pub const fn set_ent_dly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Sdctl {
    #[inline(always)]
    fn default() -> Sdctl {
        Sdctl(0)
    }
}
impl core::fmt::Debug for Sdctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdctl")
            .field("samp_size", &self.samp_size())
            .field("ent_dly", &self.ent_dly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sdctl {{ samp_size: {=u16:?}, ent_dly: {=u16:?} }}",
            self.samp_size(),
            self.ent_dly()
        )
    }
}
#[doc = "Security Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCfg(pub u32);
impl SecCfg {
    #[doc = "If set, below mentioned TRNG configuration registers cannot be programmed: Oscillator 2 Control Register (OSC2_CTL): TRNG Entropy Generation Control \\[1:0\\] Oscillator 2 Divider \\[3:2\\] Oscillator Fail Safe Limit \\[13:12\\] Oscillator Fail Safe Test \\[14\\] TRNG Seed Control Register (SDCTL) TRNG Frequency Count Minimum Limit Register (FRQMIN) TRNG Frequency Count Maximum Limit Register (FRQMAX) TRNG Statistical Check Monobit Limit Register (SCML) TRNG Statistical Check Run Length 1 Limit Register (SCR1L) TRNG Statistical Check Run Length 2 Limit Register (SCR2L) TRNG Statistical Check Run Length 3 Limit Register (SCR3L) TRNG Miscellaneous Control Register (MCTL): Sample Mode \\[1:0\\] Oscillator Divider \\[3:2\\] Reset Defaults \\[6\\] Force System Clock \\[7\\] Long Runs Continuation Mode \\[14\\] After this bit has been written to a 1, it cannot be changed."]
    #[must_use]
    #[inline(always)]
    pub const fn no_prgm(&self) -> NoPrgm {
        let val = (self.0 >> 1usize) & 0x01;
        NoPrgm::from_bits(val as u8)
    }
    #[doc = "If set, below mentioned TRNG configuration registers cannot be programmed: Oscillator 2 Control Register (OSC2_CTL): TRNG Entropy Generation Control \\[1:0\\] Oscillator 2 Divider \\[3:2\\] Oscillator Fail Safe Limit \\[13:12\\] Oscillator Fail Safe Test \\[14\\] TRNG Seed Control Register (SDCTL) TRNG Frequency Count Minimum Limit Register (FRQMIN) TRNG Frequency Count Maximum Limit Register (FRQMAX) TRNG Statistical Check Monobit Limit Register (SCML) TRNG Statistical Check Run Length 1 Limit Register (SCR1L) TRNG Statistical Check Run Length 2 Limit Register (SCR2L) TRNG Statistical Check Run Length 3 Limit Register (SCR3L) TRNG Miscellaneous Control Register (MCTL): Sample Mode \\[1:0\\] Oscillator Divider \\[3:2\\] Reset Defaults \\[6\\] Force System Clock \\[7\\] Long Runs Continuation Mode \\[14\\] After this bit has been written to a 1, it cannot be changed."]
    #[inline(always)]
    pub const fn set_no_prgm(&mut self, val: NoPrgm) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for SecCfg {
    #[inline(always)]
    fn default() -> SecCfg {
        SecCfg(0)
    }
}
impl core::fmt::Debug for SecCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCfg")
            .field("no_prgm", &self.no_prgm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SecCfg {{ no_prgm: {:?} }}", self.no_prgm())
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Test Fail, 1-Bit Run, Sampling 0s."]
    #[must_use]
    #[inline(always)]
    pub const fn tf1br0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, 1-Bit Run, Sampling 0s."]
    #[inline(always)]
    pub const fn set_tf1br0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Test Fail, 1-Bit Run, Sampling 1s."]
    #[must_use]
    #[inline(always)]
    pub const fn tf1br1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, 1-Bit Run, Sampling 1s."]
    #[inline(always)]
    pub const fn set_tf1br1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Test Fail, 2-Bit Run, Sampling 0s."]
    #[must_use]
    #[inline(always)]
    pub const fn tf2br0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, 2-Bit Run, Sampling 0s."]
    #[inline(always)]
    pub const fn set_tf2br0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Test Fail, 2-Bit Run, Sampling 1s."]
    #[must_use]
    #[inline(always)]
    pub const fn tf2br1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, 2-Bit Run, Sampling 1s."]
    #[inline(always)]
    pub const fn set_tf2br1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Test Fail, 3-Bit Run, Sampling 0s."]
    #[must_use]
    #[inline(always)]
    pub const fn tf3br0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, 3-Bit Run, Sampling 0s."]
    #[inline(always)]
    pub const fn set_tf3br0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Test Fail."]
    #[must_use]
    #[inline(always)]
    pub const fn tf3br1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail."]
    #[inline(always)]
    pub const fn set_tf3br1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Test Fail, Long Run."]
    #[must_use]
    #[inline(always)]
    pub const fn tflr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, Long Run."]
    #[inline(always)]
    pub const fn set_tflr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Test Fail, Mono Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tfmb(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, Mono Bit."]
    #[inline(always)]
    pub const fn set_tfmb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "RETRY COUNT."]
    #[must_use]
    #[inline(always)]
    pub const fn retry_ct(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "RETRY COUNT."]
    #[inline(always)]
    pub const fn set_retry_ct(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status")
            .field("tf1br0", &self.tf1br0())
            .field("tf1br1", &self.tf1br1())
            .field("tf2br0", &self.tf2br0())
            .field("tf2br1", &self.tf2br1())
            .field("tf3br0", &self.tf3br0())
            .field("tf3br1", &self.tf3br1())
            .field("tflr", &self.tflr())
            .field("tfmb", &self.tfmb())
            .field("retry_ct", &self.retry_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ tf1br0: {=bool:?}, tf1br1: {=bool:?}, tf2br0: {=bool:?}, tf2br1: {=bool:?}, tf3br0: {=bool:?}, tf3br1: {=bool:?}, tflr: {=bool:?}, tfmb: {=bool:?}, retry_ct: {=u8:?} }}",
            self.tf1br0(),
            self.tf1br1(),
            self.tf2br0(),
            self.tf2br1(),
            self.tf3br0(),
            self.tf3br1(),
            self.tflr(),
            self.tfmb(),
            self.retry_ct()
        )
    }
}
#[doc = "Version ID Register (MS)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vid1(pub u32);
impl Vid1 {
    #[doc = "Shows the IP's Minor revision of the TRNG."]
    #[must_use]
    #[inline(always)]
    pub const fn min_rev(&self) -> MinRev {
        let val = (self.0 >> 0usize) & 0xff;
        MinRev::from_bits(val as u8)
    }
    #[doc = "Shows the IP's Minor revision of the TRNG."]
    #[inline(always)]
    pub const fn set_min_rev(&mut self, val: MinRev) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Shows the IP's Major revision of the TRNG."]
    #[must_use]
    #[inline(always)]
    pub const fn maj_rev(&self) -> MajRev {
        let val = (self.0 >> 8usize) & 0xff;
        MajRev::from_bits(val as u8)
    }
    #[doc = "Shows the IP's Major revision of the TRNG."]
    #[inline(always)]
    pub const fn set_maj_rev(&mut self, val: MajRev) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "Shows the IP ID."]
    #[must_use]
    #[inline(always)]
    pub const fn ip_id(&self) -> IpId {
        let val = (self.0 >> 16usize) & 0xffff;
        IpId::from_bits(val as u16)
    }
    #[doc = "Shows the IP ID."]
    #[inline(always)]
    pub const fn set_ip_id(&mut self, val: IpId) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Vid1 {
    #[inline(always)]
    fn default() -> Vid1 {
        Vid1(0)
    }
}
impl core::fmt::Debug for Vid1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vid1")
            .field("min_rev", &self.min_rev())
            .field("maj_rev", &self.maj_rev())
            .field("ip_id", &self.ip_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vid1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Vid1 {{ min_rev: {:?}, maj_rev: {:?}, ip_id: {:?} }}",
            self.min_rev(),
            self.maj_rev(),
            self.ip_id()
        )
    }
}
#[doc = "Version ID Register (LS)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vid2(pub u32);
impl Vid2 {
    #[doc = "Shows the IP's Configuaration options for the TRNG."]
    #[must_use]
    #[inline(always)]
    pub const fn config_opt(&self) -> ConfigOpt {
        let val = (self.0 >> 0usize) & 0xff;
        ConfigOpt::from_bits(val as u8)
    }
    #[doc = "Shows the IP's Configuaration options for the TRNG."]
    #[inline(always)]
    pub const fn set_config_opt(&mut self, val: ConfigOpt) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Shows the IP's ECO revision of the TRNG."]
    #[must_use]
    #[inline(always)]
    pub const fn eco_rev(&self) -> EcoRev {
        let val = (self.0 >> 8usize) & 0xff;
        EcoRev::from_bits(val as u8)
    }
    #[doc = "Shows the IP's ECO revision of the TRNG."]
    #[inline(always)]
    pub const fn set_eco_rev(&mut self, val: EcoRev) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "Shows the integration options for the TRNG."]
    #[must_use]
    #[inline(always)]
    pub const fn intg_opt(&self) -> IntgOpt {
        let val = (self.0 >> 16usize) & 0xff;
        IntgOpt::from_bits(val as u8)
    }
    #[doc = "Shows the integration options for the TRNG."]
    #[inline(always)]
    pub const fn set_intg_opt(&mut self, val: IntgOpt) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Shows the ERA of the TRNG."]
    #[must_use]
    #[inline(always)]
    pub const fn era(&self) -> Era {
        let val = (self.0 >> 24usize) & 0xff;
        Era::from_bits(val as u8)
    }
    #[doc = "Shows the ERA of the TRNG."]
    #[inline(always)]
    pub const fn set_era(&mut self, val: Era) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for Vid2 {
    #[inline(always)]
    fn default() -> Vid2 {
        Vid2(0)
    }
}
impl core::fmt::Debug for Vid2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vid2")
            .field("config_opt", &self.config_opt())
            .field("eco_rev", &self.eco_rev())
            .field("intg_opt", &self.intg_opt())
            .field("era", &self.era())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vid2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Vid2 {{ config_opt: {:?}, eco_rev: {:?}, intg_opt: {:?}, era: {:?} }}",
            self.config_opt(),
            self.eco_rev(),
            self.intg_opt(),
            self.era()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ConfigOpt(u8);
impl ConfigOpt {
    #[doc = "TRNG_CONFIG_OPT for TRNG."]
    pub const CONFIG_OPT_VAL: Self = Self(0x0);
}
impl ConfigOpt {
    pub const fn from_bits(val: u8) -> ConfigOpt {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for ConfigOpt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CONFIG_OPT_VAL"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ConfigOpt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CONFIG_OPT_VAL"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for ConfigOpt {
    #[inline(always)]
    fn from(val: u8) -> ConfigOpt {
        ConfigOpt::from_bits(val)
    }
}
impl From<ConfigOpt> for u8 {
    #[inline(always)]
    fn from(val: ConfigOpt) -> u8 {
        ConfigOpt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EcoRev(u8);
impl EcoRev {
    #[doc = "TRNG_ECO_REV for TRNG."]
    pub const ECO_REV_VAL: Self = Self(0x0);
}
impl EcoRev {
    pub const fn from_bits(val: u8) -> EcoRev {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for EcoRev {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ECO_REV_VAL"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcoRev {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ECO_REV_VAL"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for EcoRev {
    #[inline(always)]
    fn from(val: u8) -> EcoRev {
        EcoRev::from_bits(val)
    }
}
impl From<EcoRev> for u8 {
    #[inline(always)]
    fn from(val: EcoRev) -> u8 {
        EcoRev::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Era(u8);
impl Era {
    #[doc = "ERA of the TRNG."]
    pub const ERA_VAL: Self = Self(0x0c);
}
impl Era {
    pub const fn from_bits(val: u8) -> Era {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Era {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0c => f.write_str("ERA_VAL"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Era {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0c => defmt::write!(f, "ERA_VAL"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Era {
    #[inline(always)]
    fn from(val: u8) -> Era {
        Era::from_bits(val)
    }
}
impl From<Era> for u8 {
    #[inline(always)]
    fn from(val: Era) -> u8 {
        Era::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntCtrlEntVal {
    #[doc = "Clears the INT_STATUS\\[ENT_VAL\\] bit. Will automatically set after writing."]
    ENT_VAL_CLEAR = 0x0,
    #[doc = "Enables the INT_STATUS\\[ENT_VAL\\] bit to be set, thereby enabling interrupt generation for the ENT_VAL condition."]
    ENT_VAL_ON = 0x01,
}
impl IntCtrlEntVal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntCtrlEntVal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntCtrlEntVal {
    #[inline(always)]
    fn from(val: u8) -> IntCtrlEntVal {
        IntCtrlEntVal::from_bits(val)
    }
}
impl From<IntCtrlEntVal> for u8 {
    #[inline(always)]
    fn from(val: IntCtrlEntVal) -> u8 {
        IntCtrlEntVal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntCtrlFrqCtFail {
    #[doc = "Clears the INT_STATUS\\[FRQ_CT_FAIL\\] bit. Will automatically set after writing."]
    FRQ_CT_FAIL_CLEAR = 0x0,
    #[doc = "Enables the INT_STATUS\\[FRQ_CT_FAIL\\] bit to be set, thereby enabling interrupt generation for the FRQ_CT_FAIL condition."]
    FRQ_CT_FAIL_ON = 0x01,
}
impl IntCtrlFrqCtFail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntCtrlFrqCtFail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntCtrlFrqCtFail {
    #[inline(always)]
    fn from(val: u8) -> IntCtrlFrqCtFail {
        IntCtrlFrqCtFail::from_bits(val)
    }
}
impl From<IntCtrlFrqCtFail> for u8 {
    #[inline(always)]
    fn from(val: IntCtrlFrqCtFail) -> u8 {
        IntCtrlFrqCtFail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntCtrlHwErr {
    #[doc = "Clears the INT_STATUS\\[HW_ERR\\] bit. Will automatically set after writing."]
    HW_ERR_CLEAR = 0x0,
    #[doc = "Enables the INT_STATUS\\[HW_ERR\\] bit to be set, thereby enabling interrupt generation for the HW_ERR condition."]
    HW_ERR_ON = 0x01,
}
impl IntCtrlHwErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntCtrlHwErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntCtrlHwErr {
    #[inline(always)]
    fn from(val: u8) -> IntCtrlHwErr {
        IntCtrlHwErr::from_bits(val)
    }
}
impl From<IntCtrlHwErr> for u8 {
    #[inline(always)]
    fn from(val: IntCtrlHwErr) -> u8 {
        IntCtrlHwErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntCtrlIntgFlt {
    #[doc = "Clears the INT_STATUS\\[INTG_FLT\\] bit. Will automatically set after writing."]
    INTG_FLT_CLEAR = 0x0,
    #[doc = "Enables the INT_STATUS\\[INTG_FLT\\] bit to be set, thereby enabling interrupt generation for the INTG_FLT condition."]
    INTG_FLT_ON = 0x01,
}
impl IntCtrlIntgFlt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntCtrlIntgFlt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntCtrlIntgFlt {
    #[inline(always)]
    fn from(val: u8) -> IntCtrlIntgFlt {
        IntCtrlIntgFlt::from_bits(val)
    }
}
impl From<IntCtrlIntgFlt> for u8 {
    #[inline(always)]
    fn from(val: IntCtrlIntgFlt) -> u8 {
        IntCtrlIntgFlt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntMaskEntVal {
    #[doc = "ENT_VAL interrupt is disabled."]
    ENT_VAL_MASKED = 0x0,
    #[doc = "ENT_VAL interrupt is enabled."]
    ENT_VAL_ACTIVE = 0x01,
}
impl IntMaskEntVal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntMaskEntVal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntMaskEntVal {
    #[inline(always)]
    fn from(val: u8) -> IntMaskEntVal {
        IntMaskEntVal::from_bits(val)
    }
}
impl From<IntMaskEntVal> for u8 {
    #[inline(always)]
    fn from(val: IntMaskEntVal) -> u8 {
        IntMaskEntVal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntMaskFrqCtFail {
    #[doc = "FRQ_CT_FAIL interrupt is disabled."]
    FRQ_CT_FAIL_MASKED = 0x0,
    #[doc = "FRQ_CT_FAIL interrupt is enabled."]
    FRQ_CT_FAIL_ACTIVE = 0x01,
}
impl IntMaskFrqCtFail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntMaskFrqCtFail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntMaskFrqCtFail {
    #[inline(always)]
    fn from(val: u8) -> IntMaskFrqCtFail {
        IntMaskFrqCtFail::from_bits(val)
    }
}
impl From<IntMaskFrqCtFail> for u8 {
    #[inline(always)]
    fn from(val: IntMaskFrqCtFail) -> u8 {
        IntMaskFrqCtFail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntMaskHwErr {
    #[doc = "HW_ERR interrupt is disabled."]
    HW_ERR_MASKED = 0x0,
    #[doc = "HW_ERR interrupt is enabled."]
    HW_ERR_ACTIVE = 0x01,
}
impl IntMaskHwErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntMaskHwErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntMaskHwErr {
    #[inline(always)]
    fn from(val: u8) -> IntMaskHwErr {
        IntMaskHwErr::from_bits(val)
    }
}
impl From<IntMaskHwErr> for u8 {
    #[inline(always)]
    fn from(val: IntMaskHwErr) -> u8 {
        IntMaskHwErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntMaskIntgFlt {
    #[doc = "INTG_FLT interrupt is disabled."]
    INTG_FLT_MASKED = 0x0,
    #[doc = "INTG_FLT interrupt is enabled."]
    INTG_FLT_ACTIVE = 0x01,
}
impl IntMaskIntgFlt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntMaskIntgFlt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntMaskIntgFlt {
    #[inline(always)]
    fn from(val: u8) -> IntMaskIntgFlt {
        IntMaskIntgFlt::from_bits(val)
    }
}
impl From<IntMaskIntgFlt> for u8 {
    #[inline(always)]
    fn from(val: IntMaskIntgFlt) -> u8 {
        IntMaskIntgFlt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntStatusEntVal {
    #[doc = "Busy generating entropy. Any value read from the Entropy registers is invalid."]
    ENT_VAL_INVALID = 0x0,
    #[doc = "Values read from the Entropy registers are valid."]
    ENT_VAL_VALID = 0x01,
}
impl IntStatusEntVal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntStatusEntVal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntStatusEntVal {
    #[inline(always)]
    fn from(val: u8) -> IntStatusEntVal {
        IntStatusEntVal::from_bits(val)
    }
}
impl From<IntStatusEntVal> for u8 {
    #[inline(always)]
    fn from(val: IntStatusEntVal) -> u8 {
        IntStatusEntVal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntStatusFrqCtFail {
    #[doc = "No hardware nor self test frequency errors."]
    FRQ_CT_FAIL_NO_ERR = 0x0,
    #[doc = "The frequency counter has detected a failure."]
    FRQ_CT_FAIL_ERR = 0x01,
}
impl IntStatusFrqCtFail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntStatusFrqCtFail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntStatusFrqCtFail {
    #[inline(always)]
    fn from(val: u8) -> IntStatusFrqCtFail {
        IntStatusFrqCtFail::from_bits(val)
    }
}
impl From<IntStatusFrqCtFail> for u8 {
    #[inline(always)]
    fn from(val: IntStatusFrqCtFail) -> u8 {
        IntStatusFrqCtFail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntStatusHwErr {
    #[doc = "No error."]
    HW_ERR_NO = 0x0,
    #[doc = "Error detected."]
    HW_ERR_YES = 0x01,
}
impl IntStatusHwErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntStatusHwErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntStatusHwErr {
    #[inline(always)]
    fn from(val: u8) -> IntStatusHwErr {
        IntStatusHwErr::from_bits(val)
    }
}
impl From<IntStatusHwErr> for u8 {
    #[inline(always)]
    fn from(val: IntStatusHwErr) -> u8 {
        IntStatusHwErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntStatusIntgFlt {
    #[doc = "No internal fault has been detected."]
    INTG_FLT_NO_ERR = 0x0,
    #[doc = "TRNG has detected internal fault."]
    INTG_FLT_ERR = 0x01,
}
impl IntStatusIntgFlt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntStatusIntgFlt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntStatusIntgFlt {
    #[inline(always)]
    fn from(val: u8) -> IntStatusIntgFlt {
        IntStatusIntgFlt::from_bits(val)
    }
}
impl From<IntStatusIntgFlt> for u8 {
    #[inline(always)]
    fn from(val: IntStatusIntgFlt) -> u8 {
        IntStatusIntgFlt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IntgOpt(u8);
impl IntgOpt {
    #[doc = "INTG_OPT for TRNG."]
    pub const INTG_OPT_VAL: Self = Self(0x0a);
}
impl IntgOpt {
    pub const fn from_bits(val: u8) -> IntgOpt {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for IntgOpt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0a => f.write_str("INTG_OPT_VAL"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntgOpt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0a => defmt::write!(f, "INTG_OPT_VAL"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for IntgOpt {
    #[inline(always)]
    fn from(val: u8) -> IntgOpt {
        IntgOpt::from_bits(val)
    }
}
impl From<IntgOpt> for u8 {
    #[inline(always)]
    fn from(val: IntgOpt) -> u8 {
        IntgOpt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct IpId(u16);
impl IpId {
    #[doc = "ID for TRNG."]
    pub const IP_ID_VAL: Self = Self(0x30);
}
impl IpId {
    pub const fn from_bits(val: u16) -> IpId {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for IpId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x30 => f.write_str("IP_ID_VAL"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IpId {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x30 => defmt::write!(f, "IP_ID_VAL"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for IpId {
    #[inline(always)]
    fn from(val: u16) -> IpId {
        IpId::from_bits(val)
    }
}
impl From<IpId> for u16 {
    #[inline(always)]
    fn from(val: IpId) -> u16 {
        IpId::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MajRev(u8);
impl MajRev {
    #[doc = "Major revision number for TRNG."]
    pub const MAJ_REV_VAL: Self = Self(0x14);
}
impl MajRev {
    pub const fn from_bits(val: u8) -> MajRev {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for MajRev {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x14 => f.write_str("MAJ_REV_VAL"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MajRev {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x14 => defmt::write!(f, "MAJ_REV_VAL"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for MajRev {
    #[inline(always)]
    fn from(val: u8) -> MajRev {
        MajRev::from_bits(val)
    }
}
impl From<MajRev> for u8 {
    #[inline(always)]
    fn from(val: MajRev) -> u8 {
        MajRev::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MinRev(u8);
impl MinRev {
    #[doc = "Minor revision number for TRNG."]
    pub const MIN_REV_VAL: Self = Self(0x0c);
}
impl MinRev {
    pub const fn from_bits(val: u8) -> MinRev {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for MinRev {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0c => f.write_str("MIN_REV_VAL"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MinRev {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0c => defmt::write!(f, "MIN_REV_VAL"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for MinRev {
    #[inline(always)]
    fn from(val: u8) -> MinRev {
        MinRev::from_bits(val)
    }
}
impl From<MinRev> for u8 {
    #[inline(always)]
    fn from(val: MinRev) -> u8 {
        MinRev::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NoPrgm {
    #[doc = "TRNG configuration registers can be modified."]
    NO_PRGM_OFF = 0x0,
    #[doc = "TRNG configuration registers cannot be modified."]
    NO_PRGM_ON = 0x01,
}
impl NoPrgm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NoPrgm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NoPrgm {
    #[inline(always)]
    fn from(val: u8) -> NoPrgm {
        NoPrgm::from_bits(val)
    }
}
impl From<NoPrgm> for u8 {
    #[inline(always)]
    fn from(val: NoPrgm) -> u8 {
        NoPrgm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Osc2Div {
    #[doc = "Use ring oscillator 2 with no divide."]
    OSC2_DIV_DIV1 = 0x0,
    #[doc = "Use ring oscillator 2 divided-by-2."]
    OSC2_DIV_DIV2 = 0x01,
    #[doc = "Use ring oscillator 2 divided-by-4."]
    OSC2_DIV_DIV4 = 0x02,
    #[doc = "Use ring oscillator 2 divided-by-8."]
    OSC2_DIV_DIV8 = 0x03,
}
impl Osc2Div {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Osc2Div {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Osc2Div {
    #[inline(always)]
    fn from(val: u8) -> Osc2Div {
        Osc2Div::from_bits(val)
    }
}
impl From<Osc2Div> for u8 {
    #[inline(always)]
    fn from(val: Osc2Div) -> u8 {
        Osc2Div::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Osc2OutEn {
    #[doc = "Ring oscillator 2 output is gated to an output pad."]
    OSC2_OUT_EN_0 = 0x0,
    #[doc = "Allows external viewing of divided-by-2 ring oscillator 2 if MCTL\\[PRGM\\] = 1 mode is also selected, else ring oscillator 2 output is gated to an output pad."]
    OSC2_OUT_EN_1 = 0x01,
}
impl Osc2OutEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Osc2OutEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Osc2OutEn {
    #[inline(always)]
    fn from(val: u8) -> Osc2OutEn {
        Osc2OutEn::from_bits(val)
    }
}
impl From<Osc2OutEn> for u8 {
    #[inline(always)]
    fn from(val: Osc2OutEn) -> u8 {
        Osc2OutEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscDiv {
    #[doc = "use ring oscillator with no divide."]
    OSC_DIV_DIV1 = 0x0,
    #[doc = "use ring oscillator divided-by-2."]
    OSC_DIV_DIV2 = 0x01,
    #[doc = "use ring oscillator divided-by-4."]
    OSC_DIV_DIV4 = 0x02,
    #[doc = "use ring oscillator divided-by-8."]
    OSC_DIV_DIV8 = 0x03,
}
impl OscDiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscDiv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscDiv {
    #[inline(always)]
    fn from(val: u8) -> OscDiv {
        OscDiv::from_bits(val)
    }
}
impl From<OscDiv> for u8 {
    #[inline(always)]
    fn from(val: OscDiv) -> u8 {
        OscDiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscFailsafeLmt {
    #[doc = "The limit N is 4096 (2^12) system clocks."]
    OSC_FAILSAFE_LMT_4K = 0x0,
    #[doc = "The limit N is 65536 (2^16) system clocks. (default)."]
    OSC_FAILSAFE_LMT_64K = 0x01,
    #[doc = "N is 2^20 system clocks."]
    OSC_FAILSAFE_LMT_1M = 0x02,
    #[doc = "N is 2^22 system clocks (full range of the counter being used)."]
    OSC_FAILSAFE_LMT_4M = 0x03,
}
impl OscFailsafeLmt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscFailsafeLmt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscFailsafeLmt {
    #[inline(always)]
    fn from(val: u8) -> OscFailsafeLmt {
        OscFailsafeLmt::from_bits(val)
    }
}
impl From<OscFailsafeLmt> for u8 {
    #[inline(always)]
    fn from(val: OscFailsafeLmt) -> u8 {
        OscFailsafeLmt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrngEntCtl {
    #[doc = "Single oscillator mode, using OSC1 (default)."]
    TRNG_ENT_CTL_SINGLE_OSC1 = 0x0,
    #[doc = "Dual oscillator mode."]
    TRNG_ENT_CTL_DUAL_OSCS = 0x01,
    #[doc = "Single oscillator mode, using OSC2."]
    TRNG_ENT_CTL_SINGLE_OSC2 = 0x02,
    #[doc = "Unused, (bit field cannot be written to this value)."]
    OSC2_DIV_DIV8 = 0x03,
}
impl TrngEntCtl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrngEntCtl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrngEntCtl {
    #[inline(always)]
    fn from(val: u8) -> TrngEntCtl {
        TrngEntCtl::from_bits(val)
    }
}
impl From<TrngEntCtl> for u8 {
    #[inline(always)]
    fn from(val: TrngEntCtl) -> u8 {
        TrngEntCtl::to_bits(val)
    }
}
