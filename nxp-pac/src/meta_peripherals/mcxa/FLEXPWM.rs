#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6a8c2aa 2026-01-27))"]
#[doc = "PWM."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm {
    ptr: *mut u8,
}
unsafe impl Send for Flexpwm {}
unsafe impl Sync for Flexpwm {}
impl Flexpwm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Counter Register."]
    #[inline(always)]
    pub const fn sm0cnt(self) -> crate::pac::common::Reg<Sm0cnt, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Initial Count Register."]
    #[inline(always)]
    pub const fn sm0init(self) -> crate::pac::common::Reg<Sm0init, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Control 2 Register."]
    #[inline(always)]
    pub const fn sm0ctrl2(self) -> crate::pac::common::Reg<Sm0ctrl2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn sm0ctrl(self) -> crate::pac::common::Reg<Sm0ctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Value Register 0."]
    #[inline(always)]
    pub const fn sm0val0(self) -> crate::pac::common::Reg<Sm0val0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "Value Register 1."]
    #[inline(always)]
    pub const fn sm0val1(self) -> crate::pac::common::Reg<Sm0val1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "Value Register 2."]
    #[inline(always)]
    pub const fn sm0val2(self) -> crate::pac::common::Reg<Sm0val2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "Value Register 3."]
    #[inline(always)]
    pub const fn sm0val3(self) -> crate::pac::common::Reg<Sm0val3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "Value Register 4."]
    #[inline(always)]
    pub const fn sm0val4(self) -> crate::pac::common::Reg<Sm0val4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "Value Register 5."]
    #[inline(always)]
    pub const fn sm0val5(self) -> crate::pac::common::Reg<Sm0val5, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
    #[doc = "Output Control Register."]
    #[inline(always)]
    pub const fn sm0octrl(self) -> crate::pac::common::Reg<Sm0octrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x22usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn sm0sts(self) -> crate::pac::common::Reg<Sm0sts, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn sm0inten(self) -> crate::pac::common::Reg<Sm0inten, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x26usize) as _) }
    }
    #[doc = "DMA Enable Register."]
    #[inline(always)]
    pub const fn sm0dmaen(self) -> crate::pac::common::Reg<Sm0dmaen, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Output Trigger Control Register."]
    #[inline(always)]
    pub const fn sm0tctrl(self) -> crate::pac::common::Reg<Sm0tctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2ausize) as _) }
    }
    #[doc = "Fault Disable Mapping Register 0."]
    #[inline(always)]
    pub const fn sm0dismap0(self) -> crate::pac::common::Reg<Sm0dismap0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Deadtime Count Register 0."]
    #[inline(always)]
    pub const fn sm0dtcnt0(self) -> crate::pac::common::Reg<Sm0dtcnt0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Deadtime Count Register 1."]
    #[inline(always)]
    pub const fn sm0dtcnt1(self) -> crate::pac::common::Reg<Sm0dtcnt1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x32usize) as _) }
    }
    #[doc = "Capture Control X Register."]
    #[inline(always)]
    pub const fn sm0captctrlx(
        self,
    ) -> crate::pac::common::Reg<Sm0captctrlx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Capture Compare X Register."]
    #[inline(always)]
    pub const fn sm0captcompx(
        self,
    ) -> crate::pac::common::Reg<Sm0captcompx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x3eusize) as _) }
    }
    #[doc = "Capture Value 0 Register."]
    #[inline(always)]
    pub const fn sm0cval0(self) -> crate::pac::common::Reg<Sm0cval0, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Capture Value 0 Cycle Register."]
    #[inline(always)]
    pub const fn sm0cval0cyc(self) -> crate::pac::common::Reg<Sm0cval0cyc, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x42usize) as _) }
    }
    #[doc = "Capture Value 1 Register."]
    #[inline(always)]
    pub const fn sm0cval1(self) -> crate::pac::common::Reg<Sm0cval1, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Capture Value 1 Cycle Register."]
    #[inline(always)]
    pub const fn sm0cval1cyc(self) -> crate::pac::common::Reg<Sm0cval1cyc, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x46usize) as _) }
    }
    #[doc = "Capture PWM_X Input Filter Register."]
    #[inline(always)]
    pub const fn sm0captfiltx(
        self,
    ) -> crate::pac::common::Reg<Sm0captfiltx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x5eusize) as _) }
    }
    #[doc = "Counter Register."]
    #[inline(always)]
    pub const fn sm1cnt(self) -> crate::pac::common::Reg<Sm1cnt, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Initial Count Register."]
    #[inline(always)]
    pub const fn sm1init(self) -> crate::pac::common::Reg<Sm1init, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x62usize) as _) }
    }
    #[doc = "Control 2 Register."]
    #[inline(always)]
    pub const fn sm1ctrl2(self) -> crate::pac::common::Reg<Sm1ctrl2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn sm1ctrl(self) -> crate::pac::common::Reg<Sm1ctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x66usize) as _) }
    }
    #[doc = "Value Register 0."]
    #[inline(always)]
    pub const fn sm1val0(self) -> crate::pac::common::Reg<Sm1val0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x6ausize) as _) }
    }
    #[doc = "Value Register 1."]
    #[inline(always)]
    pub const fn sm1val1(self) -> crate::pac::common::Reg<Sm1val1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x6eusize) as _) }
    }
    #[doc = "Value Register 2."]
    #[inline(always)]
    pub const fn sm1val2(self) -> crate::pac::common::Reg<Sm1val2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x72usize) as _) }
    }
    #[doc = "Value Register 3."]
    #[inline(always)]
    pub const fn sm1val3(self) -> crate::pac::common::Reg<Sm1val3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x76usize) as _) }
    }
    #[doc = "Value Register 4."]
    #[inline(always)]
    pub const fn sm1val4(self) -> crate::pac::common::Reg<Sm1val4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x7ausize) as _) }
    }
    #[doc = "Value Register 5."]
    #[inline(always)]
    pub const fn sm1val5(self) -> crate::pac::common::Reg<Sm1val5, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x7eusize) as _) }
    }
    #[doc = "Output Control Register."]
    #[inline(always)]
    pub const fn sm1octrl(self) -> crate::pac::common::Reg<Sm1octrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x82usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn sm1sts(self) -> crate::pac::common::Reg<Sm1sts, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn sm1inten(self) -> crate::pac::common::Reg<Sm1inten, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x86usize) as _) }
    }
    #[doc = "DMA Enable Register."]
    #[inline(always)]
    pub const fn sm1dmaen(self) -> crate::pac::common::Reg<Sm1dmaen, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Output Trigger Control Register."]
    #[inline(always)]
    pub const fn sm1tctrl(self) -> crate::pac::common::Reg<Sm1tctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x8ausize) as _) }
    }
    #[doc = "Fault Disable Mapping Register 0."]
    #[inline(always)]
    pub const fn sm1dismap0(self) -> crate::pac::common::Reg<Sm1dismap0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Deadtime Count Register 0."]
    #[inline(always)]
    pub const fn sm1dtcnt0(self) -> crate::pac::common::Reg<Sm1dtcnt0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Deadtime Count Register 1."]
    #[inline(always)]
    pub const fn sm1dtcnt1(self) -> crate::pac::common::Reg<Sm1dtcnt1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x92usize) as _) }
    }
    #[doc = "Capture Control X Register."]
    #[inline(always)]
    pub const fn sm1captctrlx(
        self,
    ) -> crate::pac::common::Reg<Sm1captctrlx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Capture Compare X Register."]
    #[inline(always)]
    pub const fn sm1captcompx(
        self,
    ) -> crate::pac::common::Reg<Sm1captcompx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x9eusize) as _) }
    }
    #[doc = "Capture Value 0 Register."]
    #[inline(always)]
    pub const fn sm1cval0(self) -> crate::pac::common::Reg<Sm1cval0, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Capture Value 0 Cycle Register."]
    #[inline(always)]
    pub const fn sm1cval0cyc(self) -> crate::pac::common::Reg<Sm1cval0cyc, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa2usize) as _) }
    }
    #[doc = "Capture Value 1 Register."]
    #[inline(always)]
    pub const fn sm1cval1(self) -> crate::pac::common::Reg<Sm1cval1, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Capture Value 1 Cycle Register."]
    #[inline(always)]
    pub const fn sm1cval1cyc(self) -> crate::pac::common::Reg<Sm1cval1cyc, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa6usize) as _) }
    }
    #[doc = "Phase Delay Register."]
    #[inline(always)]
    pub const fn sm1phasedly(self) -> crate::pac::common::Reg<Sm1phasedly, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Capture PWM_X Input Filter Register."]
    #[inline(always)]
    pub const fn sm1captfiltx(
        self,
    ) -> crate::pac::common::Reg<Sm1captfiltx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xbeusize) as _) }
    }
    #[doc = "Counter Register."]
    #[inline(always)]
    pub const fn sm2cnt(self) -> crate::pac::common::Reg<Sm2cnt, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Initial Count Register."]
    #[inline(always)]
    pub const fn sm2init(self) -> crate::pac::common::Reg<Sm2init, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc2usize) as _) }
    }
    #[doc = "Control 2 Register."]
    #[inline(always)]
    pub const fn sm2ctrl2(self) -> crate::pac::common::Reg<Sm2ctrl2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn sm2ctrl(self) -> crate::pac::common::Reg<Sm2ctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc6usize) as _) }
    }
    #[doc = "Value Register 0."]
    #[inline(always)]
    pub const fn sm2val0(self) -> crate::pac::common::Reg<Sm2val0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xcausize) as _) }
    }
    #[doc = "Value Register 1."]
    #[inline(always)]
    pub const fn sm2val1(self) -> crate::pac::common::Reg<Sm2val1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xceusize) as _) }
    }
    #[doc = "Value Register 2."]
    #[inline(always)]
    pub const fn sm2val2(self) -> crate::pac::common::Reg<Sm2val2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd2usize) as _) }
    }
    #[doc = "Value Register 3."]
    #[inline(always)]
    pub const fn sm2val3(self) -> crate::pac::common::Reg<Sm2val3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd6usize) as _) }
    }
    #[doc = "Value Register 4."]
    #[inline(always)]
    pub const fn sm2val4(self) -> crate::pac::common::Reg<Sm2val4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xdausize) as _) }
    }
    #[doc = "Value Register 5."]
    #[inline(always)]
    pub const fn sm2val5(self) -> crate::pac::common::Reg<Sm2val5, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xdeusize) as _) }
    }
    #[doc = "Output Control Register."]
    #[inline(always)]
    pub const fn sm2octrl(self) -> crate::pac::common::Reg<Sm2octrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe2usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn sm2sts(self) -> crate::pac::common::Reg<Sm2sts, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn sm2inten(self) -> crate::pac::common::Reg<Sm2inten, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe6usize) as _) }
    }
    #[doc = "DMA Enable Register."]
    #[inline(always)]
    pub const fn sm2dmaen(self) -> crate::pac::common::Reg<Sm2dmaen, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Output Trigger Control Register."]
    #[inline(always)]
    pub const fn sm2tctrl(self) -> crate::pac::common::Reg<Sm2tctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xeausize) as _) }
    }
    #[doc = "Fault Disable Mapping Register 0."]
    #[inline(always)]
    pub const fn sm2dismap0(self) -> crate::pac::common::Reg<Sm2dismap0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Deadtime Count Register 0."]
    #[inline(always)]
    pub const fn sm2dtcnt0(self) -> crate::pac::common::Reg<Sm2dtcnt0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Deadtime Count Register 1."]
    #[inline(always)]
    pub const fn sm2dtcnt1(self) -> crate::pac::common::Reg<Sm2dtcnt1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf2usize) as _) }
    }
    #[doc = "Capture Control X Register."]
    #[inline(always)]
    pub const fn sm2captctrlx(
        self,
    ) -> crate::pac::common::Reg<Sm2captctrlx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Capture Compare X Register."]
    #[inline(always)]
    pub const fn sm2captcompx(
        self,
    ) -> crate::pac::common::Reg<Sm2captcompx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xfeusize) as _) }
    }
    #[doc = "Capture Value 0 Register."]
    #[inline(always)]
    pub const fn sm2cval0(self) -> crate::pac::common::Reg<Sm2cval0, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Capture Value 0 Cycle Register."]
    #[inline(always)]
    pub const fn sm2cval0cyc(self) -> crate::pac::common::Reg<Sm2cval0cyc, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0102usize) as _) }
    }
    #[doc = "Capture Value 1 Register."]
    #[inline(always)]
    pub const fn sm2cval1(self) -> crate::pac::common::Reg<Sm2cval1, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Capture Value 1 Cycle Register."]
    #[inline(always)]
    pub const fn sm2cval1cyc(self) -> crate::pac::common::Reg<Sm2cval1cyc, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0106usize) as _) }
    }
    #[doc = "Phase Delay Register."]
    #[inline(always)]
    pub const fn sm2phasedly(self) -> crate::pac::common::Reg<Sm2phasedly, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Capture PWM_X Input Filter Register."]
    #[inline(always)]
    pub const fn sm2captfiltx(
        self,
    ) -> crate::pac::common::Reg<Sm2captfiltx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x011eusize) as _) }
    }
    #[doc = "Counter Register."]
    #[inline(always)]
    pub const fn sm3cnt(self) -> crate::pac::common::Reg<Sm3cnt, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Initial Count Register."]
    #[inline(always)]
    pub const fn sm3init(self) -> crate::pac::common::Reg<Sm3init, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0122usize) as _) }
    }
    #[doc = "Control 2 Register."]
    #[inline(always)]
    pub const fn sm3ctrl2(self) -> crate::pac::common::Reg<Sm3ctrl2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn sm3ctrl(self) -> crate::pac::common::Reg<Sm3ctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0126usize) as _) }
    }
    #[doc = "Value Register 0."]
    #[inline(always)]
    pub const fn sm3val0(self) -> crate::pac::common::Reg<Sm3val0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x012ausize) as _) }
    }
    #[doc = "Value Register 1."]
    #[inline(always)]
    pub const fn sm3val1(self) -> crate::pac::common::Reg<Sm3val1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x012eusize) as _) }
    }
    #[doc = "Value Register 2."]
    #[inline(always)]
    pub const fn sm3val2(self) -> crate::pac::common::Reg<Sm3val2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0132usize) as _) }
    }
    #[doc = "Value Register 3."]
    #[inline(always)]
    pub const fn sm3val3(self) -> crate::pac::common::Reg<Sm3val3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0136usize) as _) }
    }
    #[doc = "Value Register 4."]
    #[inline(always)]
    pub const fn sm3val4(self) -> crate::pac::common::Reg<Sm3val4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x013ausize) as _) }
    }
    #[doc = "Value Register 5."]
    #[inline(always)]
    pub const fn sm3val5(self) -> crate::pac::common::Reg<Sm3val5, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x013eusize) as _) }
    }
    #[doc = "Output Control Register."]
    #[inline(always)]
    pub const fn sm3octrl(self) -> crate::pac::common::Reg<Sm3octrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0142usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn sm3sts(self) -> crate::pac::common::Reg<Sm3sts, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn sm3inten(self) -> crate::pac::common::Reg<Sm3inten, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0146usize) as _) }
    }
    #[doc = "DMA Enable Register."]
    #[inline(always)]
    pub const fn sm3dmaen(self) -> crate::pac::common::Reg<Sm3dmaen, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "Output Trigger Control Register."]
    #[inline(always)]
    pub const fn sm3tctrl(self) -> crate::pac::common::Reg<Sm3tctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x014ausize) as _) }
    }
    #[doc = "Fault Disable Mapping Register 0."]
    #[inline(always)]
    pub const fn sm3dismap0(self) -> crate::pac::common::Reg<Sm3dismap0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Deadtime Count Register 0."]
    #[inline(always)]
    pub const fn sm3dtcnt0(self) -> crate::pac::common::Reg<Sm3dtcnt0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Deadtime Count Register 1."]
    #[inline(always)]
    pub const fn sm3dtcnt1(self) -> crate::pac::common::Reg<Sm3dtcnt1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0152usize) as _) }
    }
    #[doc = "Capture Control X Register."]
    #[inline(always)]
    pub const fn sm3captctrlx(
        self,
    ) -> crate::pac::common::Reg<Sm3captctrlx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "Capture Compare X Register."]
    #[inline(always)]
    pub const fn sm3captcompx(
        self,
    ) -> crate::pac::common::Reg<Sm3captcompx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x015eusize) as _) }
    }
    #[doc = "Capture Value 0 Register."]
    #[inline(always)]
    pub const fn sm3cval0(self) -> crate::pac::common::Reg<Sm3cval0, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Capture Value 0 Cycle Register."]
    #[inline(always)]
    pub const fn sm3cval0cyc(self) -> crate::pac::common::Reg<Sm3cval0cyc, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0162usize) as _) }
    }
    #[doc = "Capture Value 1 Register."]
    #[inline(always)]
    pub const fn sm3cval1(self) -> crate::pac::common::Reg<Sm3cval1, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Capture Value 1 Cycle Register."]
    #[inline(always)]
    pub const fn sm3cval1cyc(self) -> crate::pac::common::Reg<Sm3cval1cyc, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0166usize) as _) }
    }
    #[doc = "Phase Delay Register."]
    #[inline(always)]
    pub const fn sm3phasedly(self) -> crate::pac::common::Reg<Sm3phasedly, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Capture PWM_X Input Filter Register."]
    #[inline(always)]
    pub const fn sm3captfiltx(
        self,
    ) -> crate::pac::common::Reg<Sm3captfiltx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x017eusize) as _) }
    }
    #[doc = "Output Enable Register."]
    #[inline(always)]
    pub const fn outen(self) -> crate::pac::common::Reg<Outen, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Mask Register."]
    #[inline(always)]
    pub const fn mask(self) -> crate::pac::common::Reg<Mask, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0182usize) as _) }
    }
    #[doc = "Software Controlled Output Register."]
    #[inline(always)]
    pub const fn swcout(self) -> crate::pac::common::Reg<Swcout, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "PWM Source Select Register."]
    #[inline(always)]
    pub const fn dtsrcsel(self) -> crate::pac::common::Reg<Dtsrcsel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0186usize) as _) }
    }
    #[doc = "Master Control Register."]
    #[inline(always)]
    pub const fn mctrl(self) -> crate::pac::common::Reg<Mctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Master Control 2 Register."]
    #[inline(always)]
    pub const fn mctrl2(self) -> crate::pac::common::Reg<Mctrl2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x018ausize) as _) }
    }
    #[doc = "Fault Control Register."]
    #[inline(always)]
    pub const fn fctrl0(self) -> crate::pac::common::Reg<Fctrl0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Fault Status Register."]
    #[inline(always)]
    pub const fn fsts0(self) -> crate::pac::common::Reg<Fsts0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x018eusize) as _) }
    }
    #[doc = "Fault Filter Register."]
    #[inline(always)]
    pub const fn ffilt0(self) -> crate::pac::common::Reg<Ffilt0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Fault Test Register."]
    #[inline(always)]
    pub const fn ftst0(self) -> crate::pac::common::Reg<Ftst0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0192usize) as _) }
    }
    #[doc = "Fault Control 2 Register."]
    #[inline(always)]
    pub const fn fctrl20(self) -> crate::pac::common::Reg<Fctrl20, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
}
#[doc = "PWM Source Select Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtsrcsel(pub u16);
impl Dtsrcsel {
    #[doc = "Submodule 0 PWM45 Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sm0sel45(&self) -> Sm0sel45 {
        let val = (self.0 >> 0usize) & 0x03;
        Sm0sel45::from_bits(val as u8)
    }
    #[doc = "Submodule 0 PWM45 Control Select."]
    #[inline(always)]
    pub const fn set_sm0sel45(&mut self, val: Sm0sel45) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Submodule 0 PWM23 Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sm0sel23(&self) -> Sm0sel23 {
        let val = (self.0 >> 2usize) & 0x03;
        Sm0sel23::from_bits(val as u8)
    }
    #[doc = "Submodule 0 PWM23 Control Select."]
    #[inline(always)]
    pub const fn set_sm0sel23(&mut self, val: Sm0sel23) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Submodule 1 PWM45 Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sm1sel45(&self) -> Sm1sel45 {
        let val = (self.0 >> 4usize) & 0x03;
        Sm1sel45::from_bits(val as u8)
    }
    #[doc = "Submodule 1 PWM45 Control Select."]
    #[inline(always)]
    pub const fn set_sm1sel45(&mut self, val: Sm1sel45) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Submodule 1 PWM23 Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sm1sel23(&self) -> Sm1sel23 {
        let val = (self.0 >> 6usize) & 0x03;
        Sm1sel23::from_bits(val as u8)
    }
    #[doc = "Submodule 1 PWM23 Control Select."]
    #[inline(always)]
    pub const fn set_sm1sel23(&mut self, val: Sm1sel23) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Submodule 2 PWM45 Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sm2sel45(&self) -> Sm2sel45 {
        let val = (self.0 >> 8usize) & 0x03;
        Sm2sel45::from_bits(val as u8)
    }
    #[doc = "Submodule 2 PWM45 Control Select."]
    #[inline(always)]
    pub const fn set_sm2sel45(&mut self, val: Sm2sel45) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Submodule 2 PWM23 Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sm2sel23(&self) -> Sm2sel23 {
        let val = (self.0 >> 10usize) & 0x03;
        Sm2sel23::from_bits(val as u8)
    }
    #[doc = "Submodule 2 PWM23 Control Select."]
    #[inline(always)]
    pub const fn set_sm2sel23(&mut self, val: Sm2sel23) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Submodule 3 PWM45 Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sm3sel45(&self) -> Sm3sel45 {
        let val = (self.0 >> 12usize) & 0x03;
        Sm3sel45::from_bits(val as u8)
    }
    #[doc = "Submodule 3 PWM45 Control Select."]
    #[inline(always)]
    pub const fn set_sm3sel45(&mut self, val: Sm3sel45) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Submodule 3 PWM23 Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sm3sel23(&self) -> Sm3sel23 {
        let val = (self.0 >> 14usize) & 0x03;
        Sm3sel23::from_bits(val as u8)
    }
    #[doc = "Submodule 3 PWM23 Control Select."]
    #[inline(always)]
    pub const fn set_sm3sel23(&mut self, val: Sm3sel23) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Dtsrcsel {
    #[inline(always)]
    fn default() -> Dtsrcsel {
        Dtsrcsel(0)
    }
}
impl core::fmt::Debug for Dtsrcsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dtsrcsel")
            .field("sm0sel45", &self.sm0sel45())
            .field("sm0sel23", &self.sm0sel23())
            .field("sm1sel45", &self.sm1sel45())
            .field("sm1sel23", &self.sm1sel23())
            .field("sm2sel45", &self.sm2sel45())
            .field("sm2sel23", &self.sm2sel23())
            .field("sm3sel45", &self.sm3sel45())
            .field("sm3sel23", &self.sm3sel23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dtsrcsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dtsrcsel {{ sm0sel45: {:?}, sm0sel23: {:?}, sm1sel45: {:?}, sm1sel23: {:?}, sm2sel45: {:?}, sm2sel23: {:?}, sm3sel45: {:?}, sm3sel23: {:?} }}",
            self.sm0sel45(),
            self.sm0sel23(),
            self.sm1sel45(),
            self.sm1sel23(),
            self.sm2sel45(),
            self.sm2sel23(),
            self.sm3sel45(),
            self.sm3sel23()
        )
    }
}
#[doc = "Fault Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fctrl0(pub u16);
impl Fctrl0 {
    #[doc = "Fault Interrupt Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn fie(&self) -> Fie {
        let val = (self.0 >> 0usize) & 0x0f;
        Fie::from_bits(val as u8)
    }
    #[doc = "Fault Interrupt Enables."]
    #[inline(always)]
    pub const fn set_fie(&mut self, val: Fie) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
    #[doc = "Fault Safety Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn fsafe(&self) -> Fsafe {
        let val = (self.0 >> 4usize) & 0x0f;
        Fsafe::from_bits(val as u8)
    }
    #[doc = "Fault Safety Mode."]
    #[inline(always)]
    pub const fn set_fsafe(&mut self, val: Fsafe) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u16) & 0x0f) << 4usize);
    }
    #[doc = "Automatic Fault Clearing."]
    #[must_use]
    #[inline(always)]
    pub const fn fauto(&self) -> Fauto {
        let val = (self.0 >> 8usize) & 0x0f;
        Fauto::from_bits(val as u8)
    }
    #[doc = "Automatic Fault Clearing."]
    #[inline(always)]
    pub const fn set_fauto(&mut self, val: Fauto) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u16) & 0x0f) << 8usize);
    }
    #[doc = "Fault Level."]
    #[must_use]
    #[inline(always)]
    pub const fn flvl(&self) -> Flvl {
        let val = (self.0 >> 12usize) & 0x0f;
        Flvl::from_bits(val as u8)
    }
    #[doc = "Fault Level."]
    #[inline(always)]
    pub const fn set_flvl(&mut self, val: Flvl) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Fctrl0 {
    #[inline(always)]
    fn default() -> Fctrl0 {
        Fctrl0(0)
    }
}
impl core::fmt::Debug for Fctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fctrl0")
            .field("fie", &self.fie())
            .field("fsafe", &self.fsafe())
            .field("fauto", &self.fauto())
            .field("flvl", &self.flvl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fctrl0 {{ fie: {:?}, fsafe: {:?}, fauto: {:?}, flvl: {:?} }}",
            self.fie(),
            self.fsafe(),
            self.fauto(),
            self.flvl()
        )
    }
}
#[doc = "Fault Control 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fctrl20(pub u16);
impl Fctrl20 {
    #[doc = "No Combinational Path From Fault Input To PWM Output."]
    #[must_use]
    #[inline(always)]
    pub const fn nocomb(&self) -> Nocomb {
        let val = (self.0 >> 0usize) & 0x0f;
        Nocomb::from_bits(val as u8)
    }
    #[doc = "No Combinational Path From Fault Input To PWM Output."]
    #[inline(always)]
    pub const fn set_nocomb(&mut self, val: Nocomb) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
}
impl Default for Fctrl20 {
    #[inline(always)]
    fn default() -> Fctrl20 {
        Fctrl20(0)
    }
}
impl core::fmt::Debug for Fctrl20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fctrl20")
            .field("nocomb", &self.nocomb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fctrl20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fctrl20 {{ nocomb: {:?} }}", self.nocomb())
    }
}
#[doc = "Fault Filter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ffilt0(pub u16);
impl Ffilt0 {
    #[doc = "Fault Filter Period."]
    #[must_use]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fault Filter Period."]
    #[inline(always)]
    pub const fn set_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Fault Filter Count."]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Fault Filter Count."]
    #[inline(always)]
    pub const fn set_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
    #[doc = "Fault Glitch Stretch Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gstr(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Fault Glitch Stretch Enable."]
    #[inline(always)]
    pub const fn set_gstr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Ffilt0 {
    #[inline(always)]
    fn default() -> Ffilt0 {
        Ffilt0(0)
    }
}
impl core::fmt::Debug for Ffilt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ffilt0")
            .field("filt_per", &self.filt_per())
            .field("filt_cnt", &self.filt_cnt())
            .field("gstr", &self.gstr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ffilt0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ffilt0 {{ filt_per: {=u8:?}, filt_cnt: {=u8:?}, gstr: {=bool:?} }}",
            self.filt_per(),
            self.filt_cnt(),
            self.gstr()
        )
    }
}
#[doc = "Fault Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsts0(pub u16);
impl Fsts0 {
    #[doc = "Fault Flags."]
    #[must_use]
    #[inline(always)]
    pub const fn fflag(&self) -> Fflag {
        let val = (self.0 >> 0usize) & 0x0f;
        Fflag::from_bits(val as u8)
    }
    #[doc = "Fault Flags."]
    #[inline(always)]
    pub const fn set_fflag(&mut self, val: Fflag) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
    #[doc = "Full Cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn ffull(&self) -> Ffull {
        let val = (self.0 >> 4usize) & 0x0f;
        Ffull::from_bits(val as u8)
    }
    #[doc = "Full Cycle."]
    #[inline(always)]
    pub const fn set_ffull(&mut self, val: Ffull) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u16) & 0x0f) << 4usize);
    }
    #[doc = "Filtered Fault Pins."]
    #[must_use]
    #[inline(always)]
    pub const fn ffpin(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Filtered Fault Pins."]
    #[inline(always)]
    pub const fn set_ffpin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Half Cycle Fault Recovery."]
    #[must_use]
    #[inline(always)]
    pub const fn fhalf(&self) -> Fhalf {
        let val = (self.0 >> 12usize) & 0x0f;
        Fhalf::from_bits(val as u8)
    }
    #[doc = "Half Cycle Fault Recovery."]
    #[inline(always)]
    pub const fn set_fhalf(&mut self, val: Fhalf) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Fsts0 {
    #[inline(always)]
    fn default() -> Fsts0 {
        Fsts0(0)
    }
}
impl core::fmt::Debug for Fsts0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fsts0")
            .field("fflag", &self.fflag())
            .field("ffull", &self.ffull())
            .field("ffpin", &self.ffpin())
            .field("fhalf", &self.fhalf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fsts0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fsts0 {{ fflag: {:?}, ffull: {:?}, ffpin: {=u8:?}, fhalf: {:?} }}",
            self.fflag(),
            self.ffull(),
            self.ffpin(),
            self.fhalf()
        )
    }
}
#[doc = "Fault Test Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftst0(pub u16);
impl Ftst0 {
    #[doc = "Fault Test."]
    #[must_use]
    #[inline(always)]
    pub const fn ftest(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Fault Test."]
    #[inline(always)]
    pub const fn set_ftest(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
}
impl Default for Ftst0 {
    #[inline(always)]
    fn default() -> Ftst0 {
        Ftst0(0)
    }
}
impl core::fmt::Debug for Ftst0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ftst0")
            .field("ftest", &self.ftest())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ftst0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ftst0 {{ ftest: {=bool:?} }}", self.ftest())
    }
}
#[doc = "Mask Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mask(pub u16);
impl Mask {
    #[doc = "PWM_X Masks."]
    #[must_use]
    #[inline(always)]
    pub const fn maskx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Masks."]
    #[inline(always)]
    pub const fn set_maskx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Masks."]
    #[must_use]
    #[inline(always)]
    pub const fn maskb(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Masks."]
    #[inline(always)]
    pub const fn set_maskb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PWM_A Masks."]
    #[must_use]
    #[inline(always)]
    pub const fn maska(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Masks."]
    #[inline(always)]
    pub const fn set_maska(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Update Mask Bits Immediately."]
    #[must_use]
    #[inline(always)]
    pub const fn update_mask(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Update Mask Bits Immediately."]
    #[inline(always)]
    pub const fn set_update_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u16) & 0x0f) << 12usize);
    }
}
impl Default for Mask {
    #[inline(always)]
    fn default() -> Mask {
        Mask(0)
    }
}
impl core::fmt::Debug for Mask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mask")
            .field("maskx", &self.maskx())
            .field("maskb", &self.maskb())
            .field("maska", &self.maska())
            .field("update_mask", &self.update_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mask {{ maskx: {=u8:?}, maskb: {=u8:?}, maska: {=u8:?}, update_mask: {=u8:?} }}",
            self.maskx(),
            self.maskb(),
            self.maska(),
            self.update_mask()
        )
    }
}
#[doc = "Master Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mctrl(pub u16);
impl Mctrl {
    #[doc = "Load Okay."]
    #[must_use]
    #[inline(always)]
    pub const fn ldok(&self) -> Ldok {
        let val = (self.0 >> 0usize) & 0x0f;
        Ldok::from_bits(val as u8)
    }
    #[doc = "Load Okay."]
    #[inline(always)]
    pub const fn set_ldok(&mut self, val: Ldok) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
    #[doc = "Clear Load Okay."]
    #[must_use]
    #[inline(always)]
    pub const fn cldok(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Clear Load Okay."]
    #[inline(always)]
    pub const fn set_cldok(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "Run."]
    #[must_use]
    #[inline(always)]
    pub const fn run(&self) -> Run {
        let val = (self.0 >> 8usize) & 0x0f;
        Run::from_bits(val as u8)
    }
    #[doc = "Run."]
    #[inline(always)]
    pub const fn set_run(&mut self, val: Run) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u16) & 0x0f) << 8usize);
    }
    #[doc = "Current Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn ipol(&self) -> Ipol {
        let val = (self.0 >> 12usize) & 0x0f;
        Ipol::from_bits(val as u8)
    }
    #[doc = "Current Polarity."]
    #[inline(always)]
    pub const fn set_ipol(&mut self, val: Ipol) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Mctrl {
    #[inline(always)]
    fn default() -> Mctrl {
        Mctrl(0)
    }
}
impl core::fmt::Debug for Mctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mctrl")
            .field("ldok", &self.ldok())
            .field("cldok", &self.cldok())
            .field("run", &self.run())
            .field("ipol", &self.ipol())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mctrl {{ ldok: {:?}, cldok: {=u8:?}, run: {:?}, ipol: {:?} }}",
            self.ldok(),
            self.cldok(),
            self.run(),
            self.ipol()
        )
    }
}
#[doc = "Master Control 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mctrl2(pub u16);
impl Mctrl2 {
    #[doc = "Write protect."]
    #[must_use]
    #[inline(always)]
    pub const fn wrprot(&self) -> Wrprot {
        let val = (self.0 >> 2usize) & 0x03;
        Wrprot::from_bits(val as u8)
    }
    #[doc = "Write protect."]
    #[inline(always)]
    pub const fn set_wrprot(&mut self, val: Wrprot) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Stretch IPBus clock count prescaler for mux0_trig/mux1_trig/out0_trig/out1_trig/pwma_trig/pwmb_trig."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch_cnt_prsc(&self) -> StretchCntPrsc {
        let val = (self.0 >> 6usize) & 0x03;
        StretchCntPrsc::from_bits(val as u8)
    }
    #[doc = "Stretch IPBus clock count prescaler for mux0_trig/mux1_trig/out0_trig/out1_trig/pwma_trig/pwmb_trig."]
    #[inline(always)]
    pub const fn set_stretch_cnt_prsc(&mut self, val: StretchCntPrsc) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
}
impl Default for Mctrl2 {
    #[inline(always)]
    fn default() -> Mctrl2 {
        Mctrl2(0)
    }
}
impl core::fmt::Debug for Mctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mctrl2")
            .field("wrprot", &self.wrprot())
            .field("stretch_cnt_prsc", &self.stretch_cnt_prsc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mctrl2 {{ wrprot: {:?}, stretch_cnt_prsc: {:?} }}",
            self.wrprot(),
            self.stretch_cnt_prsc()
        )
    }
}
#[doc = "Output Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outen(pub u16);
impl Outen {
    #[doc = "PWM_X Output Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_en(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Output Enables."]
    #[inline(always)]
    pub const fn set_pwmx_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Output Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmb_en(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Output Enables."]
    #[inline(always)]
    pub const fn set_pwmb_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PWM_A Output Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn pwma_en(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Output Enables."]
    #[inline(always)]
    pub const fn set_pwma_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
}
impl Default for Outen {
    #[inline(always)]
    fn default() -> Outen {
        Outen(0)
    }
}
impl core::fmt::Debug for Outen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Outen")
            .field("pwmx_en", &self.pwmx_en())
            .field("pwmb_en", &self.pwmb_en())
            .field("pwma_en", &self.pwma_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Outen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Outen {{ pwmx_en: {=u8:?}, pwmb_en: {=u8:?}, pwma_en: {=u8:?} }}",
            self.pwmx_en(),
            self.pwmb_en(),
            self.pwma_en()
        )
    }
}
#[doc = "Capture Compare X Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0captcompx(pub u16);
impl Sm0captcompx {
    #[doc = "Edge Compare X."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare X."]
    #[inline(always)]
    pub const fn set_edgcmpx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter X."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter X."]
    #[inline(always)]
    pub const fn set_edgcntx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm0captcompx {
    #[inline(always)]
    fn default() -> Sm0captcompx {
        Sm0captcompx(0)
    }
}
impl core::fmt::Debug for Sm0captcompx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0captcompx")
            .field("edgcmpx", &self.edgcmpx())
            .field("edgcntx", &self.edgcntx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0captcompx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0captcompx {{ edgcmpx: {=u8:?}, edgcntx: {=u8:?} }}",
            self.edgcmpx(),
            self.edgcntx()
        )
    }
}
#[doc = "Capture Control X Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0captctrlx(pub u16);
impl Sm0captctrlx {
    #[doc = "Arm X."]
    #[must_use]
    #[inline(always)]
    pub const fn armx(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm X."]
    #[inline(always)]
    pub const fn set_armx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode Aux."]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotx(&self) -> Sm0captctrlxOneshotx {
        let val = (self.0 >> 1usize) & 0x01;
        Sm0captctrlxOneshotx::from_bits(val as u8)
    }
    #[doc = "One Shot Mode Aux."]
    #[inline(always)]
    pub const fn set_oneshotx(&mut self, val: Sm0captctrlxOneshotx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge X 0."]
    #[must_use]
    #[inline(always)]
    pub const fn edgx0(&self) -> Sm0captctrlxEdgx0 {
        let val = (self.0 >> 2usize) & 0x03;
        Sm0captctrlxEdgx0::from_bits(val as u8)
    }
    #[doc = "Edge X 0."]
    #[inline(always)]
    pub const fn set_edgx0(&mut self, val: Sm0captctrlxEdgx0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge X 1."]
    #[must_use]
    #[inline(always)]
    pub const fn edgx1(&self) -> Sm0captctrlxEdgx1 {
        let val = (self.0 >> 4usize) & 0x03;
        Sm0captctrlxEdgx1::from_bits(val as u8)
    }
    #[doc = "Edge X 1."]
    #[inline(always)]
    pub const fn set_edgx1(&mut self, val: Sm0captctrlxEdgx1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select X."]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selx(&self) -> Sm0captctrlxInpSelx {
        let val = (self.0 >> 6usize) & 0x01;
        Sm0captctrlxInpSelx::from_bits(val as u8)
    }
    #[doc = "Input Select X."]
    #[inline(always)]
    pub const fn set_inp_selx(&mut self, val: Sm0captctrlxInpSelx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter X Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter X Enable."]
    #[inline(always)]
    pub const fn set_edgcntx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture X FIFOs Water Mark."]
    #[must_use]
    #[inline(always)]
    pub const fn cfxwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture X FIFOs Water Mark."]
    #[inline(always)]
    pub const fn set_cfxwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture X0 FIFO Word Count."]
    #[must_use]
    #[inline(always)]
    pub const fn cx0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X0 FIFO Word Count."]
    #[inline(always)]
    pub const fn set_cx0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture X1 FIFO Word Count."]
    #[must_use]
    #[inline(always)]
    pub const fn cx1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X1 FIFO Word Count."]
    #[inline(always)]
    pub const fn set_cx1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm0captctrlx {
    #[inline(always)]
    fn default() -> Sm0captctrlx {
        Sm0captctrlx(0)
    }
}
impl core::fmt::Debug for Sm0captctrlx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0captctrlx")
            .field("armx", &self.armx())
            .field("oneshotx", &self.oneshotx())
            .field("edgx0", &self.edgx0())
            .field("edgx1", &self.edgx1())
            .field("inp_selx", &self.inp_selx())
            .field("edgcntx_en", &self.edgcntx_en())
            .field("cfxwm", &self.cfxwm())
            .field("cx0cnt", &self.cx0cnt())
            .field("cx1cnt", &self.cx1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0captctrlx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0captctrlx {{ armx: {=bool:?}, oneshotx: {:?}, edgx0: {:?}, edgx1: {:?}, inp_selx: {:?}, edgcntx_en: {=bool:?}, cfxwm: {=u8:?}, cx0cnt: {=u8:?}, cx1cnt: {=u8:?} }}",
            self.armx(),
            self.oneshotx(),
            self.edgx0(),
            self.edgx1(),
            self.inp_selx(),
            self.edgcntx_en(),
            self.cfxwm(),
            self.cx0cnt(),
            self.cx1cnt()
        )
    }
}
#[doc = "Capture PWM_X Input Filter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0captfiltx(pub u16);
impl Sm0captfiltx {
    #[doc = "Input Capture Filter Period."]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period."]
    #[inline(always)]
    pub const fn set_captx_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count."]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count."]
    #[inline(always)]
    pub const fn set_captx_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm0captfiltx {
    #[inline(always)]
    fn default() -> Sm0captfiltx {
        Sm0captfiltx(0)
    }
}
impl core::fmt::Debug for Sm0captfiltx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0captfiltx")
            .field("captx_filt_per", &self.captx_filt_per())
            .field("captx_filt_cnt", &self.captx_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0captfiltx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0captfiltx {{ captx_filt_per: {=u8:?}, captx_filt_cnt: {=u8:?} }}",
            self.captx_filt_per(),
            self.captx_filt_cnt()
        )
    }
}
#[doc = "Counter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cnt(pub u16);
impl Sm0cnt {
    #[doc = "Counter Register Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter Register Bits."]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm0cnt {
    #[inline(always)]
    fn default() -> Sm0cnt {
        Sm0cnt(0)
    }
}
impl core::fmt::Debug for Sm0cnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0cnt").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0cnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0cnt {{ cnt: {=u16:?} }}", self.cnt())
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0ctrl(pub u16);
impl Sm0ctrl {
    #[doc = "Double Switching Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dblen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Double Switching Enable."]
    #[inline(always)]
    pub const fn set_dblen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "PWM_X Double Switching Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dblx(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Double Switching Enable."]
    #[inline(always)]
    pub const fn set_dblx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Load Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ldmod(&self) -> Sm0ctrlLdmod {
        let val = (self.0 >> 2usize) & 0x01;
        Sm0ctrlLdmod::from_bits(val as u8)
    }
    #[doc = "Load Mode Select."]
    #[inline(always)]
    pub const fn set_ldmod(&mut self, val: Sm0ctrlLdmod) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B."]
    #[must_use]
    #[inline(always)]
    pub const fn split(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B."]
    #[inline(always)]
    pub const fn set_split(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn prsc(&self) -> Sm0ctrlPrsc {
        let val = (self.0 >> 4usize) & 0x07;
        Sm0ctrlPrsc::from_bits(val as u8)
    }
    #[doc = "Prescaler."]
    #[inline(always)]
    pub const fn set_prsc(&mut self, val: Sm0ctrlPrsc) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u16) & 0x07) << 4usize);
    }
    #[doc = "Compare Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn compmode(&self) -> Sm0ctrlCompmode {
        let val = (self.0 >> 7usize) & 0x01;
        Sm0ctrlCompmode::from_bits(val as u8)
    }
    #[doc = "Compare Mode."]
    #[inline(always)]
    pub const fn set_compmode(&mut self, val: Sm0ctrlCompmode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Deadtime."]
    #[must_use]
    #[inline(always)]
    pub const fn dt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Deadtime."]
    #[inline(always)]
    pub const fn set_dt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Full Cycle Reload."]
    #[must_use]
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Full Cycle Reload."]
    #[inline(always)]
    pub const fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Half Cycle Reload."]
    #[must_use]
    #[inline(always)]
    pub const fn half(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle Reload."]
    #[inline(always)]
    pub const fn set_half(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Load Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn ldfq(&self) -> Sm0ctrlLdfq {
        let val = (self.0 >> 12usize) & 0x0f;
        Sm0ctrlLdfq::from_bits(val as u8)
    }
    #[doc = "Load Frequency."]
    #[inline(always)]
    pub const fn set_ldfq(&mut self, val: Sm0ctrlLdfq) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Sm0ctrl {
    #[inline(always)]
    fn default() -> Sm0ctrl {
        Sm0ctrl(0)
    }
}
impl core::fmt::Debug for Sm0ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0ctrl")
            .field("dblen", &self.dblen())
            .field("dblx", &self.dblx())
            .field("ldmod", &self.ldmod())
            .field("split", &self.split())
            .field("prsc", &self.prsc())
            .field("compmode", &self.compmode())
            .field("dt", &self.dt())
            .field("full", &self.full())
            .field("half", &self.half())
            .field("ldfq", &self.ldfq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0ctrl {{ dblen: {=bool:?}, dblx: {=bool:?}, ldmod: {:?}, split: {=bool:?}, prsc: {:?}, compmode: {:?}, dt: {=u8:?}, full: {=bool:?}, half: {=bool:?}, ldfq: {:?} }}",
            self.dblen(),
            self.dblx(),
            self.ldmod(),
            self.split(),
            self.prsc(),
            self.compmode(),
            self.dt(),
            self.full(),
            self.half(),
            self.ldfq()
        )
    }
}
#[doc = "Control 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0ctrl2(pub u16);
impl Sm0ctrl2 {
    #[doc = "Clock Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel(&self) -> Sm0ctrl2ClkSel {
        let val = (self.0 >> 0usize) & 0x03;
        Sm0ctrl2ClkSel::from_bits(val as u8)
    }
    #[doc = "Clock Source Select."]
    #[inline(always)]
    pub const fn set_clk_sel(&mut self, val: Sm0ctrl2ClkSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Reload Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn reload_sel(&self) -> Sm0ctrl2ReloadSel {
        let val = (self.0 >> 2usize) & 0x01;
        Sm0ctrl2ReloadSel::from_bits(val as u8)
    }
    #[doc = "Reload Source Select."]
    #[inline(always)]
    pub const fn set_reload_sel(&mut self, val: Sm0ctrl2ReloadSel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Force Select."]
    #[must_use]
    #[inline(always)]
    pub const fn force_sel(&self) -> Sm0ctrl2ForceSel {
        let val = (self.0 >> 3usize) & 0x07;
        Sm0ctrl2ForceSel::from_bits(val as u8)
    }
    #[doc = "Force Select."]
    #[inline(always)]
    pub const fn set_force_sel(&mut self, val: Sm0ctrl2ForceSel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u16) & 0x07) << 3usize);
    }
    #[doc = "Force Initialization."]
    #[must_use]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Force Initialization."]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Force Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn frcen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Enable."]
    #[inline(always)]
    pub const fn set_frcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Initialization Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn init_sel(&self) -> Sm0ctrl2InitSel {
        let val = (self.0 >> 8usize) & 0x03;
        Sm0ctrl2InitSel::from_bits(val as u8)
    }
    #[doc = "Initialization Control Select."]
    #[inline(always)]
    pub const fn set_init_sel(&mut self, val: Sm0ctrl2InitSel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "PWM_X Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_init(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Initial Value."]
    #[inline(always)]
    pub const fn set_pwmx_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM45 Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm45_init(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PWM45 Initial Value."]
    #[inline(always)]
    pub const fn set_pwm45_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "PWM23 Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm23_init(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PWM23 Initial Value."]
    #[inline(always)]
    pub const fn set_pwm23_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Independent or Complementary Pair Operation."]
    #[must_use]
    #[inline(always)]
    pub const fn indep(&self) -> Sm0ctrl2Indep {
        let val = (self.0 >> 13usize) & 0x01;
        Sm0ctrl2Indep::from_bits(val as u8)
    }
    #[doc = "Independent or Complementary Pair Operation."]
    #[inline(always)]
    pub const fn set_indep(&mut self, val: Sm0ctrl2Indep) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Debug Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable."]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm0ctrl2 {
    #[inline(always)]
    fn default() -> Sm0ctrl2 {
        Sm0ctrl2(0)
    }
}
impl core::fmt::Debug for Sm0ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0ctrl2")
            .field("clk_sel", &self.clk_sel())
            .field("reload_sel", &self.reload_sel())
            .field("force_sel", &self.force_sel())
            .field("force", &self.force())
            .field("frcen", &self.frcen())
            .field("init_sel", &self.init_sel())
            .field("pwmx_init", &self.pwmx_init())
            .field("pwm45_init", &self.pwm45_init())
            .field("pwm23_init", &self.pwm23_init())
            .field("indep", &self.indep())
            .field("dbgen", &self.dbgen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0ctrl2 {{ clk_sel: {:?}, reload_sel: {:?}, force_sel: {:?}, force: {=bool:?}, frcen: {=bool:?}, init_sel: {:?}, pwmx_init: {=bool:?}, pwm45_init: {=bool:?}, pwm23_init: {=bool:?}, indep: {:?}, dbgen: {=bool:?} }}",
            self.clk_sel(),
            self.reload_sel(),
            self.force_sel(),
            self.force(),
            self.frcen(),
            self.init_sel(),
            self.pwmx_init(),
            self.pwm45_init(),
            self.pwm23_init(),
            self.indep(),
            self.dbgen()
        )
    }
}
#[doc = "Capture Value 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval0(pub u16);
impl Sm0cval0 {
    #[doc = "Capture Value 0."]
    #[must_use]
    #[inline(always)]
    pub const fn captval0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 0."]
    #[inline(always)]
    pub const fn set_captval0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm0cval0 {
    #[inline(always)]
    fn default() -> Sm0cval0 {
        Sm0cval0(0)
    }
}
impl core::fmt::Debug for Sm0cval0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0cval0")
            .field("captval0", &self.captval0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0cval0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0cval0 {{ captval0: {=u16:?} }}", self.captval0())
    }
}
#[doc = "Capture Value 0 Cycle Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval0cyc(pub u16);
impl Sm0cval0cyc {
    #[doc = "Capture Value 0 Cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn cval0cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 0 Cycle."]
    #[inline(always)]
    pub const fn set_cval0cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm0cval0cyc {
    #[inline(always)]
    fn default() -> Sm0cval0cyc {
        Sm0cval0cyc(0)
    }
}
impl core::fmt::Debug for Sm0cval0cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0cval0cyc")
            .field("cval0cyc", &self.cval0cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0cval0cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0cval0cyc {{ cval0cyc: {=u8:?} }}", self.cval0cyc())
    }
}
#[doc = "Capture Value 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval1(pub u16);
impl Sm0cval1 {
    #[doc = "Capture Value 1."]
    #[must_use]
    #[inline(always)]
    pub const fn captval1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 1."]
    #[inline(always)]
    pub const fn set_captval1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm0cval1 {
    #[inline(always)]
    fn default() -> Sm0cval1 {
        Sm0cval1(0)
    }
}
impl core::fmt::Debug for Sm0cval1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0cval1")
            .field("captval1", &self.captval1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0cval1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0cval1 {{ captval1: {=u16:?} }}", self.captval1())
    }
}
#[doc = "Capture Value 1 Cycle Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0cval1cyc(pub u16);
impl Sm0cval1cyc {
    #[doc = "Capture Value 1 Cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn cval1cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 1 Cycle."]
    #[inline(always)]
    pub const fn set_cval1cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm0cval1cyc {
    #[inline(always)]
    fn default() -> Sm0cval1cyc {
        Sm0cval1cyc(0)
    }
}
impl core::fmt::Debug for Sm0cval1cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0cval1cyc")
            .field("cval1cyc", &self.cval1cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0cval1cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0cval1cyc {{ cval1cyc: {=u8:?} }}", self.cval1cyc())
    }
}
#[doc = "Fault Disable Mapping Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0dismap0(pub u16);
impl Sm0dismap0 {
    #[doc = "PWM_A Fault Disable Mask 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dis0a(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Fault Disable Mask 0."]
    #[inline(always)]
    pub const fn set_dis0a(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Fault Disable Mask 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dis0b(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Fault Disable Mask 0."]
    #[inline(always)]
    pub const fn set_dis0b(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PWM_X Fault Disable Mask 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dis0x(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Fault Disable Mask 0."]
    #[inline(always)]
    pub const fn set_dis0x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
}
impl Default for Sm0dismap0 {
    #[inline(always)]
    fn default() -> Sm0dismap0 {
        Sm0dismap0(0)
    }
}
impl core::fmt::Debug for Sm0dismap0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0dismap0")
            .field("dis0a", &self.dis0a())
            .field("dis0b", &self.dis0b())
            .field("dis0x", &self.dis0x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0dismap0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0dismap0 {{ dis0a: {=u8:?}, dis0b: {=u8:?}, dis0x: {=u8:?} }}",
            self.dis0a(),
            self.dis0b(),
            self.dis0x()
        )
    }
}
#[doc = "DMA Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0dmaen(pub u16);
impl Sm0dmaen {
    #[doc = "Capture X0 FIFO DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx0de(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X0 FIFO DMA Enable."]
    #[inline(always)]
    pub const fn set_cx0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Capture X1 FIFO DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X1 FIFO DMA Enable."]
    #[inline(always)]
    pub const fn set_cx1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Capture DMA Enable Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn captde(&self) -> Sm0dmaenCaptde {
        let val = (self.0 >> 6usize) & 0x03;
        Sm0dmaenCaptde::from_bits(val as u8)
    }
    #[doc = "Capture DMA Enable Source Select."]
    #[inline(always)]
    pub const fn set_captde(&mut self, val: Sm0dmaenCaptde) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "FIFO Watermark AND Control."]
    #[must_use]
    #[inline(always)]
    pub const fn fand(&self) -> Sm0dmaenFand {
        let val = (self.0 >> 8usize) & 0x01;
        Sm0dmaenFand::from_bits(val as u8)
    }
    #[doc = "FIFO Watermark AND Control."]
    #[inline(always)]
    pub const fn set_fand(&mut self, val: Sm0dmaenFand) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Value Registers DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn valde(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Value Registers DMA Enable."]
    #[inline(always)]
    pub const fn set_valde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
}
impl Default for Sm0dmaen {
    #[inline(always)]
    fn default() -> Sm0dmaen {
        Sm0dmaen(0)
    }
}
impl core::fmt::Debug for Sm0dmaen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0dmaen")
            .field("cx0de", &self.cx0de())
            .field("cx1de", &self.cx1de())
            .field("captde", &self.captde())
            .field("fand", &self.fand())
            .field("valde", &self.valde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0dmaen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0dmaen {{ cx0de: {=bool:?}, cx1de: {=bool:?}, captde: {:?}, fand: {:?}, valde: {=bool:?} }}",
            self.cx0de(),
            self.cx1de(),
            self.captde(),
            self.fand(),
            self.valde()
        )
    }
}
#[doc = "Deadtime Count Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0dtcnt0(pub u16);
impl Sm0dtcnt0 {
    #[doc = "Deadtime Count Register 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnt0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Deadtime Count Register 0."]
    #[inline(always)]
    pub const fn set_dtcnt0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
    }
}
impl Default for Sm0dtcnt0 {
    #[inline(always)]
    fn default() -> Sm0dtcnt0 {
        Sm0dtcnt0(0)
    }
}
impl core::fmt::Debug for Sm0dtcnt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0dtcnt0")
            .field("dtcnt0", &self.dtcnt0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0dtcnt0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0dtcnt0 {{ dtcnt0: {=u16:?} }}", self.dtcnt0())
    }
}
#[doc = "Deadtime Count Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0dtcnt1(pub u16);
impl Sm0dtcnt1 {
    #[doc = "Deadtime Count Register 1."]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnt1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Deadtime Count Register 1."]
    #[inline(always)]
    pub const fn set_dtcnt1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
    }
}
impl Default for Sm0dtcnt1 {
    #[inline(always)]
    fn default() -> Sm0dtcnt1 {
        Sm0dtcnt1(0)
    }
}
impl core::fmt::Debug for Sm0dtcnt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0dtcnt1")
            .field("dtcnt1", &self.dtcnt1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0dtcnt1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0dtcnt1 {{ dtcnt1: {=u16:?} }}", self.dtcnt1())
    }
}
#[doc = "Initial Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0init(pub u16);
impl Sm0init {
    #[doc = "Initial Count Register Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn init(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Initial Count Register Bits."]
    #[inline(always)]
    pub const fn set_init(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm0init {
    #[inline(always)]
    fn default() -> Sm0init {
        Sm0init(0)
    }
}
impl core::fmt::Debug for Sm0init {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0init")
            .field("init", &self.init())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0init {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0init {{ init: {=u16:?} }}", self.init())
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0inten(pub u16);
impl Sm0inten {
    #[doc = "Compare Interrupt Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpie(&self) -> Sm0intenCmpie {
        let val = (self.0 >> 0usize) & 0x3f;
        Sm0intenCmpie::from_bits(val as u8)
    }
    #[doc = "Compare Interrupt Enables."]
    #[inline(always)]
    pub const fn set_cmpie(&mut self, val: Sm0intenCmpie) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture X 0 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx0ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 0 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cx0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture X 1 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx1ie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 1 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cx1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Reload Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn reie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_reie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
}
impl Default for Sm0inten {
    #[inline(always)]
    fn default() -> Sm0inten {
        Sm0inten(0)
    }
}
impl core::fmt::Debug for Sm0inten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0inten")
            .field("cmpie", &self.cmpie())
            .field("cx0ie", &self.cx0ie())
            .field("cx1ie", &self.cx1ie())
            .field("rie", &self.rie())
            .field("reie", &self.reie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0inten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0inten {{ cmpie: {:?}, cx0ie: {=bool:?}, cx1ie: {=bool:?}, rie: {=bool:?}, reie: {=bool:?} }}",
            self.cmpie(),
            self.cx0ie(),
            self.cx1ie(),
            self.rie(),
            self.reie()
        )
    }
}
#[doc = "Output Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0octrl(pub u16);
impl Sm0octrl {
    #[doc = "PWM_X Fault State."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmxfs(&self) -> Sm0octrlPwmxfs {
        let val = (self.0 >> 0usize) & 0x03;
        Sm0octrlPwmxfs::from_bits(val as u8)
    }
    #[doc = "PWM_X Fault State."]
    #[inline(always)]
    pub const fn set_pwmxfs(&mut self, val: Sm0octrlPwmxfs) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "PWM_B Fault State."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmbfs(&self) -> Sm0octrlPwmbfs {
        let val = (self.0 >> 2usize) & 0x03;
        Sm0octrlPwmbfs::from_bits(val as u8)
    }
    #[doc = "PWM_B Fault State."]
    #[inline(always)]
    pub const fn set_pwmbfs(&mut self, val: Sm0octrlPwmbfs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "PWM_A Fault State."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmafs(&self) -> Sm0octrlPwmafs {
        let val = (self.0 >> 4usize) & 0x03;
        Sm0octrlPwmafs::from_bits(val as u8)
    }
    #[doc = "PWM_A Fault State."]
    #[inline(always)]
    pub const fn set_pwmafs(&mut self, val: Sm0octrlPwmafs) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "PWM_X Output Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn polx(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Output Polarity."]
    #[inline(always)]
    pub const fn set_polx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "PWM_B Output Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn polb(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Output Polarity."]
    #[inline(always)]
    pub const fn set_polb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "PWM_A Output Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn pola(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Output Polarity."]
    #[inline(always)]
    pub const fn set_pola(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM_X Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_in(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Input."]
    #[inline(always)]
    pub const fn set_pwmx_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "PWM_B Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmb_in(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Input."]
    #[inline(always)]
    pub const fn set_pwmb_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "PWM_A Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pwma_in(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Input."]
    #[inline(always)]
    pub const fn set_pwma_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm0octrl {
    #[inline(always)]
    fn default() -> Sm0octrl {
        Sm0octrl(0)
    }
}
impl core::fmt::Debug for Sm0octrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0octrl")
            .field("pwmxfs", &self.pwmxfs())
            .field("pwmbfs", &self.pwmbfs())
            .field("pwmafs", &self.pwmafs())
            .field("polx", &self.polx())
            .field("polb", &self.polb())
            .field("pola", &self.pola())
            .field("pwmx_in", &self.pwmx_in())
            .field("pwmb_in", &self.pwmb_in())
            .field("pwma_in", &self.pwma_in())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0octrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0octrl {{ pwmxfs: {:?}, pwmbfs: {:?}, pwmafs: {:?}, polx: {=bool:?}, polb: {=bool:?}, pola: {=bool:?}, pwmx_in: {=bool:?}, pwmb_in: {=bool:?}, pwma_in: {=bool:?} }}",
            self.pwmxfs(),
            self.pwmbfs(),
            self.pwmafs(),
            self.polx(),
            self.polb(),
            self.pola(),
            self.pwmx_in(),
            self.pwmb_in(),
            self.pwma_in()
        )
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0sts(pub u16);
impl Sm0sts {
    #[doc = "Compare Flags."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpf(&self) -> Sm0stsCmpf {
        let val = (self.0 >> 0usize) & 0x3f;
        Sm0stsCmpf::from_bits(val as u8)
    }
    #[doc = "Compare Flags."]
    #[inline(always)]
    pub const fn set_cmpf(&mut self, val: Sm0stsCmpf) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture Flag X0."]
    #[must_use]
    #[inline(always)]
    pub const fn cfx0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X0."]
    #[inline(always)]
    pub const fn set_cfx0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture Flag X1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfx1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X1."]
    #[inline(always)]
    pub const fn set_cfx1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Reload Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Flag."]
    #[inline(always)]
    pub const fn set_rf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ref_(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Flag."]
    #[inline(always)]
    pub const fn set_ref_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Registers Updated Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ruf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Registers Updated Flag."]
    #[inline(always)]
    pub const fn set_ruf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
}
impl Default for Sm0sts {
    #[inline(always)]
    fn default() -> Sm0sts {
        Sm0sts(0)
    }
}
impl core::fmt::Debug for Sm0sts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0sts")
            .field("cmpf", &self.cmpf())
            .field("cfx0", &self.cfx0())
            .field("cfx1", &self.cfx1())
            .field("rf", &self.rf())
            .field("ref_", &self.ref_())
            .field("ruf", &self.ruf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0sts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0sts {{ cmpf: {:?}, cfx0: {=bool:?}, cfx1: {=bool:?}, rf: {=bool:?}, ref_: {=bool:?}, ruf: {=bool:?} }}",
            self.cmpf(),
            self.cfx0(),
            self.cfx1(),
            self.rf(),
            self.ref_(),
            self.ruf()
        )
    }
}
#[doc = "Output Trigger Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0tctrl(pub u16);
impl Sm0tctrl {
    #[doc = "Output Trigger Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn out_trig_en(&self) -> Sm0tctrlOutTrigEn {
        let val = (self.0 >> 0usize) & 0x3f;
        Sm0tctrlOutTrigEn::from_bits(val as u8)
    }
    #[doc = "Output Trigger Enables."]
    #[inline(always)]
    pub const fn set_out_trig_en(&mut self, val: Sm0tctrlOutTrigEn) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Trigger Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn trgfrq(&self) -> Sm0tctrlTrgfrq {
        let val = (self.0 >> 12usize) & 0x01;
        Sm0tctrlTrgfrq::from_bits(val as u8)
    }
    #[doc = "Trigger Frequency."]
    #[inline(always)]
    pub const fn set_trgfrq(&mut self, val: Sm0tctrlTrgfrq) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Mux Output Trigger 1 Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pwbot1(&self) -> Sm0tctrlPwbot1 {
        let val = (self.0 >> 14usize) & 0x01;
        Sm0tctrlPwbot1::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 1 Source Select."]
    #[inline(always)]
    pub const fn set_pwbot1(&mut self, val: Sm0tctrlPwbot1) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Mux Output Trigger 0 Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pwaot0(&self) -> Sm0tctrlPwaot0 {
        let val = (self.0 >> 15usize) & 0x01;
        Sm0tctrlPwaot0::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 0 Source Select."]
    #[inline(always)]
    pub const fn set_pwaot0(&mut self, val: Sm0tctrlPwaot0) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm0tctrl {
    #[inline(always)]
    fn default() -> Sm0tctrl {
        Sm0tctrl(0)
    }
}
impl core::fmt::Debug for Sm0tctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0tctrl")
            .field("out_trig_en", &self.out_trig_en())
            .field("trgfrq", &self.trgfrq())
            .field("pwbot1", &self.pwbot1())
            .field("pwaot0", &self.pwaot0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0tctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm0tctrl {{ out_trig_en: {:?}, trgfrq: {:?}, pwbot1: {:?}, pwaot0: {:?} }}",
            self.out_trig_en(),
            self.trgfrq(),
            self.pwbot1(),
            self.pwaot0()
        )
    }
}
#[doc = "Value Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0val0(pub u16);
impl Sm0val0 {
    #[doc = "Value 0."]
    #[must_use]
    #[inline(always)]
    pub const fn val0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 0."]
    #[inline(always)]
    pub const fn set_val0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm0val0 {
    #[inline(always)]
    fn default() -> Sm0val0 {
        Sm0val0(0)
    }
}
impl core::fmt::Debug for Sm0val0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0val0")
            .field("val0", &self.val0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0val0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0val0 {{ val0: {=u16:?} }}", self.val0())
    }
}
#[doc = "Value Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0val1(pub u16);
impl Sm0val1 {
    #[doc = "Value 1."]
    #[must_use]
    #[inline(always)]
    pub const fn val1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 1."]
    #[inline(always)]
    pub const fn set_val1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm0val1 {
    #[inline(always)]
    fn default() -> Sm0val1 {
        Sm0val1(0)
    }
}
impl core::fmt::Debug for Sm0val1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0val1")
            .field("val1", &self.val1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0val1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0val1 {{ val1: {=u16:?} }}", self.val1())
    }
}
#[doc = "Value Register 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0val2(pub u16);
impl Sm0val2 {
    #[doc = "Value 2."]
    #[must_use]
    #[inline(always)]
    pub const fn val2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 2."]
    #[inline(always)]
    pub const fn set_val2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm0val2 {
    #[inline(always)]
    fn default() -> Sm0val2 {
        Sm0val2(0)
    }
}
impl core::fmt::Debug for Sm0val2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0val2")
            .field("val2", &self.val2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0val2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0val2 {{ val2: {=u16:?} }}", self.val2())
    }
}
#[doc = "Value Register 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0val3(pub u16);
impl Sm0val3 {
    #[doc = "Value 3."]
    #[must_use]
    #[inline(always)]
    pub const fn val3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 3."]
    #[inline(always)]
    pub const fn set_val3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm0val3 {
    #[inline(always)]
    fn default() -> Sm0val3 {
        Sm0val3(0)
    }
}
impl core::fmt::Debug for Sm0val3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0val3")
            .field("val3", &self.val3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0val3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0val3 {{ val3: {=u16:?} }}", self.val3())
    }
}
#[doc = "Value Register 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0val4(pub u16);
impl Sm0val4 {
    #[doc = "Value 4."]
    #[must_use]
    #[inline(always)]
    pub const fn val4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 4."]
    #[inline(always)]
    pub const fn set_val4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm0val4 {
    #[inline(always)]
    fn default() -> Sm0val4 {
        Sm0val4(0)
    }
}
impl core::fmt::Debug for Sm0val4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0val4")
            .field("val4", &self.val4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0val4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0val4 {{ val4: {=u16:?} }}", self.val4())
    }
}
#[doc = "Value Register 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm0val5(pub u16);
impl Sm0val5 {
    #[doc = "Value 5."]
    #[must_use]
    #[inline(always)]
    pub const fn val5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 5."]
    #[inline(always)]
    pub const fn set_val5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm0val5 {
    #[inline(always)]
    fn default() -> Sm0val5 {
        Sm0val5(0)
    }
}
impl core::fmt::Debug for Sm0val5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm0val5")
            .field("val5", &self.val5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm0val5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm0val5 {{ val5: {=u16:?} }}", self.val5())
    }
}
#[doc = "Capture Compare X Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1captcompx(pub u16);
impl Sm1captcompx {
    #[doc = "Edge Compare X."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare X."]
    #[inline(always)]
    pub const fn set_edgcmpx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter X."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter X."]
    #[inline(always)]
    pub const fn set_edgcntx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm1captcompx {
    #[inline(always)]
    fn default() -> Sm1captcompx {
        Sm1captcompx(0)
    }
}
impl core::fmt::Debug for Sm1captcompx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1captcompx")
            .field("edgcmpx", &self.edgcmpx())
            .field("edgcntx", &self.edgcntx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1captcompx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1captcompx {{ edgcmpx: {=u8:?}, edgcntx: {=u8:?} }}",
            self.edgcmpx(),
            self.edgcntx()
        )
    }
}
#[doc = "Capture Control X Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1captctrlx(pub u16);
impl Sm1captctrlx {
    #[doc = "Arm X."]
    #[must_use]
    #[inline(always)]
    pub const fn armx(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm X."]
    #[inline(always)]
    pub const fn set_armx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode Aux."]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotx(&self) -> Sm1captctrlxOneshotx {
        let val = (self.0 >> 1usize) & 0x01;
        Sm1captctrlxOneshotx::from_bits(val as u8)
    }
    #[doc = "One Shot Mode Aux."]
    #[inline(always)]
    pub const fn set_oneshotx(&mut self, val: Sm1captctrlxOneshotx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge X 0."]
    #[must_use]
    #[inline(always)]
    pub const fn edgx0(&self) -> Sm1captctrlxEdgx0 {
        let val = (self.0 >> 2usize) & 0x03;
        Sm1captctrlxEdgx0::from_bits(val as u8)
    }
    #[doc = "Edge X 0."]
    #[inline(always)]
    pub const fn set_edgx0(&mut self, val: Sm1captctrlxEdgx0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge X 1."]
    #[must_use]
    #[inline(always)]
    pub const fn edgx1(&self) -> Sm1captctrlxEdgx1 {
        let val = (self.0 >> 4usize) & 0x03;
        Sm1captctrlxEdgx1::from_bits(val as u8)
    }
    #[doc = "Edge X 1."]
    #[inline(always)]
    pub const fn set_edgx1(&mut self, val: Sm1captctrlxEdgx1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select X."]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selx(&self) -> Sm1captctrlxInpSelx {
        let val = (self.0 >> 6usize) & 0x01;
        Sm1captctrlxInpSelx::from_bits(val as u8)
    }
    #[doc = "Input Select X."]
    #[inline(always)]
    pub const fn set_inp_selx(&mut self, val: Sm1captctrlxInpSelx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter X Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter X Enable."]
    #[inline(always)]
    pub const fn set_edgcntx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture X FIFOs Water Mark."]
    #[must_use]
    #[inline(always)]
    pub const fn cfxwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture X FIFOs Water Mark."]
    #[inline(always)]
    pub const fn set_cfxwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture X0 FIFO Word Count."]
    #[must_use]
    #[inline(always)]
    pub const fn cx0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X0 FIFO Word Count."]
    #[inline(always)]
    pub const fn set_cx0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture X1 FIFO Word Count."]
    #[must_use]
    #[inline(always)]
    pub const fn cx1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X1 FIFO Word Count."]
    #[inline(always)]
    pub const fn set_cx1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm1captctrlx {
    #[inline(always)]
    fn default() -> Sm1captctrlx {
        Sm1captctrlx(0)
    }
}
impl core::fmt::Debug for Sm1captctrlx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1captctrlx")
            .field("armx", &self.armx())
            .field("oneshotx", &self.oneshotx())
            .field("edgx0", &self.edgx0())
            .field("edgx1", &self.edgx1())
            .field("inp_selx", &self.inp_selx())
            .field("edgcntx_en", &self.edgcntx_en())
            .field("cfxwm", &self.cfxwm())
            .field("cx0cnt", &self.cx0cnt())
            .field("cx1cnt", &self.cx1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1captctrlx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1captctrlx {{ armx: {=bool:?}, oneshotx: {:?}, edgx0: {:?}, edgx1: {:?}, inp_selx: {:?}, edgcntx_en: {=bool:?}, cfxwm: {=u8:?}, cx0cnt: {=u8:?}, cx1cnt: {=u8:?} }}",
            self.armx(),
            self.oneshotx(),
            self.edgx0(),
            self.edgx1(),
            self.inp_selx(),
            self.edgcntx_en(),
            self.cfxwm(),
            self.cx0cnt(),
            self.cx1cnt()
        )
    }
}
#[doc = "Capture PWM_X Input Filter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1captfiltx(pub u16);
impl Sm1captfiltx {
    #[doc = "Input Capture Filter Period."]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period."]
    #[inline(always)]
    pub const fn set_captx_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count."]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count."]
    #[inline(always)]
    pub const fn set_captx_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm1captfiltx {
    #[inline(always)]
    fn default() -> Sm1captfiltx {
        Sm1captfiltx(0)
    }
}
impl core::fmt::Debug for Sm1captfiltx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1captfiltx")
            .field("captx_filt_per", &self.captx_filt_per())
            .field("captx_filt_cnt", &self.captx_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1captfiltx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1captfiltx {{ captx_filt_per: {=u8:?}, captx_filt_cnt: {=u8:?} }}",
            self.captx_filt_per(),
            self.captx_filt_cnt()
        )
    }
}
#[doc = "Counter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cnt(pub u16);
impl Sm1cnt {
    #[doc = "Counter Register Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter Register Bits."]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1cnt {
    #[inline(always)]
    fn default() -> Sm1cnt {
        Sm1cnt(0)
    }
}
impl core::fmt::Debug for Sm1cnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1cnt").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1cnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1cnt {{ cnt: {=u16:?} }}", self.cnt())
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1ctrl(pub u16);
impl Sm1ctrl {
    #[doc = "Double Switching Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dblen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Double Switching Enable."]
    #[inline(always)]
    pub const fn set_dblen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "PWM_X Double Switching Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dblx(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Double Switching Enable."]
    #[inline(always)]
    pub const fn set_dblx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Load Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ldmod(&self) -> Sm1ctrlLdmod {
        let val = (self.0 >> 2usize) & 0x01;
        Sm1ctrlLdmod::from_bits(val as u8)
    }
    #[doc = "Load Mode Select."]
    #[inline(always)]
    pub const fn set_ldmod(&mut self, val: Sm1ctrlLdmod) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B."]
    #[must_use]
    #[inline(always)]
    pub const fn split(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B."]
    #[inline(always)]
    pub const fn set_split(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn prsc(&self) -> Sm1ctrlPrsc {
        let val = (self.0 >> 4usize) & 0x07;
        Sm1ctrlPrsc::from_bits(val as u8)
    }
    #[doc = "Prescaler."]
    #[inline(always)]
    pub const fn set_prsc(&mut self, val: Sm1ctrlPrsc) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u16) & 0x07) << 4usize);
    }
    #[doc = "Compare Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn compmode(&self) -> Sm1ctrlCompmode {
        let val = (self.0 >> 7usize) & 0x01;
        Sm1ctrlCompmode::from_bits(val as u8)
    }
    #[doc = "Compare Mode."]
    #[inline(always)]
    pub const fn set_compmode(&mut self, val: Sm1ctrlCompmode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Deadtime."]
    #[must_use]
    #[inline(always)]
    pub const fn dt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Deadtime."]
    #[inline(always)]
    pub const fn set_dt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Full Cycle Reload."]
    #[must_use]
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Full Cycle Reload."]
    #[inline(always)]
    pub const fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Half Cycle Reload."]
    #[must_use]
    #[inline(always)]
    pub const fn half(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle Reload."]
    #[inline(always)]
    pub const fn set_half(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Load Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn ldfq(&self) -> Sm1ctrlLdfq {
        let val = (self.0 >> 12usize) & 0x0f;
        Sm1ctrlLdfq::from_bits(val as u8)
    }
    #[doc = "Load Frequency."]
    #[inline(always)]
    pub const fn set_ldfq(&mut self, val: Sm1ctrlLdfq) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Sm1ctrl {
    #[inline(always)]
    fn default() -> Sm1ctrl {
        Sm1ctrl(0)
    }
}
impl core::fmt::Debug for Sm1ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1ctrl")
            .field("dblen", &self.dblen())
            .field("dblx", &self.dblx())
            .field("ldmod", &self.ldmod())
            .field("split", &self.split())
            .field("prsc", &self.prsc())
            .field("compmode", &self.compmode())
            .field("dt", &self.dt())
            .field("full", &self.full())
            .field("half", &self.half())
            .field("ldfq", &self.ldfq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1ctrl {{ dblen: {=bool:?}, dblx: {=bool:?}, ldmod: {:?}, split: {=bool:?}, prsc: {:?}, compmode: {:?}, dt: {=u8:?}, full: {=bool:?}, half: {=bool:?}, ldfq: {:?} }}",
            self.dblen(),
            self.dblx(),
            self.ldmod(),
            self.split(),
            self.prsc(),
            self.compmode(),
            self.dt(),
            self.full(),
            self.half(),
            self.ldfq()
        )
    }
}
#[doc = "Control 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1ctrl2(pub u16);
impl Sm1ctrl2 {
    #[doc = "Clock Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel(&self) -> Sm1ctrl2ClkSel {
        let val = (self.0 >> 0usize) & 0x03;
        Sm1ctrl2ClkSel::from_bits(val as u8)
    }
    #[doc = "Clock Source Select."]
    #[inline(always)]
    pub const fn set_clk_sel(&mut self, val: Sm1ctrl2ClkSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Reload Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn reload_sel(&self) -> Sm1ctrl2ReloadSel {
        let val = (self.0 >> 2usize) & 0x01;
        Sm1ctrl2ReloadSel::from_bits(val as u8)
    }
    #[doc = "Reload Source Select."]
    #[inline(always)]
    pub const fn set_reload_sel(&mut self, val: Sm1ctrl2ReloadSel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Force Select."]
    #[must_use]
    #[inline(always)]
    pub const fn force_sel(&self) -> Sm1ctrl2ForceSel {
        let val = (self.0 >> 3usize) & 0x07;
        Sm1ctrl2ForceSel::from_bits(val as u8)
    }
    #[doc = "Force Select."]
    #[inline(always)]
    pub const fn set_force_sel(&mut self, val: Sm1ctrl2ForceSel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u16) & 0x07) << 3usize);
    }
    #[doc = "Force Initialization."]
    #[must_use]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Force Initialization."]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Force Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn frcen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Enable."]
    #[inline(always)]
    pub const fn set_frcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Initialization Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn init_sel(&self) -> Sm1ctrl2InitSel {
        let val = (self.0 >> 8usize) & 0x03;
        Sm1ctrl2InitSel::from_bits(val as u8)
    }
    #[doc = "Initialization Control Select."]
    #[inline(always)]
    pub const fn set_init_sel(&mut self, val: Sm1ctrl2InitSel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "PWM_X Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_init(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Initial Value."]
    #[inline(always)]
    pub const fn set_pwmx_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM45 Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm45_init(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PWM45 Initial Value."]
    #[inline(always)]
    pub const fn set_pwm45_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "PWM23 Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm23_init(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PWM23 Initial Value."]
    #[inline(always)]
    pub const fn set_pwm23_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Independent or Complementary Pair Operation."]
    #[must_use]
    #[inline(always)]
    pub const fn indep(&self) -> Sm1ctrl2Indep {
        let val = (self.0 >> 13usize) & 0x01;
        Sm1ctrl2Indep::from_bits(val as u8)
    }
    #[doc = "Independent or Complementary Pair Operation."]
    #[inline(always)]
    pub const fn set_indep(&mut self, val: Sm1ctrl2Indep) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Debug Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable."]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm1ctrl2 {
    #[inline(always)]
    fn default() -> Sm1ctrl2 {
        Sm1ctrl2(0)
    }
}
impl core::fmt::Debug for Sm1ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1ctrl2")
            .field("clk_sel", &self.clk_sel())
            .field("reload_sel", &self.reload_sel())
            .field("force_sel", &self.force_sel())
            .field("force", &self.force())
            .field("frcen", &self.frcen())
            .field("init_sel", &self.init_sel())
            .field("pwmx_init", &self.pwmx_init())
            .field("pwm45_init", &self.pwm45_init())
            .field("pwm23_init", &self.pwm23_init())
            .field("indep", &self.indep())
            .field("dbgen", &self.dbgen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1ctrl2 {{ clk_sel: {:?}, reload_sel: {:?}, force_sel: {:?}, force: {=bool:?}, frcen: {=bool:?}, init_sel: {:?}, pwmx_init: {=bool:?}, pwm45_init: {=bool:?}, pwm23_init: {=bool:?}, indep: {:?}, dbgen: {=bool:?} }}",
            self.clk_sel(),
            self.reload_sel(),
            self.force_sel(),
            self.force(),
            self.frcen(),
            self.init_sel(),
            self.pwmx_init(),
            self.pwm45_init(),
            self.pwm23_init(),
            self.indep(),
            self.dbgen()
        )
    }
}
#[doc = "Capture Value 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval0(pub u16);
impl Sm1cval0 {
    #[doc = "Capture Value 0."]
    #[must_use]
    #[inline(always)]
    pub const fn captval0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 0."]
    #[inline(always)]
    pub const fn set_captval0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1cval0 {
    #[inline(always)]
    fn default() -> Sm1cval0 {
        Sm1cval0(0)
    }
}
impl core::fmt::Debug for Sm1cval0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1cval0")
            .field("captval0", &self.captval0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1cval0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1cval0 {{ captval0: {=u16:?} }}", self.captval0())
    }
}
#[doc = "Capture Value 0 Cycle Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval0cyc(pub u16);
impl Sm1cval0cyc {
    #[doc = "Capture Value 0 Cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn cval0cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 0 Cycle."]
    #[inline(always)]
    pub const fn set_cval0cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm1cval0cyc {
    #[inline(always)]
    fn default() -> Sm1cval0cyc {
        Sm1cval0cyc(0)
    }
}
impl core::fmt::Debug for Sm1cval0cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1cval0cyc")
            .field("cval0cyc", &self.cval0cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1cval0cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1cval0cyc {{ cval0cyc: {=u8:?} }}", self.cval0cyc())
    }
}
#[doc = "Capture Value 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval1(pub u16);
impl Sm1cval1 {
    #[doc = "Capture Value 1."]
    #[must_use]
    #[inline(always)]
    pub const fn captval1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 1."]
    #[inline(always)]
    pub const fn set_captval1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1cval1 {
    #[inline(always)]
    fn default() -> Sm1cval1 {
        Sm1cval1(0)
    }
}
impl core::fmt::Debug for Sm1cval1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1cval1")
            .field("captval1", &self.captval1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1cval1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1cval1 {{ captval1: {=u16:?} }}", self.captval1())
    }
}
#[doc = "Capture Value 1 Cycle Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1cval1cyc(pub u16);
impl Sm1cval1cyc {
    #[doc = "Capture Value 1 Cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn cval1cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 1 Cycle."]
    #[inline(always)]
    pub const fn set_cval1cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm1cval1cyc {
    #[inline(always)]
    fn default() -> Sm1cval1cyc {
        Sm1cval1cyc(0)
    }
}
impl core::fmt::Debug for Sm1cval1cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1cval1cyc")
            .field("cval1cyc", &self.cval1cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1cval1cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1cval1cyc {{ cval1cyc: {=u8:?} }}", self.cval1cyc())
    }
}
#[doc = "Fault Disable Mapping Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1dismap0(pub u16);
impl Sm1dismap0 {
    #[doc = "PWM_A Fault Disable Mask 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dis0a(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Fault Disable Mask 0."]
    #[inline(always)]
    pub const fn set_dis0a(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Fault Disable Mask 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dis0b(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Fault Disable Mask 0."]
    #[inline(always)]
    pub const fn set_dis0b(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PWM_X Fault Disable Mask 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dis0x(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Fault Disable Mask 0."]
    #[inline(always)]
    pub const fn set_dis0x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
}
impl Default for Sm1dismap0 {
    #[inline(always)]
    fn default() -> Sm1dismap0 {
        Sm1dismap0(0)
    }
}
impl core::fmt::Debug for Sm1dismap0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1dismap0")
            .field("dis0a", &self.dis0a())
            .field("dis0b", &self.dis0b())
            .field("dis0x", &self.dis0x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1dismap0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1dismap0 {{ dis0a: {=u8:?}, dis0b: {=u8:?}, dis0x: {=u8:?} }}",
            self.dis0a(),
            self.dis0b(),
            self.dis0x()
        )
    }
}
#[doc = "DMA Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1dmaen(pub u16);
impl Sm1dmaen {
    #[doc = "Capture X0 FIFO DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx0de(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X0 FIFO DMA Enable."]
    #[inline(always)]
    pub const fn set_cx0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Capture X1 FIFO DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X1 FIFO DMA Enable."]
    #[inline(always)]
    pub const fn set_cx1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Capture DMA Enable Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn captde(&self) -> Sm1dmaenCaptde {
        let val = (self.0 >> 6usize) & 0x03;
        Sm1dmaenCaptde::from_bits(val as u8)
    }
    #[doc = "Capture DMA Enable Source Select."]
    #[inline(always)]
    pub const fn set_captde(&mut self, val: Sm1dmaenCaptde) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "FIFO Watermark AND Control."]
    #[must_use]
    #[inline(always)]
    pub const fn fand(&self) -> Sm1dmaenFand {
        let val = (self.0 >> 8usize) & 0x01;
        Sm1dmaenFand::from_bits(val as u8)
    }
    #[doc = "FIFO Watermark AND Control."]
    #[inline(always)]
    pub const fn set_fand(&mut self, val: Sm1dmaenFand) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Value Registers DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn valde(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Value Registers DMA Enable."]
    #[inline(always)]
    pub const fn set_valde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
}
impl Default for Sm1dmaen {
    #[inline(always)]
    fn default() -> Sm1dmaen {
        Sm1dmaen(0)
    }
}
impl core::fmt::Debug for Sm1dmaen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1dmaen")
            .field("cx0de", &self.cx0de())
            .field("cx1de", &self.cx1de())
            .field("captde", &self.captde())
            .field("fand", &self.fand())
            .field("valde", &self.valde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1dmaen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1dmaen {{ cx0de: {=bool:?}, cx1de: {=bool:?}, captde: {:?}, fand: {:?}, valde: {=bool:?} }}",
            self.cx0de(),
            self.cx1de(),
            self.captde(),
            self.fand(),
            self.valde()
        )
    }
}
#[doc = "Deadtime Count Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1dtcnt0(pub u16);
impl Sm1dtcnt0 {
    #[doc = "Deadtime Count Register 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnt0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Deadtime Count Register 0."]
    #[inline(always)]
    pub const fn set_dtcnt0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
    }
}
impl Default for Sm1dtcnt0 {
    #[inline(always)]
    fn default() -> Sm1dtcnt0 {
        Sm1dtcnt0(0)
    }
}
impl core::fmt::Debug for Sm1dtcnt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1dtcnt0")
            .field("dtcnt0", &self.dtcnt0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1dtcnt0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1dtcnt0 {{ dtcnt0: {=u16:?} }}", self.dtcnt0())
    }
}
#[doc = "Deadtime Count Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1dtcnt1(pub u16);
impl Sm1dtcnt1 {
    #[doc = "Deadtime Count Register 1."]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnt1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Deadtime Count Register 1."]
    #[inline(always)]
    pub const fn set_dtcnt1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
    }
}
impl Default for Sm1dtcnt1 {
    #[inline(always)]
    fn default() -> Sm1dtcnt1 {
        Sm1dtcnt1(0)
    }
}
impl core::fmt::Debug for Sm1dtcnt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1dtcnt1")
            .field("dtcnt1", &self.dtcnt1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1dtcnt1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1dtcnt1 {{ dtcnt1: {=u16:?} }}", self.dtcnt1())
    }
}
#[doc = "Initial Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1init(pub u16);
impl Sm1init {
    #[doc = "Initial Count Register Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn init(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Initial Count Register Bits."]
    #[inline(always)]
    pub const fn set_init(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1init {
    #[inline(always)]
    fn default() -> Sm1init {
        Sm1init(0)
    }
}
impl core::fmt::Debug for Sm1init {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1init")
            .field("init", &self.init())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1init {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1init {{ init: {=u16:?} }}", self.init())
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1inten(pub u16);
impl Sm1inten {
    #[doc = "Compare Interrupt Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpie(&self) -> Sm1intenCmpie {
        let val = (self.0 >> 0usize) & 0x3f;
        Sm1intenCmpie::from_bits(val as u8)
    }
    #[doc = "Compare Interrupt Enables."]
    #[inline(always)]
    pub const fn set_cmpie(&mut self, val: Sm1intenCmpie) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture X 0 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx0ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 0 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cx0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture X 1 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx1ie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 1 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cx1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Reload Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn reie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_reie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
}
impl Default for Sm1inten {
    #[inline(always)]
    fn default() -> Sm1inten {
        Sm1inten(0)
    }
}
impl core::fmt::Debug for Sm1inten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1inten")
            .field("cmpie", &self.cmpie())
            .field("cx0ie", &self.cx0ie())
            .field("cx1ie", &self.cx1ie())
            .field("rie", &self.rie())
            .field("reie", &self.reie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1inten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1inten {{ cmpie: {:?}, cx0ie: {=bool:?}, cx1ie: {=bool:?}, rie: {=bool:?}, reie: {=bool:?} }}",
            self.cmpie(),
            self.cx0ie(),
            self.cx1ie(),
            self.rie(),
            self.reie()
        )
    }
}
#[doc = "Output Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1octrl(pub u16);
impl Sm1octrl {
    #[doc = "PWM_X Fault State."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmxfs(&self) -> Sm1octrlPwmxfs {
        let val = (self.0 >> 0usize) & 0x03;
        Sm1octrlPwmxfs::from_bits(val as u8)
    }
    #[doc = "PWM_X Fault State."]
    #[inline(always)]
    pub const fn set_pwmxfs(&mut self, val: Sm1octrlPwmxfs) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "PWM_B Fault State."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmbfs(&self) -> Sm1octrlPwmbfs {
        let val = (self.0 >> 2usize) & 0x03;
        Sm1octrlPwmbfs::from_bits(val as u8)
    }
    #[doc = "PWM_B Fault State."]
    #[inline(always)]
    pub const fn set_pwmbfs(&mut self, val: Sm1octrlPwmbfs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "PWM_A Fault State."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmafs(&self) -> Sm1octrlPwmafs {
        let val = (self.0 >> 4usize) & 0x03;
        Sm1octrlPwmafs::from_bits(val as u8)
    }
    #[doc = "PWM_A Fault State."]
    #[inline(always)]
    pub const fn set_pwmafs(&mut self, val: Sm1octrlPwmafs) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "PWM_X Output Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn polx(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Output Polarity."]
    #[inline(always)]
    pub const fn set_polx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "PWM_B Output Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn polb(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Output Polarity."]
    #[inline(always)]
    pub const fn set_polb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "PWM_A Output Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn pola(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Output Polarity."]
    #[inline(always)]
    pub const fn set_pola(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM_X Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_in(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Input."]
    #[inline(always)]
    pub const fn set_pwmx_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "PWM_B Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmb_in(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Input."]
    #[inline(always)]
    pub const fn set_pwmb_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "PWM_A Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pwma_in(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Input."]
    #[inline(always)]
    pub const fn set_pwma_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm1octrl {
    #[inline(always)]
    fn default() -> Sm1octrl {
        Sm1octrl(0)
    }
}
impl core::fmt::Debug for Sm1octrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1octrl")
            .field("pwmxfs", &self.pwmxfs())
            .field("pwmbfs", &self.pwmbfs())
            .field("pwmafs", &self.pwmafs())
            .field("polx", &self.polx())
            .field("polb", &self.polb())
            .field("pola", &self.pola())
            .field("pwmx_in", &self.pwmx_in())
            .field("pwmb_in", &self.pwmb_in())
            .field("pwma_in", &self.pwma_in())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1octrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1octrl {{ pwmxfs: {:?}, pwmbfs: {:?}, pwmafs: {:?}, polx: {=bool:?}, polb: {=bool:?}, pola: {=bool:?}, pwmx_in: {=bool:?}, pwmb_in: {=bool:?}, pwma_in: {=bool:?} }}",
            self.pwmxfs(),
            self.pwmbfs(),
            self.pwmafs(),
            self.polx(),
            self.polb(),
            self.pola(),
            self.pwmx_in(),
            self.pwmb_in(),
            self.pwma_in()
        )
    }
}
#[doc = "Phase Delay Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1phasedly(pub u16);
impl Sm1phasedly {
    #[doc = "Initial Count Register Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn phasedly(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Initial Count Register Bits."]
    #[inline(always)]
    pub const fn set_phasedly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1phasedly {
    #[inline(always)]
    fn default() -> Sm1phasedly {
        Sm1phasedly(0)
    }
}
impl core::fmt::Debug for Sm1phasedly {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1phasedly")
            .field("phasedly", &self.phasedly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1phasedly {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1phasedly {{ phasedly: {=u16:?} }}", self.phasedly())
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1sts(pub u16);
impl Sm1sts {
    #[doc = "Compare Flags."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpf(&self) -> Sm1stsCmpf {
        let val = (self.0 >> 0usize) & 0x3f;
        Sm1stsCmpf::from_bits(val as u8)
    }
    #[doc = "Compare Flags."]
    #[inline(always)]
    pub const fn set_cmpf(&mut self, val: Sm1stsCmpf) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture Flag X0."]
    #[must_use]
    #[inline(always)]
    pub const fn cfx0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X0."]
    #[inline(always)]
    pub const fn set_cfx0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture Flag X1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfx1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X1."]
    #[inline(always)]
    pub const fn set_cfx1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Reload Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Flag."]
    #[inline(always)]
    pub const fn set_rf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ref_(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Flag."]
    #[inline(always)]
    pub const fn set_ref_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Registers Updated Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ruf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Registers Updated Flag."]
    #[inline(always)]
    pub const fn set_ruf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
}
impl Default for Sm1sts {
    #[inline(always)]
    fn default() -> Sm1sts {
        Sm1sts(0)
    }
}
impl core::fmt::Debug for Sm1sts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1sts")
            .field("cmpf", &self.cmpf())
            .field("cfx0", &self.cfx0())
            .field("cfx1", &self.cfx1())
            .field("rf", &self.rf())
            .field("ref_", &self.ref_())
            .field("ruf", &self.ruf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1sts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1sts {{ cmpf: {:?}, cfx0: {=bool:?}, cfx1: {=bool:?}, rf: {=bool:?}, ref_: {=bool:?}, ruf: {=bool:?} }}",
            self.cmpf(),
            self.cfx0(),
            self.cfx1(),
            self.rf(),
            self.ref_(),
            self.ruf()
        )
    }
}
#[doc = "Output Trigger Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1tctrl(pub u16);
impl Sm1tctrl {
    #[doc = "Output Trigger Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn out_trig_en(&self) -> Sm1tctrlOutTrigEn {
        let val = (self.0 >> 0usize) & 0x3f;
        Sm1tctrlOutTrigEn::from_bits(val as u8)
    }
    #[doc = "Output Trigger Enables."]
    #[inline(always)]
    pub const fn set_out_trig_en(&mut self, val: Sm1tctrlOutTrigEn) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Trigger Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn trgfrq(&self) -> Sm1tctrlTrgfrq {
        let val = (self.0 >> 12usize) & 0x01;
        Sm1tctrlTrgfrq::from_bits(val as u8)
    }
    #[doc = "Trigger Frequency."]
    #[inline(always)]
    pub const fn set_trgfrq(&mut self, val: Sm1tctrlTrgfrq) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Mux Output Trigger 1 Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pwbot1(&self) -> Sm1tctrlPwbot1 {
        let val = (self.0 >> 14usize) & 0x01;
        Sm1tctrlPwbot1::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 1 Source Select."]
    #[inline(always)]
    pub const fn set_pwbot1(&mut self, val: Sm1tctrlPwbot1) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Mux Output Trigger 0 Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pwaot0(&self) -> Sm1tctrlPwaot0 {
        let val = (self.0 >> 15usize) & 0x01;
        Sm1tctrlPwaot0::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 0 Source Select."]
    #[inline(always)]
    pub const fn set_pwaot0(&mut self, val: Sm1tctrlPwaot0) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm1tctrl {
    #[inline(always)]
    fn default() -> Sm1tctrl {
        Sm1tctrl(0)
    }
}
impl core::fmt::Debug for Sm1tctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1tctrl")
            .field("out_trig_en", &self.out_trig_en())
            .field("trgfrq", &self.trgfrq())
            .field("pwbot1", &self.pwbot1())
            .field("pwaot0", &self.pwaot0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1tctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm1tctrl {{ out_trig_en: {:?}, trgfrq: {:?}, pwbot1: {:?}, pwaot0: {:?} }}",
            self.out_trig_en(),
            self.trgfrq(),
            self.pwbot1(),
            self.pwaot0()
        )
    }
}
#[doc = "Value Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1val0(pub u16);
impl Sm1val0 {
    #[doc = "Value 0."]
    #[must_use]
    #[inline(always)]
    pub const fn val0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 0."]
    #[inline(always)]
    pub const fn set_val0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1val0 {
    #[inline(always)]
    fn default() -> Sm1val0 {
        Sm1val0(0)
    }
}
impl core::fmt::Debug for Sm1val0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1val0")
            .field("val0", &self.val0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1val0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1val0 {{ val0: {=u16:?} }}", self.val0())
    }
}
#[doc = "Value Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1val1(pub u16);
impl Sm1val1 {
    #[doc = "Value 1."]
    #[must_use]
    #[inline(always)]
    pub const fn val1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 1."]
    #[inline(always)]
    pub const fn set_val1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1val1 {
    #[inline(always)]
    fn default() -> Sm1val1 {
        Sm1val1(0)
    }
}
impl core::fmt::Debug for Sm1val1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1val1")
            .field("val1", &self.val1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1val1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1val1 {{ val1: {=u16:?} }}", self.val1())
    }
}
#[doc = "Value Register 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1val2(pub u16);
impl Sm1val2 {
    #[doc = "Value 2."]
    #[must_use]
    #[inline(always)]
    pub const fn val2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 2."]
    #[inline(always)]
    pub const fn set_val2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1val2 {
    #[inline(always)]
    fn default() -> Sm1val2 {
        Sm1val2(0)
    }
}
impl core::fmt::Debug for Sm1val2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1val2")
            .field("val2", &self.val2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1val2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1val2 {{ val2: {=u16:?} }}", self.val2())
    }
}
#[doc = "Value Register 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1val3(pub u16);
impl Sm1val3 {
    #[doc = "Value 3."]
    #[must_use]
    #[inline(always)]
    pub const fn val3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 3."]
    #[inline(always)]
    pub const fn set_val3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1val3 {
    #[inline(always)]
    fn default() -> Sm1val3 {
        Sm1val3(0)
    }
}
impl core::fmt::Debug for Sm1val3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1val3")
            .field("val3", &self.val3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1val3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1val3 {{ val3: {=u16:?} }}", self.val3())
    }
}
#[doc = "Value Register 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1val4(pub u16);
impl Sm1val4 {
    #[doc = "Value 4."]
    #[must_use]
    #[inline(always)]
    pub const fn val4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 4."]
    #[inline(always)]
    pub const fn set_val4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1val4 {
    #[inline(always)]
    fn default() -> Sm1val4 {
        Sm1val4(0)
    }
}
impl core::fmt::Debug for Sm1val4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1val4")
            .field("val4", &self.val4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1val4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1val4 {{ val4: {=u16:?} }}", self.val4())
    }
}
#[doc = "Value Register 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm1val5(pub u16);
impl Sm1val5 {
    #[doc = "Value 5."]
    #[must_use]
    #[inline(always)]
    pub const fn val5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 5."]
    #[inline(always)]
    pub const fn set_val5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm1val5 {
    #[inline(always)]
    fn default() -> Sm1val5 {
        Sm1val5(0)
    }
}
impl core::fmt::Debug for Sm1val5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm1val5")
            .field("val5", &self.val5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm1val5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm1val5 {{ val5: {=u16:?} }}", self.val5())
    }
}
#[doc = "Capture Compare X Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2captcompx(pub u16);
impl Sm2captcompx {
    #[doc = "Edge Compare X."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare X."]
    #[inline(always)]
    pub const fn set_edgcmpx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter X."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter X."]
    #[inline(always)]
    pub const fn set_edgcntx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm2captcompx {
    #[inline(always)]
    fn default() -> Sm2captcompx {
        Sm2captcompx(0)
    }
}
impl core::fmt::Debug for Sm2captcompx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2captcompx")
            .field("edgcmpx", &self.edgcmpx())
            .field("edgcntx", &self.edgcntx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2captcompx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2captcompx {{ edgcmpx: {=u8:?}, edgcntx: {=u8:?} }}",
            self.edgcmpx(),
            self.edgcntx()
        )
    }
}
#[doc = "Capture Control X Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2captctrlx(pub u16);
impl Sm2captctrlx {
    #[doc = "Arm X."]
    #[must_use]
    #[inline(always)]
    pub const fn armx(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm X."]
    #[inline(always)]
    pub const fn set_armx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode Aux."]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotx(&self) -> Sm2captctrlxOneshotx {
        let val = (self.0 >> 1usize) & 0x01;
        Sm2captctrlxOneshotx::from_bits(val as u8)
    }
    #[doc = "One Shot Mode Aux."]
    #[inline(always)]
    pub const fn set_oneshotx(&mut self, val: Sm2captctrlxOneshotx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge X 0."]
    #[must_use]
    #[inline(always)]
    pub const fn edgx0(&self) -> Sm2captctrlxEdgx0 {
        let val = (self.0 >> 2usize) & 0x03;
        Sm2captctrlxEdgx0::from_bits(val as u8)
    }
    #[doc = "Edge X 0."]
    #[inline(always)]
    pub const fn set_edgx0(&mut self, val: Sm2captctrlxEdgx0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge X 1."]
    #[must_use]
    #[inline(always)]
    pub const fn edgx1(&self) -> Sm2captctrlxEdgx1 {
        let val = (self.0 >> 4usize) & 0x03;
        Sm2captctrlxEdgx1::from_bits(val as u8)
    }
    #[doc = "Edge X 1."]
    #[inline(always)]
    pub const fn set_edgx1(&mut self, val: Sm2captctrlxEdgx1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select X."]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selx(&self) -> Sm2captctrlxInpSelx {
        let val = (self.0 >> 6usize) & 0x01;
        Sm2captctrlxInpSelx::from_bits(val as u8)
    }
    #[doc = "Input Select X."]
    #[inline(always)]
    pub const fn set_inp_selx(&mut self, val: Sm2captctrlxInpSelx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter X Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter X Enable."]
    #[inline(always)]
    pub const fn set_edgcntx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture X FIFOs Water Mark."]
    #[must_use]
    #[inline(always)]
    pub const fn cfxwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture X FIFOs Water Mark."]
    #[inline(always)]
    pub const fn set_cfxwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture X0 FIFO Word Count."]
    #[must_use]
    #[inline(always)]
    pub const fn cx0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X0 FIFO Word Count."]
    #[inline(always)]
    pub const fn set_cx0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture X1 FIFO Word Count."]
    #[must_use]
    #[inline(always)]
    pub const fn cx1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X1 FIFO Word Count."]
    #[inline(always)]
    pub const fn set_cx1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm2captctrlx {
    #[inline(always)]
    fn default() -> Sm2captctrlx {
        Sm2captctrlx(0)
    }
}
impl core::fmt::Debug for Sm2captctrlx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2captctrlx")
            .field("armx", &self.armx())
            .field("oneshotx", &self.oneshotx())
            .field("edgx0", &self.edgx0())
            .field("edgx1", &self.edgx1())
            .field("inp_selx", &self.inp_selx())
            .field("edgcntx_en", &self.edgcntx_en())
            .field("cfxwm", &self.cfxwm())
            .field("cx0cnt", &self.cx0cnt())
            .field("cx1cnt", &self.cx1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2captctrlx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2captctrlx {{ armx: {=bool:?}, oneshotx: {:?}, edgx0: {:?}, edgx1: {:?}, inp_selx: {:?}, edgcntx_en: {=bool:?}, cfxwm: {=u8:?}, cx0cnt: {=u8:?}, cx1cnt: {=u8:?} }}",
            self.armx(),
            self.oneshotx(),
            self.edgx0(),
            self.edgx1(),
            self.inp_selx(),
            self.edgcntx_en(),
            self.cfxwm(),
            self.cx0cnt(),
            self.cx1cnt()
        )
    }
}
#[doc = "Capture PWM_X Input Filter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2captfiltx(pub u16);
impl Sm2captfiltx {
    #[doc = "Input Capture Filter Period."]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period."]
    #[inline(always)]
    pub const fn set_captx_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count."]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count."]
    #[inline(always)]
    pub const fn set_captx_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm2captfiltx {
    #[inline(always)]
    fn default() -> Sm2captfiltx {
        Sm2captfiltx(0)
    }
}
impl core::fmt::Debug for Sm2captfiltx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2captfiltx")
            .field("captx_filt_per", &self.captx_filt_per())
            .field("captx_filt_cnt", &self.captx_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2captfiltx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2captfiltx {{ captx_filt_per: {=u8:?}, captx_filt_cnt: {=u8:?} }}",
            self.captx_filt_per(),
            self.captx_filt_cnt()
        )
    }
}
#[doc = "Counter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cnt(pub u16);
impl Sm2cnt {
    #[doc = "Counter Register Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter Register Bits."]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2cnt {
    #[inline(always)]
    fn default() -> Sm2cnt {
        Sm2cnt(0)
    }
}
impl core::fmt::Debug for Sm2cnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2cnt").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2cnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2cnt {{ cnt: {=u16:?} }}", self.cnt())
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2ctrl(pub u16);
impl Sm2ctrl {
    #[doc = "Double Switching Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dblen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Double Switching Enable."]
    #[inline(always)]
    pub const fn set_dblen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "PWM_X Double Switching Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dblx(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Double Switching Enable."]
    #[inline(always)]
    pub const fn set_dblx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Load Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ldmod(&self) -> Sm2ctrlLdmod {
        let val = (self.0 >> 2usize) & 0x01;
        Sm2ctrlLdmod::from_bits(val as u8)
    }
    #[doc = "Load Mode Select."]
    #[inline(always)]
    pub const fn set_ldmod(&mut self, val: Sm2ctrlLdmod) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B."]
    #[must_use]
    #[inline(always)]
    pub const fn split(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B."]
    #[inline(always)]
    pub const fn set_split(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn prsc(&self) -> Sm2ctrlPrsc {
        let val = (self.0 >> 4usize) & 0x07;
        Sm2ctrlPrsc::from_bits(val as u8)
    }
    #[doc = "Prescaler."]
    #[inline(always)]
    pub const fn set_prsc(&mut self, val: Sm2ctrlPrsc) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u16) & 0x07) << 4usize);
    }
    #[doc = "Compare Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn compmode(&self) -> Sm2ctrlCompmode {
        let val = (self.0 >> 7usize) & 0x01;
        Sm2ctrlCompmode::from_bits(val as u8)
    }
    #[doc = "Compare Mode."]
    #[inline(always)]
    pub const fn set_compmode(&mut self, val: Sm2ctrlCompmode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Deadtime."]
    #[must_use]
    #[inline(always)]
    pub const fn dt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Deadtime."]
    #[inline(always)]
    pub const fn set_dt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Full Cycle Reload."]
    #[must_use]
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Full Cycle Reload."]
    #[inline(always)]
    pub const fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Half Cycle Reload."]
    #[must_use]
    #[inline(always)]
    pub const fn half(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle Reload."]
    #[inline(always)]
    pub const fn set_half(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Load Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn ldfq(&self) -> Sm2ctrlLdfq {
        let val = (self.0 >> 12usize) & 0x0f;
        Sm2ctrlLdfq::from_bits(val as u8)
    }
    #[doc = "Load Frequency."]
    #[inline(always)]
    pub const fn set_ldfq(&mut self, val: Sm2ctrlLdfq) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Sm2ctrl {
    #[inline(always)]
    fn default() -> Sm2ctrl {
        Sm2ctrl(0)
    }
}
impl core::fmt::Debug for Sm2ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2ctrl")
            .field("dblen", &self.dblen())
            .field("dblx", &self.dblx())
            .field("ldmod", &self.ldmod())
            .field("split", &self.split())
            .field("prsc", &self.prsc())
            .field("compmode", &self.compmode())
            .field("dt", &self.dt())
            .field("full", &self.full())
            .field("half", &self.half())
            .field("ldfq", &self.ldfq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2ctrl {{ dblen: {=bool:?}, dblx: {=bool:?}, ldmod: {:?}, split: {=bool:?}, prsc: {:?}, compmode: {:?}, dt: {=u8:?}, full: {=bool:?}, half: {=bool:?}, ldfq: {:?} }}",
            self.dblen(),
            self.dblx(),
            self.ldmod(),
            self.split(),
            self.prsc(),
            self.compmode(),
            self.dt(),
            self.full(),
            self.half(),
            self.ldfq()
        )
    }
}
#[doc = "Control 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2ctrl2(pub u16);
impl Sm2ctrl2 {
    #[doc = "Clock Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel(&self) -> Sm2ctrl2ClkSel {
        let val = (self.0 >> 0usize) & 0x03;
        Sm2ctrl2ClkSel::from_bits(val as u8)
    }
    #[doc = "Clock Source Select."]
    #[inline(always)]
    pub const fn set_clk_sel(&mut self, val: Sm2ctrl2ClkSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Reload Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn reload_sel(&self) -> Sm2ctrl2ReloadSel {
        let val = (self.0 >> 2usize) & 0x01;
        Sm2ctrl2ReloadSel::from_bits(val as u8)
    }
    #[doc = "Reload Source Select."]
    #[inline(always)]
    pub const fn set_reload_sel(&mut self, val: Sm2ctrl2ReloadSel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Force Select."]
    #[must_use]
    #[inline(always)]
    pub const fn force_sel(&self) -> Sm2ctrl2ForceSel {
        let val = (self.0 >> 3usize) & 0x07;
        Sm2ctrl2ForceSel::from_bits(val as u8)
    }
    #[doc = "Force Select."]
    #[inline(always)]
    pub const fn set_force_sel(&mut self, val: Sm2ctrl2ForceSel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u16) & 0x07) << 3usize);
    }
    #[doc = "Force Initialization."]
    #[must_use]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Force Initialization."]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Force Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn frcen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Enable."]
    #[inline(always)]
    pub const fn set_frcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Initialization Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn init_sel(&self) -> Sm2ctrl2InitSel {
        let val = (self.0 >> 8usize) & 0x03;
        Sm2ctrl2InitSel::from_bits(val as u8)
    }
    #[doc = "Initialization Control Select."]
    #[inline(always)]
    pub const fn set_init_sel(&mut self, val: Sm2ctrl2InitSel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "PWM_X Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_init(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Initial Value."]
    #[inline(always)]
    pub const fn set_pwmx_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM45 Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm45_init(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PWM45 Initial Value."]
    #[inline(always)]
    pub const fn set_pwm45_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "PWM23 Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm23_init(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PWM23 Initial Value."]
    #[inline(always)]
    pub const fn set_pwm23_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Independent or Complementary Pair Operation."]
    #[must_use]
    #[inline(always)]
    pub const fn indep(&self) -> Sm2ctrl2Indep {
        let val = (self.0 >> 13usize) & 0x01;
        Sm2ctrl2Indep::from_bits(val as u8)
    }
    #[doc = "Independent or Complementary Pair Operation."]
    #[inline(always)]
    pub const fn set_indep(&mut self, val: Sm2ctrl2Indep) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Debug Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable."]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm2ctrl2 {
    #[inline(always)]
    fn default() -> Sm2ctrl2 {
        Sm2ctrl2(0)
    }
}
impl core::fmt::Debug for Sm2ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2ctrl2")
            .field("clk_sel", &self.clk_sel())
            .field("reload_sel", &self.reload_sel())
            .field("force_sel", &self.force_sel())
            .field("force", &self.force())
            .field("frcen", &self.frcen())
            .field("init_sel", &self.init_sel())
            .field("pwmx_init", &self.pwmx_init())
            .field("pwm45_init", &self.pwm45_init())
            .field("pwm23_init", &self.pwm23_init())
            .field("indep", &self.indep())
            .field("dbgen", &self.dbgen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2ctrl2 {{ clk_sel: {:?}, reload_sel: {:?}, force_sel: {:?}, force: {=bool:?}, frcen: {=bool:?}, init_sel: {:?}, pwmx_init: {=bool:?}, pwm45_init: {=bool:?}, pwm23_init: {=bool:?}, indep: {:?}, dbgen: {=bool:?} }}",
            self.clk_sel(),
            self.reload_sel(),
            self.force_sel(),
            self.force(),
            self.frcen(),
            self.init_sel(),
            self.pwmx_init(),
            self.pwm45_init(),
            self.pwm23_init(),
            self.indep(),
            self.dbgen()
        )
    }
}
#[doc = "Capture Value 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval0(pub u16);
impl Sm2cval0 {
    #[doc = "Capture Value 0."]
    #[must_use]
    #[inline(always)]
    pub const fn captval0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 0."]
    #[inline(always)]
    pub const fn set_captval0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2cval0 {
    #[inline(always)]
    fn default() -> Sm2cval0 {
        Sm2cval0(0)
    }
}
impl core::fmt::Debug for Sm2cval0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2cval0")
            .field("captval0", &self.captval0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2cval0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2cval0 {{ captval0: {=u16:?} }}", self.captval0())
    }
}
#[doc = "Capture Value 0 Cycle Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval0cyc(pub u16);
impl Sm2cval0cyc {
    #[doc = "Capture Value 0 Cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn cval0cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 0 Cycle."]
    #[inline(always)]
    pub const fn set_cval0cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm2cval0cyc {
    #[inline(always)]
    fn default() -> Sm2cval0cyc {
        Sm2cval0cyc(0)
    }
}
impl core::fmt::Debug for Sm2cval0cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2cval0cyc")
            .field("cval0cyc", &self.cval0cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2cval0cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2cval0cyc {{ cval0cyc: {=u8:?} }}", self.cval0cyc())
    }
}
#[doc = "Capture Value 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval1(pub u16);
impl Sm2cval1 {
    #[doc = "Capture Value 1."]
    #[must_use]
    #[inline(always)]
    pub const fn captval1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 1."]
    #[inline(always)]
    pub const fn set_captval1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2cval1 {
    #[inline(always)]
    fn default() -> Sm2cval1 {
        Sm2cval1(0)
    }
}
impl core::fmt::Debug for Sm2cval1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2cval1")
            .field("captval1", &self.captval1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2cval1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2cval1 {{ captval1: {=u16:?} }}", self.captval1())
    }
}
#[doc = "Capture Value 1 Cycle Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2cval1cyc(pub u16);
impl Sm2cval1cyc {
    #[doc = "Capture Value 1 Cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn cval1cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 1 Cycle."]
    #[inline(always)]
    pub const fn set_cval1cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm2cval1cyc {
    #[inline(always)]
    fn default() -> Sm2cval1cyc {
        Sm2cval1cyc(0)
    }
}
impl core::fmt::Debug for Sm2cval1cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2cval1cyc")
            .field("cval1cyc", &self.cval1cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2cval1cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2cval1cyc {{ cval1cyc: {=u8:?} }}", self.cval1cyc())
    }
}
#[doc = "Fault Disable Mapping Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2dismap0(pub u16);
impl Sm2dismap0 {
    #[doc = "PWM_A Fault Disable Mask 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dis0a(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Fault Disable Mask 0."]
    #[inline(always)]
    pub const fn set_dis0a(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Fault Disable Mask 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dis0b(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Fault Disable Mask 0."]
    #[inline(always)]
    pub const fn set_dis0b(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PWM_X Fault Disable Mask 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dis0x(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Fault Disable Mask 0."]
    #[inline(always)]
    pub const fn set_dis0x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
}
impl Default for Sm2dismap0 {
    #[inline(always)]
    fn default() -> Sm2dismap0 {
        Sm2dismap0(0)
    }
}
impl core::fmt::Debug for Sm2dismap0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2dismap0")
            .field("dis0a", &self.dis0a())
            .field("dis0b", &self.dis0b())
            .field("dis0x", &self.dis0x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2dismap0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2dismap0 {{ dis0a: {=u8:?}, dis0b: {=u8:?}, dis0x: {=u8:?} }}",
            self.dis0a(),
            self.dis0b(),
            self.dis0x()
        )
    }
}
#[doc = "DMA Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2dmaen(pub u16);
impl Sm2dmaen {
    #[doc = "Capture X0 FIFO DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx0de(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X0 FIFO DMA Enable."]
    #[inline(always)]
    pub const fn set_cx0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Capture X1 FIFO DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X1 FIFO DMA Enable."]
    #[inline(always)]
    pub const fn set_cx1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Capture DMA Enable Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn captde(&self) -> Sm2dmaenCaptde {
        let val = (self.0 >> 6usize) & 0x03;
        Sm2dmaenCaptde::from_bits(val as u8)
    }
    #[doc = "Capture DMA Enable Source Select."]
    #[inline(always)]
    pub const fn set_captde(&mut self, val: Sm2dmaenCaptde) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "FIFO Watermark AND Control."]
    #[must_use]
    #[inline(always)]
    pub const fn fand(&self) -> Sm2dmaenFand {
        let val = (self.0 >> 8usize) & 0x01;
        Sm2dmaenFand::from_bits(val as u8)
    }
    #[doc = "FIFO Watermark AND Control."]
    #[inline(always)]
    pub const fn set_fand(&mut self, val: Sm2dmaenFand) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Value Registers DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn valde(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Value Registers DMA Enable."]
    #[inline(always)]
    pub const fn set_valde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
}
impl Default for Sm2dmaen {
    #[inline(always)]
    fn default() -> Sm2dmaen {
        Sm2dmaen(0)
    }
}
impl core::fmt::Debug for Sm2dmaen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2dmaen")
            .field("cx0de", &self.cx0de())
            .field("cx1de", &self.cx1de())
            .field("captde", &self.captde())
            .field("fand", &self.fand())
            .field("valde", &self.valde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2dmaen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2dmaen {{ cx0de: {=bool:?}, cx1de: {=bool:?}, captde: {:?}, fand: {:?}, valde: {=bool:?} }}",
            self.cx0de(),
            self.cx1de(),
            self.captde(),
            self.fand(),
            self.valde()
        )
    }
}
#[doc = "Deadtime Count Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2dtcnt0(pub u16);
impl Sm2dtcnt0 {
    #[doc = "Deadtime Count Register 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnt0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Deadtime Count Register 0."]
    #[inline(always)]
    pub const fn set_dtcnt0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
    }
}
impl Default for Sm2dtcnt0 {
    #[inline(always)]
    fn default() -> Sm2dtcnt0 {
        Sm2dtcnt0(0)
    }
}
impl core::fmt::Debug for Sm2dtcnt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2dtcnt0")
            .field("dtcnt0", &self.dtcnt0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2dtcnt0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2dtcnt0 {{ dtcnt0: {=u16:?} }}", self.dtcnt0())
    }
}
#[doc = "Deadtime Count Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2dtcnt1(pub u16);
impl Sm2dtcnt1 {
    #[doc = "Deadtime Count Register 1."]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnt1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Deadtime Count Register 1."]
    #[inline(always)]
    pub const fn set_dtcnt1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
    }
}
impl Default for Sm2dtcnt1 {
    #[inline(always)]
    fn default() -> Sm2dtcnt1 {
        Sm2dtcnt1(0)
    }
}
impl core::fmt::Debug for Sm2dtcnt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2dtcnt1")
            .field("dtcnt1", &self.dtcnt1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2dtcnt1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2dtcnt1 {{ dtcnt1: {=u16:?} }}", self.dtcnt1())
    }
}
#[doc = "Initial Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2init(pub u16);
impl Sm2init {
    #[doc = "Initial Count Register Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn init(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Initial Count Register Bits."]
    #[inline(always)]
    pub const fn set_init(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2init {
    #[inline(always)]
    fn default() -> Sm2init {
        Sm2init(0)
    }
}
impl core::fmt::Debug for Sm2init {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2init")
            .field("init", &self.init())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2init {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2init {{ init: {=u16:?} }}", self.init())
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2inten(pub u16);
impl Sm2inten {
    #[doc = "Compare Interrupt Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpie(&self) -> Sm2intenCmpie {
        let val = (self.0 >> 0usize) & 0x3f;
        Sm2intenCmpie::from_bits(val as u8)
    }
    #[doc = "Compare Interrupt Enables."]
    #[inline(always)]
    pub const fn set_cmpie(&mut self, val: Sm2intenCmpie) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture X 0 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx0ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 0 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cx0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture X 1 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx1ie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 1 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cx1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Reload Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn reie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_reie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
}
impl Default for Sm2inten {
    #[inline(always)]
    fn default() -> Sm2inten {
        Sm2inten(0)
    }
}
impl core::fmt::Debug for Sm2inten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2inten")
            .field("cmpie", &self.cmpie())
            .field("cx0ie", &self.cx0ie())
            .field("cx1ie", &self.cx1ie())
            .field("rie", &self.rie())
            .field("reie", &self.reie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2inten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2inten {{ cmpie: {:?}, cx0ie: {=bool:?}, cx1ie: {=bool:?}, rie: {=bool:?}, reie: {=bool:?} }}",
            self.cmpie(),
            self.cx0ie(),
            self.cx1ie(),
            self.rie(),
            self.reie()
        )
    }
}
#[doc = "Output Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2octrl(pub u16);
impl Sm2octrl {
    #[doc = "PWM_X Fault State."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmxfs(&self) -> Sm2octrlPwmxfs {
        let val = (self.0 >> 0usize) & 0x03;
        Sm2octrlPwmxfs::from_bits(val as u8)
    }
    #[doc = "PWM_X Fault State."]
    #[inline(always)]
    pub const fn set_pwmxfs(&mut self, val: Sm2octrlPwmxfs) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "PWM_B Fault State."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmbfs(&self) -> Sm2octrlPwmbfs {
        let val = (self.0 >> 2usize) & 0x03;
        Sm2octrlPwmbfs::from_bits(val as u8)
    }
    #[doc = "PWM_B Fault State."]
    #[inline(always)]
    pub const fn set_pwmbfs(&mut self, val: Sm2octrlPwmbfs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "PWM_A Fault State."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmafs(&self) -> Sm2octrlPwmafs {
        let val = (self.0 >> 4usize) & 0x03;
        Sm2octrlPwmafs::from_bits(val as u8)
    }
    #[doc = "PWM_A Fault State."]
    #[inline(always)]
    pub const fn set_pwmafs(&mut self, val: Sm2octrlPwmafs) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "PWM_X Output Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn polx(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Output Polarity."]
    #[inline(always)]
    pub const fn set_polx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "PWM_B Output Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn polb(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Output Polarity."]
    #[inline(always)]
    pub const fn set_polb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "PWM_A Output Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn pola(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Output Polarity."]
    #[inline(always)]
    pub const fn set_pola(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM_X Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_in(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Input."]
    #[inline(always)]
    pub const fn set_pwmx_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "PWM_B Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmb_in(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Input."]
    #[inline(always)]
    pub const fn set_pwmb_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "PWM_A Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pwma_in(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Input."]
    #[inline(always)]
    pub const fn set_pwma_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm2octrl {
    #[inline(always)]
    fn default() -> Sm2octrl {
        Sm2octrl(0)
    }
}
impl core::fmt::Debug for Sm2octrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2octrl")
            .field("pwmxfs", &self.pwmxfs())
            .field("pwmbfs", &self.pwmbfs())
            .field("pwmafs", &self.pwmafs())
            .field("polx", &self.polx())
            .field("polb", &self.polb())
            .field("pola", &self.pola())
            .field("pwmx_in", &self.pwmx_in())
            .field("pwmb_in", &self.pwmb_in())
            .field("pwma_in", &self.pwma_in())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2octrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2octrl {{ pwmxfs: {:?}, pwmbfs: {:?}, pwmafs: {:?}, polx: {=bool:?}, polb: {=bool:?}, pola: {=bool:?}, pwmx_in: {=bool:?}, pwmb_in: {=bool:?}, pwma_in: {=bool:?} }}",
            self.pwmxfs(),
            self.pwmbfs(),
            self.pwmafs(),
            self.polx(),
            self.polb(),
            self.pola(),
            self.pwmx_in(),
            self.pwmb_in(),
            self.pwma_in()
        )
    }
}
#[doc = "Phase Delay Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2phasedly(pub u16);
impl Sm2phasedly {
    #[doc = "Initial Count Register Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn phasedly(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Initial Count Register Bits."]
    #[inline(always)]
    pub const fn set_phasedly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2phasedly {
    #[inline(always)]
    fn default() -> Sm2phasedly {
        Sm2phasedly(0)
    }
}
impl core::fmt::Debug for Sm2phasedly {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2phasedly")
            .field("phasedly", &self.phasedly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2phasedly {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2phasedly {{ phasedly: {=u16:?} }}", self.phasedly())
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2sts(pub u16);
impl Sm2sts {
    #[doc = "Compare Flags."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpf(&self) -> Sm2stsCmpf {
        let val = (self.0 >> 0usize) & 0x3f;
        Sm2stsCmpf::from_bits(val as u8)
    }
    #[doc = "Compare Flags."]
    #[inline(always)]
    pub const fn set_cmpf(&mut self, val: Sm2stsCmpf) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture Flag X0."]
    #[must_use]
    #[inline(always)]
    pub const fn cfx0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X0."]
    #[inline(always)]
    pub const fn set_cfx0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture Flag X1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfx1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X1."]
    #[inline(always)]
    pub const fn set_cfx1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Reload Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Flag."]
    #[inline(always)]
    pub const fn set_rf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ref_(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Flag."]
    #[inline(always)]
    pub const fn set_ref_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Registers Updated Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ruf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Registers Updated Flag."]
    #[inline(always)]
    pub const fn set_ruf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
}
impl Default for Sm2sts {
    #[inline(always)]
    fn default() -> Sm2sts {
        Sm2sts(0)
    }
}
impl core::fmt::Debug for Sm2sts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2sts")
            .field("cmpf", &self.cmpf())
            .field("cfx0", &self.cfx0())
            .field("cfx1", &self.cfx1())
            .field("rf", &self.rf())
            .field("ref_", &self.ref_())
            .field("ruf", &self.ruf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2sts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2sts {{ cmpf: {:?}, cfx0: {=bool:?}, cfx1: {=bool:?}, rf: {=bool:?}, ref_: {=bool:?}, ruf: {=bool:?} }}",
            self.cmpf(),
            self.cfx0(),
            self.cfx1(),
            self.rf(),
            self.ref_(),
            self.ruf()
        )
    }
}
#[doc = "Output Trigger Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2tctrl(pub u16);
impl Sm2tctrl {
    #[doc = "Output Trigger Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn out_trig_en(&self) -> Sm2tctrlOutTrigEn {
        let val = (self.0 >> 0usize) & 0x3f;
        Sm2tctrlOutTrigEn::from_bits(val as u8)
    }
    #[doc = "Output Trigger Enables."]
    #[inline(always)]
    pub const fn set_out_trig_en(&mut self, val: Sm2tctrlOutTrigEn) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Trigger Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn trgfrq(&self) -> Sm2tctrlTrgfrq {
        let val = (self.0 >> 12usize) & 0x01;
        Sm2tctrlTrgfrq::from_bits(val as u8)
    }
    #[doc = "Trigger Frequency."]
    #[inline(always)]
    pub const fn set_trgfrq(&mut self, val: Sm2tctrlTrgfrq) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Mux Output Trigger 1 Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pwbot1(&self) -> Sm2tctrlPwbot1 {
        let val = (self.0 >> 14usize) & 0x01;
        Sm2tctrlPwbot1::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 1 Source Select."]
    #[inline(always)]
    pub const fn set_pwbot1(&mut self, val: Sm2tctrlPwbot1) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Mux Output Trigger 0 Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pwaot0(&self) -> Sm2tctrlPwaot0 {
        let val = (self.0 >> 15usize) & 0x01;
        Sm2tctrlPwaot0::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 0 Source Select."]
    #[inline(always)]
    pub const fn set_pwaot0(&mut self, val: Sm2tctrlPwaot0) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm2tctrl {
    #[inline(always)]
    fn default() -> Sm2tctrl {
        Sm2tctrl(0)
    }
}
impl core::fmt::Debug for Sm2tctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2tctrl")
            .field("out_trig_en", &self.out_trig_en())
            .field("trgfrq", &self.trgfrq())
            .field("pwbot1", &self.pwbot1())
            .field("pwaot0", &self.pwaot0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2tctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm2tctrl {{ out_trig_en: {:?}, trgfrq: {:?}, pwbot1: {:?}, pwaot0: {:?} }}",
            self.out_trig_en(),
            self.trgfrq(),
            self.pwbot1(),
            self.pwaot0()
        )
    }
}
#[doc = "Value Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2val0(pub u16);
impl Sm2val0 {
    #[doc = "Value 0."]
    #[must_use]
    #[inline(always)]
    pub const fn val0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 0."]
    #[inline(always)]
    pub const fn set_val0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2val0 {
    #[inline(always)]
    fn default() -> Sm2val0 {
        Sm2val0(0)
    }
}
impl core::fmt::Debug for Sm2val0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2val0")
            .field("val0", &self.val0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2val0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2val0 {{ val0: {=u16:?} }}", self.val0())
    }
}
#[doc = "Value Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2val1(pub u16);
impl Sm2val1 {
    #[doc = "Value 1."]
    #[must_use]
    #[inline(always)]
    pub const fn val1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 1."]
    #[inline(always)]
    pub const fn set_val1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2val1 {
    #[inline(always)]
    fn default() -> Sm2val1 {
        Sm2val1(0)
    }
}
impl core::fmt::Debug for Sm2val1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2val1")
            .field("val1", &self.val1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2val1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2val1 {{ val1: {=u16:?} }}", self.val1())
    }
}
#[doc = "Value Register 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2val2(pub u16);
impl Sm2val2 {
    #[doc = "Value 2."]
    #[must_use]
    #[inline(always)]
    pub const fn val2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 2."]
    #[inline(always)]
    pub const fn set_val2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2val2 {
    #[inline(always)]
    fn default() -> Sm2val2 {
        Sm2val2(0)
    }
}
impl core::fmt::Debug for Sm2val2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2val2")
            .field("val2", &self.val2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2val2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2val2 {{ val2: {=u16:?} }}", self.val2())
    }
}
#[doc = "Value Register 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2val3(pub u16);
impl Sm2val3 {
    #[doc = "Value 3."]
    #[must_use]
    #[inline(always)]
    pub const fn val3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 3."]
    #[inline(always)]
    pub const fn set_val3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2val3 {
    #[inline(always)]
    fn default() -> Sm2val3 {
        Sm2val3(0)
    }
}
impl core::fmt::Debug for Sm2val3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2val3")
            .field("val3", &self.val3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2val3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2val3 {{ val3: {=u16:?} }}", self.val3())
    }
}
#[doc = "Value Register 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2val4(pub u16);
impl Sm2val4 {
    #[doc = "Value 4."]
    #[must_use]
    #[inline(always)]
    pub const fn val4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 4."]
    #[inline(always)]
    pub const fn set_val4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2val4 {
    #[inline(always)]
    fn default() -> Sm2val4 {
        Sm2val4(0)
    }
}
impl core::fmt::Debug for Sm2val4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2val4")
            .field("val4", &self.val4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2val4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2val4 {{ val4: {=u16:?} }}", self.val4())
    }
}
#[doc = "Value Register 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm2val5(pub u16);
impl Sm2val5 {
    #[doc = "Value 5."]
    #[must_use]
    #[inline(always)]
    pub const fn val5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 5."]
    #[inline(always)]
    pub const fn set_val5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm2val5 {
    #[inline(always)]
    fn default() -> Sm2val5 {
        Sm2val5(0)
    }
}
impl core::fmt::Debug for Sm2val5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm2val5")
            .field("val5", &self.val5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm2val5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm2val5 {{ val5: {=u16:?} }}", self.val5())
    }
}
#[doc = "Capture Compare X Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3captcompx(pub u16);
impl Sm3captcompx {
    #[doc = "Edge Compare X."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare X."]
    #[inline(always)]
    pub const fn set_edgcmpx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter X."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter X."]
    #[inline(always)]
    pub const fn set_edgcntx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Sm3captcompx {
    #[inline(always)]
    fn default() -> Sm3captcompx {
        Sm3captcompx(0)
    }
}
impl core::fmt::Debug for Sm3captcompx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3captcompx")
            .field("edgcmpx", &self.edgcmpx())
            .field("edgcntx", &self.edgcntx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3captcompx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3captcompx {{ edgcmpx: {=u8:?}, edgcntx: {=u8:?} }}",
            self.edgcmpx(),
            self.edgcntx()
        )
    }
}
#[doc = "Capture Control X Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3captctrlx(pub u16);
impl Sm3captctrlx {
    #[doc = "Arm X."]
    #[must_use]
    #[inline(always)]
    pub const fn armx(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm X."]
    #[inline(always)]
    pub const fn set_armx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode Aux."]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotx(&self) -> Sm3captctrlxOneshotx {
        let val = (self.0 >> 1usize) & 0x01;
        Sm3captctrlxOneshotx::from_bits(val as u8)
    }
    #[doc = "One Shot Mode Aux."]
    #[inline(always)]
    pub const fn set_oneshotx(&mut self, val: Sm3captctrlxOneshotx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge X 0."]
    #[must_use]
    #[inline(always)]
    pub const fn edgx0(&self) -> Sm3captctrlxEdgx0 {
        let val = (self.0 >> 2usize) & 0x03;
        Sm3captctrlxEdgx0::from_bits(val as u8)
    }
    #[doc = "Edge X 0."]
    #[inline(always)]
    pub const fn set_edgx0(&mut self, val: Sm3captctrlxEdgx0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge X 1."]
    #[must_use]
    #[inline(always)]
    pub const fn edgx1(&self) -> Sm3captctrlxEdgx1 {
        let val = (self.0 >> 4usize) & 0x03;
        Sm3captctrlxEdgx1::from_bits(val as u8)
    }
    #[doc = "Edge X 1."]
    #[inline(always)]
    pub const fn set_edgx1(&mut self, val: Sm3captctrlxEdgx1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select X."]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selx(&self) -> Sm3captctrlxInpSelx {
        let val = (self.0 >> 6usize) & 0x01;
        Sm3captctrlxInpSelx::from_bits(val as u8)
    }
    #[doc = "Input Select X."]
    #[inline(always)]
    pub const fn set_inp_selx(&mut self, val: Sm3captctrlxInpSelx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter X Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter X Enable."]
    #[inline(always)]
    pub const fn set_edgcntx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture X FIFOs Water Mark."]
    #[must_use]
    #[inline(always)]
    pub const fn cfxwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture X FIFOs Water Mark."]
    #[inline(always)]
    pub const fn set_cfxwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture X0 FIFO Word Count."]
    #[must_use]
    #[inline(always)]
    pub const fn cx0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X0 FIFO Word Count."]
    #[inline(always)]
    pub const fn set_cx0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture X1 FIFO Word Count."]
    #[must_use]
    #[inline(always)]
    pub const fn cx1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X1 FIFO Word Count."]
    #[inline(always)]
    pub const fn set_cx1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Sm3captctrlx {
    #[inline(always)]
    fn default() -> Sm3captctrlx {
        Sm3captctrlx(0)
    }
}
impl core::fmt::Debug for Sm3captctrlx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3captctrlx")
            .field("armx", &self.armx())
            .field("oneshotx", &self.oneshotx())
            .field("edgx0", &self.edgx0())
            .field("edgx1", &self.edgx1())
            .field("inp_selx", &self.inp_selx())
            .field("edgcntx_en", &self.edgcntx_en())
            .field("cfxwm", &self.cfxwm())
            .field("cx0cnt", &self.cx0cnt())
            .field("cx1cnt", &self.cx1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3captctrlx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3captctrlx {{ armx: {=bool:?}, oneshotx: {:?}, edgx0: {:?}, edgx1: {:?}, inp_selx: {:?}, edgcntx_en: {=bool:?}, cfxwm: {=u8:?}, cx0cnt: {=u8:?}, cx1cnt: {=u8:?} }}",
            self.armx(),
            self.oneshotx(),
            self.edgx0(),
            self.edgx1(),
            self.inp_selx(),
            self.edgcntx_en(),
            self.cfxwm(),
            self.cx0cnt(),
            self.cx1cnt()
        )
    }
}
#[doc = "Capture PWM_X Input Filter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3captfiltx(pub u16);
impl Sm3captfiltx {
    #[doc = "Input Capture Filter Period."]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period."]
    #[inline(always)]
    pub const fn set_captx_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count."]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count."]
    #[inline(always)]
    pub const fn set_captx_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Sm3captfiltx {
    #[inline(always)]
    fn default() -> Sm3captfiltx {
        Sm3captfiltx(0)
    }
}
impl core::fmt::Debug for Sm3captfiltx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3captfiltx")
            .field("captx_filt_per", &self.captx_filt_per())
            .field("captx_filt_cnt", &self.captx_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3captfiltx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3captfiltx {{ captx_filt_per: {=u8:?}, captx_filt_cnt: {=u8:?} }}",
            self.captx_filt_per(),
            self.captx_filt_cnt()
        )
    }
}
#[doc = "Counter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3cnt(pub u16);
impl Sm3cnt {
    #[doc = "Counter Register Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter Register Bits."]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3cnt {
    #[inline(always)]
    fn default() -> Sm3cnt {
        Sm3cnt(0)
    }
}
impl core::fmt::Debug for Sm3cnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3cnt").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3cnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3cnt {{ cnt: {=u16:?} }}", self.cnt())
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3ctrl(pub u16);
impl Sm3ctrl {
    #[doc = "Double Switching Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dblen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Double Switching Enable."]
    #[inline(always)]
    pub const fn set_dblen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "PWM_X Double Switching Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dblx(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Double Switching Enable."]
    #[inline(always)]
    pub const fn set_dblx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Load Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ldmod(&self) -> Sm3ctrlLdmod {
        let val = (self.0 >> 2usize) & 0x01;
        Sm3ctrlLdmod::from_bits(val as u8)
    }
    #[doc = "Load Mode Select."]
    #[inline(always)]
    pub const fn set_ldmod(&mut self, val: Sm3ctrlLdmod) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B."]
    #[must_use]
    #[inline(always)]
    pub const fn split(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B."]
    #[inline(always)]
    pub const fn set_split(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn prsc(&self) -> Sm3ctrlPrsc {
        let val = (self.0 >> 4usize) & 0x07;
        Sm3ctrlPrsc::from_bits(val as u8)
    }
    #[doc = "Prescaler."]
    #[inline(always)]
    pub const fn set_prsc(&mut self, val: Sm3ctrlPrsc) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u16) & 0x07) << 4usize);
    }
    #[doc = "Compare Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn compmode(&self) -> Sm3ctrlCompmode {
        let val = (self.0 >> 7usize) & 0x01;
        Sm3ctrlCompmode::from_bits(val as u8)
    }
    #[doc = "Compare Mode."]
    #[inline(always)]
    pub const fn set_compmode(&mut self, val: Sm3ctrlCompmode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Deadtime."]
    #[must_use]
    #[inline(always)]
    pub const fn dt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Deadtime."]
    #[inline(always)]
    pub const fn set_dt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Full Cycle Reload."]
    #[must_use]
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Full Cycle Reload."]
    #[inline(always)]
    pub const fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Half Cycle Reload."]
    #[must_use]
    #[inline(always)]
    pub const fn half(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle Reload."]
    #[inline(always)]
    pub const fn set_half(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Load Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn ldfq(&self) -> Sm3ctrlLdfq {
        let val = (self.0 >> 12usize) & 0x0f;
        Sm3ctrlLdfq::from_bits(val as u8)
    }
    #[doc = "Load Frequency."]
    #[inline(always)]
    pub const fn set_ldfq(&mut self, val: Sm3ctrlLdfq) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Sm3ctrl {
    #[inline(always)]
    fn default() -> Sm3ctrl {
        Sm3ctrl(0)
    }
}
impl core::fmt::Debug for Sm3ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3ctrl")
            .field("dblen", &self.dblen())
            .field("dblx", &self.dblx())
            .field("ldmod", &self.ldmod())
            .field("split", &self.split())
            .field("prsc", &self.prsc())
            .field("compmode", &self.compmode())
            .field("dt", &self.dt())
            .field("full", &self.full())
            .field("half", &self.half())
            .field("ldfq", &self.ldfq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3ctrl {{ dblen: {=bool:?}, dblx: {=bool:?}, ldmod: {:?}, split: {=bool:?}, prsc: {:?}, compmode: {:?}, dt: {=u8:?}, full: {=bool:?}, half: {=bool:?}, ldfq: {:?} }}",
            self.dblen(),
            self.dblx(),
            self.ldmod(),
            self.split(),
            self.prsc(),
            self.compmode(),
            self.dt(),
            self.full(),
            self.half(),
            self.ldfq()
        )
    }
}
#[doc = "Control 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3ctrl2(pub u16);
impl Sm3ctrl2 {
    #[doc = "Clock Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel(&self) -> Sm3ctrl2ClkSel {
        let val = (self.0 >> 0usize) & 0x03;
        Sm3ctrl2ClkSel::from_bits(val as u8)
    }
    #[doc = "Clock Source Select."]
    #[inline(always)]
    pub const fn set_clk_sel(&mut self, val: Sm3ctrl2ClkSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Reload Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn reload_sel(&self) -> Sm3ctrl2ReloadSel {
        let val = (self.0 >> 2usize) & 0x01;
        Sm3ctrl2ReloadSel::from_bits(val as u8)
    }
    #[doc = "Reload Source Select."]
    #[inline(always)]
    pub const fn set_reload_sel(&mut self, val: Sm3ctrl2ReloadSel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Force Select."]
    #[must_use]
    #[inline(always)]
    pub const fn force_sel(&self) -> Sm3ctrl2ForceSel {
        let val = (self.0 >> 3usize) & 0x07;
        Sm3ctrl2ForceSel::from_bits(val as u8)
    }
    #[doc = "Force Select."]
    #[inline(always)]
    pub const fn set_force_sel(&mut self, val: Sm3ctrl2ForceSel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u16) & 0x07) << 3usize);
    }
    #[doc = "Force Initialization."]
    #[must_use]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Force Initialization."]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Force Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn frcen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Enable."]
    #[inline(always)]
    pub const fn set_frcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Initialization Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn init_sel(&self) -> Sm3ctrl2InitSel {
        let val = (self.0 >> 8usize) & 0x03;
        Sm3ctrl2InitSel::from_bits(val as u8)
    }
    #[doc = "Initialization Control Select."]
    #[inline(always)]
    pub const fn set_init_sel(&mut self, val: Sm3ctrl2InitSel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "PWM_X Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_init(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Initial Value."]
    #[inline(always)]
    pub const fn set_pwmx_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM45 Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm45_init(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PWM45 Initial Value."]
    #[inline(always)]
    pub const fn set_pwm45_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "PWM23 Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm23_init(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PWM23 Initial Value."]
    #[inline(always)]
    pub const fn set_pwm23_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Independent or Complementary Pair Operation."]
    #[must_use]
    #[inline(always)]
    pub const fn indep(&self) -> Sm3ctrl2Indep {
        let val = (self.0 >> 13usize) & 0x01;
        Sm3ctrl2Indep::from_bits(val as u8)
    }
    #[doc = "Independent or Complementary Pair Operation."]
    #[inline(always)]
    pub const fn set_indep(&mut self, val: Sm3ctrl2Indep) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Debug Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable."]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm3ctrl2 {
    #[inline(always)]
    fn default() -> Sm3ctrl2 {
        Sm3ctrl2(0)
    }
}
impl core::fmt::Debug for Sm3ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3ctrl2")
            .field("clk_sel", &self.clk_sel())
            .field("reload_sel", &self.reload_sel())
            .field("force_sel", &self.force_sel())
            .field("force", &self.force())
            .field("frcen", &self.frcen())
            .field("init_sel", &self.init_sel())
            .field("pwmx_init", &self.pwmx_init())
            .field("pwm45_init", &self.pwm45_init())
            .field("pwm23_init", &self.pwm23_init())
            .field("indep", &self.indep())
            .field("dbgen", &self.dbgen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3ctrl2 {{ clk_sel: {:?}, reload_sel: {:?}, force_sel: {:?}, force: {=bool:?}, frcen: {=bool:?}, init_sel: {:?}, pwmx_init: {=bool:?}, pwm45_init: {=bool:?}, pwm23_init: {=bool:?}, indep: {:?}, dbgen: {=bool:?} }}",
            self.clk_sel(),
            self.reload_sel(),
            self.force_sel(),
            self.force(),
            self.frcen(),
            self.init_sel(),
            self.pwmx_init(),
            self.pwm45_init(),
            self.pwm23_init(),
            self.indep(),
            self.dbgen()
        )
    }
}
#[doc = "Capture Value 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3cval0(pub u16);
impl Sm3cval0 {
    #[doc = "Capture Value 0."]
    #[must_use]
    #[inline(always)]
    pub const fn captval0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 0."]
    #[inline(always)]
    pub const fn set_captval0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3cval0 {
    #[inline(always)]
    fn default() -> Sm3cval0 {
        Sm3cval0(0)
    }
}
impl core::fmt::Debug for Sm3cval0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3cval0")
            .field("captval0", &self.captval0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3cval0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3cval0 {{ captval0: {=u16:?} }}", self.captval0())
    }
}
#[doc = "Capture Value 0 Cycle Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3cval0cyc(pub u16);
impl Sm3cval0cyc {
    #[doc = "Capture Value 0 Cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn cval0cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 0 Cycle."]
    #[inline(always)]
    pub const fn set_cval0cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm3cval0cyc {
    #[inline(always)]
    fn default() -> Sm3cval0cyc {
        Sm3cval0cyc(0)
    }
}
impl core::fmt::Debug for Sm3cval0cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3cval0cyc")
            .field("cval0cyc", &self.cval0cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3cval0cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3cval0cyc {{ cval0cyc: {=u8:?} }}", self.cval0cyc())
    }
}
#[doc = "Capture Value 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3cval1(pub u16);
impl Sm3cval1 {
    #[doc = "Capture Value 1."]
    #[must_use]
    #[inline(always)]
    pub const fn captval1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 1."]
    #[inline(always)]
    pub const fn set_captval1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3cval1 {
    #[inline(always)]
    fn default() -> Sm3cval1 {
        Sm3cval1(0)
    }
}
impl core::fmt::Debug for Sm3cval1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3cval1")
            .field("captval1", &self.captval1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3cval1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3cval1 {{ captval1: {=u16:?} }}", self.captval1())
    }
}
#[doc = "Capture Value 1 Cycle Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3cval1cyc(pub u16);
impl Sm3cval1cyc {
    #[doc = "Capture Value 1 Cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn cval1cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 1 Cycle."]
    #[inline(always)]
    pub const fn set_cval1cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Sm3cval1cyc {
    #[inline(always)]
    fn default() -> Sm3cval1cyc {
        Sm3cval1cyc(0)
    }
}
impl core::fmt::Debug for Sm3cval1cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3cval1cyc")
            .field("cval1cyc", &self.cval1cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3cval1cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3cval1cyc {{ cval1cyc: {=u8:?} }}", self.cval1cyc())
    }
}
#[doc = "Fault Disable Mapping Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3dismap0(pub u16);
impl Sm3dismap0 {
    #[doc = "PWM_A Fault Disable Mask 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dis0a(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Fault Disable Mask 0."]
    #[inline(always)]
    pub const fn set_dis0a(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Fault Disable Mask 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dis0b(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Fault Disable Mask 0."]
    #[inline(always)]
    pub const fn set_dis0b(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PWM_X Fault Disable Mask 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dis0x(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Fault Disable Mask 0."]
    #[inline(always)]
    pub const fn set_dis0x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
}
impl Default for Sm3dismap0 {
    #[inline(always)]
    fn default() -> Sm3dismap0 {
        Sm3dismap0(0)
    }
}
impl core::fmt::Debug for Sm3dismap0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3dismap0")
            .field("dis0a", &self.dis0a())
            .field("dis0b", &self.dis0b())
            .field("dis0x", &self.dis0x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3dismap0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3dismap0 {{ dis0a: {=u8:?}, dis0b: {=u8:?}, dis0x: {=u8:?} }}",
            self.dis0a(),
            self.dis0b(),
            self.dis0x()
        )
    }
}
#[doc = "DMA Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3dmaen(pub u16);
impl Sm3dmaen {
    #[doc = "Capture X0 FIFO DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx0de(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X0 FIFO DMA Enable."]
    #[inline(always)]
    pub const fn set_cx0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Capture X1 FIFO DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X1 FIFO DMA Enable."]
    #[inline(always)]
    pub const fn set_cx1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Capture DMA Enable Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn captde(&self) -> Sm3dmaenCaptde {
        let val = (self.0 >> 6usize) & 0x03;
        Sm3dmaenCaptde::from_bits(val as u8)
    }
    #[doc = "Capture DMA Enable Source Select."]
    #[inline(always)]
    pub const fn set_captde(&mut self, val: Sm3dmaenCaptde) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "FIFO Watermark AND Control."]
    #[must_use]
    #[inline(always)]
    pub const fn fand(&self) -> Sm3dmaenFand {
        let val = (self.0 >> 8usize) & 0x01;
        Sm3dmaenFand::from_bits(val as u8)
    }
    #[doc = "FIFO Watermark AND Control."]
    #[inline(always)]
    pub const fn set_fand(&mut self, val: Sm3dmaenFand) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Value Registers DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn valde(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Value Registers DMA Enable."]
    #[inline(always)]
    pub const fn set_valde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
}
impl Default for Sm3dmaen {
    #[inline(always)]
    fn default() -> Sm3dmaen {
        Sm3dmaen(0)
    }
}
impl core::fmt::Debug for Sm3dmaen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3dmaen")
            .field("cx0de", &self.cx0de())
            .field("cx1de", &self.cx1de())
            .field("captde", &self.captde())
            .field("fand", &self.fand())
            .field("valde", &self.valde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3dmaen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3dmaen {{ cx0de: {=bool:?}, cx1de: {=bool:?}, captde: {:?}, fand: {:?}, valde: {=bool:?} }}",
            self.cx0de(),
            self.cx1de(),
            self.captde(),
            self.fand(),
            self.valde()
        )
    }
}
#[doc = "Deadtime Count Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3dtcnt0(pub u16);
impl Sm3dtcnt0 {
    #[doc = "Deadtime Count Register 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnt0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Deadtime Count Register 0."]
    #[inline(always)]
    pub const fn set_dtcnt0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
    }
}
impl Default for Sm3dtcnt0 {
    #[inline(always)]
    fn default() -> Sm3dtcnt0 {
        Sm3dtcnt0(0)
    }
}
impl core::fmt::Debug for Sm3dtcnt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3dtcnt0")
            .field("dtcnt0", &self.dtcnt0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3dtcnt0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3dtcnt0 {{ dtcnt0: {=u16:?} }}", self.dtcnt0())
    }
}
#[doc = "Deadtime Count Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3dtcnt1(pub u16);
impl Sm3dtcnt1 {
    #[doc = "Deadtime Count Register 1."]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnt1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Deadtime Count Register 1."]
    #[inline(always)]
    pub const fn set_dtcnt1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
    }
}
impl Default for Sm3dtcnt1 {
    #[inline(always)]
    fn default() -> Sm3dtcnt1 {
        Sm3dtcnt1(0)
    }
}
impl core::fmt::Debug for Sm3dtcnt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3dtcnt1")
            .field("dtcnt1", &self.dtcnt1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3dtcnt1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3dtcnt1 {{ dtcnt1: {=u16:?} }}", self.dtcnt1())
    }
}
#[doc = "Initial Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3init(pub u16);
impl Sm3init {
    #[doc = "Initial Count Register Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn init(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Initial Count Register Bits."]
    #[inline(always)]
    pub const fn set_init(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3init {
    #[inline(always)]
    fn default() -> Sm3init {
        Sm3init(0)
    }
}
impl core::fmt::Debug for Sm3init {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3init")
            .field("init", &self.init())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3init {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3init {{ init: {=u16:?} }}", self.init())
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3inten(pub u16);
impl Sm3inten {
    #[doc = "Compare Interrupt Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpie(&self) -> Sm3intenCmpie {
        let val = (self.0 >> 0usize) & 0x3f;
        Sm3intenCmpie::from_bits(val as u8)
    }
    #[doc = "Compare Interrupt Enables."]
    #[inline(always)]
    pub const fn set_cmpie(&mut self, val: Sm3intenCmpie) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture X 0 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx0ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 0 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cx0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture X 1 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx1ie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 1 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cx1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Reload Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn reie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_reie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
}
impl Default for Sm3inten {
    #[inline(always)]
    fn default() -> Sm3inten {
        Sm3inten(0)
    }
}
impl core::fmt::Debug for Sm3inten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3inten")
            .field("cmpie", &self.cmpie())
            .field("cx0ie", &self.cx0ie())
            .field("cx1ie", &self.cx1ie())
            .field("rie", &self.rie())
            .field("reie", &self.reie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3inten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3inten {{ cmpie: {:?}, cx0ie: {=bool:?}, cx1ie: {=bool:?}, rie: {=bool:?}, reie: {=bool:?} }}",
            self.cmpie(),
            self.cx0ie(),
            self.cx1ie(),
            self.rie(),
            self.reie()
        )
    }
}
#[doc = "Output Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3octrl(pub u16);
impl Sm3octrl {
    #[doc = "PWM_X Fault State."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmxfs(&self) -> Sm3octrlPwmxfs {
        let val = (self.0 >> 0usize) & 0x03;
        Sm3octrlPwmxfs::from_bits(val as u8)
    }
    #[doc = "PWM_X Fault State."]
    #[inline(always)]
    pub const fn set_pwmxfs(&mut self, val: Sm3octrlPwmxfs) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "PWM_B Fault State."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmbfs(&self) -> Sm3octrlPwmbfs {
        let val = (self.0 >> 2usize) & 0x03;
        Sm3octrlPwmbfs::from_bits(val as u8)
    }
    #[doc = "PWM_B Fault State."]
    #[inline(always)]
    pub const fn set_pwmbfs(&mut self, val: Sm3octrlPwmbfs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "PWM_A Fault State."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmafs(&self) -> Sm3octrlPwmafs {
        let val = (self.0 >> 4usize) & 0x03;
        Sm3octrlPwmafs::from_bits(val as u8)
    }
    #[doc = "PWM_A Fault State."]
    #[inline(always)]
    pub const fn set_pwmafs(&mut self, val: Sm3octrlPwmafs) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "PWM_X Output Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn polx(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Output Polarity."]
    #[inline(always)]
    pub const fn set_polx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "PWM_B Output Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn polb(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Output Polarity."]
    #[inline(always)]
    pub const fn set_polb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "PWM_A Output Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn pola(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Output Polarity."]
    #[inline(always)]
    pub const fn set_pola(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM_X Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_in(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Input."]
    #[inline(always)]
    pub const fn set_pwmx_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "PWM_B Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmb_in(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Input."]
    #[inline(always)]
    pub const fn set_pwmb_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "PWM_A Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pwma_in(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Input."]
    #[inline(always)]
    pub const fn set_pwma_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm3octrl {
    #[inline(always)]
    fn default() -> Sm3octrl {
        Sm3octrl(0)
    }
}
impl core::fmt::Debug for Sm3octrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3octrl")
            .field("pwmxfs", &self.pwmxfs())
            .field("pwmbfs", &self.pwmbfs())
            .field("pwmafs", &self.pwmafs())
            .field("polx", &self.polx())
            .field("polb", &self.polb())
            .field("pola", &self.pola())
            .field("pwmx_in", &self.pwmx_in())
            .field("pwmb_in", &self.pwmb_in())
            .field("pwma_in", &self.pwma_in())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3octrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3octrl {{ pwmxfs: {:?}, pwmbfs: {:?}, pwmafs: {:?}, polx: {=bool:?}, polb: {=bool:?}, pola: {=bool:?}, pwmx_in: {=bool:?}, pwmb_in: {=bool:?}, pwma_in: {=bool:?} }}",
            self.pwmxfs(),
            self.pwmbfs(),
            self.pwmafs(),
            self.polx(),
            self.polb(),
            self.pola(),
            self.pwmx_in(),
            self.pwmb_in(),
            self.pwma_in()
        )
    }
}
#[doc = "Phase Delay Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3phasedly(pub u16);
impl Sm3phasedly {
    #[doc = "Initial Count Register Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn phasedly(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Initial Count Register Bits."]
    #[inline(always)]
    pub const fn set_phasedly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3phasedly {
    #[inline(always)]
    fn default() -> Sm3phasedly {
        Sm3phasedly(0)
    }
}
impl core::fmt::Debug for Sm3phasedly {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3phasedly")
            .field("phasedly", &self.phasedly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3phasedly {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3phasedly {{ phasedly: {=u16:?} }}", self.phasedly())
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3sts(pub u16);
impl Sm3sts {
    #[doc = "Compare Flags."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpf(&self) -> Sm3stsCmpf {
        let val = (self.0 >> 0usize) & 0x3f;
        Sm3stsCmpf::from_bits(val as u8)
    }
    #[doc = "Compare Flags."]
    #[inline(always)]
    pub const fn set_cmpf(&mut self, val: Sm3stsCmpf) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture Flag X0."]
    #[must_use]
    #[inline(always)]
    pub const fn cfx0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X0."]
    #[inline(always)]
    pub const fn set_cfx0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture Flag X1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfx1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X1."]
    #[inline(always)]
    pub const fn set_cfx1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Reload Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Flag."]
    #[inline(always)]
    pub const fn set_rf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ref_(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Flag."]
    #[inline(always)]
    pub const fn set_ref_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Registers Updated Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ruf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Registers Updated Flag."]
    #[inline(always)]
    pub const fn set_ruf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
}
impl Default for Sm3sts {
    #[inline(always)]
    fn default() -> Sm3sts {
        Sm3sts(0)
    }
}
impl core::fmt::Debug for Sm3sts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3sts")
            .field("cmpf", &self.cmpf())
            .field("cfx0", &self.cfx0())
            .field("cfx1", &self.cfx1())
            .field("rf", &self.rf())
            .field("ref_", &self.ref_())
            .field("ruf", &self.ruf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3sts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3sts {{ cmpf: {:?}, cfx0: {=bool:?}, cfx1: {=bool:?}, rf: {=bool:?}, ref_: {=bool:?}, ruf: {=bool:?} }}",
            self.cmpf(),
            self.cfx0(),
            self.cfx1(),
            self.rf(),
            self.ref_(),
            self.ruf()
        )
    }
}
#[doc = "Output Trigger Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3tctrl(pub u16);
impl Sm3tctrl {
    #[doc = "Output Trigger Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn out_trig_en(&self) -> Sm3tctrlOutTrigEn {
        let val = (self.0 >> 0usize) & 0x3f;
        Sm3tctrlOutTrigEn::from_bits(val as u8)
    }
    #[doc = "Output Trigger Enables."]
    #[inline(always)]
    pub const fn set_out_trig_en(&mut self, val: Sm3tctrlOutTrigEn) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Trigger Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn trgfrq(&self) -> Sm3tctrlTrgfrq {
        let val = (self.0 >> 12usize) & 0x01;
        Sm3tctrlTrgfrq::from_bits(val as u8)
    }
    #[doc = "Trigger Frequency."]
    #[inline(always)]
    pub const fn set_trgfrq(&mut self, val: Sm3tctrlTrgfrq) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Mux Output Trigger 1 Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pwbot1(&self) -> Sm3tctrlPwbot1 {
        let val = (self.0 >> 14usize) & 0x01;
        Sm3tctrlPwbot1::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 1 Source Select."]
    #[inline(always)]
    pub const fn set_pwbot1(&mut self, val: Sm3tctrlPwbot1) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Mux Output Trigger 0 Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pwaot0(&self) -> Sm3tctrlPwaot0 {
        let val = (self.0 >> 15usize) & 0x01;
        Sm3tctrlPwaot0::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 0 Source Select."]
    #[inline(always)]
    pub const fn set_pwaot0(&mut self, val: Sm3tctrlPwaot0) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Sm3tctrl {
    #[inline(always)]
    fn default() -> Sm3tctrl {
        Sm3tctrl(0)
    }
}
impl core::fmt::Debug for Sm3tctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3tctrl")
            .field("out_trig_en", &self.out_trig_en())
            .field("trgfrq", &self.trgfrq())
            .field("pwbot1", &self.pwbot1())
            .field("pwaot0", &self.pwaot0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3tctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sm3tctrl {{ out_trig_en: {:?}, trgfrq: {:?}, pwbot1: {:?}, pwaot0: {:?} }}",
            self.out_trig_en(),
            self.trgfrq(),
            self.pwbot1(),
            self.pwaot0()
        )
    }
}
#[doc = "Value Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3val0(pub u16);
impl Sm3val0 {
    #[doc = "Value 0."]
    #[must_use]
    #[inline(always)]
    pub const fn val0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 0."]
    #[inline(always)]
    pub const fn set_val0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3val0 {
    #[inline(always)]
    fn default() -> Sm3val0 {
        Sm3val0(0)
    }
}
impl core::fmt::Debug for Sm3val0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3val0")
            .field("val0", &self.val0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3val0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3val0 {{ val0: {=u16:?} }}", self.val0())
    }
}
#[doc = "Value Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3val1(pub u16);
impl Sm3val1 {
    #[doc = "Value 1."]
    #[must_use]
    #[inline(always)]
    pub const fn val1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 1."]
    #[inline(always)]
    pub const fn set_val1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3val1 {
    #[inline(always)]
    fn default() -> Sm3val1 {
        Sm3val1(0)
    }
}
impl core::fmt::Debug for Sm3val1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3val1")
            .field("val1", &self.val1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3val1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3val1 {{ val1: {=u16:?} }}", self.val1())
    }
}
#[doc = "Value Register 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3val2(pub u16);
impl Sm3val2 {
    #[doc = "Value 2."]
    #[must_use]
    #[inline(always)]
    pub const fn val2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 2."]
    #[inline(always)]
    pub const fn set_val2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3val2 {
    #[inline(always)]
    fn default() -> Sm3val2 {
        Sm3val2(0)
    }
}
impl core::fmt::Debug for Sm3val2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3val2")
            .field("val2", &self.val2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3val2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3val2 {{ val2: {=u16:?} }}", self.val2())
    }
}
#[doc = "Value Register 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3val3(pub u16);
impl Sm3val3 {
    #[doc = "Value 3."]
    #[must_use]
    #[inline(always)]
    pub const fn val3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 3."]
    #[inline(always)]
    pub const fn set_val3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3val3 {
    #[inline(always)]
    fn default() -> Sm3val3 {
        Sm3val3(0)
    }
}
impl core::fmt::Debug for Sm3val3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3val3")
            .field("val3", &self.val3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3val3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3val3 {{ val3: {=u16:?} }}", self.val3())
    }
}
#[doc = "Value Register 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3val4(pub u16);
impl Sm3val4 {
    #[doc = "Value 4."]
    #[must_use]
    #[inline(always)]
    pub const fn val4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 4."]
    #[inline(always)]
    pub const fn set_val4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3val4 {
    #[inline(always)]
    fn default() -> Sm3val4 {
        Sm3val4(0)
    }
}
impl core::fmt::Debug for Sm3val4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3val4")
            .field("val4", &self.val4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3val4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3val4 {{ val4: {=u16:?} }}", self.val4())
    }
}
#[doc = "Value Register 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sm3val5(pub u16);
impl Sm3val5 {
    #[doc = "Value 5."]
    #[must_use]
    #[inline(always)]
    pub const fn val5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 5."]
    #[inline(always)]
    pub const fn set_val5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sm3val5 {
    #[inline(always)]
    fn default() -> Sm3val5 {
        Sm3val5(0)
    }
}
impl core::fmt::Debug for Sm3val5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sm3val5")
            .field("val5", &self.val5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sm3val5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sm3val5 {{ val5: {=u16:?} }}", self.val5())
    }
}
#[doc = "Software Controlled Output Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swcout(pub u16);
impl Swcout {
    #[doc = "Submodule 0 Software Controlled Output 45."]
    #[must_use]
    #[inline(always)]
    pub const fn sm0out45(&self) -> Sm0out45 {
        let val = (self.0 >> 0usize) & 0x01;
        Sm0out45::from_bits(val as u8)
    }
    #[doc = "Submodule 0 Software Controlled Output 45."]
    #[inline(always)]
    pub const fn set_sm0out45(&mut self, val: Sm0out45) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Submodule 0 Software Controlled Output 23."]
    #[must_use]
    #[inline(always)]
    pub const fn sm0out23(&self) -> Sm0out23 {
        let val = (self.0 >> 1usize) & 0x01;
        Sm0out23::from_bits(val as u8)
    }
    #[doc = "Submodule 0 Software Controlled Output 23."]
    #[inline(always)]
    pub const fn set_sm0out23(&mut self, val: Sm0out23) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Submodule 1 Software Controlled Output 45."]
    #[must_use]
    #[inline(always)]
    pub const fn sm1out45(&self) -> Sm1out45 {
        let val = (self.0 >> 2usize) & 0x01;
        Sm1out45::from_bits(val as u8)
    }
    #[doc = "Submodule 1 Software Controlled Output 45."]
    #[inline(always)]
    pub const fn set_sm1out45(&mut self, val: Sm1out45) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Submodule 1 Software Controlled Output 23."]
    #[must_use]
    #[inline(always)]
    pub const fn sm1out23(&self) -> Sm1out23 {
        let val = (self.0 >> 3usize) & 0x01;
        Sm1out23::from_bits(val as u8)
    }
    #[doc = "Submodule 1 Software Controlled Output 23."]
    #[inline(always)]
    pub const fn set_sm1out23(&mut self, val: Sm1out23) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "Submodule 2 Software Controlled Output 45."]
    #[must_use]
    #[inline(always)]
    pub const fn sm2out45(&self) -> Sm2out45 {
        let val = (self.0 >> 4usize) & 0x01;
        Sm2out45::from_bits(val as u8)
    }
    #[doc = "Submodule 2 Software Controlled Output 45."]
    #[inline(always)]
    pub const fn set_sm2out45(&mut self, val: Sm2out45) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Submodule 2 Software Controlled Output 23."]
    #[must_use]
    #[inline(always)]
    pub const fn sm2out23(&self) -> Sm2out23 {
        let val = (self.0 >> 5usize) & 0x01;
        Sm2out23::from_bits(val as u8)
    }
    #[doc = "Submodule 2 Software Controlled Output 23."]
    #[inline(always)]
    pub const fn set_sm2out23(&mut self, val: Sm2out23) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Submodule 3 Software Controlled Output 45."]
    #[must_use]
    #[inline(always)]
    pub const fn sm3out45(&self) -> Sm3out45 {
        let val = (self.0 >> 6usize) & 0x01;
        Sm3out45::from_bits(val as u8)
    }
    #[doc = "Submodule 3 Software Controlled Output 45."]
    #[inline(always)]
    pub const fn set_sm3out45(&mut self, val: Sm3out45) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Submodule 3 Software Controlled Output 23."]
    #[must_use]
    #[inline(always)]
    pub const fn sm3out23(&self) -> Sm3out23 {
        let val = (self.0 >> 7usize) & 0x01;
        Sm3out23::from_bits(val as u8)
    }
    #[doc = "Submodule 3 Software Controlled Output 23."]
    #[inline(always)]
    pub const fn set_sm3out23(&mut self, val: Sm3out23) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
}
impl Default for Swcout {
    #[inline(always)]
    fn default() -> Swcout {
        Swcout(0)
    }
}
impl core::fmt::Debug for Swcout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swcout")
            .field("sm0out45", &self.sm0out45())
            .field("sm0out23", &self.sm0out23())
            .field("sm1out45", &self.sm1out45())
            .field("sm1out23", &self.sm1out23())
            .field("sm2out45", &self.sm2out45())
            .field("sm2out23", &self.sm2out23())
            .field("sm3out45", &self.sm3out45())
            .field("sm3out23", &self.sm3out23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swcout {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Swcout {{ sm0out45: {:?}, sm0out23: {:?}, sm1out45: {:?}, sm1out23: {:?}, sm2out45: {:?}, sm2out23: {:?}, sm3out45: {:?}, sm3out23: {:?} }}",
            self.sm0out45(),
            self.sm0out23(),
            self.sm1out45(),
            self.sm1out23(),
            self.sm2out45(),
            self.sm2out23(),
            self.sm3out45(),
            self.sm3out23()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fauto {
    #[doc = "Manual fault clearing. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\] is clear at the start of a half cycle or full cycle depending on the states of FSTS\\[FHALF\\] and FSTS\\[FFULL\\]. If neither FFULL nor FHALF is set, then the fault condition cannot be cleared. This is further controlled by FCTRL\\[FSAFE\\]."]
    MANUAL = 0x0,
    #[doc = "Automatic fault clearing. PWM outputs disabled by this fault are enabled when FSTS\\[FFPINx\\] is clear at the start of a half cycle or full cycle depending on the states of FSTS\\[FHALF\\] and FSTS\\[FFULL\\] without regard to the state of FSTS\\[FFLAGx\\]. If neither FFULL nor FHALF is set, then the fault condition cannot be cleared."]
    AUTOMATIC = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Fauto {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fauto {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fauto {
    #[inline(always)]
    fn from(val: u8) -> Fauto {
        Fauto::from_bits(val)
    }
}
impl From<Fauto> for u8 {
    #[inline(always)]
    fn from(val: Fauto) -> u8 {
        Fauto::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fflag {
    #[doc = "No fault on the FAULTx pin."]
    NO_FLAG = 0x0,
    #[doc = "Fault on the FAULTx pin."]
    FLAG = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Fflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fflag {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fflag {
    #[inline(always)]
    fn from(val: u8) -> Fflag {
        Fflag::from_bits(val)
    }
}
impl From<Fflag> for u8 {
    #[inline(always)]
    fn from(val: Fflag) -> u8 {
        Fflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ffull {
    #[doc = "PWM outputs are not re-enabled at the start of a full cycle."]
    PWM_OUTPUTS_NOT_REENABLED = 0x0,
    #[doc = "PWM outputs are re-enabled at the start of a full cycle."]
    PWM_OUTPUTS_REENABLED = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Ffull {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ffull {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ffull {
    #[inline(always)]
    fn from(val: u8) -> Ffull {
        Ffull::from_bits(val)
    }
}
impl From<Ffull> for u8 {
    #[inline(always)]
    fn from(val: Ffull) -> u8 {
        Ffull::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fhalf {
    #[doc = "PWM outputs are not re-enabled at the start of a half cycle."]
    PWM_OUTPUTS_NOT_REENABLED = 0x0,
    #[doc = "PWM outputs are re-enabled at the start of a half cycle (as defined by VAL0)."]
    PWM_OUTPUTS_REENABLED = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Fhalf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fhalf {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fhalf {
    #[inline(always)]
    fn from(val: u8) -> Fhalf {
        Fhalf::from_bits(val)
    }
}
impl From<Fhalf> for u8 {
    #[inline(always)]
    fn from(val: Fhalf) -> u8 {
        Fhalf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fie {
    #[doc = "FAULTx CPU interrupt requests disabled."]
    DISABLED = 0x0,
    #[doc = "FAULTx CPU interrupt requests enabled."]
    ENABLED = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Fie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fie {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fie {
    #[inline(always)]
    fn from(val: u8) -> Fie {
        Fie::from_bits(val)
    }
}
impl From<Fie> for u8 {
    #[inline(always)]
    fn from(val: Fie) -> u8 {
        Fie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flvl {
    #[doc = "A logic 0 on the fault input indicates a fault condition."]
    LOGIC_0 = 0x0,
    #[doc = "A logic 1 on the fault input indicates a fault condition."]
    LOGIC_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Flvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flvl {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flvl {
    #[inline(always)]
    fn from(val: u8) -> Flvl {
        Flvl::from_bits(val)
    }
}
impl From<Flvl> for u8 {
    #[inline(always)]
    fn from(val: Flvl) -> u8 {
        Flvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fsafe {
    #[doc = "Normal mode. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\] is clear at the start of a half cycle or full cycle depending on the states of FSTS\\[FHALF\\] and FSTS\\[FFULL\\] without regard to the state of FSTS\\[FFPINx\\]. If neither FHALF nor FFULL is set, then the fault condition cannot be cleared. The PWM outputs disabled by this fault input will not be re-enabled until the actual FAULTx input signal de-asserts since the fault input will combinationally disable the PWM outputs (as programmed in DISMAPn)."]
    NORMAL = 0x0,
    #[doc = "Safe mode. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\] is clear and FSTS\\[FFPINx\\] is clear at the start of a half cycle or full cycle depending on the states of FSTS\\[FHALF\\] and FSTS\\[FFULL\\]. If neither FHLAF nor FFULL is set, then the fault condition cannot be cleared."]
    SAFE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Fsafe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fsafe {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fsafe {
    #[inline(always)]
    fn from(val: u8) -> Fsafe {
        Fsafe::from_bits(val)
    }
}
impl From<Fsafe> for u8 {
    #[inline(always)]
    fn from(val: Fsafe) -> u8 {
        Fsafe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipol {
    #[doc = "PWM23 is used to generate complementary PWM pair in the corresponding submodule."]
    PWM23 = 0x0,
    #[doc = "PWM45 is used to generate complementary PWM pair in the corresponding submodule."]
    PWM45 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Ipol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipol {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipol {
    #[inline(always)]
    fn from(val: u8) -> Ipol {
        Ipol::from_bits(val)
    }
}
impl From<Ipol> for u8 {
    #[inline(always)]
    fn from(val: Ipol) -> u8 {
        Ipol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ldok {
    #[doc = "Do not load new values."]
    DISABLED = 0x0,
    #[doc = "Load prescaler, modulus, and PWM values of the corresponding submodule."]
    ENABLED = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Ldok {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ldok {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ldok {
    #[inline(always)]
    fn from(val: u8) -> Ldok {
        Ldok::from_bits(val)
    }
}
impl From<Ldok> for u8 {
    #[inline(always)]
    fn from(val: Ldok) -> u8 {
        Ldok::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nocomb {
    #[doc = "There is a combinational link from the fault inputs to the PWM outputs. The fault inputs are combined with the filtered and latched fault signals to disable the PWM outputs."]
    ENABLED = 0x0,
    #[doc = "The direct combinational path from the fault inputs to the PWM outputs is disabled and the filtered and latched fault signals are used to disable the PWM outputs."]
    DISABLED = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Nocomb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nocomb {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nocomb {
    #[inline(always)]
    fn from(val: u8) -> Nocomb {
        Nocomb::from_bits(val)
    }
}
impl From<Nocomb> for u8 {
    #[inline(always)]
    fn from(val: Nocomb) -> u8 {
        Nocomb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Run {
    #[doc = "PWM counter is stopped, but PWM outputs hold the current state."]
    DISABLED = 0x0,
    #[doc = "PWM counter is started in the corresponding submodule."]
    ENABLED = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Run {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Run {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Run {
    #[inline(always)]
    fn from(val: u8) -> Run {
        Run::from_bits(val)
    }
}
impl From<Run> for u8 {
    #[inline(always)]
    fn from(val: Run) -> u8 {
        Run::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0captctrlxEdgx0 {
    #[doc = "Disabled."]
    DISABLED = 0x0,
    #[doc = "Capture falling edges."]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges."]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge."]
    ANY_EDGE = 0x03,
}
impl Sm0captctrlxEdgx0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0captctrlxEdgx0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0captctrlxEdgx0 {
    #[inline(always)]
    fn from(val: u8) -> Sm0captctrlxEdgx0 {
        Sm0captctrlxEdgx0::from_bits(val)
    }
}
impl From<Sm0captctrlxEdgx0> for u8 {
    #[inline(always)]
    fn from(val: Sm0captctrlxEdgx0) -> u8 {
        Sm0captctrlxEdgx0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0captctrlxEdgx1 {
    #[doc = "Disabled."]
    DISABLED = 0x0,
    #[doc = "Capture falling edges."]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges."]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge."]
    ANY_EDGE = 0x03,
}
impl Sm0captctrlxEdgx1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0captctrlxEdgx1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0captctrlxEdgx1 {
    #[inline(always)]
    fn from(val: u8) -> Sm0captctrlxEdgx1 {
        Sm0captctrlxEdgx1::from_bits(val)
    }
}
impl From<Sm0captctrlxEdgx1> for u8 {
    #[inline(always)]
    fn from(val: Sm0captctrlxEdgx1) -> u8 {
        Sm0captctrlxEdgx1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0captctrlxInpSelx {
    #[doc = "Raw PWM_X input signal selected as source."]
    PWM_X = 0x0,
    #[doc = "Edge Counter."]
    EDGE_COUNTER = 0x01,
}
impl Sm0captctrlxInpSelx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0captctrlxInpSelx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0captctrlxInpSelx {
    #[inline(always)]
    fn from(val: u8) -> Sm0captctrlxInpSelx {
        Sm0captctrlxInpSelx::from_bits(val)
    }
}
impl From<Sm0captctrlxInpSelx> for u8 {
    #[inline(always)]
    fn from(val: Sm0captctrlxInpSelx) -> u8 {
        Sm0captctrlxInpSelx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0captctrlxOneshotx {
    #[doc = "Free Running."]
    FREE_RUNNING = 0x0,
    #[doc = "One Shot."]
    ONE_SHOT = 0x01,
}
impl Sm0captctrlxOneshotx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0captctrlxOneshotx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0captctrlxOneshotx {
    #[inline(always)]
    fn from(val: u8) -> Sm0captctrlxOneshotx {
        Sm0captctrlxOneshotx::from_bits(val)
    }
}
impl From<Sm0captctrlxOneshotx> for u8 {
    #[inline(always)]
    fn from(val: Sm0captctrlxOneshotx) -> u8 {
        Sm0captctrlxOneshotx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0ctrl2ClkSel {
    #[doc = "The IPBus clock is used as the clock for the local prescaler and counter."]
    IPBUS = 0x0,
    #[doc = "EXT_CLK is used as the clock for the local prescaler and counter."]
    EXT_CLK = 0x01,
    #[doc = "Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it forces the clock to logic 0."]
    AUX_CLK = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sm0ctrl2ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0ctrl2ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0ctrl2ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Sm0ctrl2ClkSel {
        Sm0ctrl2ClkSel::from_bits(val)
    }
}
impl From<Sm0ctrl2ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Sm0ctrl2ClkSel) -> u8 {
        Sm0ctrl2ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0ctrl2ForceSel {
    #[doc = "The local force signal, CTRL2\\[FORCE\\], from this submodule is used to force updates."]
    LOCAL = 0x0,
    #[doc = "The master force signal from submodule 0 is used to force updates. This setting should not be used in submodule 0 as it holds the FORCE OUTPUT signal to logic 0."]
    MASTER = 0x01,
    #[doc = "The local reload signal from this submodule is used to force updates without regard to the state of LDOK."]
    LOCAL_RELOAD = 0x02,
    #[doc = "The master reload signal from submodule0 is used to force updates if LDOK is set. This setting should not be used in submodule0 as it holds the FORCE OUTPUT signal to logic 0."]
    MASTER_RELOAD = 0x03,
    #[doc = "The local sync signal from this submodule is used to force updates."]
    LOCAL_SYNC = 0x04,
    #[doc = "The master sync signal from submodule0 is used to force updates. This setting should not be used in submodule0 as it holds the FORCE OUTPUT signal to logic 0."]
    MASTER_SYNC = 0x05,
    #[doc = "The external force signal, EXT_FORCE, from outside the PWM module causes updates."]
    EXT_FORCE = 0x06,
    #[doc = "The external sync signal, EXT_SYNC, from outside the PWM module causes updates."]
    EXT_SYNC = 0x07,
}
impl Sm0ctrl2ForceSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0ctrl2ForceSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0ctrl2ForceSel {
    #[inline(always)]
    fn from(val: u8) -> Sm0ctrl2ForceSel {
        Sm0ctrl2ForceSel::from_bits(val)
    }
}
impl From<Sm0ctrl2ForceSel> for u8 {
    #[inline(always)]
    fn from(val: Sm0ctrl2ForceSel) -> u8 {
        Sm0ctrl2ForceSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0ctrl2Indep {
    #[doc = "PWM_A and PWM_B form a complementary PWM pair."]
    COMPLEMENTARY = 0x0,
    #[doc = "PWM_A and PWM_B outputs are independent PWMs."]
    INDEPENDENT = 0x01,
}
impl Sm0ctrl2Indep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0ctrl2Indep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0ctrl2Indep {
    #[inline(always)]
    fn from(val: u8) -> Sm0ctrl2Indep {
        Sm0ctrl2Indep::from_bits(val)
    }
}
impl From<Sm0ctrl2Indep> for u8 {
    #[inline(always)]
    fn from(val: Sm0ctrl2Indep) -> u8 {
        Sm0ctrl2Indep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0ctrl2InitSel {
    #[doc = "Local sync (PWM_X) causes initialization."]
    PWM_X = 0x0,
    #[doc = "Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0. The submodule counter will only re-initialize when a master reload occurs."]
    MASTER_RELOAD = 0x01,
    #[doc = "Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0."]
    MASTER_SYNC = 0x02,
    #[doc = "EXT_SYNC causes initialization."]
    EXT_SYNC = 0x03,
}
impl Sm0ctrl2InitSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0ctrl2InitSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0ctrl2InitSel {
    #[inline(always)]
    fn from(val: u8) -> Sm0ctrl2InitSel {
        Sm0ctrl2InitSel::from_bits(val)
    }
}
impl From<Sm0ctrl2InitSel> for u8 {
    #[inline(always)]
    fn from(val: Sm0ctrl2InitSel) -> u8 {
        Sm0ctrl2InitSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0ctrl2ReloadSel {
    #[doc = "The local RELOAD signal is used to reload registers."]
    LOCAL = 0x0,
    #[doc = "The master RELOAD signal (from submodule 0) is used to reload registers. This setting should not be used in submodule 0 as it forces the RELOAD signal to logic 0."]
    MASTER = 0x01,
}
impl Sm0ctrl2ReloadSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0ctrl2ReloadSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0ctrl2ReloadSel {
    #[inline(always)]
    fn from(val: u8) -> Sm0ctrl2ReloadSel {
        Sm0ctrl2ReloadSel::from_bits(val)
    }
}
impl From<Sm0ctrl2ReloadSel> for u8 {
    #[inline(always)]
    fn from(val: Sm0ctrl2ReloadSel) -> u8 {
        Sm0ctrl2ReloadSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0ctrlCompmode {
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to\" method. This means that PWM edges are only produced when the counter is equal to one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period maintains this state until a match with VAL3 clears the output in the following period."]
    EQUAL_TO = 0x0,
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to or greater than\" method. This means that PWM edges are produced when the counter is equal to or greater than one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period could go low at the start of the next period if the starting counter value is greater than (but not necessarily equal to) the new VAL3 value."]
    EQUAL_TO_OR_GREATER_THAN = 0x01,
}
impl Sm0ctrlCompmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0ctrlCompmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0ctrlCompmode {
    #[inline(always)]
    fn from(val: u8) -> Sm0ctrlCompmode {
        Sm0ctrlCompmode::from_bits(val)
    }
}
impl From<Sm0ctrlCompmode> for u8 {
    #[inline(always)]
    fn from(val: Sm0ctrlCompmode) -> u8 {
        Sm0ctrlCompmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0ctrlLdfq {
    #[doc = "Every PWM opportunity."]
    EVERYPWM = 0x0,
    #[doc = "Every 2 PWM opportunities."]
    EVERY2PWM = 0x01,
    #[doc = "Every 3 PWM opportunities."]
    EVERY3PWM = 0x02,
    #[doc = "Every 4 PWM opportunities."]
    EVERY4PWM = 0x03,
    #[doc = "Every 5 PWM opportunities."]
    EVERY5PWM = 0x04,
    #[doc = "Every 6 PWM opportunities."]
    EVERY6PWM = 0x05,
    #[doc = "Every 7 PWM opportunities."]
    EVERY7PWM = 0x06,
    #[doc = "Every 8 PWM opportunities."]
    EVERY8PWM = 0x07,
    #[doc = "Every 9 PWM opportunities."]
    EVERY9PWM = 0x08,
    #[doc = "Every 10 PWM opportunities."]
    EVERY10PWM = 0x09,
    #[doc = "Every 11 PWM opportunities."]
    EVERY11PWM = 0x0a,
    #[doc = "Every 12 PWM opportunities."]
    EVERY12PWM = 0x0b,
    #[doc = "Every 13 PWM opportunities."]
    EVERY13PWM = 0x0c,
    #[doc = "Every 14 PWM opportunities."]
    EVERY14PWM = 0x0d,
    #[doc = "Every 15 PWM opportunities."]
    EVERY15PWM = 0x0e,
    #[doc = "Every 16 PWM opportunities."]
    EVERY16PWM = 0x0f,
}
impl Sm0ctrlLdfq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0ctrlLdfq {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0ctrlLdfq {
    #[inline(always)]
    fn from(val: u8) -> Sm0ctrlLdfq {
        Sm0ctrlLdfq::from_bits(val)
    }
}
impl From<Sm0ctrlLdfq> for u8 {
    #[inline(always)]
    fn from(val: Sm0ctrlLdfq) -> u8 {
        Sm0ctrlLdfq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0ctrlLdmod {
    #[doc = "Buffered registers of this submodule are loaded and take effect at the next PWM reload if MCTRL\\[LDOK\\] is set."]
    NEXT_PWM_RELOAD = 0x0,
    #[doc = "Buffered registers of this submodule are loaded and take effect immediately upon MCTRL\\[LDOK\\] being set. In this case, it is not necessary to set CTRL\\[FULL\\] or CTRL\\[HALF\\]."]
    MTCTRL_LDOK_SET = 0x01,
}
impl Sm0ctrlLdmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0ctrlLdmod {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0ctrlLdmod {
    #[inline(always)]
    fn from(val: u8) -> Sm0ctrlLdmod {
        Sm0ctrlLdmod::from_bits(val)
    }
}
impl From<Sm0ctrlLdmod> for u8 {
    #[inline(always)]
    fn from(val: Sm0ctrlLdmod) -> u8 {
        Sm0ctrlLdmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0ctrlPrsc {
    #[doc = "Prescaler 1."]
    ONE = 0x0,
    #[doc = "Prescaler 2."]
    TWO = 0x01,
    #[doc = "Prescaler 4."]
    FOUR = 0x02,
    #[doc = "Prescaler 8."]
    EIGHT = 0x03,
    #[doc = "Prescaler 16."]
    SIXTEEN = 0x04,
    #[doc = "Prescaler 32."]
    THIRTYTWO = 0x05,
    #[doc = "Prescaler 64."]
    SIXTYFOUR = 0x06,
    #[doc = "Prescaler 128."]
    HUNDREDTWENTYEIGHT = 0x07,
}
impl Sm0ctrlPrsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0ctrlPrsc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0ctrlPrsc {
    #[inline(always)]
    fn from(val: u8) -> Sm0ctrlPrsc {
        Sm0ctrlPrsc::from_bits(val)
    }
}
impl From<Sm0ctrlPrsc> for u8 {
    #[inline(always)]
    fn from(val: Sm0ctrlPrsc) -> u8 {
        Sm0ctrlPrsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0dmaenCaptde {
    #[doc = "Read DMA requests disabled."]
    DISABLED = 0x0,
    #[doc = "Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN\\[CA1DE\\], DMAEN\\[CA0DE\\], DMAEN\\[CB1DE\\], DMAEN\\[CB0DE\\], DMAEN\\[CX1DE\\], or DMAEN\\[CX0DE\\] to be set to determine which watermark(s) the DMA request is sensitive."]
    EXCEEDFIFO = 0x01,
    #[doc = "A local synchronization (VAL1 matches counter) sets the read DMA request."]
    LOCAL_SYNC = 0x02,
    #[doc = "A local reload (STS\\[RF\\] being set) sets the read DMA request."]
    LOCAL_RELOAD = 0x03,
}
impl Sm0dmaenCaptde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0dmaenCaptde {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0dmaenCaptde {
    #[inline(always)]
    fn from(val: u8) -> Sm0dmaenCaptde {
        Sm0dmaenCaptde::from_bits(val)
    }
}
impl From<Sm0dmaenCaptde> for u8 {
    #[inline(always)]
    fn from(val: Sm0dmaenCaptde) -> u8 {
        Sm0dmaenCaptde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0dmaenFand {
    #[doc = "Selected FIFO watermarks are OR'ed together."]
    OR = 0x0,
    #[doc = "Selected FIFO watermarks are AND'ed together."]
    AND = 0x01,
}
impl Sm0dmaenFand {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0dmaenFand {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0dmaenFand {
    #[inline(always)]
    fn from(val: u8) -> Sm0dmaenFand {
        Sm0dmaenFand::from_bits(val)
    }
}
impl From<Sm0dmaenFand> for u8 {
    #[inline(always)]
    fn from(val: Sm0dmaenFand) -> u8 {
        Sm0dmaenFand::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0intenCmpie {
    #[doc = "The corresponding STS\\[CMPF\\] bit will not cause an interrupt request."]
    DISABLED = 0x0,
    #[doc = "The corresponding STS\\[CMPF\\] bit will cause an interrupt request."]
    ENABLED = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
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
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Sm0intenCmpie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0intenCmpie {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0intenCmpie {
    #[inline(always)]
    fn from(val: u8) -> Sm0intenCmpie {
        Sm0intenCmpie::from_bits(val)
    }
}
impl From<Sm0intenCmpie> for u8 {
    #[inline(always)]
    fn from(val: Sm0intenCmpie) -> u8 {
        Sm0intenCmpie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0octrlPwmafs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl Sm0octrlPwmafs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0octrlPwmafs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0octrlPwmafs {
    #[inline(always)]
    fn from(val: u8) -> Sm0octrlPwmafs {
        Sm0octrlPwmafs::from_bits(val)
    }
}
impl From<Sm0octrlPwmafs> for u8 {
    #[inline(always)]
    fn from(val: Sm0octrlPwmafs) -> u8 {
        Sm0octrlPwmafs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0octrlPwmbfs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl Sm0octrlPwmbfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0octrlPwmbfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0octrlPwmbfs {
    #[inline(always)]
    fn from(val: u8) -> Sm0octrlPwmbfs {
        Sm0octrlPwmbfs::from_bits(val)
    }
}
impl From<Sm0octrlPwmbfs> for u8 {
    #[inline(always)]
    fn from(val: Sm0octrlPwmbfs) -> u8 {
        Sm0octrlPwmbfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0octrlPwmxfs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl Sm0octrlPwmxfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0octrlPwmxfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0octrlPwmxfs {
    #[inline(always)]
    fn from(val: u8) -> Sm0octrlPwmxfs {
        Sm0octrlPwmxfs::from_bits(val)
    }
}
impl From<Sm0octrlPwmxfs> for u8 {
    #[inline(always)]
    fn from(val: Sm0octrlPwmxfs) -> u8 {
        Sm0octrlPwmxfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0out23 {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    LOGIC_0 = 0x0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    LOGIC_1 = 0x01,
}
impl Sm0out23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0out23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0out23 {
    #[inline(always)]
    fn from(val: u8) -> Sm0out23 {
        Sm0out23::from_bits(val)
    }
}
impl From<Sm0out23> for u8 {
    #[inline(always)]
    fn from(val: Sm0out23) -> u8 {
        Sm0out23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0out45 {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM45."]
    LOGIC_0 = 0x0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM45."]
    LOGIC_1 = 0x01,
}
impl Sm0out45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0out45 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0out45 {
    #[inline(always)]
    fn from(val: u8) -> Sm0out45 {
        Sm0out45::from_bits(val)
    }
}
impl From<Sm0out45> for u8 {
    #[inline(always)]
    fn from(val: Sm0out45) -> u8 {
        Sm0out45::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0sel23 {
    #[doc = "Generated SM0PWM23 signal used by the deadtime logic."]
    SM0PWM23 = 0x0,
    #[doc = "Inverted generated SM0PWM23 signal used by the deadtime logic."]
    INVERTED_SM0PWM23 = 0x01,
    #[doc = "SWCOUT\\[SM0OUT23\\] used by the deadtime logic."]
    SM0OUT23 = 0x02,
    #[doc = "PWM0_EXTA signal used by the deadtime logic."]
    PWM0_EXTA = 0x03,
}
impl Sm0sel23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0sel23 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0sel23 {
    #[inline(always)]
    fn from(val: u8) -> Sm0sel23 {
        Sm0sel23::from_bits(val)
    }
}
impl From<Sm0sel23> for u8 {
    #[inline(always)]
    fn from(val: Sm0sel23) -> u8 {
        Sm0sel23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0sel45 {
    #[doc = "Generated SM0PWM45 signal used by the deadtime logic."]
    SM0PWM45 = 0x0,
    #[doc = "Inverted generated SM0PWM45 signal used by the deadtime logic."]
    INVERTED_SM0PWM45 = 0x01,
    #[doc = "SWCOUT\\[SM0OUT45\\] used by the deadtime logic."]
    SM0OUT45 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sm0sel45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0sel45 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0sel45 {
    #[inline(always)]
    fn from(val: u8) -> Sm0sel45 {
        Sm0sel45::from_bits(val)
    }
}
impl From<Sm0sel45> for u8 {
    #[inline(always)]
    fn from(val: Sm0sel45) -> u8 {
        Sm0sel45::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0stsCmpf {
    #[doc = "No compare event has occurred for a particular VALx value."]
    NO_EVENT = 0x0,
    #[doc = "A compare event has occurred for a particular VALx value."]
    EVENT = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
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
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Sm0stsCmpf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0stsCmpf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0stsCmpf {
    #[inline(always)]
    fn from(val: u8) -> Sm0stsCmpf {
        Sm0stsCmpf::from_bits(val)
    }
}
impl From<Sm0stsCmpf> for u8 {
    #[inline(always)]
    fn from(val: Sm0stsCmpf) -> u8 {
        Sm0stsCmpf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0tctrlOutTrigEn {
    _RESERVED_0 = 0x0,
    #[doc = "PWM_OUT_TRIG0 will set when the counter value matches the VAL0 value."]
    VAL0 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
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
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Sm0tctrlOutTrigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0tctrlOutTrigEn {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0tctrlOutTrigEn {
    #[inline(always)]
    fn from(val: u8) -> Sm0tctrlOutTrigEn {
        Sm0tctrlOutTrigEn::from_bits(val)
    }
}
impl From<Sm0tctrlOutTrigEn> for u8 {
    #[inline(always)]
    fn from(val: Sm0tctrlOutTrigEn) -> u8 {
        Sm0tctrlOutTrigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0tctrlPwaot0 {
    #[doc = "Route the PWM_OUT_TRIG0 signal to PWM_MUX_TRIG0 port."]
    PWM_OUT_TRIG0_SIGNAL = 0x0,
    #[doc = "Route the PWM_A output to the PWM_MUX_TRIG0 port."]
    PWMA_OUTPUT = 0x01,
}
impl Sm0tctrlPwaot0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0tctrlPwaot0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0tctrlPwaot0 {
    #[inline(always)]
    fn from(val: u8) -> Sm0tctrlPwaot0 {
        Sm0tctrlPwaot0::from_bits(val)
    }
}
impl From<Sm0tctrlPwaot0> for u8 {
    #[inline(always)]
    fn from(val: Sm0tctrlPwaot0) -> u8 {
        Sm0tctrlPwaot0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0tctrlPwbot1 {
    #[doc = "Route the PWM_OUT_TRIG1 signal to PWM_MUX_TRIG1 port."]
    PWM_OUT_TRIG1_SIGNAL = 0x0,
    #[doc = "Route the PWM_B output to the PWM_MUX_TRIG1 port."]
    PWMB_OUTPUT = 0x01,
}
impl Sm0tctrlPwbot1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0tctrlPwbot1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0tctrlPwbot1 {
    #[inline(always)]
    fn from(val: u8) -> Sm0tctrlPwbot1 {
        Sm0tctrlPwbot1::from_bits(val)
    }
}
impl From<Sm0tctrlPwbot1> for u8 {
    #[inline(always)]
    fn from(val: Sm0tctrlPwbot1) -> u8 {
        Sm0tctrlPwbot1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0tctrlTrgfrq {
    #[doc = "Trigger outputs are generated during every PWM period even if the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
    EVERYPWM = 0x0,
    #[doc = "Trigger outputs are generated only during the final PWM period prior to a reload opportunity when the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
    FINALPWM = 0x01,
}
impl Sm0tctrlTrgfrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0tctrlTrgfrq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0tctrlTrgfrq {
    #[inline(always)]
    fn from(val: u8) -> Sm0tctrlTrgfrq {
        Sm0tctrlTrgfrq::from_bits(val)
    }
}
impl From<Sm0tctrlTrgfrq> for u8 {
    #[inline(always)]
    fn from(val: Sm0tctrlTrgfrq) -> u8 {
        Sm0tctrlTrgfrq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1captctrlxEdgx0 {
    #[doc = "Disabled."]
    DISABLED = 0x0,
    #[doc = "Capture falling edges."]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges."]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge."]
    ANY_EDGE = 0x03,
}
impl Sm1captctrlxEdgx0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1captctrlxEdgx0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1captctrlxEdgx0 {
    #[inline(always)]
    fn from(val: u8) -> Sm1captctrlxEdgx0 {
        Sm1captctrlxEdgx0::from_bits(val)
    }
}
impl From<Sm1captctrlxEdgx0> for u8 {
    #[inline(always)]
    fn from(val: Sm1captctrlxEdgx0) -> u8 {
        Sm1captctrlxEdgx0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1captctrlxEdgx1 {
    #[doc = "Disabled."]
    DISABLED = 0x0,
    #[doc = "Capture falling edges."]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges."]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge."]
    ANY_EDGE = 0x03,
}
impl Sm1captctrlxEdgx1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1captctrlxEdgx1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1captctrlxEdgx1 {
    #[inline(always)]
    fn from(val: u8) -> Sm1captctrlxEdgx1 {
        Sm1captctrlxEdgx1::from_bits(val)
    }
}
impl From<Sm1captctrlxEdgx1> for u8 {
    #[inline(always)]
    fn from(val: Sm1captctrlxEdgx1) -> u8 {
        Sm1captctrlxEdgx1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1captctrlxInpSelx {
    #[doc = "Raw PWM_X input signal selected as source."]
    PWM_X = 0x0,
    #[doc = "Edge Counter."]
    EDGE_COUNTER = 0x01,
}
impl Sm1captctrlxInpSelx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1captctrlxInpSelx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1captctrlxInpSelx {
    #[inline(always)]
    fn from(val: u8) -> Sm1captctrlxInpSelx {
        Sm1captctrlxInpSelx::from_bits(val)
    }
}
impl From<Sm1captctrlxInpSelx> for u8 {
    #[inline(always)]
    fn from(val: Sm1captctrlxInpSelx) -> u8 {
        Sm1captctrlxInpSelx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1captctrlxOneshotx {
    #[doc = "Free Running."]
    FREE_RUNNING = 0x0,
    #[doc = "One Shot."]
    ONE_SHOT = 0x01,
}
impl Sm1captctrlxOneshotx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1captctrlxOneshotx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1captctrlxOneshotx {
    #[inline(always)]
    fn from(val: u8) -> Sm1captctrlxOneshotx {
        Sm1captctrlxOneshotx::from_bits(val)
    }
}
impl From<Sm1captctrlxOneshotx> for u8 {
    #[inline(always)]
    fn from(val: Sm1captctrlxOneshotx) -> u8 {
        Sm1captctrlxOneshotx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1ctrl2ClkSel {
    #[doc = "The IPBus clock is used as the clock for the local prescaler and counter."]
    IPBUS = 0x0,
    #[doc = "EXT_CLK is used as the clock for the local prescaler and counter."]
    EXT_CLK = 0x01,
    #[doc = "Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it forces the clock to logic 0."]
    AUX_CLK = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sm1ctrl2ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1ctrl2ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1ctrl2ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Sm1ctrl2ClkSel {
        Sm1ctrl2ClkSel::from_bits(val)
    }
}
impl From<Sm1ctrl2ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Sm1ctrl2ClkSel) -> u8 {
        Sm1ctrl2ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1ctrl2ForceSel {
    #[doc = "The local force signal, CTRL2\\[FORCE\\], from this submodule is used to force updates."]
    LOCAL = 0x0,
    #[doc = "The master force signal from submodule 0 is used to force updates. This setting should not be used in submodule 0 as it holds the FORCE OUTPUT signal to logic 0."]
    MASTER = 0x01,
    #[doc = "The local reload signal from this submodule is used to force updates without regard to the state of LDOK."]
    LOCAL_RELOAD = 0x02,
    #[doc = "The master reload signal from submodule0 is used to force updates if LDOK is set. This setting should not be used in submodule0 as it holds the FORCE OUTPUT signal to logic 0."]
    MASTER_RELOAD = 0x03,
    #[doc = "The local sync signal from this submodule is used to force updates."]
    LOCAL_SYNC = 0x04,
    #[doc = "The master sync signal from submodule0 is used to force updates. This setting should not be used in submodule0 as it holds the FORCE OUTPUT signal to logic 0."]
    MASTER_SYNC = 0x05,
    #[doc = "The external force signal, EXT_FORCE, from outside the PWM module causes updates."]
    EXT_FORCE = 0x06,
    #[doc = "The external sync signal, EXT_SYNC, from outside the PWM module causes updates."]
    EXT_SYNC = 0x07,
}
impl Sm1ctrl2ForceSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1ctrl2ForceSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1ctrl2ForceSel {
    #[inline(always)]
    fn from(val: u8) -> Sm1ctrl2ForceSel {
        Sm1ctrl2ForceSel::from_bits(val)
    }
}
impl From<Sm1ctrl2ForceSel> for u8 {
    #[inline(always)]
    fn from(val: Sm1ctrl2ForceSel) -> u8 {
        Sm1ctrl2ForceSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1ctrl2Indep {
    #[doc = "PWM_A and PWM_B form a complementary PWM pair."]
    COMPLEMENTARY = 0x0,
    #[doc = "PWM_A and PWM_B outputs are independent PWMs."]
    INDEPENDENT = 0x01,
}
impl Sm1ctrl2Indep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1ctrl2Indep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1ctrl2Indep {
    #[inline(always)]
    fn from(val: u8) -> Sm1ctrl2Indep {
        Sm1ctrl2Indep::from_bits(val)
    }
}
impl From<Sm1ctrl2Indep> for u8 {
    #[inline(always)]
    fn from(val: Sm1ctrl2Indep) -> u8 {
        Sm1ctrl2Indep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1ctrl2InitSel {
    #[doc = "Local sync (PWM_X) causes initialization."]
    PWM_X = 0x0,
    #[doc = "Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0. The submodule counter will only re-initialize when a master reload occurs."]
    MASTER_RELOAD = 0x01,
    #[doc = "Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0."]
    MASTER_SYNC = 0x02,
    #[doc = "EXT_SYNC causes initialization."]
    EXT_SYNC = 0x03,
}
impl Sm1ctrl2InitSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1ctrl2InitSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1ctrl2InitSel {
    #[inline(always)]
    fn from(val: u8) -> Sm1ctrl2InitSel {
        Sm1ctrl2InitSel::from_bits(val)
    }
}
impl From<Sm1ctrl2InitSel> for u8 {
    #[inline(always)]
    fn from(val: Sm1ctrl2InitSel) -> u8 {
        Sm1ctrl2InitSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1ctrl2ReloadSel {
    #[doc = "The local RELOAD signal is used to reload registers."]
    LOCAL = 0x0,
    #[doc = "The master RELOAD signal (from submodule 0) is used to reload registers. This setting should not be used in submodule 0 as it forces the RELOAD signal to logic 0."]
    MASTER = 0x01,
}
impl Sm1ctrl2ReloadSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1ctrl2ReloadSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1ctrl2ReloadSel {
    #[inline(always)]
    fn from(val: u8) -> Sm1ctrl2ReloadSel {
        Sm1ctrl2ReloadSel::from_bits(val)
    }
}
impl From<Sm1ctrl2ReloadSel> for u8 {
    #[inline(always)]
    fn from(val: Sm1ctrl2ReloadSel) -> u8 {
        Sm1ctrl2ReloadSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1ctrlCompmode {
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to\" method. This means that PWM edges are only produced when the counter is equal to one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period maintains this state until a match with VAL3 clears the output in the following period."]
    EQUAL_TO = 0x0,
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to or greater than\" method. This means that PWM edges are produced when the counter is equal to or greater than one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period could go low at the start of the next period if the starting counter value is greater than (but not necessarily equal to) the new VAL3 value."]
    EQUAL_TO_OR_GREATER_THAN = 0x01,
}
impl Sm1ctrlCompmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1ctrlCompmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1ctrlCompmode {
    #[inline(always)]
    fn from(val: u8) -> Sm1ctrlCompmode {
        Sm1ctrlCompmode::from_bits(val)
    }
}
impl From<Sm1ctrlCompmode> for u8 {
    #[inline(always)]
    fn from(val: Sm1ctrlCompmode) -> u8 {
        Sm1ctrlCompmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1ctrlLdfq {
    #[doc = "Every PWM opportunity."]
    EVERYPWM = 0x0,
    #[doc = "Every 2 PWM opportunities."]
    EVERY2PWM = 0x01,
    #[doc = "Every 3 PWM opportunities."]
    EVERY3PWM = 0x02,
    #[doc = "Every 4 PWM opportunities."]
    EVERY4PWM = 0x03,
    #[doc = "Every 5 PWM opportunities."]
    EVERY5PWM = 0x04,
    #[doc = "Every 6 PWM opportunities."]
    EVERY6PWM = 0x05,
    #[doc = "Every 7 PWM opportunities."]
    EVERY7PWM = 0x06,
    #[doc = "Every 8 PWM opportunities."]
    EVERY8PWM = 0x07,
    #[doc = "Every 9 PWM opportunities."]
    EVERY9PWM = 0x08,
    #[doc = "Every 10 PWM opportunities."]
    EVERY10PWM = 0x09,
    #[doc = "Every 11 PWM opportunities."]
    EVERY11PWM = 0x0a,
    #[doc = "Every 12 PWM opportunities."]
    EVERY12PWM = 0x0b,
    #[doc = "Every 13 PWM opportunities."]
    EVERY13PWM = 0x0c,
    #[doc = "Every 14 PWM opportunities."]
    EVERY14PWM = 0x0d,
    #[doc = "Every 15 PWM opportunities."]
    EVERY15PWM = 0x0e,
    #[doc = "Every 16 PWM opportunities."]
    EVERY16PWM = 0x0f,
}
impl Sm1ctrlLdfq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1ctrlLdfq {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1ctrlLdfq {
    #[inline(always)]
    fn from(val: u8) -> Sm1ctrlLdfq {
        Sm1ctrlLdfq::from_bits(val)
    }
}
impl From<Sm1ctrlLdfq> for u8 {
    #[inline(always)]
    fn from(val: Sm1ctrlLdfq) -> u8 {
        Sm1ctrlLdfq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1ctrlLdmod {
    #[doc = "Buffered registers of this submodule are loaded and take effect at the next PWM reload if MCTRL\\[LDOK\\] is set."]
    NEXT_PWM_RELOAD = 0x0,
    #[doc = "Buffered registers of this submodule are loaded and take effect immediately upon MCTRL\\[LDOK\\] being set. In this case, it is not necessary to set CTRL\\[FULL\\] or CTRL\\[HALF\\]."]
    MTCTRL_LDOK_SET = 0x01,
}
impl Sm1ctrlLdmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1ctrlLdmod {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1ctrlLdmod {
    #[inline(always)]
    fn from(val: u8) -> Sm1ctrlLdmod {
        Sm1ctrlLdmod::from_bits(val)
    }
}
impl From<Sm1ctrlLdmod> for u8 {
    #[inline(always)]
    fn from(val: Sm1ctrlLdmod) -> u8 {
        Sm1ctrlLdmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1ctrlPrsc {
    #[doc = "Prescaler 1."]
    ONE = 0x0,
    #[doc = "Prescaler 2."]
    TWO = 0x01,
    #[doc = "Prescaler 4."]
    FOUR = 0x02,
    #[doc = "Prescaler 8."]
    EIGHT = 0x03,
    #[doc = "Prescaler 16."]
    SIXTEEN = 0x04,
    #[doc = "Prescaler 32."]
    THIRTYTWO = 0x05,
    #[doc = "Prescaler 64."]
    SIXTYFOUR = 0x06,
    #[doc = "Prescaler 128."]
    HUNDREDTWENTYEIGHT = 0x07,
}
impl Sm1ctrlPrsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1ctrlPrsc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1ctrlPrsc {
    #[inline(always)]
    fn from(val: u8) -> Sm1ctrlPrsc {
        Sm1ctrlPrsc::from_bits(val)
    }
}
impl From<Sm1ctrlPrsc> for u8 {
    #[inline(always)]
    fn from(val: Sm1ctrlPrsc) -> u8 {
        Sm1ctrlPrsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1dmaenCaptde {
    #[doc = "Read DMA requests disabled."]
    DISABLED = 0x0,
    #[doc = "Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN\\[CA1DE\\], DMAEN\\[CA0DE\\], DMAEN\\[CB1DE\\], DMAEN\\[CB0DE\\], DMAEN\\[CX1DE\\], or DMAEN\\[CX0DE\\] to be set to determine which watermark(s) the DMA request is sensitive."]
    EXCEEDFIFO = 0x01,
    #[doc = "A local synchronization (VAL1 matches counter) sets the read DMA request."]
    LOCAL_SYNC = 0x02,
    #[doc = "A local reload (STS\\[RF\\] being set) sets the read DMA request."]
    LOCAL_RELOAD = 0x03,
}
impl Sm1dmaenCaptde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1dmaenCaptde {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1dmaenCaptde {
    #[inline(always)]
    fn from(val: u8) -> Sm1dmaenCaptde {
        Sm1dmaenCaptde::from_bits(val)
    }
}
impl From<Sm1dmaenCaptde> for u8 {
    #[inline(always)]
    fn from(val: Sm1dmaenCaptde) -> u8 {
        Sm1dmaenCaptde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1dmaenFand {
    #[doc = "Selected FIFO watermarks are OR'ed together."]
    OR = 0x0,
    #[doc = "Selected FIFO watermarks are AND'ed together."]
    AND = 0x01,
}
impl Sm1dmaenFand {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1dmaenFand {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1dmaenFand {
    #[inline(always)]
    fn from(val: u8) -> Sm1dmaenFand {
        Sm1dmaenFand::from_bits(val)
    }
}
impl From<Sm1dmaenFand> for u8 {
    #[inline(always)]
    fn from(val: Sm1dmaenFand) -> u8 {
        Sm1dmaenFand::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1intenCmpie {
    #[doc = "The corresponding STS\\[CMPF\\] bit will not cause an interrupt request."]
    DISABLED = 0x0,
    #[doc = "The corresponding STS\\[CMPF\\] bit will cause an interrupt request."]
    ENABLED = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
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
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Sm1intenCmpie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1intenCmpie {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1intenCmpie {
    #[inline(always)]
    fn from(val: u8) -> Sm1intenCmpie {
        Sm1intenCmpie::from_bits(val)
    }
}
impl From<Sm1intenCmpie> for u8 {
    #[inline(always)]
    fn from(val: Sm1intenCmpie) -> u8 {
        Sm1intenCmpie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1octrlPwmafs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl Sm1octrlPwmafs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1octrlPwmafs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1octrlPwmafs {
    #[inline(always)]
    fn from(val: u8) -> Sm1octrlPwmafs {
        Sm1octrlPwmafs::from_bits(val)
    }
}
impl From<Sm1octrlPwmafs> for u8 {
    #[inline(always)]
    fn from(val: Sm1octrlPwmafs) -> u8 {
        Sm1octrlPwmafs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1octrlPwmbfs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl Sm1octrlPwmbfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1octrlPwmbfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1octrlPwmbfs {
    #[inline(always)]
    fn from(val: u8) -> Sm1octrlPwmbfs {
        Sm1octrlPwmbfs::from_bits(val)
    }
}
impl From<Sm1octrlPwmbfs> for u8 {
    #[inline(always)]
    fn from(val: Sm1octrlPwmbfs) -> u8 {
        Sm1octrlPwmbfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1octrlPwmxfs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl Sm1octrlPwmxfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1octrlPwmxfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1octrlPwmxfs {
    #[inline(always)]
    fn from(val: u8) -> Sm1octrlPwmxfs {
        Sm1octrlPwmxfs::from_bits(val)
    }
}
impl From<Sm1octrlPwmxfs> for u8 {
    #[inline(always)]
    fn from(val: Sm1octrlPwmxfs) -> u8 {
        Sm1octrlPwmxfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1out23 {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 1 instead of PWM23."]
    LOGIC_0 = 0x0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM23."]
    LOGIC_1 = 0x01,
}
impl Sm1out23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1out23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1out23 {
    #[inline(always)]
    fn from(val: u8) -> Sm1out23 {
        Sm1out23::from_bits(val)
    }
}
impl From<Sm1out23> for u8 {
    #[inline(always)]
    fn from(val: Sm1out23) -> u8 {
        Sm1out23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1out45 {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 1 instead of PWM45."]
    LOGIC_0 = 0x0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM45."]
    LOGIC_1 = 0x01,
}
impl Sm1out45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1out45 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1out45 {
    #[inline(always)]
    fn from(val: u8) -> Sm1out45 {
        Sm1out45::from_bits(val)
    }
}
impl From<Sm1out45> for u8 {
    #[inline(always)]
    fn from(val: Sm1out45) -> u8 {
        Sm1out45::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1sel23 {
    #[doc = "Generated SM1PWM23 signal used by the deadtime logic."]
    SM1PWM23 = 0x0,
    #[doc = "Inverted generated SM1PWM23 signal used by the deadtime logic."]
    INVERTED_SM1PWM23 = 0x01,
    #[doc = "SWCOUT\\[SM1OUT23\\] used by the deadtime logic."]
    SM1OUT23 = 0x02,
    #[doc = "PWM1_EXTA signal used by the deadtime logic."]
    PWM1_EXTA = 0x03,
}
impl Sm1sel23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1sel23 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1sel23 {
    #[inline(always)]
    fn from(val: u8) -> Sm1sel23 {
        Sm1sel23::from_bits(val)
    }
}
impl From<Sm1sel23> for u8 {
    #[inline(always)]
    fn from(val: Sm1sel23) -> u8 {
        Sm1sel23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1sel45 {
    #[doc = "Generated SM1PWM45 signal used by the deadtime logic."]
    SM1PWM45 = 0x0,
    #[doc = "Inverted generated SM1PWM45 signal used by the deadtime logic."]
    INVERTED_SM1PWM45 = 0x01,
    #[doc = "SWCOUT\\[SM1OUT45\\] used by the deadtime logic."]
    SM1OUT45 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sm1sel45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1sel45 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1sel45 {
    #[inline(always)]
    fn from(val: u8) -> Sm1sel45 {
        Sm1sel45::from_bits(val)
    }
}
impl From<Sm1sel45> for u8 {
    #[inline(always)]
    fn from(val: Sm1sel45) -> u8 {
        Sm1sel45::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1stsCmpf {
    #[doc = "No compare event has occurred for a particular VALx value."]
    NO_EVENT = 0x0,
    #[doc = "A compare event has occurred for a particular VALx value."]
    EVENT = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
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
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Sm1stsCmpf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1stsCmpf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1stsCmpf {
    #[inline(always)]
    fn from(val: u8) -> Sm1stsCmpf {
        Sm1stsCmpf::from_bits(val)
    }
}
impl From<Sm1stsCmpf> for u8 {
    #[inline(always)]
    fn from(val: Sm1stsCmpf) -> u8 {
        Sm1stsCmpf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1tctrlOutTrigEn {
    _RESERVED_0 = 0x0,
    #[doc = "PWM_OUT_TRIG0 will set when the counter value matches the VAL0 value."]
    VAL0 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
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
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Sm1tctrlOutTrigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1tctrlOutTrigEn {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1tctrlOutTrigEn {
    #[inline(always)]
    fn from(val: u8) -> Sm1tctrlOutTrigEn {
        Sm1tctrlOutTrigEn::from_bits(val)
    }
}
impl From<Sm1tctrlOutTrigEn> for u8 {
    #[inline(always)]
    fn from(val: Sm1tctrlOutTrigEn) -> u8 {
        Sm1tctrlOutTrigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1tctrlPwaot0 {
    #[doc = "Route the PWM_OUT_TRIG0 signal to PWM_MUX_TRIG0 port."]
    PWM_OUT_TRIG0_SIGNAL = 0x0,
    #[doc = "Route the PWM_A output to the PWM_MUX_TRIG0 port."]
    PWMA_OUTPUT = 0x01,
}
impl Sm1tctrlPwaot0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1tctrlPwaot0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1tctrlPwaot0 {
    #[inline(always)]
    fn from(val: u8) -> Sm1tctrlPwaot0 {
        Sm1tctrlPwaot0::from_bits(val)
    }
}
impl From<Sm1tctrlPwaot0> for u8 {
    #[inline(always)]
    fn from(val: Sm1tctrlPwaot0) -> u8 {
        Sm1tctrlPwaot0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1tctrlPwbot1 {
    #[doc = "Route the PWM_OUT_TRIG1 signal to PWM_MUX_TRIG1 port."]
    PWM_OUT_TRIG1_SIGNAL = 0x0,
    #[doc = "Route the PWM_B output to the PWM_MUX_TRIG1 port."]
    PWMB_OUTPUT = 0x01,
}
impl Sm1tctrlPwbot1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1tctrlPwbot1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1tctrlPwbot1 {
    #[inline(always)]
    fn from(val: u8) -> Sm1tctrlPwbot1 {
        Sm1tctrlPwbot1::from_bits(val)
    }
}
impl From<Sm1tctrlPwbot1> for u8 {
    #[inline(always)]
    fn from(val: Sm1tctrlPwbot1) -> u8 {
        Sm1tctrlPwbot1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1tctrlTrgfrq {
    #[doc = "Trigger outputs are generated during every PWM period even if the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
    EVERYPWM = 0x0,
    #[doc = "Trigger outputs are generated only during the final PWM period prior to a reload opportunity when the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
    FINALPWM = 0x01,
}
impl Sm1tctrlTrgfrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1tctrlTrgfrq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1tctrlTrgfrq {
    #[inline(always)]
    fn from(val: u8) -> Sm1tctrlTrgfrq {
        Sm1tctrlTrgfrq::from_bits(val)
    }
}
impl From<Sm1tctrlTrgfrq> for u8 {
    #[inline(always)]
    fn from(val: Sm1tctrlTrgfrq) -> u8 {
        Sm1tctrlTrgfrq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2captctrlxEdgx0 {
    #[doc = "Disabled."]
    DISABLED = 0x0,
    #[doc = "Capture falling edges."]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges."]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge."]
    ANY_EDGE = 0x03,
}
impl Sm2captctrlxEdgx0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2captctrlxEdgx0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2captctrlxEdgx0 {
    #[inline(always)]
    fn from(val: u8) -> Sm2captctrlxEdgx0 {
        Sm2captctrlxEdgx0::from_bits(val)
    }
}
impl From<Sm2captctrlxEdgx0> for u8 {
    #[inline(always)]
    fn from(val: Sm2captctrlxEdgx0) -> u8 {
        Sm2captctrlxEdgx0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2captctrlxEdgx1 {
    #[doc = "Disabled."]
    DISABLED = 0x0,
    #[doc = "Capture falling edges."]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges."]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge."]
    ANY_EDGE = 0x03,
}
impl Sm2captctrlxEdgx1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2captctrlxEdgx1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2captctrlxEdgx1 {
    #[inline(always)]
    fn from(val: u8) -> Sm2captctrlxEdgx1 {
        Sm2captctrlxEdgx1::from_bits(val)
    }
}
impl From<Sm2captctrlxEdgx1> for u8 {
    #[inline(always)]
    fn from(val: Sm2captctrlxEdgx1) -> u8 {
        Sm2captctrlxEdgx1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2captctrlxInpSelx {
    #[doc = "Raw PWM_X input signal selected as source."]
    PWM_X = 0x0,
    #[doc = "Edge Counter."]
    EDGE_COUNTER = 0x01,
}
impl Sm2captctrlxInpSelx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2captctrlxInpSelx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2captctrlxInpSelx {
    #[inline(always)]
    fn from(val: u8) -> Sm2captctrlxInpSelx {
        Sm2captctrlxInpSelx::from_bits(val)
    }
}
impl From<Sm2captctrlxInpSelx> for u8 {
    #[inline(always)]
    fn from(val: Sm2captctrlxInpSelx) -> u8 {
        Sm2captctrlxInpSelx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2captctrlxOneshotx {
    #[doc = "Free Running."]
    FREE_RUNNING = 0x0,
    #[doc = "One Shot."]
    ONE_SHOT = 0x01,
}
impl Sm2captctrlxOneshotx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2captctrlxOneshotx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2captctrlxOneshotx {
    #[inline(always)]
    fn from(val: u8) -> Sm2captctrlxOneshotx {
        Sm2captctrlxOneshotx::from_bits(val)
    }
}
impl From<Sm2captctrlxOneshotx> for u8 {
    #[inline(always)]
    fn from(val: Sm2captctrlxOneshotx) -> u8 {
        Sm2captctrlxOneshotx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2ctrl2ClkSel {
    #[doc = "The IPBus clock is used as the clock for the local prescaler and counter."]
    IPBUS = 0x0,
    #[doc = "EXT_CLK is used as the clock for the local prescaler and counter."]
    EXT_CLK = 0x01,
    #[doc = "Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it forces the clock to logic 0."]
    AUX_CLK = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sm2ctrl2ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2ctrl2ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2ctrl2ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Sm2ctrl2ClkSel {
        Sm2ctrl2ClkSel::from_bits(val)
    }
}
impl From<Sm2ctrl2ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Sm2ctrl2ClkSel) -> u8 {
        Sm2ctrl2ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2ctrl2ForceSel {
    #[doc = "The local force signal, CTRL2\\[FORCE\\], from this submodule is used to force updates."]
    LOCAL = 0x0,
    #[doc = "The master force signal from submodule 0 is used to force updates. This setting should not be used in submodule 0 as it holds the FORCE OUTPUT signal to logic 0."]
    MASTER = 0x01,
    #[doc = "The local reload signal from this submodule is used to force updates without regard to the state of LDOK."]
    LOCAL_RELOAD = 0x02,
    #[doc = "The master reload signal from submodule0 is used to force updates if LDOK is set. This setting should not be used in submodule0 as it holds the FORCE OUTPUT signal to logic 0."]
    MASTER_RELOAD = 0x03,
    #[doc = "The local sync signal from this submodule is used to force updates."]
    LOCAL_SYNC = 0x04,
    #[doc = "The master sync signal from submodule0 is used to force updates. This setting should not be used in submodule0 as it holds the FORCE OUTPUT signal to logic 0."]
    MASTER_SYNC = 0x05,
    #[doc = "The external force signal, EXT_FORCE, from outside the PWM module causes updates."]
    EXT_FORCE = 0x06,
    #[doc = "The external sync signal, EXT_SYNC, from outside the PWM module causes updates."]
    EXT_SYNC = 0x07,
}
impl Sm2ctrl2ForceSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2ctrl2ForceSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2ctrl2ForceSel {
    #[inline(always)]
    fn from(val: u8) -> Sm2ctrl2ForceSel {
        Sm2ctrl2ForceSel::from_bits(val)
    }
}
impl From<Sm2ctrl2ForceSel> for u8 {
    #[inline(always)]
    fn from(val: Sm2ctrl2ForceSel) -> u8 {
        Sm2ctrl2ForceSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2ctrl2Indep {
    #[doc = "PWM_A and PWM_B form a complementary PWM pair."]
    COMPLEMENTARY = 0x0,
    #[doc = "PWM_A and PWM_B outputs are independent PWMs."]
    INDEPENDENT = 0x01,
}
impl Sm2ctrl2Indep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2ctrl2Indep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2ctrl2Indep {
    #[inline(always)]
    fn from(val: u8) -> Sm2ctrl2Indep {
        Sm2ctrl2Indep::from_bits(val)
    }
}
impl From<Sm2ctrl2Indep> for u8 {
    #[inline(always)]
    fn from(val: Sm2ctrl2Indep) -> u8 {
        Sm2ctrl2Indep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2ctrl2InitSel {
    #[doc = "Local sync (PWM_X) causes initialization."]
    PWM_X = 0x0,
    #[doc = "Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0. The submodule counter will only re-initialize when a master reload occurs."]
    MASTER_RELOAD = 0x01,
    #[doc = "Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0."]
    MASTER_SYNC = 0x02,
    #[doc = "EXT_SYNC causes initialization."]
    EXT_SYNC = 0x03,
}
impl Sm2ctrl2InitSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2ctrl2InitSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2ctrl2InitSel {
    #[inline(always)]
    fn from(val: u8) -> Sm2ctrl2InitSel {
        Sm2ctrl2InitSel::from_bits(val)
    }
}
impl From<Sm2ctrl2InitSel> for u8 {
    #[inline(always)]
    fn from(val: Sm2ctrl2InitSel) -> u8 {
        Sm2ctrl2InitSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2ctrl2ReloadSel {
    #[doc = "The local RELOAD signal is used to reload registers."]
    LOCAL = 0x0,
    #[doc = "The master RELOAD signal (from submodule 0) is used to reload registers. This setting should not be used in submodule 0 as it forces the RELOAD signal to logic 0."]
    MASTER = 0x01,
}
impl Sm2ctrl2ReloadSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2ctrl2ReloadSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2ctrl2ReloadSel {
    #[inline(always)]
    fn from(val: u8) -> Sm2ctrl2ReloadSel {
        Sm2ctrl2ReloadSel::from_bits(val)
    }
}
impl From<Sm2ctrl2ReloadSel> for u8 {
    #[inline(always)]
    fn from(val: Sm2ctrl2ReloadSel) -> u8 {
        Sm2ctrl2ReloadSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2ctrlCompmode {
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to\" method. This means that PWM edges are only produced when the counter is equal to one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period maintains this state until a match with VAL3 clears the output in the following period."]
    EQUAL_TO = 0x0,
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to or greater than\" method. This means that PWM edges are produced when the counter is equal to or greater than one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period could go low at the start of the next period if the starting counter value is greater than (but not necessarily equal to) the new VAL3 value."]
    EQUAL_TO_OR_GREATER_THAN = 0x01,
}
impl Sm2ctrlCompmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2ctrlCompmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2ctrlCompmode {
    #[inline(always)]
    fn from(val: u8) -> Sm2ctrlCompmode {
        Sm2ctrlCompmode::from_bits(val)
    }
}
impl From<Sm2ctrlCompmode> for u8 {
    #[inline(always)]
    fn from(val: Sm2ctrlCompmode) -> u8 {
        Sm2ctrlCompmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2ctrlLdfq {
    #[doc = "Every PWM opportunity."]
    EVERYPWM = 0x0,
    #[doc = "Every 2 PWM opportunities."]
    EVERY2PWM = 0x01,
    #[doc = "Every 3 PWM opportunities."]
    EVERY3PWM = 0x02,
    #[doc = "Every 4 PWM opportunities."]
    EVERY4PWM = 0x03,
    #[doc = "Every 5 PWM opportunities."]
    EVERY5PWM = 0x04,
    #[doc = "Every 6 PWM opportunities."]
    EVERY6PWM = 0x05,
    #[doc = "Every 7 PWM opportunities."]
    EVERY7PWM = 0x06,
    #[doc = "Every 8 PWM opportunities."]
    EVERY8PWM = 0x07,
    #[doc = "Every 9 PWM opportunities."]
    EVERY9PWM = 0x08,
    #[doc = "Every 10 PWM opportunities."]
    EVERY10PWM = 0x09,
    #[doc = "Every 11 PWM opportunities."]
    EVERY11PWM = 0x0a,
    #[doc = "Every 12 PWM opportunities."]
    EVERY12PWM = 0x0b,
    #[doc = "Every 13 PWM opportunities."]
    EVERY13PWM = 0x0c,
    #[doc = "Every 14 PWM opportunities."]
    EVERY14PWM = 0x0d,
    #[doc = "Every 15 PWM opportunities."]
    EVERY15PWM = 0x0e,
    #[doc = "Every 16 PWM opportunities."]
    EVERY16PWM = 0x0f,
}
impl Sm2ctrlLdfq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2ctrlLdfq {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2ctrlLdfq {
    #[inline(always)]
    fn from(val: u8) -> Sm2ctrlLdfq {
        Sm2ctrlLdfq::from_bits(val)
    }
}
impl From<Sm2ctrlLdfq> for u8 {
    #[inline(always)]
    fn from(val: Sm2ctrlLdfq) -> u8 {
        Sm2ctrlLdfq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2ctrlLdmod {
    #[doc = "Buffered registers of this submodule are loaded and take effect at the next PWM reload if MCTRL\\[LDOK\\] is set."]
    NEXT_PWM_RELOAD = 0x0,
    #[doc = "Buffered registers of this submodule are loaded and take effect immediately upon MCTRL\\[LDOK\\] being set. In this case, it is not necessary to set CTRL\\[FULL\\] or CTRL\\[HALF\\]."]
    MTCTRL_LDOK_SET = 0x01,
}
impl Sm2ctrlLdmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2ctrlLdmod {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2ctrlLdmod {
    #[inline(always)]
    fn from(val: u8) -> Sm2ctrlLdmod {
        Sm2ctrlLdmod::from_bits(val)
    }
}
impl From<Sm2ctrlLdmod> for u8 {
    #[inline(always)]
    fn from(val: Sm2ctrlLdmod) -> u8 {
        Sm2ctrlLdmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2ctrlPrsc {
    #[doc = "Prescaler 1."]
    ONE = 0x0,
    #[doc = "Prescaler 2."]
    TWO = 0x01,
    #[doc = "Prescaler 4."]
    FOUR = 0x02,
    #[doc = "Prescaler 8."]
    EIGHT = 0x03,
    #[doc = "Prescaler 16."]
    SIXTEEN = 0x04,
    #[doc = "Prescaler 32."]
    THIRTYTWO = 0x05,
    #[doc = "Prescaler 64."]
    SIXTYFOUR = 0x06,
    #[doc = "Prescaler 128."]
    HUNDREDTWENTYEIGHT = 0x07,
}
impl Sm2ctrlPrsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2ctrlPrsc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2ctrlPrsc {
    #[inline(always)]
    fn from(val: u8) -> Sm2ctrlPrsc {
        Sm2ctrlPrsc::from_bits(val)
    }
}
impl From<Sm2ctrlPrsc> for u8 {
    #[inline(always)]
    fn from(val: Sm2ctrlPrsc) -> u8 {
        Sm2ctrlPrsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2dmaenCaptde {
    #[doc = "Read DMA requests disabled."]
    DISABLED = 0x0,
    #[doc = "Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN\\[CA1DE\\], DMAEN\\[CA0DE\\], DMAEN\\[CB1DE\\], DMAEN\\[CB0DE\\], DMAEN\\[CX1DE\\], or DMAEN\\[CX0DE\\] to be set to determine which watermark(s) the DMA request is sensitive."]
    EXCEEDFIFO = 0x01,
    #[doc = "A local synchronization (VAL1 matches counter) sets the read DMA request."]
    LOCAL_SYNC = 0x02,
    #[doc = "A local reload (STS\\[RF\\] being set) sets the read DMA request."]
    LOCAL_RELOAD = 0x03,
}
impl Sm2dmaenCaptde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2dmaenCaptde {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2dmaenCaptde {
    #[inline(always)]
    fn from(val: u8) -> Sm2dmaenCaptde {
        Sm2dmaenCaptde::from_bits(val)
    }
}
impl From<Sm2dmaenCaptde> for u8 {
    #[inline(always)]
    fn from(val: Sm2dmaenCaptde) -> u8 {
        Sm2dmaenCaptde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2dmaenFand {
    #[doc = "Selected FIFO watermarks are OR'ed together."]
    OR = 0x0,
    #[doc = "Selected FIFO watermarks are AND'ed together."]
    AND = 0x01,
}
impl Sm2dmaenFand {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2dmaenFand {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2dmaenFand {
    #[inline(always)]
    fn from(val: u8) -> Sm2dmaenFand {
        Sm2dmaenFand::from_bits(val)
    }
}
impl From<Sm2dmaenFand> for u8 {
    #[inline(always)]
    fn from(val: Sm2dmaenFand) -> u8 {
        Sm2dmaenFand::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2intenCmpie {
    #[doc = "The corresponding STS\\[CMPF\\] bit will not cause an interrupt request."]
    DISABLED = 0x0,
    #[doc = "The corresponding STS\\[CMPF\\] bit will cause an interrupt request."]
    ENABLED = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
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
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Sm2intenCmpie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2intenCmpie {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2intenCmpie {
    #[inline(always)]
    fn from(val: u8) -> Sm2intenCmpie {
        Sm2intenCmpie::from_bits(val)
    }
}
impl From<Sm2intenCmpie> for u8 {
    #[inline(always)]
    fn from(val: Sm2intenCmpie) -> u8 {
        Sm2intenCmpie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2octrlPwmafs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl Sm2octrlPwmafs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2octrlPwmafs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2octrlPwmafs {
    #[inline(always)]
    fn from(val: u8) -> Sm2octrlPwmafs {
        Sm2octrlPwmafs::from_bits(val)
    }
}
impl From<Sm2octrlPwmafs> for u8 {
    #[inline(always)]
    fn from(val: Sm2octrlPwmafs) -> u8 {
        Sm2octrlPwmafs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2octrlPwmbfs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl Sm2octrlPwmbfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2octrlPwmbfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2octrlPwmbfs {
    #[inline(always)]
    fn from(val: u8) -> Sm2octrlPwmbfs {
        Sm2octrlPwmbfs::from_bits(val)
    }
}
impl From<Sm2octrlPwmbfs> for u8 {
    #[inline(always)]
    fn from(val: Sm2octrlPwmbfs) -> u8 {
        Sm2octrlPwmbfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2octrlPwmxfs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl Sm2octrlPwmxfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2octrlPwmxfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2octrlPwmxfs {
    #[inline(always)]
    fn from(val: u8) -> Sm2octrlPwmxfs {
        Sm2octrlPwmxfs::from_bits(val)
    }
}
impl From<Sm2octrlPwmxfs> for u8 {
    #[inline(always)]
    fn from(val: Sm2octrlPwmxfs) -> u8 {
        Sm2octrlPwmxfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2out23 {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 2 instead of PWM23."]
    LOGIC_0 = 0x0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM23."]
    LOGIC_1 = 0x01,
}
impl Sm2out23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2out23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2out23 {
    #[inline(always)]
    fn from(val: u8) -> Sm2out23 {
        Sm2out23::from_bits(val)
    }
}
impl From<Sm2out23> for u8 {
    #[inline(always)]
    fn from(val: Sm2out23) -> u8 {
        Sm2out23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2out45 {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 2 instead of PWM45."]
    LOGIC_0 = 0x0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM45."]
    LOGIC_1 = 0x01,
}
impl Sm2out45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2out45 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2out45 {
    #[inline(always)]
    fn from(val: u8) -> Sm2out45 {
        Sm2out45::from_bits(val)
    }
}
impl From<Sm2out45> for u8 {
    #[inline(always)]
    fn from(val: Sm2out45) -> u8 {
        Sm2out45::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2sel23 {
    #[doc = "Generated SM2PWM23 signal used by the deadtime logic."]
    SM2PWM23 = 0x0,
    #[doc = "Inverted generated SM2PWM23 signal used by the deadtime logic."]
    INVERTED_SM2PWM23 = 0x01,
    #[doc = "SWCOUT\\[SM2OUT23\\] used by the deadtime logic."]
    SM2OUT23 = 0x02,
    #[doc = "PWM2_EXTA signal used by the deadtime logic."]
    PWM2_EXTA = 0x03,
}
impl Sm2sel23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2sel23 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2sel23 {
    #[inline(always)]
    fn from(val: u8) -> Sm2sel23 {
        Sm2sel23::from_bits(val)
    }
}
impl From<Sm2sel23> for u8 {
    #[inline(always)]
    fn from(val: Sm2sel23) -> u8 {
        Sm2sel23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2sel45 {
    #[doc = "Generated SM2PWM45 signal used by the deadtime logic."]
    SM2PWM45 = 0x0,
    #[doc = "Inverted generated SM2PWM45 signal used by the deadtime logic."]
    INVERTED_SM2PWM45 = 0x01,
    #[doc = "SWCOUT\\[SM2OUT45\\] used by the deadtime logic."]
    SM2OUT45 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sm2sel45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2sel45 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2sel45 {
    #[inline(always)]
    fn from(val: u8) -> Sm2sel45 {
        Sm2sel45::from_bits(val)
    }
}
impl From<Sm2sel45> for u8 {
    #[inline(always)]
    fn from(val: Sm2sel45) -> u8 {
        Sm2sel45::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2stsCmpf {
    #[doc = "No compare event has occurred for a particular VALx value."]
    NO_EVENT = 0x0,
    #[doc = "A compare event has occurred for a particular VALx value."]
    EVENT = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
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
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Sm2stsCmpf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2stsCmpf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2stsCmpf {
    #[inline(always)]
    fn from(val: u8) -> Sm2stsCmpf {
        Sm2stsCmpf::from_bits(val)
    }
}
impl From<Sm2stsCmpf> for u8 {
    #[inline(always)]
    fn from(val: Sm2stsCmpf) -> u8 {
        Sm2stsCmpf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2tctrlOutTrigEn {
    _RESERVED_0 = 0x0,
    #[doc = "PWM_OUT_TRIG0 will set when the counter value matches the VAL0 value."]
    VAL0 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
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
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Sm2tctrlOutTrigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2tctrlOutTrigEn {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2tctrlOutTrigEn {
    #[inline(always)]
    fn from(val: u8) -> Sm2tctrlOutTrigEn {
        Sm2tctrlOutTrigEn::from_bits(val)
    }
}
impl From<Sm2tctrlOutTrigEn> for u8 {
    #[inline(always)]
    fn from(val: Sm2tctrlOutTrigEn) -> u8 {
        Sm2tctrlOutTrigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2tctrlPwaot0 {
    #[doc = "Route the PWM_OUT_TRIG0 signal to PWM_MUX_TRIG0 port."]
    PWM_OUT_TRIG0_SIGNAL = 0x0,
    #[doc = "Route the PWM_A output to the PWM_MUX_TRIG0 port."]
    PWMA_OUTPUT = 0x01,
}
impl Sm2tctrlPwaot0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2tctrlPwaot0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2tctrlPwaot0 {
    #[inline(always)]
    fn from(val: u8) -> Sm2tctrlPwaot0 {
        Sm2tctrlPwaot0::from_bits(val)
    }
}
impl From<Sm2tctrlPwaot0> for u8 {
    #[inline(always)]
    fn from(val: Sm2tctrlPwaot0) -> u8 {
        Sm2tctrlPwaot0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2tctrlPwbot1 {
    #[doc = "Route the PWM_OUT_TRIG1 signal to PWM_MUX_TRIG1 port."]
    PWM_OUT_TRIG1_SIGNAL = 0x0,
    #[doc = "Route the PWM_B output to the PWM_MUX_TRIG1 port."]
    PWMB_OUTPUT = 0x01,
}
impl Sm2tctrlPwbot1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2tctrlPwbot1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2tctrlPwbot1 {
    #[inline(always)]
    fn from(val: u8) -> Sm2tctrlPwbot1 {
        Sm2tctrlPwbot1::from_bits(val)
    }
}
impl From<Sm2tctrlPwbot1> for u8 {
    #[inline(always)]
    fn from(val: Sm2tctrlPwbot1) -> u8 {
        Sm2tctrlPwbot1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2tctrlTrgfrq {
    #[doc = "Trigger outputs are generated during every PWM period even if the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
    EVERYPWM = 0x0,
    #[doc = "Trigger outputs are generated only during the final PWM period prior to a reload opportunity when the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
    FINALPWM = 0x01,
}
impl Sm2tctrlTrgfrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2tctrlTrgfrq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2tctrlTrgfrq {
    #[inline(always)]
    fn from(val: u8) -> Sm2tctrlTrgfrq {
        Sm2tctrlTrgfrq::from_bits(val)
    }
}
impl From<Sm2tctrlTrgfrq> for u8 {
    #[inline(always)]
    fn from(val: Sm2tctrlTrgfrq) -> u8 {
        Sm2tctrlTrgfrq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3captctrlxEdgx0 {
    #[doc = "Disabled."]
    DISABLED = 0x0,
    #[doc = "Capture falling edges."]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges."]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge."]
    ANY_EDGE = 0x03,
}
impl Sm3captctrlxEdgx0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3captctrlxEdgx0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3captctrlxEdgx0 {
    #[inline(always)]
    fn from(val: u8) -> Sm3captctrlxEdgx0 {
        Sm3captctrlxEdgx0::from_bits(val)
    }
}
impl From<Sm3captctrlxEdgx0> for u8 {
    #[inline(always)]
    fn from(val: Sm3captctrlxEdgx0) -> u8 {
        Sm3captctrlxEdgx0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3captctrlxEdgx1 {
    #[doc = "Disabled."]
    DISABLED = 0x0,
    #[doc = "Capture falling edges."]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges."]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge."]
    ANY_EDGE = 0x03,
}
impl Sm3captctrlxEdgx1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3captctrlxEdgx1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3captctrlxEdgx1 {
    #[inline(always)]
    fn from(val: u8) -> Sm3captctrlxEdgx1 {
        Sm3captctrlxEdgx1::from_bits(val)
    }
}
impl From<Sm3captctrlxEdgx1> for u8 {
    #[inline(always)]
    fn from(val: Sm3captctrlxEdgx1) -> u8 {
        Sm3captctrlxEdgx1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3captctrlxInpSelx {
    #[doc = "Raw PWM_X input signal selected as source."]
    PWM_X = 0x0,
    #[doc = "Edge Counter."]
    EDGE_COUNTER = 0x01,
}
impl Sm3captctrlxInpSelx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3captctrlxInpSelx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3captctrlxInpSelx {
    #[inline(always)]
    fn from(val: u8) -> Sm3captctrlxInpSelx {
        Sm3captctrlxInpSelx::from_bits(val)
    }
}
impl From<Sm3captctrlxInpSelx> for u8 {
    #[inline(always)]
    fn from(val: Sm3captctrlxInpSelx) -> u8 {
        Sm3captctrlxInpSelx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3captctrlxOneshotx {
    #[doc = "Free Running."]
    FREE_RUNNING = 0x0,
    #[doc = "One Shot."]
    ONE_SHOT = 0x01,
}
impl Sm3captctrlxOneshotx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3captctrlxOneshotx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3captctrlxOneshotx {
    #[inline(always)]
    fn from(val: u8) -> Sm3captctrlxOneshotx {
        Sm3captctrlxOneshotx::from_bits(val)
    }
}
impl From<Sm3captctrlxOneshotx> for u8 {
    #[inline(always)]
    fn from(val: Sm3captctrlxOneshotx) -> u8 {
        Sm3captctrlxOneshotx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3ctrl2ClkSel {
    #[doc = "The IPBus clock is used as the clock for the local prescaler and counter."]
    IPBUS = 0x0,
    #[doc = "EXT_CLK is used as the clock for the local prescaler and counter."]
    EXT_CLK = 0x01,
    #[doc = "Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it forces the clock to logic 0."]
    AUX_CLK = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sm3ctrl2ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3ctrl2ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3ctrl2ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Sm3ctrl2ClkSel {
        Sm3ctrl2ClkSel::from_bits(val)
    }
}
impl From<Sm3ctrl2ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Sm3ctrl2ClkSel) -> u8 {
        Sm3ctrl2ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3ctrl2ForceSel {
    #[doc = "The local force signal, CTRL2\\[FORCE\\], from this submodule is used to force updates."]
    LOCAL = 0x0,
    #[doc = "The master force signal from submodule 0 is used to force updates. This setting should not be used in submodule 0 as it holds the FORCE OUTPUT signal to logic 0."]
    MASTER = 0x01,
    #[doc = "The local reload signal from this submodule is used to force updates without regard to the state of LDOK."]
    LOCAL_RELOAD = 0x02,
    #[doc = "The master reload signal from submodule0 is used to force updates if LDOK is set. This setting should not be used in submodule0 as it holds the FORCE OUTPUT signal to logic 0."]
    MASTER_RELOAD = 0x03,
    #[doc = "The local sync signal from this submodule is used to force updates."]
    LOCAL_SYNC = 0x04,
    #[doc = "The master sync signal from submodule0 is used to force updates. This setting should not be used in submodule0 as it holds the FORCE OUTPUT signal to logic 0."]
    MASTER_SYNC = 0x05,
    #[doc = "The external force signal, EXT_FORCE, from outside the PWM module causes updates."]
    EXT_FORCE = 0x06,
    #[doc = "The external sync signal, EXT_SYNC, from outside the PWM module causes updates."]
    EXT_SYNC = 0x07,
}
impl Sm3ctrl2ForceSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3ctrl2ForceSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3ctrl2ForceSel {
    #[inline(always)]
    fn from(val: u8) -> Sm3ctrl2ForceSel {
        Sm3ctrl2ForceSel::from_bits(val)
    }
}
impl From<Sm3ctrl2ForceSel> for u8 {
    #[inline(always)]
    fn from(val: Sm3ctrl2ForceSel) -> u8 {
        Sm3ctrl2ForceSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3ctrl2Indep {
    #[doc = "PWM_A and PWM_B form a complementary PWM pair."]
    COMPLEMENTARY = 0x0,
    #[doc = "PWM_A and PWM_B outputs are independent PWMs."]
    INDEPENDENT = 0x01,
}
impl Sm3ctrl2Indep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3ctrl2Indep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3ctrl2Indep {
    #[inline(always)]
    fn from(val: u8) -> Sm3ctrl2Indep {
        Sm3ctrl2Indep::from_bits(val)
    }
}
impl From<Sm3ctrl2Indep> for u8 {
    #[inline(always)]
    fn from(val: Sm3ctrl2Indep) -> u8 {
        Sm3ctrl2Indep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3ctrl2InitSel {
    #[doc = "Local sync (PWM_X) causes initialization."]
    PWM_X = 0x0,
    #[doc = "Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0. The submodule counter will only re-initialize when a master reload occurs."]
    MASTER_RELOAD = 0x01,
    #[doc = "Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0."]
    MASTER_SYNC = 0x02,
    #[doc = "EXT_SYNC causes initialization."]
    EXT_SYNC = 0x03,
}
impl Sm3ctrl2InitSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3ctrl2InitSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3ctrl2InitSel {
    #[inline(always)]
    fn from(val: u8) -> Sm3ctrl2InitSel {
        Sm3ctrl2InitSel::from_bits(val)
    }
}
impl From<Sm3ctrl2InitSel> for u8 {
    #[inline(always)]
    fn from(val: Sm3ctrl2InitSel) -> u8 {
        Sm3ctrl2InitSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3ctrl2ReloadSel {
    #[doc = "The local RELOAD signal is used to reload registers."]
    LOCAL = 0x0,
    #[doc = "The master RELOAD signal (from submodule 0) is used to reload registers. This setting should not be used in submodule 0 as it forces the RELOAD signal to logic 0."]
    MASTER = 0x01,
}
impl Sm3ctrl2ReloadSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3ctrl2ReloadSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3ctrl2ReloadSel {
    #[inline(always)]
    fn from(val: u8) -> Sm3ctrl2ReloadSel {
        Sm3ctrl2ReloadSel::from_bits(val)
    }
}
impl From<Sm3ctrl2ReloadSel> for u8 {
    #[inline(always)]
    fn from(val: Sm3ctrl2ReloadSel) -> u8 {
        Sm3ctrl2ReloadSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3ctrlCompmode {
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to\" method. This means that PWM edges are only produced when the counter is equal to one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period maintains this state until a match with VAL3 clears the output in the following period."]
    EQUAL_TO = 0x0,
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to or greater than\" method. This means that PWM edges are produced when the counter is equal to or greater than one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period could go low at the start of the next period if the starting counter value is greater than (but not necessarily equal to) the new VAL3 value."]
    EQUAL_TO_OR_GREATER_THAN = 0x01,
}
impl Sm3ctrlCompmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3ctrlCompmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3ctrlCompmode {
    #[inline(always)]
    fn from(val: u8) -> Sm3ctrlCompmode {
        Sm3ctrlCompmode::from_bits(val)
    }
}
impl From<Sm3ctrlCompmode> for u8 {
    #[inline(always)]
    fn from(val: Sm3ctrlCompmode) -> u8 {
        Sm3ctrlCompmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3ctrlLdfq {
    #[doc = "Every PWM opportunity."]
    EVERYPWM = 0x0,
    #[doc = "Every 2 PWM opportunities."]
    EVERY2PWM = 0x01,
    #[doc = "Every 3 PWM opportunities."]
    EVERY3PWM = 0x02,
    #[doc = "Every 4 PWM opportunities."]
    EVERY4PWM = 0x03,
    #[doc = "Every 5 PWM opportunities."]
    EVERY5PWM = 0x04,
    #[doc = "Every 6 PWM opportunities."]
    EVERY6PWM = 0x05,
    #[doc = "Every 7 PWM opportunities."]
    EVERY7PWM = 0x06,
    #[doc = "Every 8 PWM opportunities."]
    EVERY8PWM = 0x07,
    #[doc = "Every 9 PWM opportunities."]
    EVERY9PWM = 0x08,
    #[doc = "Every 10 PWM opportunities."]
    EVERY10PWM = 0x09,
    #[doc = "Every 11 PWM opportunities."]
    EVERY11PWM = 0x0a,
    #[doc = "Every 12 PWM opportunities."]
    EVERY12PWM = 0x0b,
    #[doc = "Every 13 PWM opportunities."]
    EVERY13PWM = 0x0c,
    #[doc = "Every 14 PWM opportunities."]
    EVERY14PWM = 0x0d,
    #[doc = "Every 15 PWM opportunities."]
    EVERY15PWM = 0x0e,
    #[doc = "Every 16 PWM opportunities."]
    EVERY16PWM = 0x0f,
}
impl Sm3ctrlLdfq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3ctrlLdfq {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3ctrlLdfq {
    #[inline(always)]
    fn from(val: u8) -> Sm3ctrlLdfq {
        Sm3ctrlLdfq::from_bits(val)
    }
}
impl From<Sm3ctrlLdfq> for u8 {
    #[inline(always)]
    fn from(val: Sm3ctrlLdfq) -> u8 {
        Sm3ctrlLdfq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3ctrlLdmod {
    #[doc = "Buffered registers of this submodule are loaded and take effect at the next PWM reload if MCTRL\\[LDOK\\] is set."]
    NEXT_PWM_RELOAD = 0x0,
    #[doc = "Buffered registers of this submodule are loaded and take effect immediately upon MCTRL\\[LDOK\\] being set. In this case, it is not necessary to set CTRL\\[FULL\\] or CTRL\\[HALF\\]."]
    MTCTRL_LDOK_SET = 0x01,
}
impl Sm3ctrlLdmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3ctrlLdmod {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3ctrlLdmod {
    #[inline(always)]
    fn from(val: u8) -> Sm3ctrlLdmod {
        Sm3ctrlLdmod::from_bits(val)
    }
}
impl From<Sm3ctrlLdmod> for u8 {
    #[inline(always)]
    fn from(val: Sm3ctrlLdmod) -> u8 {
        Sm3ctrlLdmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3ctrlPrsc {
    #[doc = "Prescaler 1."]
    ONE = 0x0,
    #[doc = "Prescaler 2."]
    TWO = 0x01,
    #[doc = "Prescaler 4."]
    FOUR = 0x02,
    #[doc = "Prescaler 8."]
    EIGHT = 0x03,
    #[doc = "Prescaler 16."]
    SIXTEEN = 0x04,
    #[doc = "Prescaler 32."]
    THIRTYTWO = 0x05,
    #[doc = "Prescaler 64."]
    SIXTYFOUR = 0x06,
    #[doc = "Prescaler 128."]
    HUNDREDTWENTYEIGHT = 0x07,
}
impl Sm3ctrlPrsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3ctrlPrsc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3ctrlPrsc {
    #[inline(always)]
    fn from(val: u8) -> Sm3ctrlPrsc {
        Sm3ctrlPrsc::from_bits(val)
    }
}
impl From<Sm3ctrlPrsc> for u8 {
    #[inline(always)]
    fn from(val: Sm3ctrlPrsc) -> u8 {
        Sm3ctrlPrsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3dmaenCaptde {
    #[doc = "Read DMA requests disabled."]
    DISABLED = 0x0,
    #[doc = "Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN\\[CA1DE\\], DMAEN\\[CA0DE\\], DMAEN\\[CB1DE\\], DMAEN\\[CB0DE\\], DMAEN\\[CX1DE\\], or DMAEN\\[CX0DE\\] to be set to determine which watermark(s) the DMA request is sensitive."]
    EXCEEDFIFO = 0x01,
    #[doc = "A local synchronization (VAL1 matches counter) sets the read DMA request."]
    LOCAL_SYNC = 0x02,
    #[doc = "A local reload (STS\\[RF\\] being set) sets the read DMA request."]
    LOCAL_RELOAD = 0x03,
}
impl Sm3dmaenCaptde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3dmaenCaptde {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3dmaenCaptde {
    #[inline(always)]
    fn from(val: u8) -> Sm3dmaenCaptde {
        Sm3dmaenCaptde::from_bits(val)
    }
}
impl From<Sm3dmaenCaptde> for u8 {
    #[inline(always)]
    fn from(val: Sm3dmaenCaptde) -> u8 {
        Sm3dmaenCaptde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3dmaenFand {
    #[doc = "Selected FIFO watermarks are OR'ed together."]
    OR = 0x0,
    #[doc = "Selected FIFO watermarks are AND'ed together."]
    AND = 0x01,
}
impl Sm3dmaenFand {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3dmaenFand {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3dmaenFand {
    #[inline(always)]
    fn from(val: u8) -> Sm3dmaenFand {
        Sm3dmaenFand::from_bits(val)
    }
}
impl From<Sm3dmaenFand> for u8 {
    #[inline(always)]
    fn from(val: Sm3dmaenFand) -> u8 {
        Sm3dmaenFand::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3intenCmpie {
    #[doc = "The corresponding STS\\[CMPF\\] bit will not cause an interrupt request."]
    DISABLED = 0x0,
    #[doc = "The corresponding STS\\[CMPF\\] bit will cause an interrupt request."]
    ENABLED = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
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
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Sm3intenCmpie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3intenCmpie {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3intenCmpie {
    #[inline(always)]
    fn from(val: u8) -> Sm3intenCmpie {
        Sm3intenCmpie::from_bits(val)
    }
}
impl From<Sm3intenCmpie> for u8 {
    #[inline(always)]
    fn from(val: Sm3intenCmpie) -> u8 {
        Sm3intenCmpie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3octrlPwmafs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl Sm3octrlPwmafs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3octrlPwmafs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3octrlPwmafs {
    #[inline(always)]
    fn from(val: u8) -> Sm3octrlPwmafs {
        Sm3octrlPwmafs::from_bits(val)
    }
}
impl From<Sm3octrlPwmafs> for u8 {
    #[inline(always)]
    fn from(val: Sm3octrlPwmafs) -> u8 {
        Sm3octrlPwmafs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3octrlPwmbfs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl Sm3octrlPwmbfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3octrlPwmbfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3octrlPwmbfs {
    #[inline(always)]
    fn from(val: u8) -> Sm3octrlPwmbfs {
        Sm3octrlPwmbfs::from_bits(val)
    }
}
impl From<Sm3octrlPwmbfs> for u8 {
    #[inline(always)]
    fn from(val: Sm3octrlPwmbfs) -> u8 {
        Sm3octrlPwmbfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3octrlPwmxfs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl Sm3octrlPwmxfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3octrlPwmxfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3octrlPwmxfs {
    #[inline(always)]
    fn from(val: u8) -> Sm3octrlPwmxfs {
        Sm3octrlPwmxfs::from_bits(val)
    }
}
impl From<Sm3octrlPwmxfs> for u8 {
    #[inline(always)]
    fn from(val: Sm3octrlPwmxfs) -> u8 {
        Sm3octrlPwmxfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3out23 {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 3 instead of PWM23."]
    LOGIC_0 = 0x0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM23."]
    LOGIC_1 = 0x01,
}
impl Sm3out23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3out23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3out23 {
    #[inline(always)]
    fn from(val: u8) -> Sm3out23 {
        Sm3out23::from_bits(val)
    }
}
impl From<Sm3out23> for u8 {
    #[inline(always)]
    fn from(val: Sm3out23) -> u8 {
        Sm3out23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3out45 {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 3 instead of PWM45."]
    LOGIC_0 = 0x0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM45."]
    LOGIC_1 = 0x01,
}
impl Sm3out45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3out45 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3out45 {
    #[inline(always)]
    fn from(val: u8) -> Sm3out45 {
        Sm3out45::from_bits(val)
    }
}
impl From<Sm3out45> for u8 {
    #[inline(always)]
    fn from(val: Sm3out45) -> u8 {
        Sm3out45::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3sel23 {
    #[doc = "Generated SM3PWM23 signal used by the deadtime logic."]
    SM3PWM23 = 0x0,
    #[doc = "Inverted generated SM3PWM23 signal used by the deadtime logic."]
    INVERTED_SM3PWM23 = 0x01,
    #[doc = "SWCOUT\\[SM3OUT23\\] used by the deadtime logic."]
    SM3OUT23 = 0x02,
    #[doc = "PWM3_EXTA signal used by the deadtime logic."]
    PWM3_EXTA = 0x03,
}
impl Sm3sel23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3sel23 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3sel23 {
    #[inline(always)]
    fn from(val: u8) -> Sm3sel23 {
        Sm3sel23::from_bits(val)
    }
}
impl From<Sm3sel23> for u8 {
    #[inline(always)]
    fn from(val: Sm3sel23) -> u8 {
        Sm3sel23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3sel45 {
    #[doc = "Generated SM3PWM45 signal used by the deadtime logic."]
    SM3PWM45 = 0x0,
    #[doc = "Inverted generated SM3PWM45 signal used by the deadtime logic."]
    INVERTED_SM3PWM45 = 0x01,
    #[doc = "SWCOUT\\[SM3OUT45\\] used by the deadtime logic."]
    SM3OUT45 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sm3sel45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3sel45 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3sel45 {
    #[inline(always)]
    fn from(val: u8) -> Sm3sel45 {
        Sm3sel45::from_bits(val)
    }
}
impl From<Sm3sel45> for u8 {
    #[inline(always)]
    fn from(val: Sm3sel45) -> u8 {
        Sm3sel45::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3stsCmpf {
    #[doc = "No compare event has occurred for a particular VALx value."]
    NO_EVENT = 0x0,
    #[doc = "A compare event has occurred for a particular VALx value."]
    EVENT = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
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
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Sm3stsCmpf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3stsCmpf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3stsCmpf {
    #[inline(always)]
    fn from(val: u8) -> Sm3stsCmpf {
        Sm3stsCmpf::from_bits(val)
    }
}
impl From<Sm3stsCmpf> for u8 {
    #[inline(always)]
    fn from(val: Sm3stsCmpf) -> u8 {
        Sm3stsCmpf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3tctrlOutTrigEn {
    _RESERVED_0 = 0x0,
    #[doc = "PWM_OUT_TRIG0 will set when the counter value matches the VAL0 value."]
    VAL0 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
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
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Sm3tctrlOutTrigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3tctrlOutTrigEn {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3tctrlOutTrigEn {
    #[inline(always)]
    fn from(val: u8) -> Sm3tctrlOutTrigEn {
        Sm3tctrlOutTrigEn::from_bits(val)
    }
}
impl From<Sm3tctrlOutTrigEn> for u8 {
    #[inline(always)]
    fn from(val: Sm3tctrlOutTrigEn) -> u8 {
        Sm3tctrlOutTrigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3tctrlPwaot0 {
    #[doc = "Route the PWM_OUT_TRIG0 signal to PWM_MUX_TRIG0 port."]
    PWM_OUT_TRIG0_SIGNAL = 0x0,
    #[doc = "Route the PWM_A output to the PWM_MUX_TRIG0 port."]
    PWMA_OUTPUT = 0x01,
}
impl Sm3tctrlPwaot0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3tctrlPwaot0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3tctrlPwaot0 {
    #[inline(always)]
    fn from(val: u8) -> Sm3tctrlPwaot0 {
        Sm3tctrlPwaot0::from_bits(val)
    }
}
impl From<Sm3tctrlPwaot0> for u8 {
    #[inline(always)]
    fn from(val: Sm3tctrlPwaot0) -> u8 {
        Sm3tctrlPwaot0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3tctrlPwbot1 {
    #[doc = "Route the PWM_OUT_TRIG1 signal to PWM_MUX_TRIG1 port."]
    PWM_OUT_TRIG1_SIGNAL = 0x0,
    #[doc = "Route the PWM_B output to the PWM_MUX_TRIG1 port."]
    PWMB_OUTPUT = 0x01,
}
impl Sm3tctrlPwbot1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3tctrlPwbot1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3tctrlPwbot1 {
    #[inline(always)]
    fn from(val: u8) -> Sm3tctrlPwbot1 {
        Sm3tctrlPwbot1::from_bits(val)
    }
}
impl From<Sm3tctrlPwbot1> for u8 {
    #[inline(always)]
    fn from(val: Sm3tctrlPwbot1) -> u8 {
        Sm3tctrlPwbot1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3tctrlTrgfrq {
    #[doc = "Trigger outputs are generated during every PWM period even if the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
    EVERYPWM = 0x0,
    #[doc = "Trigger outputs are generated only during the final PWM period prior to a reload opportunity when the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
    FINALPWM = 0x01,
}
impl Sm3tctrlTrgfrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3tctrlTrgfrq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3tctrlTrgfrq {
    #[inline(always)]
    fn from(val: u8) -> Sm3tctrlTrgfrq {
        Sm3tctrlTrgfrq::from_bits(val)
    }
}
impl From<Sm3tctrlTrgfrq> for u8 {
    #[inline(always)]
    fn from(val: Sm3tctrlTrgfrq) -> u8 {
        Sm3tctrlTrgfrq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StretchCntPrsc {
    #[doc = "Stretch count is zero, no stretch."]
    DISABLED = 0x0,
    #[doc = "Stretch mux0_trig/mux1_trig/out0_trig/out1_trig/pwma_trig/pwmb_trig for 2 IPBus clock period."]
    ENABLED = 0x01,
    #[doc = "Stretch mux0_trig/mux1_trig/out0_trig/out1_trig/pwma_trig/pwmb_trig for 4 IPBus clock period."]
    DISABLED_LOCKED = 0x02,
    #[doc = "Stretch mux0_trig/mux1_trig/out0_trig/out1_trig/pwma_trig/pwmb_trig for 8 IPBus clock period."]
    ENABLED_LOCKED = 0x03,
}
impl StretchCntPrsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StretchCntPrsc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StretchCntPrsc {
    #[inline(always)]
    fn from(val: u8) -> StretchCntPrsc {
        StretchCntPrsc::from_bits(val)
    }
}
impl From<StretchCntPrsc> for u8 {
    #[inline(always)]
    fn from(val: StretchCntPrsc) -> u8 {
        StretchCntPrsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wrprot {
    #[doc = "Write protection off (default)."]
    DISABLED = 0x0,
    #[doc = "Write protection on."]
    ENABLED = 0x01,
    #[doc = "Write protection off and locked until chip reset."]
    DISABLED_LOCKED = 0x02,
    #[doc = "Write protection on and locked until chip reset."]
    ENABLED_LOCKED = 0x03,
}
impl Wrprot {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wrprot {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wrprot {
    #[inline(always)]
    fn from(val: u8) -> Wrprot {
        Wrprot::from_bits(val)
    }
}
impl From<Wrprot> for u8 {
    #[inline(always)]
    fn from(val: Wrprot) -> u8 {
        Wrprot::to_bits(val)
    }
}
