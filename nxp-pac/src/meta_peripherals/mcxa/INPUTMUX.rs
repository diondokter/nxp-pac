#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (2fd28c5 2026-04-02))"]
#[doc = "INPUTMUX."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inputmux {
    ptr: *mut u8,
}
unsafe impl Send for Inputmux {}
unsafe impl Sync for Inputmux {}
impl Inputmux {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Capture select register for CTIMER inputs."]
    #[inline(always)]
    pub const fn ctimer0cap(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ctimer0cap, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _)
        }
    }
    #[doc = "Trigger register for TIMER0."]
    #[inline(always)]
    pub const fn timer0trig(self) -> crate::pac::common::Reg<Timer0trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Capture select register for CTIMER inputs."]
    #[inline(always)]
    pub const fn ctimer1cap(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ctimer1cap, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _)
        }
    }
    #[doc = "Trigger register for TIMER1."]
    #[inline(always)]
    pub const fn timer1trig(self) -> crate::pac::common::Reg<Timer1trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Capture select register for CTIMER inputs."]
    #[inline(always)]
    pub const fn ctimer2cap(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ctimer2cap, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 4usize) as _)
        }
    }
    #[doc = "Trigger register for TIMER2 inputs."]
    #[inline(always)]
    pub const fn timer2trig(self) -> crate::pac::common::Reg<Timer2trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "SmartDMA Trigger Input Connections."]
    #[inline(always)]
    pub const fn smart_dma_trig(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<SmartDmaTrig, crate::pac::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _)
        }
    }
    #[doc = "Selection for frequency measurement reference clock."]
    #[inline(always)]
    pub const fn freqmeas_ref(
        self,
    ) -> crate::pac::common::Reg<FreqmeasRef, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Selection for frequency measurement reference clock."]
    #[inline(always)]
    pub const fn freqmeas_tar(
        self,
    ) -> crate::pac::common::Reg<FreqmeasTar, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Capture select register for CTIMER inputs."]
    #[inline(always)]
    pub const fn ctimer3cap(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ctimer3cap, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize + n * 4usize) as _)
        }
    }
    #[doc = "Trigger register for TIMER3."]
    #[inline(always)]
    pub const fn timer3trig(self) -> crate::pac::common::Reg<Timer3trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Capture select register for CTIMER inputs."]
    #[inline(always)]
    pub const fn ctimer4cap(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ctimer4cap, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize + n * 4usize) as _)
        }
    }
    #[doc = "Trigger register for TIMER4."]
    #[inline(always)]
    pub const fn timer4trig(self) -> crate::pac::common::Reg<Timer4trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "AOI1 trigger input connections 0."]
    #[inline(always)]
    pub const fn aoi1_input(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<AoiInput, crate::pac::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "CMP0 input connections."]
    #[inline(always)]
    pub const fn cmp0_trig(self) -> crate::pac::common::Reg<CmpTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "ADC Trigger input connections."]
    #[inline(always)]
    pub const fn adc0_trig(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<AdcTrig, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize + n * 4usize) as _)
        }
    }
    #[doc = "ADC Trigger input connections."]
    #[inline(always)]
    pub const fn adc2_trig(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<AdcTrig, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize + n * 4usize) as _)
        }
    }
    #[doc = "ADC Trigger input connections."]
    #[inline(always)]
    pub const fn adc1_trig(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<AdcTrig, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize + n * 4usize) as _)
        }
    }
    #[doc = "ADC Trigger input connections."]
    #[inline(always)]
    pub const fn adc3_trig(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<AdcTrig, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e0usize + n * 4usize) as _)
        }
    }
    #[doc = "DAC0 Trigger input connections."]
    #[inline(always)]
    pub const fn dac0_trig(self) -> crate::pac::common::Reg<DacTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc0_trig(self) -> crate::pac::common::Reg<QdcTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0360usize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc0_home(self) -> crate::pac::common::Reg<Qdc0home, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0364usize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc0_index(self) -> crate::pac::common::Reg<Qdc0index, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0368usize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc0_phaseb(self) -> crate::pac::common::Reg<Qdc0phaseb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x036cusize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc0_phasea(self) -> crate::pac::common::Reg<Qdc0phasea, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0370usize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc0_icap1(self) -> crate::pac::common::Reg<Qdc0icap1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0374usize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc0_icap2(self) -> crate::pac::common::Reg<Qdc0icap2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0378usize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc0_icap3(self) -> crate::pac::common::Reg<Qdc0icap3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x037cusize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc1_trig(self) -> crate::pac::common::Reg<QdcTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc1_home(self) -> crate::pac::common::Reg<Qdc1home, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0384usize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc1_index(self) -> crate::pac::common::Reg<Qdc1index, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0388usize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc1_phaseb(self) -> crate::pac::common::Reg<Qdc1phaseb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x038cusize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc1_phasea(self) -> crate::pac::common::Reg<Qdc1phasea, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0390usize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc1_icap1(self) -> crate::pac::common::Reg<Qdc1icap1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0394usize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc1_icap2(self) -> crate::pac::common::Reg<Qdc1icap2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0398usize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc1_icap3(self) -> crate::pac::common::Reg<Qdc1icap3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x039cusize) as _) }
    }
    #[doc = "PWM0 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm0_sm0_exta0(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a0usize) as _) }
    }
    #[doc = "PWM0 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm0_sm0_extsync(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a4usize) as _) }
    }
    #[doc = "PWM0 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm0_sm1_exta(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a8usize) as _) }
    }
    #[doc = "PWM0 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm0_sm1_extsync(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03acusize) as _) }
    }
    #[doc = "PWM0 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm0_sm2_exta(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b0usize) as _) }
    }
    #[doc = "PWM0 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm0_sm2_extsync(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b4usize) as _) }
    }
    #[doc = "PWM0 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm0_sm3_exta0(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b8usize) as _) }
    }
    #[doc = "PWM0 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm0_sm3_extsync(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03bcusize) as _) }
    }
    #[doc = "PWM0 Fault Input Trigger Connections."]
    #[inline(always)]
    pub const fn flex_pwm0_fault(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c0usize + n * 4usize) as _)
        }
    }
    #[doc = "PWM0 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm0_force(self) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d0usize) as _) }
    }
    #[doc = "PWM1 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm1_sm0_exta0(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e0usize) as _) }
    }
    #[doc = "PWM1 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm1_sm0_extsync(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e4usize) as _) }
    }
    #[doc = "PWM1 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm1_sm1_exta(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e8usize) as _) }
    }
    #[doc = "PWM1 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm1_sm1_extsync(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ecusize) as _) }
    }
    #[doc = "PWM1 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm1_sm2_exta(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize) as _) }
    }
    #[doc = "PWM1 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm1_sm2_extsync(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[doc = "PWM1 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm1_sm3_exta0(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f8usize) as _) }
    }
    #[doc = "PWM1 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm1_sm3_extsync(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
    #[doc = "PWM1 Fault Input Trigger Connections."]
    #[inline(always)]
    pub const fn flex_pwm1_fault(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _)
        }
    }
    #[doc = "PWM1 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm1_force(self) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "PWM0 external clock trigger."]
    #[inline(always)]
    pub const fn pwm0_ext_clk(self) -> crate::pac::common::Reg<Pwm0extClk, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "PWM1 external clock trigger."]
    #[inline(always)]
    pub const fn pwm1_ext_clk(self) -> crate::pac::common::Reg<Pwm1extClk, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0424usize) as _) }
    }
    #[doc = "AOI0 trigger input connections 0."]
    #[inline(always)]
    pub const fn aoi0_input(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<AoiInput, crate::pac::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize + n * 4usize) as _)
        }
    }
    #[doc = "USB-FS trigger input connections."]
    #[inline(always)]
    pub const fn usbfs_trig(self) -> crate::pac::common::Reg<UsbfsTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0480usize) as _) }
    }
    #[doc = "EXT trigger connections."]
    #[inline(always)]
    pub const fn ext_trig(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<ExtTrig, crate::pac::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c0usize + n * 4usize) as _)
        }
    }
    #[doc = "CMP1 input connections."]
    #[inline(always)]
    pub const fn cmp1_trig(self) -> crate::pac::common::Reg<CmpTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e0usize) as _) }
    }
    #[doc = "CMP2 input connections."]
    #[inline(always)]
    pub const fn cmp2_trig(self) -> crate::pac::common::Reg<CmpTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "LPI2C2 trigger input connections."]
    #[inline(always)]
    pub const fn lpi2c2_trig(self) -> crate::pac::common::Reg<Lpi2cTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
    }
    #[doc = "LPI2C3 trigger input connections."]
    #[inline(always)]
    pub const fn lpi2c3_trig(self) -> crate::pac::common::Reg<Lpi2cTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
    }
    #[doc = "LPI2C0 trigger input connections."]
    #[inline(always)]
    pub const fn lpi2c0_trig(self) -> crate::pac::common::Reg<Lpi2cTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a0usize) as _) }
    }
    #[doc = "LPI2C1 trigger input connections."]
    #[inline(always)]
    pub const fn lpi2c1_trig(self) -> crate::pac::common::Reg<Lpi2cTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "LPSPI0 trigger input connections."]
    #[inline(always)]
    pub const fn lpspi0_trig(self) -> crate::pac::common::Reg<LpspiTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e0usize) as _) }
    }
    #[doc = "LPSPI1 trigger input connections."]
    #[inline(always)]
    pub const fn lpspi1_trig(self) -> crate::pac::common::Reg<LpspiTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "LPUART0 trigger input connections."]
    #[inline(always)]
    pub const fn lpuart(self, n: usize) -> crate::pac::common::Reg<Lpuart, crate::pac::common::RW> {
        assert!(n < 6usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize + n * 32usize) as _)
        }
    }
    #[doc = "FlexIO Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexio_trig(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<FlexioTrig, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x06e0usize + n * 4usize) as _)
        }
    }
    #[doc = "Trigger filter prescaller."]
    #[inline(always)]
    pub const fn trigfil_prsc(
        self,
    ) -> crate::pac::common::Reg<TrigfilPrsc, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a00usize) as _) }
    }
    #[doc = "Trigger filter stat."]
    #[inline(always)]
    pub const fn trigfil_stat0(
        self,
    ) -> crate::pac::common::Reg<TrigfilStat, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a04usize) as _) }
    }
    #[doc = "TRIGFIL control."]
    #[inline(always)]
    pub const fn trigfil(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Trigfil, crate::pac::common::RW> {
        assert!(n < 12usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a10usize + n * 4usize) as _)
        }
    }
}
#[doc = "ADC Trigger input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcTrig(pub u32);
impl AdcTrig {
    #[doc = "ADC0 trigger inputs."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> AdcTrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        AdcTrigTrigin::from_bits(val as u8)
    }
    #[doc = "ADC0 trigger inputs."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: AdcTrigTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for AdcTrig {
    #[inline(always)]
    fn default() -> AdcTrig {
        AdcTrig(0)
    }
}
impl core::fmt::Debug for AdcTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcTrig")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AdcTrig {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "AOI0 trigger input connections 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AoiInput(pub u32);
impl AoiInput {
    #[doc = "AOI0 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> AoiInputInp {
        let val = (self.0 >> 0usize) & 0x7f;
        AoiInputInp::from_bits(val as u8)
    }
    #[doc = "AOI0 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: AoiInputInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for AoiInput {
    #[inline(always)]
    fn default() -> AoiInput {
        AoiInput(0)
    }
}
impl core::fmt::Debug for AoiInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AoiInput")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AoiInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AoiInput {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "CMP0 input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmpTrig(pub u32);
impl CmpTrig {
    #[doc = "CMP0 input trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> CmpTrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        CmpTrigTrigin::from_bits(val as u8)
    }
    #[doc = "CMP0 input trigger."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: CmpTrigTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for CmpTrig {
    #[inline(always)]
    fn default() -> CmpTrig {
        CmpTrig(0)
    }
}
impl core::fmt::Debug for CmpTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CmpTrig")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CmpTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CmpTrig {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "Capture select register for CTIMER inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer0cap(pub u32);
impl Ctimer0cap {
    #[doc = "Input number for CTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Ctimer0capInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Ctimer0capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER0."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Ctimer0capInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer0cap {
    #[inline(always)]
    fn default() -> Ctimer0cap {
        Ctimer0cap(0)
    }
}
impl core::fmt::Debug for Ctimer0cap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer0cap")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer0cap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer0cap {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture select register for CTIMER inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer1cap(pub u32);
impl Ctimer1cap {
    #[doc = "Input number for CTIMER1."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Ctimer1capInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Ctimer1capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER1."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Ctimer1capInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer1cap {
    #[inline(always)]
    fn default() -> Ctimer1cap {
        Ctimer1cap(0)
    }
}
impl core::fmt::Debug for Ctimer1cap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer1cap")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer1cap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer1cap {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture select register for CTIMER inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer2cap(pub u32);
impl Ctimer2cap {
    #[doc = "Input number for CTIMER2."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Ctimer2capInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Ctimer2capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER2."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Ctimer2capInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer2cap {
    #[inline(always)]
    fn default() -> Ctimer2cap {
        Ctimer2cap(0)
    }
}
impl core::fmt::Debug for Ctimer2cap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer2cap")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer2cap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer2cap {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture select register for CTIMER inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer3cap(pub u32);
impl Ctimer3cap {
    #[doc = "Input number for CTIMER3."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Ctimer3capInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Ctimer3capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER3."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Ctimer3capInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer3cap {
    #[inline(always)]
    fn default() -> Ctimer3cap {
        Ctimer3cap(0)
    }
}
impl core::fmt::Debug for Ctimer3cap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer3cap")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer3cap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer3cap {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture select register for CTIMER inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer4cap(pub u32);
impl Ctimer4cap {
    #[doc = "Input number for CTIMER4."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Ctimer4capInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Ctimer4capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER4."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Ctimer4capInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer4cap {
    #[inline(always)]
    fn default() -> Ctimer4cap {
        Ctimer4cap(0)
    }
}
impl core::fmt::Debug for Ctimer4cap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer4cap")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer4cap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer4cap {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "DAC0 Trigger input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacTrig(pub u32);
impl DacTrig {
    #[doc = "DAC0 trigger input."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> DacTrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        DacTrigTrigin::from_bits(val as u8)
    }
    #[doc = "DAC0 trigger input."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: DacTrigTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for DacTrig {
    #[inline(always)]
    fn default() -> DacTrig {
        DacTrig(0)
    }
}
impl core::fmt::Debug for DacTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacTrig")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DacTrig {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "EXT trigger connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtTrig(pub u32);
impl ExtTrig {
    #[doc = "EXT trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> ExtTrigInp {
        let val = (self.0 >> 0usize) & 0x1f;
        ExtTrigInp::from_bits(val as u8)
    }
    #[doc = "EXT trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: ExtTrigInp) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for ExtTrig {
    #[inline(always)]
    fn default() -> ExtTrig {
        ExtTrig(0)
    }
}
impl core::fmt::Debug for ExtTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ExtTrig").field("inp", &self.inp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ExtTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ExtTrig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "PWM0 Fault Input Trigger Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm(pub u32);
impl FlexPwm {
    #[doc = "FAULT input connections for PWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> FlexPwmTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        FlexPwmTrigin::from_bits(val as u8)
    }
    #[doc = "FAULT input connections for PWM0."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: FlexPwmTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FlexPwm {
    #[inline(always)]
    fn default() -> FlexPwm {
        FlexPwm(0)
    }
}
impl core::fmt::Debug for FlexPwm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexPwm")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexPwm {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexPwm {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "FlexIO Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexioTrig(pub u32);
impl FlexioTrig {
    #[doc = "Input number for FlexIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> FlexioTrigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        FlexioTrigInp::from_bits(val as u8)
    }
    #[doc = "Input number for FlexIO0."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: FlexioTrigInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for FlexioTrig {
    #[inline(always)]
    fn default() -> FlexioTrig {
        FlexioTrig(0)
    }
}
impl core::fmt::Debug for FlexioTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexioTrig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexioTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexioTrig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Selection for frequency measurement reference clock."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FreqmeasRef(pub u32);
impl FreqmeasRef {
    #[doc = "Clock source number (binary value) for frequency measure function target clock."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> FreqmeasRefInp {
        let val = (self.0 >> 0usize) & 0x7f;
        FreqmeasRefInp::from_bits(val as u8)
    }
    #[doc = "Clock source number (binary value) for frequency measure function target clock."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: FreqmeasRefInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for FreqmeasRef {
    #[inline(always)]
    fn default() -> FreqmeasRef {
        FreqmeasRef(0)
    }
}
impl core::fmt::Debug for FreqmeasRef {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FreqmeasRef")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FreqmeasRef {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FreqmeasRef {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Selection for frequency measurement reference clock."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FreqmeasTar(pub u32);
impl FreqmeasTar {
    #[doc = "Clock source number (binary value) for frequency measure function target clock."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> FreqmeasTarInp {
        let val = (self.0 >> 0usize) & 0x7f;
        FreqmeasTarInp::from_bits(val as u8)
    }
    #[doc = "Clock source number (binary value) for frequency measure function target clock."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: FreqmeasTarInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for FreqmeasTar {
    #[inline(always)]
    fn default() -> FreqmeasTar {
        FreqmeasTar(0)
    }
}
impl core::fmt::Debug for FreqmeasTar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FreqmeasTar")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FreqmeasTar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FreqmeasTar {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LPI2C0 trigger input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2cTrig(pub u32);
impl Lpi2cTrig {
    #[doc = "LPI2C0 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Lpi2cTrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        Lpi2cTrigInp::from_bits(val as u8)
    }
    #[doc = "LPI2C0 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Lpi2cTrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Lpi2cTrig {
    #[inline(always)]
    fn default() -> Lpi2cTrig {
        Lpi2cTrig(0)
    }
}
impl core::fmt::Debug for Lpi2cTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2cTrig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2cTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpi2cTrig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LPSPI0 trigger input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpspiTrig(pub u32);
impl LpspiTrig {
    #[doc = "LPSPI0 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> LpspiTrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        LpspiTrigInp::from_bits(val as u8)
    }
    #[doc = "LPSPI0 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: LpspiTrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for LpspiTrig {
    #[inline(always)]
    fn default() -> LpspiTrig {
        LpspiTrig(0)
    }
}
impl core::fmt::Debug for LpspiTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpspiTrig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpspiTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LpspiTrig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LPUART0 trigger input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart(pub u32);
impl Lpuart {
    #[doc = "LPUART0 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> LpuartInp {
        let val = (self.0 >> 0usize) & 0x3f;
        LpuartInp::from_bits(val as u8)
    }
    #[doc = "LPUART0 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: LpuartInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Lpuart {
    #[inline(always)]
    fn default() -> Lpuart {
        Lpuart(0)
    }
}
impl core::fmt::Debug for Lpuart {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart").field("inp", &self.inp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "PWM0 external clock trigger."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm0extClk(pub u32);
impl Pwm0extClk {
    #[doc = "Trigger input connections for PWM."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> Pwm0extClkTrigin {
        let val = (self.0 >> 0usize) & 0x0f;
        Pwm0extClkTrigin::from_bits(val as u8)
    }
    #[doc = "Trigger input connections for PWM."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: Pwm0extClkTrigin) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Pwm0extClk {
    #[inline(always)]
    fn default() -> Pwm0extClk {
        Pwm0extClk(0)
    }
}
impl core::fmt::Debug for Pwm0extClk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwm0extClk")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwm0extClk {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pwm0extClk {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "PWM1 external clock trigger."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm1extClk(pub u32);
impl Pwm1extClk {
    #[doc = "Trigger input connections for PWM."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> Pwm1extClkTrigin {
        let val = (self.0 >> 0usize) & 0x0f;
        Pwm1extClkTrigin::from_bits(val as u8)
    }
    #[doc = "Trigger input connections for PWM."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: Pwm1extClkTrigin) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Pwm1extClk {
    #[inline(always)]
    fn default() -> Pwm1extClk {
        Pwm1extClk(0)
    }
}
impl core::fmt::Debug for Pwm1extClk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwm1extClk")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwm1extClk {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pwm1extClk {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "QDC0 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc0home(pub u32);
impl Qdc0home {
    #[doc = "QDC0 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> QdcHomeInp {
        let val = (self.0 >> 0usize) & 0x7f;
        QdcHomeInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcHomeInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc0home {
    #[inline(always)]
    fn default() -> Qdc0home {
        Qdc0home(0)
    }
}
impl core::fmt::Debug for Qdc0home {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc0home")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc0home {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc0home {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC0 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc0icap1(pub u32);
impl Qdc0icap1 {
    #[doc = "QDC0 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> QdcIcapInp {
        let val = (self.0 >> 0usize) & 0x7f;
        QdcIcapInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcIcapInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc0icap1 {
    #[inline(always)]
    fn default() -> Qdc0icap1 {
        Qdc0icap1(0)
    }
}
impl core::fmt::Debug for Qdc0icap1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc0icap1")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc0icap1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc0icap1 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC0 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc0icap2(pub u32);
impl Qdc0icap2 {
    #[doc = "QDC0 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> QdcIcapInp {
        let val = (self.0 >> 0usize) & 0x7f;
        QdcIcapInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcIcapInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc0icap2 {
    #[inline(always)]
    fn default() -> Qdc0icap2 {
        Qdc0icap2(0)
    }
}
impl core::fmt::Debug for Qdc0icap2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc0icap2")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc0icap2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc0icap2 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC0 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc0icap3(pub u32);
impl Qdc0icap3 {
    #[doc = "QDC0 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> QdcIcapInp {
        let val = (self.0 >> 0usize) & 0x7f;
        QdcIcapInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcIcapInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc0icap3 {
    #[inline(always)]
    fn default() -> Qdc0icap3 {
        Qdc0icap3(0)
    }
}
impl core::fmt::Debug for Qdc0icap3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc0icap3")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc0icap3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc0icap3 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC0 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc0index(pub u32);
impl Qdc0index {
    #[doc = "QDC0 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> QdcIndexInp {
        let val = (self.0 >> 0usize) & 0x7f;
        QdcIndexInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcIndexInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc0index {
    #[inline(always)]
    fn default() -> Qdc0index {
        Qdc0index(0)
    }
}
impl core::fmt::Debug for Qdc0index {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc0index")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc0index {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc0index {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC0 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc0phasea(pub u32);
impl Qdc0phasea {
    #[doc = "QDC0 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Qdc0phaseaInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Qdc0phaseaInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Qdc0phaseaInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc0phasea {
    #[inline(always)]
    fn default() -> Qdc0phasea {
        Qdc0phasea(0)
    }
}
impl core::fmt::Debug for Qdc0phasea {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc0phasea")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc0phasea {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc0phasea {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC0 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc0phaseb(pub u32);
impl Qdc0phaseb {
    #[doc = "QDC0 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Qdc0phasebInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Qdc0phasebInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Qdc0phasebInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc0phaseb {
    #[inline(always)]
    fn default() -> Qdc0phaseb {
        Qdc0phaseb(0)
    }
}
impl core::fmt::Debug for Qdc0phaseb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc0phaseb")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc0phaseb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc0phaseb {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC1 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc1home(pub u32);
impl Qdc1home {
    #[doc = "QDC1 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> QdcHomeInp {
        let val = (self.0 >> 0usize) & 0x7f;
        QdcHomeInp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcHomeInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc1home {
    #[inline(always)]
    fn default() -> Qdc1home {
        Qdc1home(0)
    }
}
impl core::fmt::Debug for Qdc1home {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc1home")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc1home {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc1home {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC1 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc1icap1(pub u32);
impl Qdc1icap1 {
    #[doc = "QDC1 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> QdcIcapInp {
        let val = (self.0 >> 0usize) & 0x7f;
        QdcIcapInp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcIcapInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc1icap1 {
    #[inline(always)]
    fn default() -> Qdc1icap1 {
        Qdc1icap1(0)
    }
}
impl core::fmt::Debug for Qdc1icap1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc1icap1")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc1icap1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc1icap1 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC1 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc1icap2(pub u32);
impl Qdc1icap2 {
    #[doc = "QDC1 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> QdcIcapInp {
        let val = (self.0 >> 0usize) & 0x7f;
        QdcIcapInp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcIcapInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc1icap2 {
    #[inline(always)]
    fn default() -> Qdc1icap2 {
        Qdc1icap2(0)
    }
}
impl core::fmt::Debug for Qdc1icap2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc1icap2")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc1icap2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc1icap2 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC1 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc1icap3(pub u32);
impl Qdc1icap3 {
    #[doc = "QDC1 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> QdcIcapInp {
        let val = (self.0 >> 0usize) & 0x7f;
        QdcIcapInp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcIcapInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc1icap3 {
    #[inline(always)]
    fn default() -> Qdc1icap3 {
        Qdc1icap3(0)
    }
}
impl core::fmt::Debug for Qdc1icap3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc1icap3")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc1icap3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc1icap3 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC1 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc1index(pub u32);
impl Qdc1index {
    #[doc = "QDC1 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> QdcIndexInp {
        let val = (self.0 >> 0usize) & 0x7f;
        QdcIndexInp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcIndexInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc1index {
    #[inline(always)]
    fn default() -> Qdc1index {
        Qdc1index(0)
    }
}
impl core::fmt::Debug for Qdc1index {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc1index")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc1index {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc1index {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC1 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc1phasea(pub u32);
impl Qdc1phasea {
    #[doc = "QDC0 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Qdc1phaseaInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Qdc1phaseaInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Qdc1phaseaInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc1phasea {
    #[inline(always)]
    fn default() -> Qdc1phasea {
        Qdc1phasea(0)
    }
}
impl core::fmt::Debug for Qdc1phasea {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc1phasea")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc1phasea {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc1phasea {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC1 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc1phaseb(pub u32);
impl Qdc1phaseb {
    #[doc = "QDC1 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Qdc1phasebInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Qdc1phasebInp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Qdc1phasebInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc1phaseb {
    #[inline(always)]
    fn default() -> Qdc1phaseb {
        Qdc1phaseb(0)
    }
}
impl core::fmt::Debug for Qdc1phaseb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc1phaseb")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc1phaseb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc1phaseb {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC0 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdcTrig(pub u32);
impl QdcTrig {
    #[doc = "QDC0 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> QdcTrigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        QdcTrigInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcTrigInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for QdcTrig {
    #[inline(always)]
    fn default() -> QdcTrig {
        QdcTrig(0)
    }
}
impl core::fmt::Debug for QdcTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QdcTrig").field("inp", &self.inp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for QdcTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "QdcTrig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "SmartDMA Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmartDmaTrig(pub u32);
impl SmartDmaTrig {
    #[doc = "Input number for SmartDMA."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> SmartDmaTrigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        SmartDmaTrigInp::from_bits(val as u8)
    }
    #[doc = "Input number for SmartDMA."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: SmartDmaTrigInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for SmartDmaTrig {
    #[inline(always)]
    fn default() -> SmartDmaTrig {
        SmartDmaTrig(0)
    }
}
impl core::fmt::Debug for SmartDmaTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmartDmaTrig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmartDmaTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SmartDmaTrig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Trigger register for TIMER0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer0trig(pub u32);
impl Timer0trig {
    #[doc = "Input number for CTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Timer0trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Timer0trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER0."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Timer0trigInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Timer0trig {
    #[inline(always)]
    fn default() -> Timer0trig {
        Timer0trig(0)
    }
}
impl core::fmt::Debug for Timer0trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer0trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer0trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timer0trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Trigger register for TIMER1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1trig(pub u32);
impl Timer1trig {
    #[doc = "Input number for CTIMER1."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Timer1trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Timer1trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER1."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Timer1trigInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Timer1trig {
    #[inline(always)]
    fn default() -> Timer1trig {
        Timer1trig(0)
    }
}
impl core::fmt::Debug for Timer1trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer1trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer1trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timer1trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Trigger register for TIMER2 inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2trig(pub u32);
impl Timer2trig {
    #[doc = "Input number for CTIMER2."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Timer2trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Timer2trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER2."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Timer2trigInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Timer2trig {
    #[inline(always)]
    fn default() -> Timer2trig {
        Timer2trig(0)
    }
}
impl core::fmt::Debug for Timer2trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer2trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer2trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timer2trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Trigger register for TIMER3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer3trig(pub u32);
impl Timer3trig {
    #[doc = "Input number for CTIMER3."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Timer3trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Timer3trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER3."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Timer3trigInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Timer3trig {
    #[inline(always)]
    fn default() -> Timer3trig {
        Timer3trig(0)
    }
}
impl core::fmt::Debug for Timer3trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer3trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer3trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timer3trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Trigger register for TIMER4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer4trig(pub u32);
impl Timer4trig {
    #[doc = "Input number for CTIMER4."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Timer4trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Timer4trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER4."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Timer4trigInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Timer4trig {
    #[inline(always)]
    fn default() -> Timer4trig {
        Timer4trig(0)
    }
}
impl core::fmt::Debug for Timer4trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer4trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer4trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timer4trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "TRIGFIL control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trigfil(pub u32);
impl Trigfil {
    #[doc = "Input Filter Sample Period."]
    #[must_use]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Filter Sample Period."]
    #[inline(always)]
    pub const fn set_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Input Filter Sample Count."]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Filter Sample Count."]
    #[inline(always)]
    pub const fn set_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
}
impl Default for Trigfil {
    #[inline(always)]
    fn default() -> Trigfil {
        Trigfil(0)
    }
}
impl core::fmt::Debug for Trigfil {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trigfil")
            .field("filt_per", &self.filt_per())
            .field("filt_cnt", &self.filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trigfil {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trigfil {{ filt_per: {=u8:?}, filt_cnt: {=u8:?} }}",
            self.filt_per(),
            self.filt_cnt()
        )
    }
}
#[doc = "Trigger filter prescaller."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrigfilPrsc(pub u32);
impl TrigfilPrsc {
    #[doc = "Filter Prescaller Value."]
    #[must_use]
    #[inline(always)]
    pub const fn filt_scale_val(&self) -> FiltScaleVal {
        let val = (self.0 >> 0usize) & 0x03;
        FiltScaleVal::from_bits(val as u8)
    }
    #[doc = "Filter Prescaller Value."]
    #[inline(always)]
    pub const fn set_filt_scale_val(&mut self, val: FiltScaleVal) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Enable trigger filter prescaller."]
    #[must_use]
    #[inline(always)]
    pub const fn filt_scale_en(&self) -> FiltScaleEn {
        let val = (self.0 >> 31usize) & 0x01;
        FiltScaleEn::from_bits(val as u8)
    }
    #[doc = "Enable trigger filter prescaller."]
    #[inline(always)]
    pub const fn set_filt_scale_en(&mut self, val: FiltScaleEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for TrigfilPrsc {
    #[inline(always)]
    fn default() -> TrigfilPrsc {
        TrigfilPrsc(0)
    }
}
impl core::fmt::Debug for TrigfilPrsc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrigfilPrsc")
            .field("filt_scale_val", &self.filt_scale_val())
            .field("filt_scale_en", &self.filt_scale_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrigfilPrsc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TrigfilPrsc {{ filt_scale_val: {:?}, filt_scale_en: {:?} }}",
            self.filt_scale_val(),
            self.filt_scale_en()
        )
    }
}
#[doc = "Trigger filter stat."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrigfilStat(pub u32);
impl TrigfilStat {
    #[doc = "TRIG_IN value."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_in0_val(&self) -> TrigInVal {
        let val = (self.0 >> 0usize) & 0x01;
        TrigInVal::from_bits(val as u8)
    }
    #[doc = "TRIG_IN value."]
    #[inline(always)]
    pub const fn set_trig_in0_val(&mut self, val: TrigInVal) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "TRIG_IN value."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_in1_val(&self) -> TrigInVal {
        let val = (self.0 >> 1usize) & 0x01;
        TrigInVal::from_bits(val as u8)
    }
    #[doc = "TRIG_IN value."]
    #[inline(always)]
    pub const fn set_trig_in1_val(&mut self, val: TrigInVal) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "TRIG_IN value."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_in2_val(&self) -> TrigInVal {
        let val = (self.0 >> 2usize) & 0x01;
        TrigInVal::from_bits(val as u8)
    }
    #[doc = "TRIG_IN value."]
    #[inline(always)]
    pub const fn set_trig_in2_val(&mut self, val: TrigInVal) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "TRIG_IN value."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_in3_val(&self) -> TrigInVal {
        let val = (self.0 >> 3usize) & 0x01;
        TrigInVal::from_bits(val as u8)
    }
    #[doc = "TRIG_IN value."]
    #[inline(always)]
    pub const fn set_trig_in3_val(&mut self, val: TrigInVal) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "TRIG_IN value."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_in4_val(&self) -> TrigInVal {
        let val = (self.0 >> 4usize) & 0x01;
        TrigInVal::from_bits(val as u8)
    }
    #[doc = "TRIG_IN value."]
    #[inline(always)]
    pub const fn set_trig_in4_val(&mut self, val: TrigInVal) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "TRIG_IN value."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_in5_val(&self) -> TrigInVal {
        let val = (self.0 >> 5usize) & 0x01;
        TrigInVal::from_bits(val as u8)
    }
    #[doc = "TRIG_IN value."]
    #[inline(always)]
    pub const fn set_trig_in5_val(&mut self, val: TrigInVal) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "TRIG_IN value."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_in6_val(&self) -> TrigInVal {
        let val = (self.0 >> 6usize) & 0x01;
        TrigInVal::from_bits(val as u8)
    }
    #[doc = "TRIG_IN value."]
    #[inline(always)]
    pub const fn set_trig_in6_val(&mut self, val: TrigInVal) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "TRIG_IN value."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_in7_val(&self) -> TrigInVal {
        let val = (self.0 >> 7usize) & 0x01;
        TrigInVal::from_bits(val as u8)
    }
    #[doc = "TRIG_IN value."]
    #[inline(always)]
    pub const fn set_trig_in7_val(&mut self, val: TrigInVal) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "TRIG_IN value."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_in8_val(&self) -> TrigInVal {
        let val = (self.0 >> 8usize) & 0x01;
        TrigInVal::from_bits(val as u8)
    }
    #[doc = "TRIG_IN value."]
    #[inline(always)]
    pub const fn set_trig_in8_val(&mut self, val: TrigInVal) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "TRIG_IN value."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_in9_val(&self) -> TrigInVal {
        let val = (self.0 >> 9usize) & 0x01;
        TrigInVal::from_bits(val as u8)
    }
    #[doc = "TRIG_IN value."]
    #[inline(always)]
    pub const fn set_trig_in9_val(&mut self, val: TrigInVal) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "TRIG_IN value."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_in10_val(&self) -> TrigInVal {
        let val = (self.0 >> 10usize) & 0x01;
        TrigInVal::from_bits(val as u8)
    }
    #[doc = "TRIG_IN value."]
    #[inline(always)]
    pub const fn set_trig_in10_val(&mut self, val: TrigInVal) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "TRIG_IN value."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_in11_val(&self) -> TrigInVal {
        let val = (self.0 >> 11usize) & 0x01;
        TrigInVal::from_bits(val as u8)
    }
    #[doc = "TRIG_IN value."]
    #[inline(always)]
    pub const fn set_trig_in11_val(&mut self, val: TrigInVal) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
}
impl Default for TrigfilStat {
    #[inline(always)]
    fn default() -> TrigfilStat {
        TrigfilStat(0)
    }
}
impl core::fmt::Debug for TrigfilStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrigfilStat")
            .field("trig_in0_val", &self.trig_in0_val())
            .field("trig_in1_val", &self.trig_in1_val())
            .field("trig_in2_val", &self.trig_in2_val())
            .field("trig_in3_val", &self.trig_in3_val())
            .field("trig_in4_val", &self.trig_in4_val())
            .field("trig_in5_val", &self.trig_in5_val())
            .field("trig_in6_val", &self.trig_in6_val())
            .field("trig_in7_val", &self.trig_in7_val())
            .field("trig_in8_val", &self.trig_in8_val())
            .field("trig_in9_val", &self.trig_in9_val())
            .field("trig_in10_val", &self.trig_in10_val())
            .field("trig_in11_val", &self.trig_in11_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrigfilStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TrigfilStat {{ trig_in0_val: {:?}, trig_in1_val: {:?}, trig_in2_val: {:?}, trig_in3_val: {:?}, trig_in4_val: {:?}, trig_in5_val: {:?}, trig_in6_val: {:?}, trig_in7_val: {:?}, trig_in8_val: {:?}, trig_in9_val: {:?}, trig_in10_val: {:?}, trig_in11_val: {:?} }}",
            self.trig_in0_val(),
            self.trig_in1_val(),
            self.trig_in2_val(),
            self.trig_in3_val(),
            self.trig_in4_val(),
            self.trig_in5_val(),
            self.trig_in6_val(),
            self.trig_in7_val(),
            self.trig_in8_val(),
            self.trig_in9_val(),
            self.trig_in10_val(),
            self.trig_in11_val()
        )
    }
}
#[doc = "USB-FS trigger input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbfsTrig(pub u32);
impl UsbfsTrig {
    #[doc = "USB-FS trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> UsbfsTrigInp {
        let val = (self.0 >> 0usize) & 0x0f;
        UsbfsTrigInp::from_bits(val as u8)
    }
    #[doc = "USB-FS trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: UsbfsTrigInp) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for UsbfsTrig {
    #[inline(always)]
    fn default() -> UsbfsTrig {
        UsbfsTrig(0)
    }
}
impl core::fmt::Debug for UsbfsTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbfsTrig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbfsTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UsbfsTrig {{ inp: {:?} }}", self.inp())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcTrigTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    VAL1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    VAL6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    VAL7 = 0x07,
    #[doc = "CMP2_OUT input is selected."]
    VAL8 = 0x08,
    #[doc = "CTimer0_MAT0 input is selected."]
    VAL9 = 0x09,
    #[doc = "CTimer0_MAT1 input is selected."]
    VAL10 = 0x0a,
    #[doc = "CTimer1_MAT0 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CTimer1_MAT1 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CTimer2_MAT0 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CTimer2_MAT1 input is selected."]
    VAL14 = 0x0e,
    #[doc = "LPTMR0 input is selected."]
    VAL15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    VAL17 = 0x11,
    #[doc = "PWM0_SM0_OUT_TRIG0 input is selected."]
    VAL18 = 0x12,
    #[doc = "PWM0_SM0_OUT_TRIG1 input is selected."]
    VAL19 = 0x13,
    #[doc = "PWM0_SM1_OUT_TRIG0 input is selected."]
    VAL20 = 0x14,
    #[doc = "PWM0_SM1_OUT_TRIG1 input is selected."]
    VAL21 = 0x15,
    #[doc = "PWM0_SM2_OUT_TRIG0 input is selected."]
    VAL22 = 0x16,
    #[doc = "PWM0_SM2_OUT_TRIG1 input is selected."]
    VAL23 = 0x17,
    #[doc = "PWM0_SM3_OUT_TRIG0 input is selected."]
    VAL24 = 0x18,
    #[doc = "PWM0_SM3_OUT_TRIG1 input is selected."]
    VAL25 = 0x19,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    VAL26 = 0x1a,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    VAL27 = 0x1b,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    VAL28 = 0x1c,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    VAL29 = 0x1d,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    VAL30 = 0x1e,
    #[doc = "WUU."]
    VAL31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL33 = 0x21,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL34 = 0x22,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL35 = 0x23,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL36 = 0x24,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    VAL37 = 0x25,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    VAL38 = 0x26,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    VAL39 = 0x27,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    VAL40 = 0x28,
    #[doc = "CTimer3_MAT0 input is selected."]
    VAL41 = 0x29,
    #[doc = "CTimer3_MAT1 input is selected."]
    VAL42 = 0x2a,
    #[doc = "CTimer4_MAT0 input is selected."]
    VAL43 = 0x2b,
    #[doc = "CTimer4_MAT1 input is selected."]
    VAL44 = 0x2c,
    #[doc = "FlexIO CH0 input is selected."]
    VAL45 = 0x2d,
    #[doc = "FlexIO CH1 input is selected."]
    VAL46 = 0x2e,
    #[doc = "FlexIO CH2 input is selected."]
    VAL47 = 0x2f,
    #[doc = "FlexIO CH3 input is selected."]
    VAL48 = 0x30,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    VAL49 = 0x31,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    VAL50 = 0x32,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    VAL51 = 0x33,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    VAL52 = 0x34,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    VAL53 = 0x35,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    VAL54 = 0x36,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    VAL55 = 0x37,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    VAL56 = 0x38,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    VAL57 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl AdcTrigTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcTrigTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcTrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> AdcTrigTrigin {
        AdcTrigTrigin::from_bits(val)
    }
}
impl From<AdcTrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: AdcTrigTrigin) -> u8 {
        AdcTrigTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AoiInputInp {
    _RESERVED_0 = 0x0,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    VAL1 = 0x01,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    VAL2 = 0x02,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    VAL3 = 0x03,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    VAL4 = 0x04,
    #[doc = "CMP0_OUT input is selected."]
    VAL5 = 0x05,
    #[doc = "CMP1_OUT input is selected."]
    VAL6 = 0x06,
    #[doc = "CMP2_OUT input is selected."]
    VAL7 = 0x07,
    #[doc = "CTimer0_MAT0 input is selected."]
    VAL8 = 0x08,
    #[doc = "CTimer0_MAT1 input is selected."]
    VAL9 = 0x09,
    #[doc = "CTimer0_MAT2 input is selected."]
    VAL10 = 0x0a,
    #[doc = "CTimer0_MAT3 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CTimer1_MAT0 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CTimer1_MAT1 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CTimer1_MAT2 input is selected."]
    VAL14 = 0x0e,
    #[doc = "CTimer1_MAT3 input is selected."]
    VAL15 = 0x0f,
    #[doc = "CTimer2_MAT0 input is selected."]
    VAL16 = 0x10,
    #[doc = "CTimer2_MAT1 input is selected."]
    VAL17 = 0x11,
    #[doc = "CTimer2_MAT2 input is selected."]
    VAL18 = 0x12,
    #[doc = "CTimer2_MAT3 input is selected."]
    VAL19 = 0x13,
    #[doc = "LPTMR0 input is selected."]
    VAL20 = 0x14,
    _RESERVED_15 = 0x15,
    #[doc = "QDC0_CMP_FLAG0 input is selected."]
    VAL22 = 0x16,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    VAL23 = 0x17,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    VAL24 = 0x18,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    VAL25 = 0x19,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    VAL26 = 0x1a,
    #[doc = "PWM0_SM0_MUX_TRIG0 0 input is selected."]
    VAL27 = 0x1b,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    VAL28 = 0x1c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    VAL29 = 0x1d,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    VAL30 = 0x1e,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    VAL31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    VAL32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    VAL33 = 0x21,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected."]
    VAL34 = 0x22,
    #[doc = "TRIG_IN0 input is selected."]
    VAL35 = 0x23,
    #[doc = "TRIG_IN1 input is selected."]
    VAL36 = 0x24,
    #[doc = "TRIG_IN2 input is selected."]
    VAL37 = 0x25,
    #[doc = "TRIG_IN3 input is selected."]
    VAL38 = 0x26,
    #[doc = "TRIG_IN4 input is selected."]
    VAL39 = 0x27,
    #[doc = "TRIG_IN5 input is selected."]
    VAL40 = 0x28,
    #[doc = "TRIG_IN6 input is selected."]
    VAL41 = 0x29,
    #[doc = "TRIG_IN7 input is selected."]
    VAL42 = 0x2a,
    #[doc = "TRIG_IN8 input is selected."]
    VAL43 = 0x2b,
    #[doc = "TRIG_IN9 input is selected."]
    VAL44 = 0x2c,
    #[doc = "TRIG_IN10 input is selected."]
    VAL45 = 0x2d,
    #[doc = "TRIG_IN11 input is selected."]
    VAL46 = 0x2e,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    VAL47 = 0x2f,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    VAL48 = 0x30,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    VAL49 = 0x31,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    VAL50 = 0x32,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    VAL51 = 0x33,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    VAL52 = 0x34,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    VAL53 = 0x35,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    VAL54 = 0x36,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    VAL55 = 0x37,
    #[doc = "CTimer3_MAT0 input is selected."]
    VAL56 = 0x38,
    #[doc = "CTimer3_MAT1 input is selected."]
    VAL57 = 0x39,
    #[doc = "CTimer3_MAT2 input is selected."]
    VAL58 = 0x3a,
    #[doc = "CTimer3_MAT3 input is selected."]
    VAL59 = 0x3b,
    #[doc = "CTimer4_MAT0 input is selected."]
    VAL60 = 0x3c,
    #[doc = "CTimer4_MAT1 input is selected."]
    VAL61 = 0x3d,
    #[doc = "CTimer4_MAT2 input is selected."]
    VAL62 = 0x3e,
    #[doc = "CTimer4_MAT3 input is selected."]
    VAL63 = 0x3f,
    #[doc = "FlexIO CH0 input is selected."]
    VAL64 = 0x40,
    #[doc = "FlexIO CH1 input is selected."]
    VAL65 = 0x41,
    #[doc = "FlexIO CH2 input is selected."]
    VAL66 = 0x42,
    #[doc = "FlexIO CH3 input is selected."]
    VAL67 = 0x43,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    VAL68 = 0x44,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    VAL69 = 0x45,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    VAL70 = 0x46,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    VAL71 = 0x47,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    VAL72 = 0x48,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    VAL73 = 0x49,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    VAL74 = 0x4a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    VAL75 = 0x4b,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    VAL76 = 0x4c,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    VAL77 = 0x4d,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    VAL78 = 0x4e,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    VAL79 = 0x4f,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    VAL80 = 0x50,
    #[doc = "PWM0_SM0_A_Output."]
    VAL81 = 0x51,
    #[doc = "PWM0_SM0_B_Output."]
    VAL82 = 0x52,
    #[doc = "PWM0_SM1_A_Output."]
    VAL83 = 0x53,
    #[doc = "PWM0_SM1_B_Output."]
    VAL84 = 0x54,
    #[doc = "PWM0_SM2_A_Output."]
    VAL85 = 0x55,
    #[doc = "PWM0_SM2_B_Output."]
    VAL86 = 0x56,
    #[doc = "PWM0_SM3_A_Output."]
    VAL87 = 0x57,
    #[doc = "PWM0_SM3_B_Output."]
    VAL88 = 0x58,
    #[doc = "ADC2_tcomp\\[0\\] input is selected."]
    VAL89 = 0x59,
    #[doc = "ADC2_tcomp\\[1\\] input is selected."]
    VAL90 = 0x5a,
    #[doc = "ADC2_tcomp\\[2\\] input is selected."]
    VAL91 = 0x5b,
    #[doc = "ADC2_tcomp\\[3\\] input is selected."]
    VAL92 = 0x5c,
    #[doc = "ADC3_tcomp\\[0\\] input is selected."]
    VAL93 = 0x5d,
    #[doc = "ADC3_tcomp\\[1\\] input is selected."]
    VAL94 = 0x5e,
    #[doc = "ADC3_tcomp\\[2\\] input is selected."]
    VAL95 = 0x5f,
    #[doc = "ADC3_tcomp\\[3\\] input is selected."]
    VAL96 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl AoiInputInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AoiInputInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AoiInputInp {
    #[inline(always)]
    fn from(val: u8) -> AoiInputInp {
        AoiInputInp::from_bits(val)
    }
}
impl From<AoiInputInp> for u8 {
    #[inline(always)]
    fn from(val: AoiInputInp) -> u8 {
        AoiInputInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmpTrigTrigin {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL5 = 0x05,
    #[doc = "CMP1_OUT input is selected."]
    VAL6 = 0x06,
    #[doc = "CMP2_OUT input is selected."]
    VAL7 = 0x07,
    #[doc = "CTimer0_MAT0 input is selected."]
    VAL8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    VAL9 = 0x09,
    #[doc = "CTimer1_MAT0."]
    VAL10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CTimer2_MAT0 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    VAL13 = 0x0d,
    #[doc = "LPTMR0 input is selected."]
    VAL14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "QDC0_POS_MATCH0."]
    VAL16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    VAL17 = 0x11,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    VAL18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    VAL19 = 0x13,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    VAL20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    VAL21 = 0x15,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    VAL22 = 0x16,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    VAL23 = 0x17,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected."]
    VAL24 = 0x18,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    VAL25 = 0x19,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    VAL26 = 0x1a,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    VAL27 = 0x1b,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    VAL28 = 0x1c,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    VAL29 = 0x1d,
    #[doc = "WUU input is selected."]
    VAL30 = 0x1e,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL31 = 0x1f,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL32 = 0x20,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL33 = 0x21,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL34 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    #[doc = "CTimer3_MAT0."]
    VAL39 = 0x27,
    #[doc = "CTimer3_MAT1."]
    VAL40 = 0x28,
    #[doc = "CTimer4_MAT0 input is selected."]
    VAL41 = 0x29,
    #[doc = "CTimer4_MAT1 input is selected."]
    VAL42 = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    VAL47 = 0x2f,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    VAL48 = 0x30,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    VAL49 = 0x31,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    VAL50 = 0x32,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    VAL51 = 0x33,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    VAL52 = 0x34,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    VAL53 = 0x35,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    VAL54 = 0x36,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    VAL55 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl CmpTrigTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmpTrigTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmpTrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> CmpTrigTrigin {
        CmpTrigTrigin::from_bits(val)
    }
}
impl From<CmpTrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: CmpTrigTrigin) -> u8 {
        CmpTrigTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer0capInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_INP0 input is selected."]
    VAL1 = 0x01,
    #[doc = "CT_INP1 input is selected."]
    VAL2 = 0x02,
    #[doc = "CT_INP2 input is selected."]
    VAL3 = 0x03,
    #[doc = "CT_INP3 input is selected."]
    VAL4 = 0x04,
    #[doc = "CT_INP4 input is selected."]
    VAL5 = 0x05,
    #[doc = "CT_INP5 input is selected."]
    VAL6 = 0x06,
    #[doc = "CT_INP6 input is selected."]
    VAL7 = 0x07,
    #[doc = "CT_INP7 input is selected."]
    VAL8 = 0x08,
    #[doc = "CT_INP8 input is selected."]
    VAL9 = 0x09,
    #[doc = "CT_INP9 input is selected."]
    VAL10 = 0x0a,
    #[doc = "CT_INP10 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CT_INP11 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CT_INP12 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CT_INP13 input is selected."]
    VAL14 = 0x0e,
    #[doc = "CT_INP14 input is selected."]
    VAL15 = 0x0f,
    #[doc = "CT_INP15 input is selected."]
    VAL16 = 0x10,
    #[doc = "CT_INP16 input is selected."]
    VAL17 = 0x11,
    #[doc = "CT_INP17 input is selected."]
    VAL18 = 0x12,
    #[doc = "CT_INP18 input is selected."]
    VAL19 = 0x13,
    #[doc = "CT_INP19 input is selected."]
    VAL20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    VAL21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    VAL26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    VAL27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    VAL28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    VAL29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    VAL30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    VAL31 = 0x1f,
    #[doc = "CMP2_OUT is selected."]
    VAL32 = 0x20,
    #[doc = "CTimer1_MAT1 input is selected."]
    VAL33 = 0x21,
    #[doc = "CTimer1_MAT2 input is selected."]
    VAL34 = 0x22,
    #[doc = "CTimer1_MAT3 input is selected."]
    VAL35 = 0x23,
    #[doc = "CTimer2_MAT1 input is selected."]
    VAL36 = 0x24,
    #[doc = "CTimer2_MAT2 input is selected."]
    VAL37 = 0x25,
    #[doc = "CTimer2_MAT3 input is selected."]
    VAL38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    VAL39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    VAL40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    VAL41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    VAL42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    VAL43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    VAL44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    VAL45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    VAL46 = 0x2e,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    VAL47 = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    VAL48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    VAL49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    VAL50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    VAL51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    VAL52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    VAL53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    VAL54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    VAL55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    VAL56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    VAL57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    VAL58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    VAL59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    VAL60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    VAL61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    VAL62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    VAL63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    VAL64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    VAL65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    VAL66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    VAL67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    VAL68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    VAL69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    VAL70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    VAL75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    VAL76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    VAL77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    VAL78 = 0x4e,
    #[doc = "CTimer3_MAT1 input is selected."]
    VAL79 = 0x4f,
    #[doc = "CTimer3_MAT2 input is selected."]
    VAL80 = 0x50,
    #[doc = "CTimer3_MAT3 input is selected."]
    VAL81 = 0x51,
    #[doc = "CTimer4_MAT1 input is selected."]
    VAL82 = 0x52,
    #[doc = "CTimer4_MAT2 input is selected."]
    VAL83 = 0x53,
    #[doc = "CTimer4_MAT3 input is selected."]
    VAL84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    VAL85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    VAL86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    VAL87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    VAL88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    VAL89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    VAL90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    VAL91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    VAL92 = 0x5c,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    VAL93 = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    VAL94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    VAL95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    VAL96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    VAL97 = 0x61,
    #[doc = "LPUART5 Received Data Word input is selected."]
    VAL98 = 0x62,
    #[doc = "LPUART5 Transmitted Data Word input is selected."]
    VAL99 = 0x63,
    #[doc = "LPUART5 Receive Line Idle input is selected."]
    VAL100 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    #[doc = "ADC2_tcomp\\[0\\] input is selected."]
    VAL105 = 0x69,
    #[doc = "ADC2_tcomp\\[1\\] input is selected."]
    VAL106 = 0x6a,
    #[doc = "ADC2_tcomp\\[2\\] input is selected."]
    VAL107 = 0x6b,
    #[doc = "ADC2_tcomp\\[3\\] input is selected."]
    VAL108 = 0x6c,
    #[doc = "ADC3_tcomp\\[0\\] input is selected."]
    VAL109 = 0x6d,
    #[doc = "ADC3_tcomp\\[1\\] input is selected."]
    VAL110 = 0x6e,
    #[doc = "ADC3_tcomp\\[2\\] input is selected."]
    VAL111 = 0x6f,
    #[doc = "ADC3_tcomp\\[3\\] input is selected."]
    VAL112 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ctimer0capInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer0capInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer0capInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer0capInp {
        Ctimer0capInp::from_bits(val)
    }
}
impl From<Ctimer0capInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer0capInp) -> u8 {
        Ctimer0capInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer1capInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_INP0 input is selected."]
    VAL1 = 0x01,
    #[doc = "CT_INP1 input is selected."]
    VAL2 = 0x02,
    #[doc = "CT_INP2 input is selected."]
    VAL3 = 0x03,
    #[doc = "CT_INP3 input is selected."]
    VAL4 = 0x04,
    #[doc = "CT_INP4 input is selected."]
    VAL5 = 0x05,
    #[doc = "CT_INP5 input is selected."]
    VAL6 = 0x06,
    #[doc = "CT_INP6 input is selected."]
    VAL7 = 0x07,
    #[doc = "CT_INP7 input is selected."]
    VAL8 = 0x08,
    #[doc = "CT_INP8 input is selected."]
    VAL9 = 0x09,
    #[doc = "CT_INP9 input is selected."]
    VAL10 = 0x0a,
    #[doc = "CT_INP10 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CT_INP11 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CT_INP12 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CT_INP13 input is selected."]
    VAL14 = 0x0e,
    #[doc = "CT_INP14 input is selected."]
    VAL15 = 0x0f,
    #[doc = "CT_INP15 input is selected."]
    VAL16 = 0x10,
    #[doc = "CT_INP16 input is selected."]
    VAL17 = 0x11,
    #[doc = "CT_INP17 input is selected."]
    VAL18 = 0x12,
    #[doc = "CT_INP18 input is selected."]
    VAL19 = 0x13,
    #[doc = "CT_INP19 input is selected."]
    VAL20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    VAL21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    VAL26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    VAL27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    VAL28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    VAL29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    VAL30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    VAL31 = 0x1f,
    #[doc = "CMP2_OUT is selected."]
    VAL32 = 0x20,
    #[doc = "CTimer0_MAT1 input is selected."]
    VAL33 = 0x21,
    #[doc = "CTimer0_MAT2 input is selected."]
    VAL34 = 0x22,
    #[doc = "CTimer0_MAT3 input is selected."]
    VAL35 = 0x23,
    #[doc = "CTimer2_MAT1 input is selected."]
    VAL36 = 0x24,
    #[doc = "CTimer2_MAT2 input is selected."]
    VAL37 = 0x25,
    #[doc = "CTimer2_MAT3 input is selected."]
    VAL38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    VAL39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    VAL40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    VAL41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    VAL42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    VAL43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    VAL44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    VAL45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    VAL46 = 0x2e,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    VAL47 = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    VAL48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    VAL49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    VAL50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    VAL51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    VAL52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    VAL53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    VAL54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    VAL55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    VAL56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    VAL57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    VAL58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    VAL59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    VAL60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    VAL61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    VAL62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    VAL63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    VAL64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    VAL65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    VAL66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    VAL67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    VAL68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    VAL69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    VAL70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    VAL75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    VAL76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    VAL77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    VAL78 = 0x4e,
    #[doc = "CTimer3_MAT1 input is selected."]
    VAL79 = 0x4f,
    #[doc = "CTimer3_MAT2 input is selected."]
    VAL80 = 0x50,
    #[doc = "CTimer3_MAT3 input is selected."]
    VAL81 = 0x51,
    #[doc = "CTimer4_MAT1 input is selected."]
    VAL82 = 0x52,
    #[doc = "CTimer4_MAT2 input is selected."]
    VAL83 = 0x53,
    #[doc = "CTimer4_MAT3 input is selected."]
    VAL84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    VAL85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    VAL86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    VAL87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    VAL88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    VAL89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    VAL90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    VAL91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    VAL92 = 0x5c,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    VAL93 = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    VAL94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    VAL95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    VAL96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    VAL97 = 0x61,
    #[doc = "LPUART5 Received Data Word input is selected."]
    VAL98 = 0x62,
    #[doc = "LPUART5 Transmitted Data Word input is selected."]
    VAL99 = 0x63,
    #[doc = "LPUART5 Receive Line Idle input is selected."]
    VAL100 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    #[doc = "ADC2_tcomp\\[0\\] input is selected."]
    VAL105 = 0x69,
    #[doc = "ADC2_tcomp\\[1\\] input is selected."]
    VAL106 = 0x6a,
    #[doc = "ADC2_tcomp\\[2\\] input is selected."]
    VAL107 = 0x6b,
    #[doc = "ADC2_tcomp\\[3\\] input is selected."]
    VAL108 = 0x6c,
    #[doc = "ADC3_tcomp\\[0\\] input is selected."]
    VAL109 = 0x6d,
    #[doc = "ADC3_tcomp\\[1\\] input is selected."]
    VAL110 = 0x6e,
    #[doc = "ADC3_tcomp\\[2\\] input is selected."]
    VAL111 = 0x6f,
    #[doc = "ADC3_tcomp\\[3\\] input is selected."]
    VAL112 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ctimer1capInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer1capInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer1capInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer1capInp {
        Ctimer1capInp::from_bits(val)
    }
}
impl From<Ctimer1capInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer1capInp) -> u8 {
        Ctimer1capInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer2capInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_INP0 input is selected."]
    VAL1 = 0x01,
    #[doc = "CT_INP1 input is selected."]
    VAL2 = 0x02,
    #[doc = "CT_INP2 input is selected."]
    VAL3 = 0x03,
    #[doc = "CT_INP3 input is selected."]
    VAL4 = 0x04,
    #[doc = "CT_INP4 input is selected."]
    VAL5 = 0x05,
    #[doc = "CT_INP5 input is selected."]
    VAL6 = 0x06,
    #[doc = "CT_INP6 input is selected."]
    VAL7 = 0x07,
    #[doc = "CT_INP7 input is selected."]
    VAL8 = 0x08,
    #[doc = "CT_INP8 input is selected."]
    VAL9 = 0x09,
    #[doc = "CT_INP9 input is selected."]
    VAL10 = 0x0a,
    #[doc = "CT_INP10 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CT_INP11 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CT_INP12 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CT_INP13 input is selected."]
    VAL14 = 0x0e,
    #[doc = "CT_INP14 input is selected."]
    VAL15 = 0x0f,
    #[doc = "CT_INP15 input is selected."]
    VAL16 = 0x10,
    #[doc = "CT_INP16 input is selected."]
    VAL17 = 0x11,
    #[doc = "CT_INP17 input is selected."]
    VAL18 = 0x12,
    #[doc = "CT_INP18 input is selected."]
    VAL19 = 0x13,
    #[doc = "CT_INP19 input is selected."]
    VAL20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    VAL21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    VAL26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    VAL27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    VAL28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    VAL29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    VAL30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    VAL31 = 0x1f,
    #[doc = "CMP2_OUT is selected."]
    VAL32 = 0x20,
    #[doc = "CTimer0_MAT1 input is selected."]
    VAL33 = 0x21,
    #[doc = "CTimer0_MAT2 input is selected."]
    VAL34 = 0x22,
    #[doc = "CTimer0_MAT3 input is selected."]
    VAL35 = 0x23,
    #[doc = "CTimer1_MAT1 input is selected."]
    VAL36 = 0x24,
    #[doc = "CTimer1_MAT2 input is selected."]
    VAL37 = 0x25,
    #[doc = "CTimer1_MAT3 input is selected."]
    VAL38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    VAL39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    VAL40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    VAL41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    VAL42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    VAL43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    VAL44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    VAL45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    VAL46 = 0x2e,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    VAL47 = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    VAL48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    VAL49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    VAL50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    VAL51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    VAL52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    VAL53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    VAL54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    VAL55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    VAL56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    VAL57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    VAL58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    VAL59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    VAL60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    VAL61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    VAL62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    VAL63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    VAL64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    VAL65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    VAL66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    VAL67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    VAL68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    VAL69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    VAL70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    VAL75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    VAL76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    VAL77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    VAL78 = 0x4e,
    #[doc = "CTimer3_MAT1 input is selected."]
    VAL79 = 0x4f,
    #[doc = "CTimer3_MAT2 input is selected."]
    VAL80 = 0x50,
    #[doc = "CTimer3_MAT3 input is selected."]
    VAL81 = 0x51,
    #[doc = "CTimer4_MAT1 input is selected."]
    VAL82 = 0x52,
    #[doc = "CTimer4_MAT2 input is selected."]
    VAL83 = 0x53,
    #[doc = "CTimer4_MAT3 input is selected."]
    VAL84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    VAL85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    VAL86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    VAL87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    VAL88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    VAL89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    VAL90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    VAL91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    VAL92 = 0x5c,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    VAL93 = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    VAL94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    VAL95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    VAL96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    VAL97 = 0x61,
    #[doc = "LPUART5 Received Data Word input is selected."]
    VAL98 = 0x62,
    #[doc = "LPUART5 Transmitted Data Word input is selected."]
    VAL99 = 0x63,
    #[doc = "LPUART5 Receive Line Idle input is selected."]
    VAL100 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    #[doc = "ADC2_tcomp\\[0\\] input is selected."]
    VAL105 = 0x69,
    #[doc = "ADC2_tcomp\\[1\\] input is selected."]
    VAL106 = 0x6a,
    #[doc = "ADC2_tcomp\\[2\\] input is selected."]
    VAL107 = 0x6b,
    #[doc = "ADC2_tcomp\\[3\\] input is selected."]
    VAL108 = 0x6c,
    #[doc = "ADC3_tcomp\\[0\\] input is selected."]
    VAL109 = 0x6d,
    #[doc = "ADC3_tcomp\\[1\\] input is selected."]
    VAL110 = 0x6e,
    #[doc = "ADC3_tcomp\\[2\\] input is selected."]
    VAL111 = 0x6f,
    #[doc = "ADC3_tcomp\\[3\\] input is selected."]
    VAL112 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ctimer2capInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer2capInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer2capInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer2capInp {
        Ctimer2capInp::from_bits(val)
    }
}
impl From<Ctimer2capInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer2capInp) -> u8 {
        Ctimer2capInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer3capInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_INP0 input is selected."]
    VAL1 = 0x01,
    #[doc = "CT_INP1 input is selected."]
    VAL2 = 0x02,
    #[doc = "CT_INP2 input is selected."]
    VAL3 = 0x03,
    #[doc = "CT_INP3 input is selected."]
    VAL4 = 0x04,
    #[doc = "CT_INP4 input is selected."]
    VAL5 = 0x05,
    #[doc = "CT_INP5 input is selected."]
    VAL6 = 0x06,
    #[doc = "CT_INP6 input is selected."]
    VAL7 = 0x07,
    #[doc = "CT_INP7 input is selected."]
    VAL8 = 0x08,
    #[doc = "CT_INP8 input is selected."]
    VAL9 = 0x09,
    #[doc = "CT_INP9 input is selected."]
    VAL10 = 0x0a,
    #[doc = "CT_INP10 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CT_INP11 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CT_INP12 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CT_INP13 input is selected."]
    VAL14 = 0x0e,
    #[doc = "CT_INP14 input is selected."]
    VAL15 = 0x0f,
    #[doc = "CT_INP15 input is selected."]
    VAL16 = 0x10,
    #[doc = "CT_INP16 input is selected."]
    VAL17 = 0x11,
    #[doc = "CT_INP17 input is selected."]
    VAL18 = 0x12,
    #[doc = "CT_INP18 input is selected."]
    VAL19 = 0x13,
    #[doc = "CT_INP19 input is selected."]
    VAL20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    VAL21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    VAL26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    VAL27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    VAL28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    VAL29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    VAL30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    VAL31 = 0x1f,
    #[doc = "CMP2_OUT is selected."]
    VAL32 = 0x20,
    #[doc = "CTimer0_MAT1 input is selected."]
    VAL33 = 0x21,
    #[doc = "CTimer0_MAT2 input is selected."]
    VAL34 = 0x22,
    #[doc = "CTimer0_MAT3 input is selected."]
    VAL35 = 0x23,
    #[doc = "CTimer1_MAT1 input is selected."]
    VAL36 = 0x24,
    #[doc = "CTimer1_MAT2 input is selected."]
    VAL37 = 0x25,
    #[doc = "CTimer1_MAT3 input is selected."]
    VAL38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    VAL39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    VAL40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    VAL41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    VAL42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    VAL43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    VAL44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    VAL45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    VAL46 = 0x2e,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    VAL47 = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    VAL48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    VAL49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    VAL50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    VAL51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    VAL52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    VAL53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    VAL54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    VAL55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    VAL56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    VAL57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    VAL58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    VAL59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    VAL60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    VAL61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    VAL62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    VAL63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    VAL64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    VAL65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    VAL66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    VAL67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    VAL68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    VAL69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    VAL70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    VAL75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    VAL76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    VAL77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    VAL78 = 0x4e,
    #[doc = "CTimer2_MAT1 input is selected."]
    VAL79 = 0x4f,
    #[doc = "CTimer2_MAT2 input is selected."]
    VAL80 = 0x50,
    #[doc = "CTimer2_MAT3 input is selected."]
    VAL81 = 0x51,
    #[doc = "CTimer4_MAT1 input is selected."]
    VAL82 = 0x52,
    #[doc = "CTimer4_MAT2 input is selected."]
    VAL83 = 0x53,
    #[doc = "CTimer4_MAT3 input is selected."]
    VAL84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    VAL85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    VAL86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    VAL87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    VAL88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    VAL89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    VAL90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    VAL91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    VAL92 = 0x5c,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    VAL93 = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    VAL94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    VAL95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    VAL96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    VAL97 = 0x61,
    #[doc = "LPUART5 Received Data Word input is selected."]
    VAL98 = 0x62,
    #[doc = "LPUART5 Transmitted Data Word input is selected."]
    VAL99 = 0x63,
    #[doc = "LPUART5 Receive Line Idle input is selected."]
    VAL100 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    #[doc = "ADC2_tcomp\\[0\\] input is selected."]
    VAL105 = 0x69,
    #[doc = "ADC2_tcomp\\[1\\] input is selected."]
    VAL106 = 0x6a,
    #[doc = "ADC2_tcomp\\[2\\] input is selected."]
    VAL107 = 0x6b,
    #[doc = "ADC2_tcomp\\[3\\] input is selected."]
    VAL108 = 0x6c,
    #[doc = "ADC3_tcomp\\[0\\] input is selected."]
    VAL109 = 0x6d,
    #[doc = "ADC3_tcomp\\[1\\] input is selected."]
    VAL110 = 0x6e,
    #[doc = "ADC3_tcomp\\[2\\] input is selected."]
    VAL111 = 0x6f,
    #[doc = "ADC3_tcomp\\[3\\] input is selected."]
    VAL112 = 0x70,
    #[doc = "TRIG_IN0 input is selected."]
    VAL113 = 0x71,
    #[doc = "TRIG_IN1 input is selected."]
    VAL114 = 0x72,
    #[doc = "TRIG_IN2 input is selected."]
    VAL115 = 0x73,
    #[doc = "TRIG_IN3 input is selected."]
    VAL116 = 0x74,
    #[doc = "TRIG_IN4 input is selected."]
    VAL117 = 0x75,
    #[doc = "TRIG_IN5 input is selected."]
    VAL118 = 0x76,
    #[doc = "TRIG_IN6 input is selected."]
    VAL119 = 0x77,
    #[doc = "TRIG_IN7 input is selected."]
    VAL120 = 0x78,
    #[doc = "TRIG_IN8 input is selected."]
    VAL121 = 0x79,
    #[doc = "TRIG_IN9 input is selected."]
    VAL122 = 0x7a,
    #[doc = "TRIG_IN10 input is selected."]
    VAL123 = 0x7b,
    #[doc = "TRIG_IN11 input is selected."]
    VAL124 = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ctimer3capInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer3capInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer3capInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer3capInp {
        Ctimer3capInp::from_bits(val)
    }
}
impl From<Ctimer3capInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer3capInp) -> u8 {
        Ctimer3capInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer4capInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_INP0 input is selected."]
    VAL1 = 0x01,
    #[doc = "CT_INP1 input is selected."]
    VAL2 = 0x02,
    #[doc = "CT_INP2 input is selected."]
    VAL3 = 0x03,
    #[doc = "CT_INP3 input is selected."]
    VAL4 = 0x04,
    #[doc = "CT_INP4 input is selected."]
    VAL5 = 0x05,
    #[doc = "CT_INP5 input is selected."]
    VAL6 = 0x06,
    #[doc = "CT_INP6 input is selected."]
    VAL7 = 0x07,
    #[doc = "CT_INP7 input is selected."]
    VAL8 = 0x08,
    #[doc = "CT_INP8 input is selected."]
    VAL9 = 0x09,
    #[doc = "CT_INP9 input is selected."]
    VAL10 = 0x0a,
    #[doc = "CT_INP10 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CT_INP11 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CT_INP12 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CT_INP13 input is selected."]
    VAL14 = 0x0e,
    #[doc = "CT_INP14 input is selected."]
    VAL15 = 0x0f,
    #[doc = "CT_INP15 input is selected."]
    VAL16 = 0x10,
    #[doc = "CT_INP16 input is selected."]
    VAL17 = 0x11,
    #[doc = "CT_INP17 input is selected."]
    VAL18 = 0x12,
    #[doc = "CT_INP18 input is selected."]
    VAL19 = 0x13,
    #[doc = "CT_INP19 input is selected."]
    VAL20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    VAL21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    VAL26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    VAL27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    VAL28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    VAL29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    VAL30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    VAL31 = 0x1f,
    #[doc = "CMP2_OUT is selected."]
    VAL32 = 0x20,
    #[doc = "CTimer0_MAT1 input is selected."]
    VAL33 = 0x21,
    #[doc = "CTimer0_MAT2 input is selected."]
    VAL34 = 0x22,
    #[doc = "CTimer0_MAT3 input is selected."]
    VAL35 = 0x23,
    #[doc = "CTimer1_MAT1 input is selected."]
    VAL36 = 0x24,
    #[doc = "CTimer1_MAT2 input is selected."]
    VAL37 = 0x25,
    #[doc = "CTimer1_MAT3 input is selected."]
    VAL38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    VAL39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    VAL40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    VAL41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    VAL42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    VAL43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    VAL44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    VAL45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    VAL46 = 0x2e,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    VAL47 = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    VAL48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    VAL49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    VAL50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    VAL51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    VAL52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    VAL53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    VAL54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    VAL55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    VAL56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    VAL57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    VAL58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    VAL59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    VAL60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    VAL61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    VAL62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    VAL63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    VAL64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    VAL65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    VAL66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    VAL67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    VAL68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    VAL69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    VAL70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    VAL75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    VAL76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    VAL77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    VAL78 = 0x4e,
    #[doc = "CTimer2_MAT1 input is selected."]
    VAL79 = 0x4f,
    #[doc = "CTimer2_MAT2 input is selected."]
    VAL80 = 0x50,
    #[doc = "CTimer2_MAT3 input is selected."]
    VAL81 = 0x51,
    #[doc = "CTimer3_MAT1 input is selected."]
    VAL82 = 0x52,
    #[doc = "CTimer3_MAT2 input is selected."]
    VAL83 = 0x53,
    #[doc = "CTimer3_MAT3 input is selected."]
    VAL84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    VAL85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    VAL86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    VAL87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    VAL88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    VAL89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    VAL90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    VAL91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    VAL92 = 0x5c,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    VAL93 = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    VAL94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    VAL95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    VAL96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    VAL97 = 0x61,
    #[doc = "LPUART5 Received Data Word input is selected."]
    VAL98 = 0x62,
    #[doc = "LPUART5 Transmitted Data Word input is selected."]
    VAL99 = 0x63,
    #[doc = "LPUART5 Receive Line Idle input is selected."]
    VAL100 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    #[doc = "ADC2_tcomp\\[0\\] input is selected."]
    VAL105 = 0x69,
    #[doc = "ADC2_tcomp\\[1\\] input is selected."]
    VAL106 = 0x6a,
    #[doc = "ADC2_tcomp\\[2\\] input is selected."]
    VAL107 = 0x6b,
    #[doc = "ADC2_tcomp\\[3\\] input is selected."]
    VAL108 = 0x6c,
    #[doc = "ADC3_tcomp\\[0\\] input is selected."]
    VAL109 = 0x6d,
    #[doc = "ADC3_tcomp\\[1\\] input is selected."]
    VAL110 = 0x6e,
    #[doc = "ADC3_tcomp\\[2\\] input is selected."]
    VAL111 = 0x6f,
    #[doc = "ADC3_tcomp\\[3\\] input is selected."]
    VAL112 = 0x70,
    #[doc = "TRIG_IN0 input is selected."]
    VAL113 = 0x71,
    #[doc = "TRIG_IN1 input is selected."]
    VAL114 = 0x72,
    #[doc = "TRIG_IN2 input is selected."]
    VAL115 = 0x73,
    #[doc = "TRIG_IN3 input is selected."]
    VAL116 = 0x74,
    #[doc = "TRIG_IN4 input is selected."]
    VAL117 = 0x75,
    #[doc = "TRIG_IN5 input is selected."]
    VAL118 = 0x76,
    #[doc = "TRIG_IN6 input is selected."]
    VAL119 = 0x77,
    #[doc = "TRIG_IN7 input is selected."]
    VAL120 = 0x78,
    #[doc = "TRIG_IN8 input is selected."]
    VAL121 = 0x79,
    #[doc = "TRIG_IN9 input is selected."]
    VAL122 = 0x7a,
    #[doc = "TRIG_IN10 input is selected."]
    VAL123 = 0x7b,
    #[doc = "TRIG_IN11 input is selected."]
    VAL124 = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Ctimer4capInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer4capInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer4capInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer4capInp {
        Ctimer4capInp::from_bits(val)
    }
}
impl From<Ctimer4capInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer4capInp) -> u8 {
        Ctimer4capInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DacTrigTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV."]
    VAL1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    VAL6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    VAL7 = 0x07,
    #[doc = "CMP2_OUT input is selected."]
    VAL8 = 0x08,
    #[doc = "CTimer0_MAT0 input is selected."]
    VAL9 = 0x09,
    #[doc = "CTimer0_MAT1 input is selected."]
    VAL10 = 0x0a,
    #[doc = "CTimer1_MAT0 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CTimer1_MAT1 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CTimer2_MAT0 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CTimer2_MAT1 input is selected."]
    VAL14 = 0x0e,
    #[doc = "LPTMR0 input is selected."]
    VAL15 = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    VAL18 = 0x12,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    VAL19 = 0x13,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    VAL20 = 0x14,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    VAL21 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    VAL26 = 0x1a,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    VAL27 = 0x1b,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    VAL28 = 0x1c,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    VAL29 = 0x1d,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    VAL30 = 0x1e,
    #[doc = "WUU input is selected."]
    VAL31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL33 = 0x21,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL34 = 0x22,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL35 = 0x23,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL36 = 0x24,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    VAL37 = 0x25,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    VAL38 = 0x26,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    VAL39 = 0x27,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    VAL40 = 0x28,
    #[doc = "CTimer3_MAT0 input is selected."]
    VAL41 = 0x29,
    #[doc = "CTimer3_MAT1 input is selected."]
    VAL42 = 0x2a,
    #[doc = "CTimer4_MAT0 input is selected."]
    VAL43 = 0x2b,
    #[doc = "CTimer4_MAT1 input is selected."]
    VAL44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    VAL50 = 0x32,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    VAL51 = 0x33,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    VAL52 = 0x34,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    VAL53 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    #[doc = "ADC2_tcomp\\[0\\] input is selected."]
    VAL58 = 0x3a,
    #[doc = "ADC2_tcomp\\[1\\] input is selected."]
    VAL59 = 0x3b,
    #[doc = "ADC3_tcomp\\[0\\] input is selected."]
    VAL60 = 0x3c,
    #[doc = "ADC3_tcomp\\[1\\] input is selected."]
    VAL61 = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl DacTrigTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DacTrigTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DacTrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> DacTrigTrigin {
        DacTrigTrigin::from_bits(val)
    }
}
impl From<DacTrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: DacTrigTrigin) -> u8 {
        DacTrigTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExtTrigInp {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    VAL6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    VAL7 = 0x07,
    #[doc = "CMP2_OUT input is selected."]
    VAL8 = 0x08,
    #[doc = "LPUART0 ipp_do_lpuart_txd input is selected."]
    VAL9 = 0x09,
    #[doc = "LPUART1 ipp_do_lpuart_txd input is selected."]
    VAL10 = 0x0a,
    #[doc = "LPUART2 ipp_do_lpuart_txd input is selected."]
    VAL11 = 0x0b,
    #[doc = "LPUART3 ipp_do_lpuart_txd input is selected."]
    VAL12 = 0x0c,
    #[doc = "LPUART4 ipp_do_lpuart_txd input is selected."]
    VAL13 = 0x0d,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL14 = 0x0e,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL15 = 0x0f,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL16 = 0x10,
    #[doc = "RTC_1Hz_CLK input is selected."]
    VAL17 = 0x11,
    #[doc = "LPUART5 ipp_do_lpuart_txd input is selected."]
    VAL18 = 0x12,
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
impl ExtTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExtTrigInp {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExtTrigInp {
    #[inline(always)]
    fn from(val: u8) -> ExtTrigInp {
        ExtTrigInp::from_bits(val)
    }
}
impl From<ExtTrigInp> for u8 {
    #[inline(always)]
    fn from(val: ExtTrigInp) -> u8 {
        ExtTrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FiltScaleEn {
    #[doc = "Disable prescaller."]
    VAL2 = 0x0,
    #[doc = "Enabled prescaller."]
    VAL1 = 0x01,
}
impl FiltScaleEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FiltScaleEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FiltScaleEn {
    #[inline(always)]
    fn from(val: u8) -> FiltScaleEn {
        FiltScaleEn::from_bits(val)
    }
}
impl From<FiltScaleEn> for u8 {
    #[inline(always)]
    fn from(val: FiltScaleEn) -> u8 {
        FiltScaleEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FiltScaleVal {
    #[doc = "Bypass the clock."]
    VAL0 = 0x0,
    #[doc = "Divide 2."]
    VAL1 = 0x01,
    #[doc = "Divide 4."]
    VAL2 = 0x02,
    #[doc = "Divide 8."]
    VAL3 = 0x03,
}
impl FiltScaleVal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FiltScaleVal {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FiltScaleVal {
    #[inline(always)]
    fn from(val: u8) -> FiltScaleVal {
        FiltScaleVal::from_bits(val)
    }
}
impl From<FiltScaleVal> for u8 {
    #[inline(always)]
    fn from(val: FiltScaleVal) -> u8 {
        FiltScaleVal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwmTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    VAL1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    VAL6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    VAL7 = 0x07,
    #[doc = "CMP2_OUT input is selected."]
    VAL8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    VAL9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected."]
    VAL10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    VAL14 = 0x0e,
    #[doc = "QDC0_CMP_FLAG0 input is selected."]
    VAL15 = 0x0f,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    VAL16 = 0x10,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    VAL17 = 0x11,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    VAL18 = 0x12,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    VAL19 = 0x13,
    #[doc = "TRIG_IN0 input is selected."]
    VAL20 = 0x14,
    #[doc = "TRIG_IN1 input is selected."]
    VAL21 = 0x15,
    #[doc = "TRIG_IN2 input is selected."]
    VAL22 = 0x16,
    #[doc = "TRIG_IN3 input is selected."]
    VAL23 = 0x17,
    #[doc = "TRIG_IN4 input is selected."]
    VAL24 = 0x18,
    #[doc = "TRIG_IN5 input is selected."]
    VAL25 = 0x19,
    #[doc = "TRIG_IN6 input is selected."]
    VAL26 = 0x1a,
    #[doc = "TRIG_IN7 input is selected."]
    VAL27 = 0x1b,
    #[doc = "TRIG_IN8 input is selected."]
    VAL28 = 0x1c,
    #[doc = "TRIG_IN9 input is selected."]
    VAL29 = 0x1d,
    #[doc = "TRIG_IN10 input is selected."]
    VAL30 = 0x1e,
    #[doc = "TRIG_IN11 input is selected."]
    VAL31 = 0x1f,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    VAL32 = 0x20,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    VAL33 = 0x21,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    VAL34 = 0x22,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    VAL35 = 0x23,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    VAL36 = 0x24,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL37 = 0x25,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL38 = 0x26,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL39 = 0x27,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL40 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    #[doc = "CTimer3_MAT2 input is selected."]
    VAL45 = 0x2d,
    #[doc = "CTimer3_MAT3 input is selected."]
    VAL46 = 0x2e,
    #[doc = "CTimer4_MAT2 input is selected."]
    VAL47 = 0x2f,
    #[doc = "CTimer4_MAT3 input is selected."]
    VAL48 = 0x30,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    VAL49 = 0x31,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    VAL50 = 0x32,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    VAL51 = 0x33,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    VAL52 = 0x34,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    VAL53 = 0x35,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    VAL54 = 0x36,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    VAL55 = 0x37,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    VAL56 = 0x38,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    VAL57 = 0x39,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    VAL58 = 0x3a,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    VAL59 = 0x3b,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    VAL60 = 0x3c,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    VAL61 = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwmTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwmTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwmTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwmTrigin {
        FlexPwmTrigin::from_bits(val)
    }
}
impl From<FlexPwmTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwmTrigin) -> u8 {
        FlexPwmTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioTrigInp {
    _RESERVED_0 = 0x0,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL1 = 0x01,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL4 = 0x04,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    VAL5 = 0x05,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    VAL6 = 0x06,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    VAL7 = 0x07,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    VAL8 = 0x08,
    #[doc = "CMP0_OUT input is selected."]
    VAL9 = 0x09,
    #[doc = "CMP1_OUT input is selected."]
    VAL10 = 0x0a,
    #[doc = "CMP2_OUT input is selected."]
    VAL11 = 0x0b,
    #[doc = "CTimer0_MAT1 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CTimer0_MAT2 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CTimer1_MAT1 input is selected."]
    VAL14 = 0x0e,
    #[doc = "CTimer1_MAT2 input is selected."]
    VAL15 = 0x0f,
    #[doc = "CTimer2_MAT1 input is selected."]
    VAL16 = 0x10,
    #[doc = "CTimer2_MAT2 input is selected."]
    VAL17 = 0x11,
    #[doc = "LPTMR0 input is selected."]
    VAL18 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    VAL20 = 0x14,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    VAL21 = 0x15,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    VAL22 = 0x16,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    VAL23 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    VAL24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    VAL25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    VAL26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    VAL27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    VAL28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    VAL29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    VAL30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    VAL31 = 0x1f,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    VAL32 = 0x20,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    VAL33 = 0x21,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    VAL34 = 0x22,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    VAL35 = 0x23,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    VAL36 = 0x24,
    #[doc = "WUU input is selected."]
    VAL37 = 0x25,
    #[doc = "LPI2C0 Master End of Packet."]
    VAL38 = 0x26,
    #[doc = "LPI2C0 Slave End of Packet."]
    VAL39 = 0x27,
    #[doc = "LPI2C1 Master End of Packet."]
    VAL40 = 0x28,
    #[doc = "LPI2C1 Slave End of Packet."]
    VAL41 = 0x29,
    #[doc = "LPSPI0 End of Frame."]
    VAL42 = 0x2a,
    #[doc = "LPSPI0 Received Data Word."]
    VAL43 = 0x2b,
    #[doc = "LPSPI1 End of Frame."]
    VAL44 = 0x2c,
    #[doc = "LPSPI1 Received Data Word."]
    VAL45 = 0x2d,
    #[doc = "LPUART0 Received Data Word."]
    VAL46 = 0x2e,
    #[doc = "LPUART0 Transmitted Data Word."]
    VAL47 = 0x2f,
    #[doc = "LPUART0 Receive Line Idle."]
    VAL48 = 0x30,
    #[doc = "LPUART1 Received Data Word."]
    VAL49 = 0x31,
    #[doc = "LPUART1 Transmitted Data Word."]
    VAL50 = 0x32,
    #[doc = "LPUART1 Receive Line Idle."]
    VAL51 = 0x33,
    #[doc = "LPUART2 Received Data Word."]
    VAL52 = 0x34,
    #[doc = "LPUART2 Transmitted Data Word."]
    VAL53 = 0x35,
    #[doc = "LPUART2 Receive Line Idle."]
    VAL54 = 0x36,
    #[doc = "LPUART3 Received Data Word."]
    VAL55 = 0x37,
    #[doc = "LPUART3 Transmitted Data Word."]
    VAL56 = 0x38,
    #[doc = "LPUART3 Receive Line Idle."]
    VAL57 = 0x39,
    #[doc = "LPUART4 Received Data Word."]
    VAL58 = 0x3a,
    #[doc = "LPUART4 Transmitted Data Word."]
    VAL59 = 0x3b,
    #[doc = "LPUART4 Receive Line Idle."]
    VAL60 = 0x3c,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL61 = 0x3d,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL62 = 0x3e,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL63 = 0x3f,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL64 = 0x40,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    VAL65 = 0x41,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    VAL66 = 0x42,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    VAL67 = 0x43,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    VAL68 = 0x44,
    #[doc = "CTimer3_MAT2 input is selected."]
    VAL69 = 0x45,
    #[doc = "CTimer3_MAT3 input is selected."]
    VAL70 = 0x46,
    #[doc = "CTimer4_MAT2 input is selected."]
    VAL71 = 0x47,
    #[doc = "CTimer4_MAT3 input is selected."]
    VAL72 = 0x48,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    VAL73 = 0x49,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    VAL74 = 0x4a,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    VAL75 = 0x4b,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    VAL76 = 0x4c,
    #[doc = "LPI2C2 Master End of Packet."]
    VAL77 = 0x4d,
    #[doc = "LPI2C2 Slave End of Packet."]
    VAL78 = 0x4e,
    #[doc = "LPI2C3 Master End of Packet."]
    VAL79 = 0x4f,
    #[doc = "LPI2C3 Slave End of Packet."]
    VAL80 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl FlexioTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexioTrigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexioTrigInp {
    #[inline(always)]
    fn from(val: u8) -> FlexioTrigInp {
        FlexioTrigInp::from_bits(val)
    }
}
impl From<FlexioTrigInp> for u8 {
    #[inline(always)]
    fn from(val: FlexioTrigInp) -> u8 {
        FlexioTrigInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct FreqmeasRefInp(u8);
impl FreqmeasRefInp {
    #[doc = "clk_in input is selected."]
    pub const VAL1: Self = Self(0x01);
    #[doc = "FRO_OSC_12M input is selected."]
    pub const VAL2: Self = Self(0x02);
    #[doc = "fro_hf_div input is selected."]
    pub const VAL3: Self = Self(0x03);
    #[doc = "clk_16k\\[1\\] input is selected."]
    pub const VAL5: Self = Self(0x05);
    #[doc = "SLOW_CLK input is selected."]
    pub const VAL6: Self = Self(0x06);
    #[doc = "FREQME_CLK_IN0 input is selected."]
    pub const VAL7: Self = Self(0x07);
    #[doc = "FREQME_CLK_IN1 input is selected input is selected."]
    pub const VAL8: Self = Self(0x08);
    #[doc = "AOI0_OUT0 input is selected."]
    pub const VAL9: Self = Self(0x09);
    #[doc = "AOI0_OUT1."]
    pub const VAL10: Self = Self(0x0a);
    #[doc = "PWM0_SM0_MUX_TRIG0."]
    pub const VAL11: Self = Self(0x0b);
    #[doc = "PWM0_SM0_MUX_TRIG1."]
    pub const VAL12: Self = Self(0x0c);
    #[doc = "PWM0_SM1_MUX_TRIG0."]
    pub const VAL13: Self = Self(0x0d);
    #[doc = "PWM0_SM1_MUX_TRIG1."]
    pub const VAL14: Self = Self(0x0e);
    #[doc = "PWM0_SM2_MUX_TRIG0."]
    pub const VAL15: Self = Self(0x0f);
    #[doc = "PWM0_SM2_MUX_TRIG1."]
    pub const VAL16: Self = Self(0x10);
    #[doc = "PWM0_SM3_MUX_TRIG0."]
    pub const VAL17: Self = Self(0x11);
    #[doc = "PWM0_SM3_MUX_TRIG1."]
    pub const VAL18: Self = Self(0x12);
    #[doc = "AOI1_OUT0 input is selected."]
    pub const VAL32: Self = Self(0x20);
    #[doc = "AOI1_OUT1 input is selected."]
    pub const VAL33: Self = Self(0x21);
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    pub const VAL34: Self = Self(0x22);
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    pub const VAL35: Self = Self(0x23);
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    pub const VAL36: Self = Self(0x24);
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    pub const VAL37: Self = Self(0x25);
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    pub const VAL38: Self = Self(0x26);
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    pub const VAL39: Self = Self(0x27);
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    pub const VAL40: Self = Self(0x28);
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    pub const VAL41: Self = Self(0x29);
}
impl FreqmeasRefInp {
    pub const fn from_bits(val: u8) -> FreqmeasRefInp {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for FreqmeasRefInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("VAL1"),
            0x02 => f.write_str("VAL2"),
            0x03 => f.write_str("VAL3"),
            0x05 => f.write_str("VAL5"),
            0x06 => f.write_str("VAL6"),
            0x07 => f.write_str("VAL7"),
            0x08 => f.write_str("VAL8"),
            0x09 => f.write_str("VAL9"),
            0x0a => f.write_str("VAL10"),
            0x0b => f.write_str("VAL11"),
            0x0c => f.write_str("VAL12"),
            0x0d => f.write_str("VAL13"),
            0x0e => f.write_str("VAL14"),
            0x0f => f.write_str("VAL15"),
            0x10 => f.write_str("VAL16"),
            0x11 => f.write_str("VAL17"),
            0x12 => f.write_str("VAL18"),
            0x20 => f.write_str("VAL32"),
            0x21 => f.write_str("VAL33"),
            0x22 => f.write_str("VAL34"),
            0x23 => f.write_str("VAL35"),
            0x24 => f.write_str("VAL36"),
            0x25 => f.write_str("VAL37"),
            0x26 => f.write_str("VAL38"),
            0x27 => f.write_str("VAL39"),
            0x28 => f.write_str("VAL40"),
            0x29 => f.write_str("VAL41"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FreqmeasRefInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "VAL1"),
            0x02 => defmt::write!(f, "VAL2"),
            0x03 => defmt::write!(f, "VAL3"),
            0x05 => defmt::write!(f, "VAL5"),
            0x06 => defmt::write!(f, "VAL6"),
            0x07 => defmt::write!(f, "VAL7"),
            0x08 => defmt::write!(f, "VAL8"),
            0x09 => defmt::write!(f, "VAL9"),
            0x0a => defmt::write!(f, "VAL10"),
            0x0b => defmt::write!(f, "VAL11"),
            0x0c => defmt::write!(f, "VAL12"),
            0x0d => defmt::write!(f, "VAL13"),
            0x0e => defmt::write!(f, "VAL14"),
            0x0f => defmt::write!(f, "VAL15"),
            0x10 => defmt::write!(f, "VAL16"),
            0x11 => defmt::write!(f, "VAL17"),
            0x12 => defmt::write!(f, "VAL18"),
            0x20 => defmt::write!(f, "VAL32"),
            0x21 => defmt::write!(f, "VAL33"),
            0x22 => defmt::write!(f, "VAL34"),
            0x23 => defmt::write!(f, "VAL35"),
            0x24 => defmt::write!(f, "VAL36"),
            0x25 => defmt::write!(f, "VAL37"),
            0x26 => defmt::write!(f, "VAL38"),
            0x27 => defmt::write!(f, "VAL39"),
            0x28 => defmt::write!(f, "VAL40"),
            0x29 => defmt::write!(f, "VAL41"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for FreqmeasRefInp {
    #[inline(always)]
    fn from(val: u8) -> FreqmeasRefInp {
        FreqmeasRefInp::from_bits(val)
    }
}
impl From<FreqmeasRefInp> for u8 {
    #[inline(always)]
    fn from(val: FreqmeasRefInp) -> u8 {
        FreqmeasRefInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct FreqmeasTarInp(u8);
impl FreqmeasTarInp {
    #[doc = "clk_in input is selected."]
    pub const VAL1: Self = Self(0x01);
    #[doc = "FRO_OSC_12M input is selected."]
    pub const VAL2: Self = Self(0x02);
    #[doc = "fro_hf_div input is selected."]
    pub const VAL3: Self = Self(0x03);
    #[doc = "clk_16k\\[1\\] input is selected."]
    pub const VAL5: Self = Self(0x05);
    #[doc = "SLOW_CLK input is selected."]
    pub const VAL6: Self = Self(0x06);
    #[doc = "FREQME_CLK_IN0 input is selected."]
    pub const VAL7: Self = Self(0x07);
    #[doc = "FREQME_CLK_IN1 input is selected input is selected."]
    pub const VAL8: Self = Self(0x08);
    #[doc = "AOI0_OUT0 input is selected."]
    pub const VAL9: Self = Self(0x09);
    #[doc = "AOI0_OUT1."]
    pub const VAL10: Self = Self(0x0a);
    #[doc = "PWM0_SM0_MUX_TRIG0."]
    pub const VAL11: Self = Self(0x0b);
    #[doc = "PWM0_SM0_MUX_TRIG1."]
    pub const VAL12: Self = Self(0x0c);
    #[doc = "PWM0_SM1_MUX_TRIG0."]
    pub const VAL13: Self = Self(0x0d);
    #[doc = "PWM0_SM1_MUX_TRIG1."]
    pub const VAL14: Self = Self(0x0e);
    #[doc = "PWM0_SM2_MUX_TRIG0."]
    pub const VAL15: Self = Self(0x0f);
    #[doc = "PWM0_SM2_MUX_TRIG1."]
    pub const VAL16: Self = Self(0x10);
    #[doc = "PWM0_SM3_MUX_TRIG0."]
    pub const VAL17: Self = Self(0x11);
    #[doc = "PWM0_SM3_MUX_TRIG1."]
    pub const VAL18: Self = Self(0x12);
    #[doc = "AOI1_OUT0 input is selected."]
    pub const VAL32: Self = Self(0x20);
    #[doc = "AOI1_OUT1 input is selected."]
    pub const VAL33: Self = Self(0x21);
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    pub const VAL34: Self = Self(0x22);
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    pub const VAL35: Self = Self(0x23);
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    pub const VAL36: Self = Self(0x24);
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    pub const VAL37: Self = Self(0x25);
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    pub const VAL38: Self = Self(0x26);
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    pub const VAL39: Self = Self(0x27);
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    pub const VAL40: Self = Self(0x28);
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    pub const VAL41: Self = Self(0x29);
}
impl FreqmeasTarInp {
    pub const fn from_bits(val: u8) -> FreqmeasTarInp {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for FreqmeasTarInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("VAL1"),
            0x02 => f.write_str("VAL2"),
            0x03 => f.write_str("VAL3"),
            0x05 => f.write_str("VAL5"),
            0x06 => f.write_str("VAL6"),
            0x07 => f.write_str("VAL7"),
            0x08 => f.write_str("VAL8"),
            0x09 => f.write_str("VAL9"),
            0x0a => f.write_str("VAL10"),
            0x0b => f.write_str("VAL11"),
            0x0c => f.write_str("VAL12"),
            0x0d => f.write_str("VAL13"),
            0x0e => f.write_str("VAL14"),
            0x0f => f.write_str("VAL15"),
            0x10 => f.write_str("VAL16"),
            0x11 => f.write_str("VAL17"),
            0x12 => f.write_str("VAL18"),
            0x20 => f.write_str("VAL32"),
            0x21 => f.write_str("VAL33"),
            0x22 => f.write_str("VAL34"),
            0x23 => f.write_str("VAL35"),
            0x24 => f.write_str("VAL36"),
            0x25 => f.write_str("VAL37"),
            0x26 => f.write_str("VAL38"),
            0x27 => f.write_str("VAL39"),
            0x28 => f.write_str("VAL40"),
            0x29 => f.write_str("VAL41"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FreqmeasTarInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "VAL1"),
            0x02 => defmt::write!(f, "VAL2"),
            0x03 => defmt::write!(f, "VAL3"),
            0x05 => defmt::write!(f, "VAL5"),
            0x06 => defmt::write!(f, "VAL6"),
            0x07 => defmt::write!(f, "VAL7"),
            0x08 => defmt::write!(f, "VAL8"),
            0x09 => defmt::write!(f, "VAL9"),
            0x0a => defmt::write!(f, "VAL10"),
            0x0b => defmt::write!(f, "VAL11"),
            0x0c => defmt::write!(f, "VAL12"),
            0x0d => defmt::write!(f, "VAL13"),
            0x0e => defmt::write!(f, "VAL14"),
            0x0f => defmt::write!(f, "VAL15"),
            0x10 => defmt::write!(f, "VAL16"),
            0x11 => defmt::write!(f, "VAL17"),
            0x12 => defmt::write!(f, "VAL18"),
            0x20 => defmt::write!(f, "VAL32"),
            0x21 => defmt::write!(f, "VAL33"),
            0x22 => defmt::write!(f, "VAL34"),
            0x23 => defmt::write!(f, "VAL35"),
            0x24 => defmt::write!(f, "VAL36"),
            0x25 => defmt::write!(f, "VAL37"),
            0x26 => defmt::write!(f, "VAL38"),
            0x27 => defmt::write!(f, "VAL39"),
            0x28 => defmt::write!(f, "VAL40"),
            0x29 => defmt::write!(f, "VAL41"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for FreqmeasTarInp {
    #[inline(always)]
    fn from(val: u8) -> FreqmeasTarInp {
        FreqmeasTarInp::from_bits(val)
    }
}
impl From<FreqmeasTarInp> for u8 {
    #[inline(always)]
    fn from(val: FreqmeasTarInp) -> u8 {
        FreqmeasTarInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2cTrigInp {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    VAL6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    VAL7 = 0x07,
    #[doc = "CMP2_OUT input is selected."]
    VAL8 = 0x08,
    #[doc = "CTimer0_MAT0 input is selected."]
    VAL9 = 0x09,
    #[doc = "CTimer0_MAT1 input is selected."]
    VAL10 = 0x0a,
    #[doc = "CTimer1_MAT0 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CTimer1_MAT1 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CTimer2_MAT0 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CTimer2_MAT1 input is selected."]
    VAL14 = 0x0e,
    #[doc = "LPTMR0 input is selected."]
    VAL15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "TRIG_IN0 input is selected."]
    VAL17 = 0x11,
    #[doc = "TRIG_IN1 input is selected."]
    VAL18 = 0x12,
    #[doc = "TRIG_IN2 input is selected."]
    VAL19 = 0x13,
    #[doc = "TRIG_IN3 input is selected."]
    VAL20 = 0x14,
    #[doc = "TRIG_IN4 input is selected."]
    VAL21 = 0x15,
    #[doc = "TRIG_IN5 input is selected."]
    VAL22 = 0x16,
    #[doc = "TRIG_IN6 input is selected."]
    VAL23 = 0x17,
    #[doc = "TRIG_IN7 input is selected."]
    VAL24 = 0x18,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    VAL25 = 0x19,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    VAL26 = 0x1a,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    VAL27 = 0x1b,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    VAL28 = 0x1c,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    VAL29 = 0x1d,
    #[doc = "WUU input is selected."]
    VAL30 = 0x1e,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL31 = 0x1f,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL32 = 0x20,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL33 = 0x21,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL34 = 0x22,
    #[doc = "CTimer3_MAT2 input is selected."]
    VAL35 = 0x23,
    #[doc = "CTimer3_MAT3 input is selected."]
    VAL36 = 0x24,
    #[doc = "CTimer4_MAT2 input is selected."]
    VAL37 = 0x25,
    #[doc = "CTimer4_MAT3 input is selected."]
    VAL38 = 0x26,
    #[doc = "FlexIO CH0 input is selected."]
    VAL39 = 0x27,
    #[doc = "FlexIO CH1 input is selected."]
    VAL40 = 0x28,
    #[doc = "FlexIO CH2 input is selected."]
    VAL41 = 0x29,
    #[doc = "FlexIO CH3 input is selected."]
    VAL42 = 0x2a,
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
impl Lpi2cTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2cTrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2cTrigInp {
    #[inline(always)]
    fn from(val: u8) -> Lpi2cTrigInp {
        Lpi2cTrigInp::from_bits(val)
    }
}
impl From<Lpi2cTrigInp> for u8 {
    #[inline(always)]
    fn from(val: Lpi2cTrigInp) -> u8 {
        Lpi2cTrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpspiTrigInp {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    VAL6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    VAL7 = 0x07,
    #[doc = "CMP2_OUT input is selected."]
    VAL8 = 0x08,
    #[doc = "CTimer0_MAT1 input is selected."]
    VAL9 = 0x09,
    #[doc = "CTimer0_MAT2 input is selected."]
    VAL10 = 0x0a,
    #[doc = "CTimer1_MAT1 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CTimer1_MAT2 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CTimer2_MAT1 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CTimer2_MAT2 input is selected."]
    VAL14 = 0x0e,
    #[doc = "LPTMR0 input is selected."]
    VAL15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "TRIG_IN0 input is selected."]
    VAL17 = 0x11,
    #[doc = "TRIG_IN1 input is selected."]
    VAL18 = 0x12,
    #[doc = "TRIG_IN2 input is selected."]
    VAL19 = 0x13,
    #[doc = "TRIG_IN3 input is selected."]
    VAL20 = 0x14,
    #[doc = "TRIG_IN4 input is selected."]
    VAL21 = 0x15,
    #[doc = "TRIG_IN5 input is selected."]
    VAL22 = 0x16,
    #[doc = "TRIG_IN6 input is selected."]
    VAL23 = 0x17,
    #[doc = "TRIG_IN7 input is selected."]
    VAL24 = 0x18,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    VAL25 = 0x19,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    VAL26 = 0x1a,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    VAL27 = 0x1b,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    VAL28 = 0x1c,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    VAL29 = 0x1d,
    #[doc = "WUU input is selected."]
    VAL30 = 0x1e,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL31 = 0x1f,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL32 = 0x20,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL33 = 0x21,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL34 = 0x22,
    #[doc = "CTimer3_MAT2 input is selected."]
    VAL35 = 0x23,
    #[doc = "CTimer3_MAT3 input is selected."]
    VAL36 = 0x24,
    #[doc = "CTimer4_MAT2 input is selected."]
    VAL37 = 0x25,
    #[doc = "CTimer4_MAT3 input is selected."]
    VAL38 = 0x26,
    #[doc = "FlexIO CH0 input is selected."]
    VAL39 = 0x27,
    #[doc = "FlexIO CH1 input is selected."]
    VAL40 = 0x28,
    #[doc = "FlexIO CH2 input is selected."]
    VAL41 = 0x29,
    #[doc = "FlexIO CH3 input is selected."]
    VAL42 = 0x2a,
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
impl LpspiTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpspiTrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpspiTrigInp {
    #[inline(always)]
    fn from(val: u8) -> LpspiTrigInp {
        LpspiTrigInp::from_bits(val)
    }
}
impl From<LpspiTrigInp> for u8 {
    #[inline(always)]
    fn from(val: LpspiTrigInp) -> u8 {
        LpspiTrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpuartInp {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    VAL6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    VAL7 = 0x07,
    #[doc = "CMP2_OUT input is selected."]
    VAL8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    VAL9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected."]
    VAL10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    VAL14 = 0x0e,
    #[doc = "LPTMR0 input is selected."]
    VAL15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "TRIG_IN0 input is selected."]
    VAL17 = 0x11,
    #[doc = "TRIG_IN1 input is selected."]
    VAL18 = 0x12,
    #[doc = "TRIG_IN2 input is selected."]
    VAL19 = 0x13,
    #[doc = "TRIG_IN3 input is selected."]
    VAL20 = 0x14,
    #[doc = "TRIG_IN4 input is selected."]
    VAL21 = 0x15,
    #[doc = "TRIG_IN5 input is selected."]
    VAL22 = 0x16,
    #[doc = "TRIG_IN6 input is selected."]
    VAL23 = 0x17,
    #[doc = "TRIG_IN7 input is selected."]
    VAL24 = 0x18,
    #[doc = "TRIG_IN8 input is selected."]
    VAL25 = 0x19,
    #[doc = "TRIG_IN9 input is selected."]
    VAL26 = 0x1a,
    #[doc = "TRIG_IN10 input is selected."]
    VAL27 = 0x1b,
    #[doc = "TRIG_IN11 input is selected."]
    VAL28 = 0x1c,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    VAL29 = 0x1d,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    VAL30 = 0x1e,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    VAL31 = 0x1f,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    VAL32 = 0x20,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    VAL33 = 0x21,
    #[doc = "WUU selected."]
    VAL34 = 0x22,
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected."]
    VAL35 = 0x23,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL36 = 0x24,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL37 = 0x25,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL38 = 0x26,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL39 = 0x27,
    #[doc = "CTimer3_MAT2 input is selected."]
    VAL40 = 0x28,
    #[doc = "CTimer3_MAT3 input is selected."]
    VAL41 = 0x29,
    #[doc = "CTimer4_MAT2 input is selected."]
    VAL42 = 0x2a,
    #[doc = "CTimer4_MAT3 input is selected."]
    VAL43 = 0x2b,
    #[doc = "FlexIO CH0 input is selected."]
    VAL44 = 0x2c,
    #[doc = "FlexIO CH1 input is selected."]
    VAL45 = 0x2d,
    #[doc = "FlexIO CH2 input is selected."]
    VAL46 = 0x2e,
    #[doc = "FlexIO CH3 input is selected."]
    VAL47 = 0x2f,
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
impl LpuartInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpuartInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpuartInp {
    #[inline(always)]
    fn from(val: u8) -> LpuartInp {
        LpuartInp::from_bits(val)
    }
}
impl From<LpuartInp> for u8 {
    #[inline(always)]
    fn from(val: LpuartInp) -> u8 {
        LpuartInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwm0extClkTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "clk_16k\\[1\\] input is selected."]
    VAL1 = 0x01,
    #[doc = "clk_in input is selected."]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL4 = 0x04,
    #[doc = "EXTTRIG_IN0 input is selected."]
    VAL5 = 0x05,
    #[doc = "EXTTRIG_IN7 input is selected."]
    VAL6 = 0x06,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL7 = 0x07,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pwm0extClkTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwm0extClkTrigin {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwm0extClkTrigin {
    #[inline(always)]
    fn from(val: u8) -> Pwm0extClkTrigin {
        Pwm0extClkTrigin::from_bits(val)
    }
}
impl From<Pwm0extClkTrigin> for u8 {
    #[inline(always)]
    fn from(val: Pwm0extClkTrigin) -> u8 {
        Pwm0extClkTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwm1extClkTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "clk_16k\\[1\\] input is selected."]
    VAL1 = 0x01,
    #[doc = "clk_in input is selected."]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL4 = 0x04,
    #[doc = "EXTTRIG_IN0 input is selected."]
    VAL5 = 0x05,
    #[doc = "EXTTRIG_IN7 input is selected."]
    VAL6 = 0x06,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL7 = 0x07,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pwm1extClkTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwm1extClkTrigin {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwm1extClkTrigin {
    #[inline(always)]
    fn from(val: u8) -> Pwm1extClkTrigin {
        Pwm1extClkTrigin::from_bits(val)
    }
}
impl From<Pwm1extClkTrigin> for u8 {
    #[inline(always)]
    fn from(val: Pwm1extClkTrigin) -> u8 {
        Pwm1extClkTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc0phaseaInp {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    VAL1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    VAL6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    VAL7 = 0x07,
    #[doc = "CMP2_OUT input is selected."]
    VAL8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    VAL9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    VAL10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    VAL14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    VAL16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    VAL17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    VAL18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    VAL19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    VAL20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    VAL21 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    VAL24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    VAL25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    VAL26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    VAL27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    VAL28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    VAL29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    VAL30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    VAL31 = 0x1f,
    #[doc = "TRIG_IN8 input is selected."]
    VAL32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    VAL33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    VAL34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    VAL35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    VAL36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    VAL37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    VAL38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    VAL39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    VAL40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    VAL49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    VAL50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    VAL51 = 0x33,
    #[doc = "CTimer4_MAT3 input is selected."]
    VAL52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    VAL62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    VAL63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    VAL64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    VAL65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    VAL66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
    VAL67 = 0x43,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    VAL68 = 0x44,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    VAL69 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Qdc0phaseaInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc0phaseaInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc0phaseaInp {
    #[inline(always)]
    fn from(val: u8) -> Qdc0phaseaInp {
        Qdc0phaseaInp::from_bits(val)
    }
}
impl From<Qdc0phaseaInp> for u8 {
    #[inline(always)]
    fn from(val: Qdc0phaseaInp) -> u8 {
        Qdc0phaseaInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc0phasebInp {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    VAL1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    VAL6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    VAL7 = 0x07,
    #[doc = "CMP2_OUT input is selected."]
    VAL8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    VAL9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    VAL10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    VAL14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    VAL16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    VAL17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    VAL18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    VAL19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    VAL20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    VAL21 = 0x15,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    VAL22 = 0x16,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected."]
    VAL23 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    VAL24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    VAL25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    VAL26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    VAL27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    VAL28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    VAL29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    VAL30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    VAL31 = 0x1f,
    #[doc = "TRIG_IN8 input is selected."]
    VAL32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    VAL33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    VAL34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    VAL35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    VAL36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    VAL37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    VAL38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    VAL39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    VAL40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    VAL49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    VAL50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    VAL51 = 0x33,
    #[doc = "CTimer4_MAT3 input is selected."]
    VAL52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    VAL62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    VAL63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    VAL64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    VAL65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    VAL66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
    VAL67 = 0x43,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    VAL68 = 0x44,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    VAL69 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Qdc0phasebInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc0phasebInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc0phasebInp {
    #[inline(always)]
    fn from(val: u8) -> Qdc0phasebInp {
        Qdc0phasebInp::from_bits(val)
    }
}
impl From<Qdc0phasebInp> for u8 {
    #[inline(always)]
    fn from(val: Qdc0phasebInp) -> u8 {
        Qdc0phasebInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc1phaseaInp {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    VAL1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    VAL6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    VAL7 = 0x07,
    #[doc = "CMP2_OUT input is selected."]
    VAL8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    VAL9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    VAL10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    VAL14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    VAL16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    VAL17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    VAL18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    VAL19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    VAL20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    VAL21 = 0x15,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    VAL22 = 0x16,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected."]
    VAL23 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    VAL24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    VAL25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    VAL26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    VAL27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    VAL28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    VAL29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    VAL30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    VAL31 = 0x1f,
    #[doc = "TRIG_IN8 input is selected."]
    VAL32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    VAL33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    VAL34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    VAL35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    VAL36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    VAL37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    VAL38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    VAL39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    VAL40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    VAL49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    VAL50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    VAL51 = 0x33,
    #[doc = "CTimer4_MAT3 input is selected."]
    VAL52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    VAL62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    VAL63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    VAL64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    VAL65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    VAL66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
    VAL67 = 0x43,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    VAL68 = 0x44,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    VAL69 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Qdc1phaseaInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc1phaseaInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc1phaseaInp {
    #[inline(always)]
    fn from(val: u8) -> Qdc1phaseaInp {
        Qdc1phaseaInp::from_bits(val)
    }
}
impl From<Qdc1phaseaInp> for u8 {
    #[inline(always)]
    fn from(val: Qdc1phaseaInp) -> u8 {
        Qdc1phaseaInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc1phasebInp {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    VAL1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    VAL6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    VAL7 = 0x07,
    #[doc = "CMP2_OUT input is selected."]
    VAL8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    VAL9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    VAL10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    VAL14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    VAL16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    VAL17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    VAL18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    VAL19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    VAL20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    VAL21 = 0x15,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    VAL22 = 0x16,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected."]
    VAL23 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    VAL24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    VAL25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    VAL26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    VAL27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    VAL28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    VAL29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    VAL30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    VAL31 = 0x1f,
    #[doc = "TRIG_IN8 input is selected."]
    VAL32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    VAL33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    VAL34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    VAL35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    VAL36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    VAL37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    VAL38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    VAL39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    VAL40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    VAL49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    VAL50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    VAL51 = 0x33,
    #[doc = "CTimer4_MAT3 input is selected."]
    VAL52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    VAL62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    VAL63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    VAL64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    VAL65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    VAL66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
    VAL67 = 0x43,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    VAL68 = 0x44,
    #[doc = "PWM1_SM3_MUX_TRIG1 inout is selected."]
    VAL69 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Qdc1phasebInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc1phasebInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc1phasebInp {
    #[inline(always)]
    fn from(val: u8) -> Qdc1phasebInp {
        Qdc1phasebInp::from_bits(val)
    }
}
impl From<Qdc1phasebInp> for u8 {
    #[inline(always)]
    fn from(val: Qdc1phasebInp) -> u8 {
        Qdc1phasebInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum QdcHomeInp {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    VAL1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    VAL6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    VAL7 = 0x07,
    #[doc = "CMP2_OUT input is selected."]
    VAL8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    VAL9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    VAL10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    VAL14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    VAL16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    VAL17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    VAL18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    VAL19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    VAL20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    VAL21 = 0x15,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    VAL22 = 0x16,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected."]
    VAL23 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    VAL24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    VAL25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    VAL26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    VAL27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    VAL28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    VAL29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    VAL30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    VAL31 = 0x1f,
    #[doc = "TRIG_IN8 input is selected."]
    VAL32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    VAL33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    VAL34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    VAL35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    VAL36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    VAL37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    VAL38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    VAL39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    VAL40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    VAL49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    VAL50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    VAL51 = 0x33,
    #[doc = "CTimer4_MAT3 input is selected."]
    VAL52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    VAL62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    VAL63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    VAL64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    VAL65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    VAL66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
    VAL67 = 0x43,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    VAL68 = 0x44,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    VAL69 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl QdcHomeInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> QdcHomeInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for QdcHomeInp {
    #[inline(always)]
    fn from(val: u8) -> QdcHomeInp {
        QdcHomeInp::from_bits(val)
    }
}
impl From<QdcHomeInp> for u8 {
    #[inline(always)]
    fn from(val: QdcHomeInp) -> u8 {
        QdcHomeInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum QdcIcapInp {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    VAL1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    VAL6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    VAL7 = 0x07,
    #[doc = "CMP2_OUT input is selected."]
    VAL8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    VAL9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    VAL10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    VAL14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    VAL16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    VAL17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    VAL18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    VAL19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    VAL20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    VAL21 = 0x15,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    VAL22 = 0x16,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected."]
    VAL23 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    VAL24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    VAL25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    VAL26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    VAL27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    VAL28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    VAL29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    VAL30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    VAL31 = 0x1f,
    #[doc = "TRIG_IN8 input is selected."]
    VAL32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    VAL33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    VAL34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    VAL35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    VAL36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    VAL37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    VAL38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    VAL39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    VAL40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    VAL49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    VAL50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    VAL51 = 0x33,
    #[doc = "CTimer4_MAT3 input is selected."]
    VAL52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    VAL62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    VAL63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    VAL64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    VAL65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    VAL66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
    VAL67 = 0x43,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    VAL68 = 0x44,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    VAL69 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl QdcIcapInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> QdcIcapInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for QdcIcapInp {
    #[inline(always)]
    fn from(val: u8) -> QdcIcapInp {
        QdcIcapInp::from_bits(val)
    }
}
impl From<QdcIcapInp> for u8 {
    #[inline(always)]
    fn from(val: QdcIcapInp) -> u8 {
        QdcIcapInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum QdcIndexInp {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    VAL1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    VAL6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    VAL7 = 0x07,
    #[doc = "CMP2_OUT input is selected."]
    VAL8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    VAL9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    VAL10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    VAL14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    VAL16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    VAL17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    VAL18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    VAL19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    VAL20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    VAL21 = 0x15,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    VAL22 = 0x16,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected."]
    VAL23 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    VAL24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    VAL25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    VAL26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    VAL27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    VAL28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    VAL29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    VAL30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    VAL31 = 0x1f,
    #[doc = "TRIG_IN8 input is selected."]
    VAL32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    VAL33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    VAL34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    VAL35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    VAL36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    VAL37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    VAL38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    VAL39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    VAL40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    VAL49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    VAL50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    VAL51 = 0x33,
    #[doc = "CTimer4_MAT3 input is selected."]
    VAL52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    VAL62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    VAL63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    VAL64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    VAL65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    VAL66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
    VAL67 = 0x43,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    VAL68 = 0x44,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    VAL69 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl QdcIndexInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> QdcIndexInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for QdcIndexInp {
    #[inline(always)]
    fn from(val: u8) -> QdcIndexInp {
        QdcIndexInp::from_bits(val)
    }
}
impl From<QdcIndexInp> for u8 {
    #[inline(always)]
    fn from(val: QdcIndexInp) -> u8 {
        QdcIndexInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum QdcTrigInp {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    VAL1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    VAL6 = 0x06,
    #[doc = "CMP1_OUT input is selected."]
    VAL7 = 0x07,
    #[doc = "CMP2_OUT input is selected."]
    VAL8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    VAL9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    VAL10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    VAL14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    VAL16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    VAL17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    VAL18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    VAL19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    VAL20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    VAL21 = 0x15,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    VAL22 = 0x16,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected."]
    VAL23 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    VAL24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    VAL25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    VAL26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    VAL27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    VAL28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    VAL29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    VAL30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    VAL31 = 0x1f,
    #[doc = "TRIG_IN8 input is selected."]
    VAL32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    VAL33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    VAL34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    VAL35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    VAL36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    VAL37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    VAL38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    VAL39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    VAL40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    VAL49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    VAL50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    VAL51 = 0x33,
    #[doc = "CTimer4_MAT3 input is selected."]
    VAL52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    VAL62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    VAL63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    VAL64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    VAL65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    VAL66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
    VAL67 = 0x43,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    VAL68 = 0x44,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    VAL69 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl QdcTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> QdcTrigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for QdcTrigInp {
    #[inline(always)]
    fn from(val: u8) -> QdcTrigInp {
        QdcTrigInp::from_bits(val)
    }
}
impl From<QdcTrigInp> for u8 {
    #[inline(always)]
    fn from(val: QdcTrigInp) -> u8 {
        QdcTrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmartDmaTrigInp {
    _RESERVED_0 = 0x0,
    #[doc = "GPIO P0_16 input is selected."]
    VAL1 = 0x01,
    #[doc = "GPIO P0_17 input is selected."]
    VAL2 = 0x02,
    #[doc = "GPIO P1_8 input is selected."]
    VAL3 = 0x03,
    #[doc = "GPIO P1_9 input is selected."]
    VAL4 = 0x04,
    #[doc = "GPIO P1_10 input is selected."]
    VAL5 = 0x05,
    #[doc = "GPIO P1_11 input is selected."]
    VAL6 = 0x06,
    #[doc = "GPIO P1_12 input is selected."]
    VAL7 = 0x07,
    #[doc = "GPIO P1_13 input is selected."]
    VAL8 = 0x08,
    #[doc = "GPIO P2_0 input is selected."]
    VAL9 = 0x09,
    #[doc = "GPIO P2_1 input is selected."]
    VAL10 = 0x0a,
    #[doc = "GPIO P2_2 input is selected."]
    VAL11 = 0x0b,
    #[doc = "GPIO P2_3 input is selected."]
    VAL12 = 0x0c,
    #[doc = "GPIO P2_6 input is selected."]
    VAL13 = 0x0d,
    #[doc = "GPIO P3_8 input is selected."]
    VAL14 = 0x0e,
    #[doc = "GPIO P3_9 input is selected."]
    VAL15 = 0x0f,
    #[doc = "GPIO P3_10 input is selected."]
    VAL16 = 0x10,
    #[doc = "GPIO P3_11 input is selected."]
    VAL17 = 0x11,
    #[doc = "GPIO P3_12 input is seclected."]
    VAL18 = 0x12,
    #[doc = "GPIO0 Pin Event Trig input is selected."]
    VAL19 = 0x13,
    #[doc = "GPIO1 Pin Event Trig input is selected."]
    VAL20 = 0x14,
    #[doc = "GPIO2 Pin Event Trig input is selected."]
    VAL21 = 0x15,
    #[doc = "GPIO3 Pin Event Trig input is selected."]
    VAL22 = 0x16,
    #[doc = "GPIO4 Pin Event Trig input is selected."]
    VAL23 = 0x17,
    #[doc = "ARM_TXEV input is selected."]
    VAL24 = 0x18,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL25 = 0x19,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL26 = 0x1a,
    #[doc = "DMA_IRQ input is selected."]
    VAL27 = 0x1b,
    #[doc = "MAU_IRQ input is selected."]
    VAL28 = 0x1c,
    #[doc = "WUU_IRQ input is selected."]
    VAL29 = 0x1d,
    #[doc = "CTimer0_MAT2 input is selected."]
    VAL30 = 0x1e,
    #[doc = "CTimer0_MAT3 input is selected."]
    VAL31 = 0x1f,
    #[doc = "CTimer1_MAT2 input is selected."]
    VAL32 = 0x20,
    #[doc = "CTimer1_MAT3 input is selected."]
    VAL33 = 0x21,
    #[doc = "CTimer2_MAT2 input is selected."]
    VAL34 = 0x22,
    #[doc = "CTimer2_MAT3 input is selected."]
    VAL35 = 0x23,
    #[doc = "CTimer3_MAT2 input is selected."]
    VAL36 = 0x24,
    #[doc = "CTimer3_MAT3 input is selected."]
    VAL37 = 0x25,
    #[doc = "CTimer4_MAT2 input is selected."]
    VAL38 = 0x26,
    #[doc = "CTimer4_MAT3 input is selected."]
    VAL39 = 0x27,
    #[doc = "OSTIMER_IRQ input is selected."]
    VAL40 = 0x28,
    #[doc = "PWM0_IRQ input is selected."]
    VAL41 = 0x29,
    #[doc = "PWM1_IRQ input is selected."]
    VAL42 = 0x2a,
    #[doc = "QDC0_IRQ input is selected."]
    VAL43 = 0x2b,
    #[doc = "QDC1_IRQ input is selected."]
    VAL44 = 0x2c,
    #[doc = "RTC_Alarm_IRQ input is selected."]
    VAL45 = 0x2d,
    #[doc = "RTC_1Hz_IRQ input is selected."]
    VAL46 = 0x2e,
    #[doc = "uTICK_IRQ input is selected."]
    VAL47 = 0x2f,
    #[doc = "WDT_IRQ input is selected."]
    VAL48 = 0x30,
    #[doc = "Wakeup_Timer_IRQ input is selected."]
    VAL49 = 0x31,
    #[doc = "CAN0_IRQ input is selected."]
    VAL50 = 0x32,
    #[doc = "CAN1_IRQ input is selected."]
    VAL51 = 0x33,
    #[doc = "FlexIO_IRQ input is selected."]
    VAL52 = 0x34,
    #[doc = "FlexIO_Shifer0_DMA_Req input is selected."]
    VAL53 = 0x35,
    #[doc = "FlexIO_Shifer1_DMA_Req input is selected."]
    VAL54 = 0x36,
    #[doc = "FlexIO_Shifer2_DMA_Req input is selected."]
    VAL55 = 0x37,
    #[doc = "FlexIO_Shifer3_DMA_Req input is selected."]
    VAL56 = 0x38,
    #[doc = "I3C0_IRQ input is selected."]
    VAL57 = 0x39,
    #[doc = "LPI2C0_IRQ input is selected."]
    VAL58 = 0x3a,
    #[doc = "LPI2C1_IRQ input is selected."]
    VAL59 = 0x3b,
    #[doc = "LPSPI0_IRQ input is selected."]
    VAL60 = 0x3c,
    #[doc = "LPSPI1_IRQ input is selected."]
    VAL61 = 0x3d,
    #[doc = "LPUART0_IRQ input is selected."]
    VAL62 = 0x3e,
    #[doc = "LPUART1_IRQ input is selected."]
    VAL63 = 0x3f,
    #[doc = "LPUART2_IRQ input is selected."]
    VAL64 = 0x40,
    #[doc = "LPUART3_IRQ input is selected."]
    VAL65 = 0x41,
    #[doc = "USB0_SOF input is selected."]
    VAL66 = 0x42,
    _RESERVED_43 = 0x43,
    #[doc = "ADC0_IRQ input is selected."]
    VAL68 = 0x44,
    #[doc = "ADC1_IRQ input is selected."]
    VAL69 = 0x45,
    #[doc = "ADC2_IRQ input is selected."]
    VAL70 = 0x46,
    #[doc = "ADC3_IRQ input is selected."]
    VAL71 = 0x47,
    #[doc = "CMP0_IRQ input is selected."]
    VAL72 = 0x48,
    #[doc = "CMP1_IRQ input is selected."]
    VAL73 = 0x49,
    #[doc = "CMP2_IRQ input is selected."]
    VAL74 = 0x4a,
    #[doc = "CMP0_OUT input is selected."]
    VAL75 = 0x4b,
    #[doc = "CMP1_OUT input is selected."]
    VAL76 = 0x4c,
    #[doc = "CMP2_OUT input is selected."]
    VAL77 = 0x4d,
    #[doc = "DAC0_IRQ input is selected."]
    VAL78 = 0x4e,
    #[doc = "SLCD_IRQ input is selected."]
    VAL79 = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl SmartDmaTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmartDmaTrigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmartDmaTrigInp {
    #[inline(always)]
    fn from(val: u8) -> SmartDmaTrigInp {
        SmartDmaTrigInp::from_bits(val)
    }
}
impl From<SmartDmaTrigInp> for u8 {
    #[inline(always)]
    fn from(val: SmartDmaTrigInp) -> u8 {
        SmartDmaTrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer0trigInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_INP0 input is selected."]
    VAL1 = 0x01,
    #[doc = "CT_INP1 input is selected."]
    VAL2 = 0x02,
    #[doc = "CT_INP2 input is selected."]
    VAL3 = 0x03,
    #[doc = "CT_INP3 input is selected."]
    VAL4 = 0x04,
    #[doc = "CT_INP4 input is selected."]
    VAL5 = 0x05,
    #[doc = "CT_INP5 input is selected."]
    VAL6 = 0x06,
    #[doc = "CT_INP6 input is selected."]
    VAL7 = 0x07,
    #[doc = "CT_INP7 input is selected."]
    VAL8 = 0x08,
    #[doc = "CT_INP8 input is selected."]
    VAL9 = 0x09,
    #[doc = "CT_INP9 input is selected."]
    VAL10 = 0x0a,
    #[doc = "CT_INP10 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CT_INP11 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CT_INP12 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CT_INP13 input is selected."]
    VAL14 = 0x0e,
    #[doc = "CT_INP14 input is selected."]
    VAL15 = 0x0f,
    #[doc = "CT_INP15 input is selected."]
    VAL16 = 0x10,
    #[doc = "CT_INP16 input is selected."]
    VAL17 = 0x11,
    #[doc = "CT_INP17 input is selected."]
    VAL18 = 0x12,
    #[doc = "CT_INP18 input is selected."]
    VAL19 = 0x13,
    #[doc = "CT_INP19 input is selected."]
    VAL20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    VAL21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    VAL26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    VAL27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    VAL28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    VAL29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    VAL30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    VAL31 = 0x1f,
    #[doc = "CMP2_OUT is selected."]
    VAL32 = 0x20,
    #[doc = "CTimer1_MAT1 input is selected."]
    VAL33 = 0x21,
    #[doc = "CTimer1_MAT2 input is selected."]
    VAL34 = 0x22,
    #[doc = "CTimer1_MAT3 input is selected."]
    VAL35 = 0x23,
    #[doc = "CTimer2_MAT1 input is selected."]
    VAL36 = 0x24,
    #[doc = "CTimer2_MAT2 input is selected."]
    VAL37 = 0x25,
    #[doc = "CTimer2_MAT3 input is selected."]
    VAL38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    VAL39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    VAL40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    VAL41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    VAL42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    VAL43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    VAL44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    VAL45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    VAL46 = 0x2e,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    VAL47 = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    VAL48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    VAL49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    VAL50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    VAL51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    VAL52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    VAL53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    VAL54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    VAL55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    VAL56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    VAL57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    VAL58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    VAL59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    VAL60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    VAL61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    VAL62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    VAL63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    VAL64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    VAL65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    VAL66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    VAL67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    VAL68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    VAL69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    VAL70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    VAL75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    VAL76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    VAL77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    VAL78 = 0x4e,
    #[doc = "CTimer3_MAT1 input is selected."]
    VAL79 = 0x4f,
    #[doc = "CTimer3_MAT2 input is selected."]
    VAL80 = 0x50,
    #[doc = "CTimer3_MAT3 input is selected."]
    VAL81 = 0x51,
    #[doc = "CTimer4_MAT1 input is selected."]
    VAL82 = 0x52,
    #[doc = "CTimer4_MAT2 input is selected."]
    VAL83 = 0x53,
    #[doc = "CTimer4_MAT3 input is selected."]
    VAL84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    VAL85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    VAL86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    VAL87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    VAL88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    VAL89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    VAL90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    VAL91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    VAL92 = 0x5c,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    VAL93 = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    VAL94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    VAL95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    VAL96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    VAL97 = 0x61,
    #[doc = "LPUART5 Received Data Word input is selected."]
    VAL98 = 0x62,
    #[doc = "LPUART5 Transmitted Data Word input is selected."]
    VAL99 = 0x63,
    #[doc = "LPUART5 Receive Line Idle input is selected."]
    VAL100 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    #[doc = "ADC2_tcomp\\[0\\] input is selected."]
    VAL105 = 0x69,
    #[doc = "ADC2_tcomp\\[1\\] input is selected."]
    VAL106 = 0x6a,
    #[doc = "ADC2_tcomp\\[2\\] input is selected."]
    VAL107 = 0x6b,
    #[doc = "ADC2_tcomp\\[3\\] input is selected."]
    VAL108 = 0x6c,
    #[doc = "ADC3_tcomp\\[0\\] input is selected."]
    VAL109 = 0x6d,
    #[doc = "ADC3_tcomp\\[1\\] input is selected."]
    VAL110 = 0x6e,
    #[doc = "ADC3_tcomp\\[2\\] input is selected."]
    VAL111 = 0x6f,
    #[doc = "ADC3_tcomp\\[3\\] input is selected."]
    VAL112 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Timer0trigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer0trigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer0trigInp {
    #[inline(always)]
    fn from(val: u8) -> Timer0trigInp {
        Timer0trigInp::from_bits(val)
    }
}
impl From<Timer0trigInp> for u8 {
    #[inline(always)]
    fn from(val: Timer0trigInp) -> u8 {
        Timer0trigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer1trigInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_INP0 input is selected."]
    VAL1 = 0x01,
    #[doc = "CT_INP1 input is selected."]
    VAL2 = 0x02,
    #[doc = "CT_INP2 input is selected."]
    VAL3 = 0x03,
    #[doc = "CT_INP3 input is selected."]
    VAL4 = 0x04,
    #[doc = "CT_INP4 input is selected."]
    VAL5 = 0x05,
    #[doc = "CT_INP5 input is selected."]
    VAL6 = 0x06,
    #[doc = "CT_INP6 input is selected."]
    VAL7 = 0x07,
    #[doc = "CT_INP7 input is selected."]
    VAL8 = 0x08,
    #[doc = "CT_INP8 input is selected."]
    VAL9 = 0x09,
    #[doc = "CT_INP9 input is selected."]
    VAL10 = 0x0a,
    #[doc = "CT_INP10 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CT_INP11 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CT_INP12 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CT_INP13 input is selected."]
    VAL14 = 0x0e,
    #[doc = "CT_INP14 input is selected."]
    VAL15 = 0x0f,
    #[doc = "CT_INP15 input is selected."]
    VAL16 = 0x10,
    #[doc = "CT_INP16 input is selected."]
    VAL17 = 0x11,
    #[doc = "CT_INP17 input is selected."]
    VAL18 = 0x12,
    #[doc = "CT_INP18 input is selected."]
    VAL19 = 0x13,
    #[doc = "CT_INP19 input is selected."]
    VAL20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    VAL21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    VAL26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    VAL27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    VAL28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    VAL29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    VAL30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    VAL31 = 0x1f,
    #[doc = "CMP2_OUT is selected."]
    VAL32 = 0x20,
    #[doc = "CTimer0_MAT1 input is selected."]
    VAL33 = 0x21,
    #[doc = "CTimer0_MAT2 input is selected."]
    VAL34 = 0x22,
    #[doc = "CTimer0_MAT3 input is selected."]
    VAL35 = 0x23,
    #[doc = "CTimer2_MAT1 input is selected."]
    VAL36 = 0x24,
    #[doc = "CTimer2_MAT2 input is selected."]
    VAL37 = 0x25,
    #[doc = "CTimer2_MAT3 input is selected."]
    VAL38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    VAL39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    VAL40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    VAL41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    VAL42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    VAL43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    VAL44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    VAL45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    VAL46 = 0x2e,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    VAL47 = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    VAL48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    VAL49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    VAL50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    VAL51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    VAL52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    VAL53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    VAL54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    VAL55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    VAL56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    VAL57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    VAL58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    VAL59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    VAL60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    VAL61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    VAL62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    VAL63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    VAL64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    VAL65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    VAL66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    VAL67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    VAL68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    VAL69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    VAL70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    VAL75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    VAL76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    VAL77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    VAL78 = 0x4e,
    #[doc = "CTimer3_MAT1 input is selected."]
    VAL79 = 0x4f,
    #[doc = "CTimer3_MAT2 input is selected."]
    VAL80 = 0x50,
    #[doc = "CTimer3_MAT3 input is selected."]
    VAL81 = 0x51,
    #[doc = "CTimer4_MAT1 input is selected."]
    VAL82 = 0x52,
    #[doc = "CTimer4_MAT2 input is selected."]
    VAL83 = 0x53,
    #[doc = "CTimer4_MAT3 input is selected."]
    VAL84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    VAL85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    VAL86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    VAL87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    VAL88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    VAL89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    VAL90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    VAL91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    VAL92 = 0x5c,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    VAL93 = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    VAL94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    VAL95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    VAL96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    VAL97 = 0x61,
    #[doc = "LPUART5 Received Data Word input is selected."]
    VAL98 = 0x62,
    #[doc = "LPUART5 Transmitted Data Word input is selected."]
    VAL99 = 0x63,
    #[doc = "LPUART5 Receive Line Idle input is selected."]
    VAL100 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    #[doc = "ADC2_tcomp\\[0\\] input is selected."]
    VAL105 = 0x69,
    #[doc = "ADC2_tcomp\\[1\\] input is selected."]
    VAL106 = 0x6a,
    #[doc = "ADC2_tcomp\\[2\\] input is selected."]
    VAL107 = 0x6b,
    #[doc = "ADC2_tcomp\\[3\\] input is selected."]
    VAL108 = 0x6c,
    #[doc = "ADC3_tcomp\\[0\\] input is selected."]
    VAL109 = 0x6d,
    #[doc = "ADC3_tcomp\\[1\\] input is selected."]
    VAL110 = 0x6e,
    #[doc = "ADC3_tcomp\\[2\\] input is selected."]
    VAL111 = 0x6f,
    #[doc = "ADC3_tcomp\\[3\\] input is selected."]
    VAL112 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Timer1trigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer1trigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer1trigInp {
    #[inline(always)]
    fn from(val: u8) -> Timer1trigInp {
        Timer1trigInp::from_bits(val)
    }
}
impl From<Timer1trigInp> for u8 {
    #[inline(always)]
    fn from(val: Timer1trigInp) -> u8 {
        Timer1trigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer2trigInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_INP0 input is selected."]
    VAL1 = 0x01,
    #[doc = "CT_INP1 input is selected."]
    VAL2 = 0x02,
    #[doc = "CT_INP2 input is selected."]
    VAL3 = 0x03,
    #[doc = "CT_INP3 input is selected."]
    VAL4 = 0x04,
    #[doc = "CT_INP4 input is selected."]
    VAL5 = 0x05,
    #[doc = "CT_INP5 input is selected."]
    VAL6 = 0x06,
    #[doc = "CT_INP6 input is selected."]
    VAL7 = 0x07,
    #[doc = "CT_INP7 input is selected."]
    VAL8 = 0x08,
    #[doc = "CT_INP8 input is selected."]
    VAL9 = 0x09,
    #[doc = "CT_INP9 input is selected."]
    VAL10 = 0x0a,
    #[doc = "CT_INP10 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CT_INP11 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CT_INP12 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CT_INP13 input is selected."]
    VAL14 = 0x0e,
    #[doc = "CT_INP14 input is selected."]
    VAL15 = 0x0f,
    #[doc = "CT_INP15 input is selected."]
    VAL16 = 0x10,
    #[doc = "CT_INP16 input is selected."]
    VAL17 = 0x11,
    #[doc = "CT_INP17 input is selected."]
    VAL18 = 0x12,
    #[doc = "CT_INP18 input is selected."]
    VAL19 = 0x13,
    #[doc = "CT_INP19 input is selected."]
    VAL20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    VAL21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    VAL26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    VAL27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    VAL28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    VAL29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    VAL30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    VAL31 = 0x1f,
    #[doc = "CMP2_OUT is selected."]
    VAL32 = 0x20,
    #[doc = "CTimer0_MAT1 input is selected."]
    VAL33 = 0x21,
    #[doc = "CTimer0_MAT2 input is selected."]
    VAL34 = 0x22,
    #[doc = "CTimer0_MAT3 input is selected."]
    VAL35 = 0x23,
    #[doc = "CTimer1_MAT1 input is selected."]
    VAL36 = 0x24,
    #[doc = "CTimer1_MAT2 input is selected."]
    VAL37 = 0x25,
    #[doc = "CTimer1_MAT3 input is selected."]
    VAL38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    VAL39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    VAL40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    VAL41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    VAL42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    VAL43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    VAL44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    VAL45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    VAL46 = 0x2e,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    VAL47 = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    VAL48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    VAL49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    VAL50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    VAL51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    VAL52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    VAL53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    VAL54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    VAL55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    VAL56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    VAL57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    VAL58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    VAL59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    VAL60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    VAL61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    VAL62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    VAL63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    VAL64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    VAL65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    VAL66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    VAL67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    VAL68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    VAL69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    VAL70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    VAL75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    VAL76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    VAL77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    VAL78 = 0x4e,
    #[doc = "CTimer3_MAT1 input is selected."]
    VAL79 = 0x4f,
    #[doc = "CTimer3_MAT2 input is selected."]
    VAL80 = 0x50,
    #[doc = "CTimer3_MAT3 input is selected."]
    VAL81 = 0x51,
    #[doc = "CTimer4_MAT1 input is selected."]
    VAL82 = 0x52,
    #[doc = "CTimer4_MAT2 input is selected."]
    VAL83 = 0x53,
    #[doc = "CTimer4_MAT3 input is selected."]
    VAL84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    VAL85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    VAL86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    VAL87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    VAL88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    VAL89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    VAL90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    VAL91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    VAL92 = 0x5c,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    VAL93 = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    VAL94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    VAL95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    VAL96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    VAL97 = 0x61,
    #[doc = "LPUART5 Received Data Word input is selected."]
    VAL98 = 0x62,
    #[doc = "LPUART5 Transmitted Data Word input is selected."]
    VAL99 = 0x63,
    #[doc = "LPUART5 Receive Line Idle input is selected."]
    VAL100 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    #[doc = "ADC2_tcomp\\[0\\] input is selected."]
    VAL105 = 0x69,
    #[doc = "ADC2_tcomp\\[1\\] input is selected."]
    VAL106 = 0x6a,
    #[doc = "ADC2_tcomp\\[2\\] input is selected."]
    VAL107 = 0x6b,
    #[doc = "ADC2_tcomp\\[3\\] input is selected."]
    VAL108 = 0x6c,
    #[doc = "ADC3_tcomp\\[0\\] input is selected."]
    VAL109 = 0x6d,
    #[doc = "ADC3_tcomp\\[1\\] input is selected."]
    VAL110 = 0x6e,
    #[doc = "ADC3_tcomp\\[2\\] input is selected."]
    VAL111 = 0x6f,
    #[doc = "ADC3_tcomp\\[3\\] input is selected."]
    VAL112 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Timer2trigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer2trigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer2trigInp {
    #[inline(always)]
    fn from(val: u8) -> Timer2trigInp {
        Timer2trigInp::from_bits(val)
    }
}
impl From<Timer2trigInp> for u8 {
    #[inline(always)]
    fn from(val: Timer2trigInp) -> u8 {
        Timer2trigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer3trigInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_INP0 input is selected."]
    VAL1 = 0x01,
    #[doc = "CT_INP1 input is selected."]
    VAL2 = 0x02,
    #[doc = "CT_INP2 input is selected."]
    VAL3 = 0x03,
    #[doc = "CT_INP3 input is selected."]
    VAL4 = 0x04,
    #[doc = "CT_INP4 input is selected."]
    VAL5 = 0x05,
    #[doc = "CT_INP5 input is selected."]
    VAL6 = 0x06,
    #[doc = "CT_INP6 input is selected."]
    VAL7 = 0x07,
    #[doc = "CT_INP7 input is selected."]
    VAL8 = 0x08,
    #[doc = "CT_INP8 input is selected."]
    VAL9 = 0x09,
    #[doc = "CT_INP9 input is selected."]
    VAL10 = 0x0a,
    #[doc = "CT_INP10 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CT_INP11 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CT_INP12 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CT_INP13 input is selected."]
    VAL14 = 0x0e,
    #[doc = "CT_INP14 input is selected."]
    VAL15 = 0x0f,
    #[doc = "CT_INP15 input is selected."]
    VAL16 = 0x10,
    #[doc = "CT_INP16 input is selected."]
    VAL17 = 0x11,
    #[doc = "CT_INP17 input is selected."]
    VAL18 = 0x12,
    #[doc = "CT_INP18 input is selected."]
    VAL19 = 0x13,
    #[doc = "CT_INP19 input is selected."]
    VAL20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    VAL21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    VAL26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    VAL27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    VAL28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    VAL29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    VAL30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    VAL31 = 0x1f,
    #[doc = "CMP2_OUT is selected."]
    VAL32 = 0x20,
    #[doc = "CTimer0_MAT1 input is selected."]
    VAL33 = 0x21,
    #[doc = "CTimer0_MAT2 input is selected."]
    VAL34 = 0x22,
    #[doc = "CTimer0_MAT3 input is selected."]
    VAL35 = 0x23,
    #[doc = "CTimer1_MAT1 input is selected."]
    VAL36 = 0x24,
    #[doc = "CTimer1_MAT2 input is selected."]
    VAL37 = 0x25,
    #[doc = "CTimer1_MAT3 input is selected."]
    VAL38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    VAL39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    VAL40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    VAL41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    VAL42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    VAL43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    VAL44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    VAL45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    VAL46 = 0x2e,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    VAL47 = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    VAL48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    VAL49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    VAL50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    VAL51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    VAL52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    VAL53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    VAL54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    VAL55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    VAL56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    VAL57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    VAL58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    VAL59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    VAL60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    VAL61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    VAL62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    VAL63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    VAL64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    VAL65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    VAL66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    VAL67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    VAL68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    VAL69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    VAL70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    VAL75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    VAL76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    VAL77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    VAL78 = 0x4e,
    #[doc = "CTimer2_MAT1 input is selected."]
    VAL79 = 0x4f,
    #[doc = "CTimer2_MAT2 input is selected."]
    VAL80 = 0x50,
    #[doc = "CTimer2_MAT3 input is selected."]
    VAL81 = 0x51,
    #[doc = "CTimer4_MAT1 input is selected."]
    VAL82 = 0x52,
    #[doc = "CTimer4_MAT2 input is selected."]
    VAL83 = 0x53,
    #[doc = "CTimer4_MAT3 input is selected."]
    VAL84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    VAL85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    VAL86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    VAL87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    VAL88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    VAL89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    VAL90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    VAL91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    VAL92 = 0x5c,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    VAL93 = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    VAL94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    VAL95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    VAL96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    VAL97 = 0x61,
    #[doc = "LPUART5 Received Data Word input is selected."]
    VAL98 = 0x62,
    #[doc = "LPUART5 Transmitted Data Word input is selected."]
    VAL99 = 0x63,
    #[doc = "LPUART5 Receive Line Idle input is selected."]
    VAL100 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    #[doc = "ADC2_tcomp\\[0\\] input is selected."]
    VAL105 = 0x69,
    #[doc = "ADC2_tcomp\\[1\\] input is selected."]
    VAL106 = 0x6a,
    #[doc = "ADC2_tcomp\\[2\\] input is selected."]
    VAL107 = 0x6b,
    #[doc = "ADC2_tcomp\\[3\\] input is selected."]
    VAL108 = 0x6c,
    #[doc = "ADC3_tcomp\\[0\\] input is selected."]
    VAL109 = 0x6d,
    #[doc = "ADC3_tcomp\\[1\\] input is selected."]
    VAL110 = 0x6e,
    #[doc = "ADC3_tcomp\\[2\\] input is selected."]
    VAL111 = 0x6f,
    #[doc = "ADC3_tcomp\\[3\\] input is selected."]
    VAL112 = 0x70,
    #[doc = "TRIG_IN0 input is selected."]
    VAL113 = 0x71,
    #[doc = "TRIG_IN1 input is selected."]
    VAL114 = 0x72,
    #[doc = "TRIG_IN2 input is selected."]
    VAL115 = 0x73,
    #[doc = "TRIG_IN3 input is selected."]
    VAL116 = 0x74,
    #[doc = "TRIG_IN4 input is selected."]
    VAL117 = 0x75,
    #[doc = "TRIG_IN5 input is selected."]
    VAL118 = 0x76,
    #[doc = "TRIG_IN6 input is selected."]
    VAL119 = 0x77,
    #[doc = "TRIG_IN7 input is selected."]
    VAL120 = 0x78,
    #[doc = "TRIG_IN8 input is selected."]
    VAL121 = 0x79,
    #[doc = "TRIG_IN9 input is selected."]
    VAL122 = 0x7a,
    #[doc = "TRIG_IN10 input is selected."]
    VAL123 = 0x7b,
    #[doc = "TRIG_IN11 input is selected."]
    VAL124 = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Timer3trigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer3trigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer3trigInp {
    #[inline(always)]
    fn from(val: u8) -> Timer3trigInp {
        Timer3trigInp::from_bits(val)
    }
}
impl From<Timer3trigInp> for u8 {
    #[inline(always)]
    fn from(val: Timer3trigInp) -> u8 {
        Timer3trigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer4trigInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_INP0 input is selected."]
    VAL1 = 0x01,
    #[doc = "CT_INP1 input is selected."]
    VAL2 = 0x02,
    #[doc = "CT_INP2 input is selected."]
    VAL3 = 0x03,
    #[doc = "CT_INP3 input is selected."]
    VAL4 = 0x04,
    #[doc = "CT_INP4 input is selected."]
    VAL5 = 0x05,
    #[doc = "CT_INP5 input is selected."]
    VAL6 = 0x06,
    #[doc = "CT_INP6 input is selected."]
    VAL7 = 0x07,
    #[doc = "CT_INP7 input is selected."]
    VAL8 = 0x08,
    #[doc = "CT_INP8 input is selected."]
    VAL9 = 0x09,
    #[doc = "CT_INP9 input is selected."]
    VAL10 = 0x0a,
    #[doc = "CT_INP10 input is selected."]
    VAL11 = 0x0b,
    #[doc = "CT_INP11 input is selected."]
    VAL12 = 0x0c,
    #[doc = "CT_INP12 input is selected."]
    VAL13 = 0x0d,
    #[doc = "CT_INP13 input is selected."]
    VAL14 = 0x0e,
    #[doc = "CT_INP14 input is selected."]
    VAL15 = 0x0f,
    #[doc = "CT_INP15 input is selected."]
    VAL16 = 0x10,
    #[doc = "CT_INP16 input is selected."]
    VAL17 = 0x11,
    #[doc = "CT_INP17 input is selected."]
    VAL18 = 0x12,
    #[doc = "CT_INP18 input is selected."]
    VAL19 = 0x13,
    #[doc = "CT_INP19 input is selected."]
    VAL20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    VAL21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    VAL22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    VAL23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    VAL24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    VAL25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    VAL26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    VAL27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    VAL28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    VAL29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    VAL30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    VAL31 = 0x1f,
    #[doc = "CMP2_OUT is selected."]
    VAL32 = 0x20,
    #[doc = "CTimer0_MAT1 input is selected."]
    VAL33 = 0x21,
    #[doc = "CTimer0_MAT2 input is selected."]
    VAL34 = 0x22,
    #[doc = "CTimer0_MAT3 input is selected."]
    VAL35 = 0x23,
    #[doc = "CTimer1_MAT1 input is selected."]
    VAL36 = 0x24,
    #[doc = "CTimer1_MAT2 input is selected."]
    VAL37 = 0x25,
    #[doc = "CTimer1_MAT3 input is selected."]
    VAL38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    VAL39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    VAL40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    VAL41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    VAL42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    VAL43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    VAL44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    VAL45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    VAL46 = 0x2e,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    VAL47 = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    VAL48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    VAL49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    VAL50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    VAL51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    VAL52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    VAL53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    VAL54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    VAL55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    VAL56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    VAL57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    VAL58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    VAL59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    VAL60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    VAL61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    VAL62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    VAL63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    VAL64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    VAL65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    VAL66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    VAL67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    VAL68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    VAL69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    VAL70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    VAL71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    VAL72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    VAL73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    VAL74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    VAL75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    VAL76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    VAL77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    VAL78 = 0x4e,
    #[doc = "CTimer2_MAT1 input is selected."]
    VAL79 = 0x4f,
    #[doc = "CTimer2_MAT2 input is selected."]
    VAL80 = 0x50,
    #[doc = "CTimer2_MAT3 input is selected."]
    VAL81 = 0x51,
    #[doc = "CTimer3_MAT1 input is selected."]
    VAL82 = 0x52,
    #[doc = "CTimer3_MAT2 input is selected."]
    VAL83 = 0x53,
    #[doc = "CTimer3_MAT3 input is selected."]
    VAL84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    VAL85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    VAL86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    VAL87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    VAL88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    VAL89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    VAL90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    VAL91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    VAL92 = 0x5c,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    VAL93 = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    VAL94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    VAL95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    VAL96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    VAL97 = 0x61,
    #[doc = "LPUART5 Received Data Word input is selected."]
    VAL98 = 0x62,
    #[doc = "LPUART5 Transmitted Data Word input is selected."]
    VAL99 = 0x63,
    #[doc = "LPUART5 Receive Line Idle input is selected."]
    VAL100 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    #[doc = "ADC2_tcomp\\[0\\] input is selected."]
    VAL105 = 0x69,
    #[doc = "ADC2_tcomp\\[1\\] input is selected."]
    VAL106 = 0x6a,
    #[doc = "ADC2_tcomp\\[2\\] input is selected."]
    VAL107 = 0x6b,
    #[doc = "ADC2_tcomp\\[3\\] input is selected."]
    VAL108 = 0x6c,
    #[doc = "ADC3_tcomp\\[0\\] input is selected."]
    VAL109 = 0x6d,
    #[doc = "ADC3_tcomp\\[1\\] input is selected."]
    VAL110 = 0x6e,
    #[doc = "ADC3_tcomp\\[2\\] input is selected."]
    VAL111 = 0x6f,
    #[doc = "ADC3_tcomp\\[3\\] input is selected."]
    VAL112 = 0x70,
    #[doc = "TRIG_IN0 input is selected."]
    VAL113 = 0x71,
    #[doc = "TRIG_IN1 input is selected."]
    VAL114 = 0x72,
    #[doc = "TRIG_IN2 input is selected."]
    VAL115 = 0x73,
    #[doc = "TRIG_IN3 input is selected."]
    VAL116 = 0x74,
    #[doc = "TRIG_IN4 input is selected."]
    VAL117 = 0x75,
    #[doc = "TRIG_IN5 input is selected."]
    VAL118 = 0x76,
    #[doc = "TRIG_IN6 input is selected."]
    VAL119 = 0x77,
    #[doc = "TRIG_IN7 input is selected."]
    VAL120 = 0x78,
    #[doc = "TRIG_IN8 input is selected."]
    VAL121 = 0x79,
    #[doc = "TRIG_IN9 input is selected."]
    VAL122 = 0x7a,
    #[doc = "TRIG_IN10 input is selected."]
    VAL123 = 0x7b,
    #[doc = "TRIG_IN11 input is selected."]
    VAL124 = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl Timer4trigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer4trigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer4trigInp {
    #[inline(always)]
    fn from(val: u8) -> Timer4trigInp {
        Timer4trigInp::from_bits(val)
    }
}
impl From<Timer4trigInp> for u8 {
    #[inline(always)]
    fn from(val: Timer4trigInp) -> u8 {
        Timer4trigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrigInVal {
    #[doc = "TRIG_IN0 is 0."]
    VAL0 = 0x0,
    #[doc = "TRIG_IN0 is 1."]
    VAL1 = 0x01,
}
impl TrigInVal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrigInVal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrigInVal {
    #[inline(always)]
    fn from(val: u8) -> TrigInVal {
        TrigInVal::from_bits(val)
    }
}
impl From<TrigInVal> for u8 {
    #[inline(always)]
    fn from(val: TrigInVal) -> u8 {
        TrigInVal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbfsTrigInp {
    _RESERVED_0 = 0x0,
    #[doc = "LPUART0 lpuart_trg_txdata input is selected."]
    VAL1 = 0x01,
    #[doc = "LPUART1 lpuart_trg_txdata input is selected."]
    VAL2 = 0x02,
    #[doc = "LPUART2 lpuart_trg_txdata input is selected."]
    VAL3 = 0x03,
    #[doc = "LPUART3 lpuart_trg_txdata input is selected."]
    VAL4 = 0x04,
    #[doc = "LPUART4 lpuart_trg_txdata input is selected."]
    VAL5 = 0x05,
    #[doc = "LPUART5 lpuart_trg_txdata input is selected."]
    VAL6 = 0x06,
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
impl UsbfsTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbfsTrigInp {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbfsTrigInp {
    #[inline(always)]
    fn from(val: u8) -> UsbfsTrigInp {
        UsbfsTrigInp::from_bits(val)
    }
}
impl From<UsbfsTrigInp> for u8 {
    #[inline(always)]
    fn from(val: UsbfsTrigInp) -> u8 {
        UsbfsTrigInp::to_bits(val)
    }
}
