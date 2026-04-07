#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (2fd28c5 2026-04-02))"]
#[doc = "SYSCON."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscon {
    ptr: *mut u8,
}
unsafe impl Send for Syscon {}
unsafe impl Sync for Syscon {}
impl Syscon {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "AHB Matrix Remap Control."]
    #[inline(always)]
    pub const fn remap(self) -> crate::pac::common::Reg<Remap, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "AHB Matrix Priority Control."]
    #[inline(always)]
    pub const fn ahbmatprio(self) -> crate::pac::common::Reg<Ahbmatprio, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "NMI Source Select."]
    #[inline(always)]
    pub const fn nmisrc(self) -> crate::pac::common::Reg<Nmisrc, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Protect Level Control."]
    #[inline(always)]
    pub const fn protlvl(self) -> crate::pac::common::Reg<Protlvl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Non-Secure CPU0 System Tick Calibration."]
    #[inline(always)]
    pub const fn cpu0nstckcal(
        self,
    ) -> crate::pac::common::Reg<Cpu0nstckcal, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "System Clock Divider."]
    #[inline(always)]
    pub const fn ahbclkdiv(self) -> crate::pac::common::Reg<Ahbclkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "BUS_CLK Clock Divider."]
    #[inline(always)]
    pub const fn busclkdiv(self) -> crate::pac::common::Reg<Busclkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "SLOW_CLK Clock Divider."]
    #[inline(always)]
    pub const fn slowclkdiv(self) -> crate::pac::common::Reg<Slowclkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "FRO_HF_DIV Clock Divider."]
    #[inline(always)]
    pub const fn frohfdiv(self) -> crate::pac::common::Reg<Frohfdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "FRO_LF_DIV Clock Divider."]
    #[inline(always)]
    pub const fn frolfdiv(self) -> crate::pac::common::Reg<Frolfdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "PLL1_CLK_DIV Clock Divider."]
    #[inline(always)]
    pub const fn pll1clkdiv(self) -> crate::pac::common::Reg<Pll1clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Clock Configuration Unlock."]
    #[inline(always)]
    pub const fn clkunlock(self) -> crate::pac::common::Reg<Clkunlock, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Gray to Binary Converter Gray Code \\[31:0\\]."]
    #[inline(always)]
    pub const fn gray_code_lsb(
        self,
    ) -> crate::pac::common::Reg<GrayCodeLsb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Gray to Binary Converter Gray Code \\[41:32\\]."]
    #[inline(always)]
    pub const fn gray_code_msb(
        self,
    ) -> crate::pac::common::Reg<GrayCodeMsb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Gray to Binary Converter Binary Code \\[31:0\\]."]
    #[inline(always)]
    pub const fn binary_code_lsb(
        self,
    ) -> crate::pac::common::Reg<BinaryCodeLsb, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "Gray to Binary Converter Binary Code \\[41:32\\]."]
    #[inline(always)]
    pub const fn binary_code_msb(
        self,
    ) -> crate::pac::common::Reg<BinaryCodeMsb, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "Ethernet Control."]
    #[inline(always)]
    pub const fn enet_ctrl(self) -> crate::pac::common::Reg<EnetCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "Sideband Flow Control."]
    #[inline(always)]
    pub const fn enet_sbd_flow_ctrl(
        self,
    ) -> crate::pac::common::Reg<EnetSbdFlowCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "NVM Control."]
    #[inline(always)]
    pub const fn nvm_ctrl(self) -> crate::pac::common::Reg<NvmCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "SmartDMA Interrupt Hijack."]
    #[inline(always)]
    pub const fn smart_dmaint(
        self,
    ) -> crate::pac::common::Reg<SmartDmaint, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Immediate cessation of execution following the completion of ROM execution."]
    #[inline(always)]
    pub const fn bootrom(self) -> crate::pac::common::Reg<u32, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "Controls Shared RAM Integration."]
    #[inline(always)]
    pub const fn ram_casp_ctrl(
        self,
    ) -> crate::pac::common::Reg<RamCaspCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "CPU Status."]
    #[inline(always)]
    pub const fn cpustat(self) -> crate::pac::common::Reg<Cpustat, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
    }
    #[doc = "LPCAC Control."]
    #[inline(always)]
    pub const fn lpcac_ctrl(self) -> crate::pac::common::Reg<LpcacCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0324usize) as _) }
    }
    #[doc = "I3C Misc Control."]
    #[inline(always)]
    pub const fn i3c_misc_ctrl(
        self,
    ) -> crate::pac::common::Reg<I3cMiscCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0328usize) as _) }
    }
    #[doc = "CTIMER Global Start Enable."]
    #[inline(always)]
    pub const fn ctimerglobalstarten(
        self,
    ) -> crate::pac::common::Reg<Ctimerglobalstarten, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
    }
    #[doc = "RAM Control."]
    #[inline(always)]
    pub const fn ram_ctrl(self) -> crate::pac::common::Reg<RamCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0444usize) as _) }
    }
    #[doc = "JTAG Chip ID."]
    #[inline(always)]
    pub const fn jtag_id(self) -> crate::pac::common::Reg<JtagId, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x07f0usize) as _) }
    }
    #[doc = "Device Type."]
    #[inline(always)]
    pub const fn device_type(self) -> crate::pac::common::Reg<DeviceType, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x07f4usize) as _) }
    }
    #[doc = "Device ID."]
    #[inline(always)]
    pub const fn device_id0(self) -> crate::pac::common::Reg<DeviceId0, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x07f8usize) as _) }
    }
}
#[doc = "System Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbclkdiv(pub u32);
impl Ahbclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> AhbclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        AhbclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: AhbclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbclkdiv {
    #[inline(always)]
    fn default() -> Ahbclkdiv {
        Ahbclkdiv(0)
    }
}
impl core::fmt::Debug for Ahbclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbclkdiv")
            .field("div", &self.div())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbclkdiv {{ div: {=u8:?}, unstab: {:?} }}",
            self.div(),
            self.unstab()
        )
    }
}
#[doc = "AHB Matrix Priority Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbmatprio(pub u32);
impl Ahbmatprio {
    #[doc = "CPU0 C-AHB bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_cbus(&self) -> Cpu0Cbus {
        let val = (self.0 >> 0usize) & 0x03;
        Cpu0Cbus::from_bits(val as u8)
    }
    #[doc = "CPU0 C-AHB bus master priority level."]
    #[inline(always)]
    pub const fn set_cpu0_cbus(&mut self, val: Cpu0Cbus) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CPU0 S-AHB bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_sbus(&self) -> AhbmatprioCpu0Sbus {
        let val = (self.0 >> 2usize) & 0x03;
        AhbmatprioCpu0Sbus::from_bits(val as u8)
    }
    #[doc = "CPU0 S-AHB bus master priority level."]
    #[inline(always)]
    pub const fn set_cpu0_sbus(&mut self, val: AhbmatprioCpu0Sbus) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "SmartDMA-I bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_cbus_smart_dma_i(&self) -> Cpu1CbusSmartDmaI {
        let val = (self.0 >> 4usize) & 0x03;
        Cpu1CbusSmartDmaI::from_bits(val as u8)
    }
    #[doc = "SmartDMA-I bus master priority level."]
    #[inline(always)]
    pub const fn set_cpu1_cbus_smart_dma_i(&mut self, val: Cpu1CbusSmartDmaI) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "SmartDMA-D bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_sbus_smart_dma_d(&self) -> Cpu1SbusSmartDmaD {
        let val = (self.0 >> 6usize) & 0x03;
        Cpu1SbusSmartDmaD::from_bits(val as u8)
    }
    #[doc = "SmartDMA-D bus master priority level."]
    #[inline(always)]
    pub const fn set_cpu1_sbus_smart_dma_d(&mut self, val: Cpu1SbusSmartDmaD) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "DMA0 controller bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> AhbmatprioDma0 {
        let val = (self.0 >> 8usize) & 0x03;
        AhbmatprioDma0::from_bits(val as u8)
    }
    #[doc = "DMA0 controller bus master priority level."]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: AhbmatprioDma0) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "DMA1 controller bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1(&self) -> AhbmatprioDma1 {
        let val = (self.0 >> 10usize) & 0x03;
        AhbmatprioDma1::from_bits(val as u8)
    }
    #[doc = "DMA1 controller bus master priority level."]
    #[inline(always)]
    pub const fn set_dma1(&mut self, val: AhbmatprioDma1) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "PKC and ELS bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc_els(&self) -> AhbmatprioPkcEls {
        let val = (self.0 >> 12usize) & 0x03;
        AhbmatprioPkcEls::from_bits(val as u8)
    }
    #[doc = "PKC and ELS bus master priority level."]
    #[inline(always)]
    pub const fn set_pkc_els(&mut self, val: AhbmatprioPkcEls) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "ESPI bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn coolflux_y_espi(&self) -> AhbmatprioCoolfluxYEspi {
        let val = (self.0 >> 20usize) & 0x03;
        AhbmatprioCoolfluxYEspi::from_bits(val as u8)
    }
    #[doc = "ESPI bus master priority level."]
    #[inline(always)]
    pub const fn set_coolflux_y_espi(&mut self, val: AhbmatprioCoolfluxYEspi) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "ENET bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_fs_enet(&self) -> AhbmatprioUsbFsEnet {
        let val = (self.0 >> 24usize) & 0x03;
        AhbmatprioUsbFsEnet::from_bits(val as u8)
    }
    #[doc = "ENET bus master priority level."]
    #[inline(always)]
    pub const fn set_usb_fs_enet(&mut self, val: AhbmatprioUsbFsEnet) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "USB-HS bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_hs(&self) -> AhbmatprioUsbHs {
        let val = (self.0 >> 26usize) & 0x03;
        AhbmatprioUsbHs::from_bits(val as u8)
    }
    #[doc = "USB-HS bus master priority level."]
    #[inline(always)]
    pub const fn set_usb_hs(&mut self, val: AhbmatprioUsbHs) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
}
impl Default for Ahbmatprio {
    #[inline(always)]
    fn default() -> Ahbmatprio {
        Ahbmatprio(0)
    }
}
impl core::fmt::Debug for Ahbmatprio {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbmatprio")
            .field("cpu0_cbus", &self.cpu0_cbus())
            .field("cpu0_sbus", &self.cpu0_sbus())
            .field("cpu1_cbus_smart_dma_i", &self.cpu1_cbus_smart_dma_i())
            .field("cpu1_sbus_smart_dma_d", &self.cpu1_sbus_smart_dma_d())
            .field("dma0", &self.dma0())
            .field("dma1", &self.dma1())
            .field("pkc_els", &self.pkc_els())
            .field("coolflux_y_espi", &self.coolflux_y_espi())
            .field("usb_fs_enet", &self.usb_fs_enet())
            .field("usb_hs", &self.usb_hs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbmatprio {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbmatprio {{ cpu0_cbus: {:?}, cpu0_sbus: {:?}, cpu1_cbus_smart_dma_i: {:?}, cpu1_sbus_smart_dma_d: {:?}, dma0: {:?}, dma1: {:?}, pkc_els: {:?}, coolflux_y_espi: {:?}, usb_fs_enet: {:?}, usb_hs: {:?} }}",
            self.cpu0_cbus(),
            self.cpu0_sbus(),
            self.cpu1_cbus_smart_dma_i(),
            self.cpu1_sbus_smart_dma_d(),
            self.dma0(),
            self.dma1(),
            self.pkc_els(),
            self.coolflux_y_espi(),
            self.usb_fs_enet(),
            self.usb_hs()
        )
    }
}
#[doc = "Gray to Binary Converter Binary Code \\[31:0\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BinaryCodeLsb(pub u32);
impl BinaryCodeLsb {
    #[doc = "Binary code \\[31:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn code_bin_31_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Binary code \\[31:0\\]."]
    #[inline(always)]
    pub const fn set_code_bin_31_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BinaryCodeLsb {
    #[inline(always)]
    fn default() -> BinaryCodeLsb {
        BinaryCodeLsb(0)
    }
}
impl core::fmt::Debug for BinaryCodeLsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BinaryCodeLsb")
            .field("code_bin_31_0", &self.code_bin_31_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BinaryCodeLsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BinaryCodeLsb {{ code_bin_31_0: {=u32:?} }}",
            self.code_bin_31_0()
        )
    }
}
#[doc = "Gray to Binary Converter Binary Code \\[41:32\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BinaryCodeMsb(pub u32);
impl BinaryCodeMsb {
    #[doc = "Binary code \\[41:32\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn code_bin_41_32(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Binary code \\[41:32\\]."]
    #[inline(always)]
    pub const fn set_code_bin_41_32(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for BinaryCodeMsb {
    #[inline(always)]
    fn default() -> BinaryCodeMsb {
        BinaryCodeMsb(0)
    }
}
impl core::fmt::Debug for BinaryCodeMsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BinaryCodeMsb")
            .field("code_bin_41_32", &self.code_bin_41_32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BinaryCodeMsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BinaryCodeMsb {{ code_bin_41_32: {=u16:?} }}",
            self.code_bin_41_32()
        )
    }
}
#[doc = "BUS_CLK Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busclkdiv(pub u32);
impl Busclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> BusclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        BusclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: BusclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> BusclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        BusclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: BusclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> BusclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        BusclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: BusclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Busclkdiv {
    #[inline(always)]
    fn default() -> Busclkdiv {
        Busclkdiv(0)
    }
}
impl core::fmt::Debug for Busclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Busclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Busclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Busclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "Clock Configuration Unlock."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkunlock(pub u32);
impl Clkunlock {
    #[doc = "Controls clock configuration registers access (for example, SLOWCLKDIV, BUSCLKDIV, AHBCLKDIV, FROHFDIV, FROLFDIV, PLLxCLKDIV, MRCC_xxx_CLKDIV, MRCC_xxx_CLKSEL, MRCC_GLB_xxx)."]
    #[must_use]
    #[inline(always)]
    pub const fn unlock(&self) -> Unlock {
        let val = (self.0 >> 0usize) & 0x01;
        Unlock::from_bits(val as u8)
    }
    #[doc = "Controls clock configuration registers access (for example, SLOWCLKDIV, BUSCLKDIV, AHBCLKDIV, FROHFDIV, FROLFDIV, PLLxCLKDIV, MRCC_xxx_CLKDIV, MRCC_xxx_CLKSEL, MRCC_GLB_xxx)."]
    #[inline(always)]
    pub const fn set_unlock(&mut self, val: Unlock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Clkunlock {
    #[inline(always)]
    fn default() -> Clkunlock {
        Clkunlock(0)
    }
}
impl core::fmt::Debug for Clkunlock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkunlock")
            .field("unlock", &self.unlock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkunlock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Clkunlock {{ unlock: {:?} }}", self.unlock())
    }
}
#[doc = "Non-Secure CPU0 System Tick Calibration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu0nstckcal(pub u32);
impl Cpu0nstckcal {
    #[doc = "Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[must_use]
    #[inline(always)]
    pub const fn tenms(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[inline(always)]
    pub const fn set_tenms(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Indicates whether the TENMS value is exact."]
    #[must_use]
    #[inline(always)]
    pub const fn skew(&self) -> Skew {
        let val = (self.0 >> 24usize) & 0x01;
        Skew::from_bits(val as u8)
    }
    #[doc = "Indicates whether the TENMS value is exact."]
    #[inline(always)]
    pub const fn set_skew(&mut self, val: Skew) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Indicates whether the device provides a reference clock to the processor."]
    #[must_use]
    #[inline(always)]
    pub const fn noref(&self) -> Noref {
        let val = (self.0 >> 25usize) & 0x01;
        Noref::from_bits(val as u8)
    }
    #[doc = "Indicates whether the device provides a reference clock to the processor."]
    #[inline(always)]
    pub const fn set_noref(&mut self, val: Noref) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for Cpu0nstckcal {
    #[inline(always)]
    fn default() -> Cpu0nstckcal {
        Cpu0nstckcal(0)
    }
}
impl core::fmt::Debug for Cpu0nstckcal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpu0nstckcal")
            .field("tenms", &self.tenms())
            .field("skew", &self.skew())
            .field("noref", &self.noref())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpu0nstckcal {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpu0nstckcal {{ tenms: {=u32:?}, skew: {:?}, noref: {:?} }}",
            self.tenms(),
            self.skew(),
            self.noref()
        )
    }
}
#[doc = "CPU Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpustat(pub u32);
impl Cpustat {
    #[doc = "CPU0 sleeping state."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0sleeping(&self) -> Cpu0sleeping {
        let val = (self.0 >> 0usize) & 0x01;
        Cpu0sleeping::from_bits(val as u8)
    }
    #[doc = "CPU0 sleeping state."]
    #[inline(always)]
    pub const fn set_cpu0sleeping(&mut self, val: Cpu0sleeping) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "CPU0 lockup state."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0lockup(&self) -> Cpu0lockup {
        let val = (self.0 >> 2usize) & 0x01;
        Cpu0lockup::from_bits(val as u8)
    }
    #[doc = "CPU0 lockup state."]
    #[inline(always)]
    pub const fn set_cpu0lockup(&mut self, val: Cpu0lockup) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for Cpustat {
    #[inline(always)]
    fn default() -> Cpustat {
        Cpustat(0)
    }
}
impl core::fmt::Debug for Cpustat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpustat")
            .field("cpu0sleeping", &self.cpu0sleeping())
            .field("cpu0lockup", &self.cpu0lockup())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpustat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpustat {{ cpu0sleeping: {:?}, cpu0lockup: {:?} }}",
            self.cpu0sleeping(),
            self.cpu0lockup()
        )
    }
}
#[doc = "CTIMER Global Start Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimerglobalstarten(pub u32);
impl Ctimerglobalstarten {
    #[doc = "Enables the CTIMER0 function clock."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0_clk_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER0 function clock."]
    #[inline(always)]
    pub const fn set_ctimer0_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the CTIMER1 function clock."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1_clk_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER1 function clock."]
    #[inline(always)]
    pub const fn set_ctimer1_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the CTIMER2 function clock."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2_clk_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER2 function clock."]
    #[inline(always)]
    pub const fn set_ctimer2_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables the CTIMER3 function clock."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer3_clk_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER3 function clock."]
    #[inline(always)]
    pub const fn set_ctimer3_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the CTIMER4 function clock."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4_clk_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER4 function clock."]
    #[inline(always)]
    pub const fn set_ctimer4_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Ctimerglobalstarten {
    #[inline(always)]
    fn default() -> Ctimerglobalstarten {
        Ctimerglobalstarten(0)
    }
}
impl core::fmt::Debug for Ctimerglobalstarten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimerglobalstarten")
            .field("ctimer0_clk_en", &self.ctimer0_clk_en())
            .field("ctimer1_clk_en", &self.ctimer1_clk_en())
            .field("ctimer2_clk_en", &self.ctimer2_clk_en())
            .field("ctimer3_clk_en", &self.ctimer3_clk_en())
            .field("ctimer4_clk_en", &self.ctimer4_clk_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimerglobalstarten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctimerglobalstarten {{ ctimer0_clk_en: {=bool:?}, ctimer1_clk_en: {=bool:?}, ctimer2_clk_en: {=bool:?}, ctimer3_clk_en: {=bool:?}, ctimer4_clk_en: {=bool:?} }}",
            self.ctimer0_clk_en(),
            self.ctimer1_clk_en(),
            self.ctimer2_clk_en(),
            self.ctimer3_clk_en(),
            self.ctimer4_clk_en()
        )
    }
}
#[doc = "Device ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DeviceId0(pub u32);
impl DeviceId0 {
    #[doc = "Indicates the device's ram size."]
    #[must_use]
    #[inline(always)]
    pub const fn ram_size(&self) -> RamSize {
        let val = (self.0 >> 0usize) & 0x0f;
        RamSize::from_bits(val as u8)
    }
    #[doc = "Indicates the device's ram size."]
    #[inline(always)]
    pub const fn set_ram_size(&mut self, val: RamSize) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Indicates the device's flash size."]
    #[must_use]
    #[inline(always)]
    pub const fn flash_size(&self) -> FlashSize {
        let val = (self.0 >> 4usize) & 0x0f;
        FlashSize::from_bits(val as u8)
    }
    #[doc = "Indicates the device's flash size."]
    #[inline(always)]
    pub const fn set_flash_size(&mut self, val: FlashSize) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Indicates the device's ROM revision."]
    #[must_use]
    #[inline(always)]
    pub const fn rom_rev_minor(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates the device's ROM revision."]
    #[inline(always)]
    pub const fn set_rom_rev_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn security(&self) -> Security {
        let val = (self.0 >> 24usize) & 0x0f;
        Security::from_bits(val as u8)
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_security(&mut self, val: Security) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for DeviceId0 {
    #[inline(always)]
    fn default() -> DeviceId0 {
        DeviceId0(0)
    }
}
impl core::fmt::Debug for DeviceId0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DeviceId0")
            .field("ram_size", &self.ram_size())
            .field("flash_size", &self.flash_size())
            .field("rom_rev_minor", &self.rom_rev_minor())
            .field("security", &self.security())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DeviceId0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DeviceId0 {{ ram_size: {:?}, flash_size: {:?}, rom_rev_minor: {=u8:?}, security: {:?} }}",
            self.ram_size(),
            self.flash_size(),
            self.rom_rev_minor(),
            self.security()
        )
    }
}
#[doc = "Device Type."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DeviceType(pub u32);
impl DeviceType {
    #[doc = "Indicates the device part number."]
    #[must_use]
    #[inline(always)]
    pub const fn device_type_num(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Indicates the device part number."]
    #[inline(always)]
    pub const fn set_device_type_num(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Indicates the device type."]
    #[must_use]
    #[inline(always)]
    pub const fn device_type_sec(&self) -> DeviceTypeSec {
        let val = (self.0 >> 16usize) & 0x01;
        DeviceTypeSec::from_bits(val as u8)
    }
    #[doc = "Indicates the device type."]
    #[inline(always)]
    pub const fn set_device_type_sec(&mut self, val: DeviceTypeSec) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Indicates the device's package type."]
    #[must_use]
    #[inline(always)]
    pub const fn device_type_pkg(&self) -> DeviceTypePkg {
        let val = (self.0 >> 20usize) & 0x0f;
        DeviceTypePkg::from_bits(val as u8)
    }
    #[doc = "Indicates the device's package type."]
    #[inline(always)]
    pub const fn set_device_type_pkg(&mut self, val: DeviceTypePkg) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Indicates the device's pin number."]
    #[must_use]
    #[inline(always)]
    pub const fn device_type_pin(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the device's pin number."]
    #[inline(always)]
    pub const fn set_device_type_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for DeviceType {
    #[inline(always)]
    fn default() -> DeviceType {
        DeviceType(0)
    }
}
impl core::fmt::Debug for DeviceType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DeviceType")
            .field("device_type_num", &self.device_type_num())
            .field("device_type_sec", &self.device_type_sec())
            .field("device_type_pkg", &self.device_type_pkg())
            .field("device_type_pin", &self.device_type_pin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DeviceType {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DeviceType {{ device_type_num: {=u16:?}, device_type_sec: {:?}, device_type_pkg: {:?}, device_type_pin: {=u8:?} }}",
            self.device_type_num(),
            self.device_type_sec(),
            self.device_type_pkg(),
            self.device_type_pin()
        )
    }
}
#[doc = "Ethernet Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EnetCtrl(pub u32);
impl EnetCtrl {
    #[doc = "Selects external PHY interface."]
    #[must_use]
    #[inline(always)]
    pub const fn phy_intf(&self) -> PhyIntf {
        let val = (self.0 >> 2usize) & 0x01;
        PhyIntf::from_bits(val as u8)
    }
    #[doc = "Selects external PHY interface."]
    #[inline(always)]
    pub const fn set_phy_intf(&mut self, val: PhyIntf) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Selects external PHY or on-chip 10BASE-T1S."]
    #[must_use]
    #[inline(always)]
    pub const fn phy_sel(&self) -> PhySel {
        let val = (self.0 >> 3usize) & 0x01;
        PhySel::from_bits(val as u8)
    }
    #[doc = "Selects external PHY or on-chip 10BASE-T1S."]
    #[inline(always)]
    pub const fn set_phy_sel(&mut self, val: PhySel) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for EnetCtrl {
    #[inline(always)]
    fn default() -> EnetCtrl {
        EnetCtrl(0)
    }
}
impl core::fmt::Debug for EnetCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EnetCtrl")
            .field("phy_intf", &self.phy_intf())
            .field("phy_sel", &self.phy_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EnetCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EnetCtrl {{ phy_intf: {:?}, phy_sel: {:?} }}",
            self.phy_intf(),
            self.phy_sel()
        )
    }
}
#[doc = "Sideband Flow Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EnetSbdFlowCtrl(pub u32);
impl EnetSbdFlowCtrl {
    #[doc = "Sideband Flow Control for channel0."]
    #[must_use]
    #[inline(always)]
    pub const fn sel_ch0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Sideband Flow Control for channel0."]
    #[inline(always)]
    pub const fn set_sel_ch0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for EnetSbdFlowCtrl {
    #[inline(always)]
    fn default() -> EnetSbdFlowCtrl {
        EnetSbdFlowCtrl(0)
    }
}
impl core::fmt::Debug for EnetSbdFlowCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EnetSbdFlowCtrl")
            .field("sel_ch0", &self.sel_ch0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EnetSbdFlowCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EnetSbdFlowCtrl {{ sel_ch0: {=bool:?} }}",
            self.sel_ch0()
        )
    }
}
#[doc = "FRO_HF_DIV Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frohfdiv(pub u32);
impl Frohfdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> FrohfdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        FrohfdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: FrohfdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> FrohfdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        FrohfdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: FrohfdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> FrohfdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        FrohfdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: FrohfdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Frohfdiv {
    #[inline(always)]
    fn default() -> Frohfdiv {
        Frohfdiv(0)
    }
}
impl core::fmt::Debug for Frohfdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frohfdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frohfdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Frohfdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "FRO_LF_DIV Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frolfdiv(pub u32);
impl Frolfdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> FrolfdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        FrolfdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: FrolfdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> FrolfdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        FrolfdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: FrolfdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> FrolfdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        FrolfdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: FrolfdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Frolfdiv {
    #[inline(always)]
    fn default() -> Frolfdiv {
        Frolfdiv(0)
    }
}
impl core::fmt::Debug for Frolfdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frolfdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frolfdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Frolfdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "Gray to Binary Converter Gray Code \\[31:0\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrayCodeLsb(pub u32);
impl GrayCodeLsb {
    #[doc = "Gray code \\[31:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn code_gray_31_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Gray code \\[31:0\\]."]
    #[inline(always)]
    pub const fn set_code_gray_31_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GrayCodeLsb {
    #[inline(always)]
    fn default() -> GrayCodeLsb {
        GrayCodeLsb(0)
    }
}
impl core::fmt::Debug for GrayCodeLsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GrayCodeLsb")
            .field("code_gray_31_0", &self.code_gray_31_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GrayCodeLsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GrayCodeLsb {{ code_gray_31_0: {=u32:?} }}",
            self.code_gray_31_0()
        )
    }
}
#[doc = "Gray to Binary Converter Gray Code \\[41:32\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrayCodeMsb(pub u32);
impl GrayCodeMsb {
    #[doc = "Gray code \\[41:32\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn code_gray_41_32(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Gray code \\[41:32\\]."]
    #[inline(always)]
    pub const fn set_code_gray_41_32(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for GrayCodeMsb {
    #[inline(always)]
    fn default() -> GrayCodeMsb {
        GrayCodeMsb(0)
    }
}
impl core::fmt::Debug for GrayCodeMsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GrayCodeMsb")
            .field("code_gray_41_32", &self.code_gray_41_32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GrayCodeMsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GrayCodeMsb {{ code_gray_41_32: {=u16:?} }}",
            self.code_gray_41_32()
        )
    }
}
#[doc = "I3C Misc Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3cMiscCtrl(pub u32);
impl I3cMiscCtrl {
    #[doc = "Disables/enables the I3C0 filter function on SCL pin."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0_scl_filt(&self) -> I3c0SclFilt {
        let val = (self.0 >> 0usize) & 0x0f;
        I3c0SclFilt::from_bits(val as u8)
    }
    #[doc = "Disables/enables the I3C0 filter function on SCL pin."]
    #[inline(always)]
    pub const fn set_i3c0_scl_filt(&mut self, val: I3c0SclFilt) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Disables/enables the I3C0 filter function on SDA pin."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0_sda_filt(&self) -> I3c0SdaFilt {
        let val = (self.0 >> 4usize) & 0x0f;
        I3c0SdaFilt::from_bits(val as u8)
    }
    #[doc = "Disables/enables the I3C0 filter function on SDA pin."]
    #[inline(always)]
    pub const fn set_i3c0_sda_filt(&mut self, val: I3c0SdaFilt) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Disables/enables the I3C1 filter I3C1 function on SCL pin."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c1_scl_filt(&self) -> I3c1SclFilt {
        let val = (self.0 >> 8usize) & 0x0f;
        I3c1SclFilt::from_bits(val as u8)
    }
    #[doc = "Disables/enables the I3C1 filter I3C1 function on SCL pin."]
    #[inline(always)]
    pub const fn set_i3c1_scl_filt(&mut self, val: I3c1SclFilt) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Disables/enables the I3C1 filter function on SDA pin."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c1_sda_filt(&self) -> I3c1SdaFilt {
        let val = (self.0 >> 12usize) & 0x0f;
        I3c1SdaFilt::from_bits(val as u8)
    }
    #[doc = "Disables/enables the I3C1 filter function on SDA pin."]
    #[inline(always)]
    pub const fn set_i3c1_sda_filt(&mut self, val: I3c1SdaFilt) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Disables/enables the I3C2 filter function on SCL pin."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c2_scl_filt(&self) -> I3c2SclFilt {
        let val = (self.0 >> 16usize) & 0x0f;
        I3c2SclFilt::from_bits(val as u8)
    }
    #[doc = "Disables/enables the I3C2 filter function on SCL pin."]
    #[inline(always)]
    pub const fn set_i3c2_scl_filt(&mut self, val: I3c2SclFilt) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Disables/enables the I3C2 filter function on SDA pin."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c2_sda_filt(&self) -> I3c2SdaFilt {
        let val = (self.0 >> 20usize) & 0x0f;
        I3c2SdaFilt::from_bits(val as u8)
    }
    #[doc = "Disables/enables the I3C2 filter function on SDA pin."]
    #[inline(always)]
    pub const fn set_i3c2_sda_filt(&mut self, val: I3c2SdaFilt) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Disables/enables the I3C3 filter function on SCL pin."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c3_scl_filt(&self) -> I3c3SclFilt {
        let val = (self.0 >> 24usize) & 0x0f;
        I3c3SclFilt::from_bits(val as u8)
    }
    #[doc = "Disables/enables the I3C3 filter function on SCL pin."]
    #[inline(always)]
    pub const fn set_i3c3_scl_filt(&mut self, val: I3c3SclFilt) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "Disables/enables the I3C3 filter function on SDA pin."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c3_sda_filt(&self) -> I3c3SdaFilt {
        let val = (self.0 >> 28usize) & 0x0f;
        I3c3SdaFilt::from_bits(val as u8)
    }
    #[doc = "Disables/enables the I3C3 filter function on SDA pin."]
    #[inline(always)]
    pub const fn set_i3c3_sda_filt(&mut self, val: I3c3SdaFilt) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for I3cMiscCtrl {
    #[inline(always)]
    fn default() -> I3cMiscCtrl {
        I3cMiscCtrl(0)
    }
}
impl core::fmt::Debug for I3cMiscCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3cMiscCtrl")
            .field("i3c0_scl_filt", &self.i3c0_scl_filt())
            .field("i3c0_sda_filt", &self.i3c0_sda_filt())
            .field("i3c1_scl_filt", &self.i3c1_scl_filt())
            .field("i3c1_sda_filt", &self.i3c1_sda_filt())
            .field("i3c2_scl_filt", &self.i3c2_scl_filt())
            .field("i3c2_sda_filt", &self.i3c2_sda_filt())
            .field("i3c3_scl_filt", &self.i3c3_scl_filt())
            .field("i3c3_sda_filt", &self.i3c3_sda_filt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3cMiscCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3cMiscCtrl {{ i3c0_scl_filt: {:?}, i3c0_sda_filt: {:?}, i3c1_scl_filt: {:?}, i3c1_sda_filt: {:?}, i3c2_scl_filt: {:?}, i3c2_sda_filt: {:?}, i3c3_scl_filt: {:?}, i3c3_sda_filt: {:?} }}",
            self.i3c0_scl_filt(),
            self.i3c0_sda_filt(),
            self.i3c1_scl_filt(),
            self.i3c1_sda_filt(),
            self.i3c2_scl_filt(),
            self.i3c2_sda_filt(),
            self.i3c3_scl_filt(),
            self.i3c3_sda_filt()
        )
    }
}
#[doc = "JTAG Chip ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JtagId(pub u32);
impl JtagId {
    #[doc = "Indicates the device ID."]
    #[must_use]
    #[inline(always)]
    pub const fn jtag_id(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Indicates the device ID."]
    #[inline(always)]
    pub const fn set_jtag_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for JtagId {
    #[inline(always)]
    fn default() -> JtagId {
        JtagId(0)
    }
}
impl core::fmt::Debug for JtagId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JtagId")
            .field("jtag_id", &self.jtag_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JtagId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "JtagId {{ jtag_id: {=u32:?} }}", self.jtag_id())
    }
}
#[doc = "LPCAC Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpcacCtrl(pub u32);
impl LpcacCtrl {
    #[doc = "Disables/enables the cache function."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_lpcac(&self) -> DisLpcac {
        let val = (self.0 >> 0usize) & 0x01;
        DisLpcac::from_bits(val as u8)
    }
    #[doc = "Disables/enables the cache function."]
    #[inline(always)]
    pub const fn set_dis_lpcac(&mut self, val: DisLpcac) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Clears the cache function."]
    #[must_use]
    #[inline(always)]
    pub const fn clr_lpcac(&self) -> ClrLpcac {
        let val = (self.0 >> 1usize) & 0x01;
        ClrLpcac::from_bits(val as u8)
    }
    #[doc = "Clears the cache function."]
    #[inline(always)]
    pub const fn set_clr_lpcac(&mut self, val: ClrLpcac) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Forces no allocation."]
    #[must_use]
    #[inline(always)]
    pub const fn frc_no_alloc(&self) -> FrcNoAlloc {
        let val = (self.0 >> 2usize) & 0x01;
        FrcNoAlloc::from_bits(val as u8)
    }
    #[doc = "Forces no allocation."]
    #[inline(always)]
    pub const fn set_frc_no_alloc(&mut self, val: FrcNoAlloc) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Disable LPCAC Write Through Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_lpcac_wtbf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Disable LPCAC Write Through Buffer."]
    #[inline(always)]
    pub const fn set_dis_lpcac_wtbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Limit LPCAC Write Through Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn lim_lpcac_wtbf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Limit LPCAC Write Through Buffer."]
    #[inline(always)]
    pub const fn set_lim_lpcac_wtbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "LPCAC XOM(eXecute-Only-Memory) attribute control."]
    #[must_use]
    #[inline(always)]
    pub const fn lpcac_xom(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "LPCAC XOM(eXecute-Only-Memory) attribute control."]
    #[inline(always)]
    pub const fn set_lpcac_xom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Request LPCAC memories."]
    #[must_use]
    #[inline(always)]
    pub const fn lpcac_mem_req(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Request LPCAC memories."]
    #[inline(always)]
    pub const fn set_lpcac_mem_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for LpcacCtrl {
    #[inline(always)]
    fn default() -> LpcacCtrl {
        LpcacCtrl(0)
    }
}
impl core::fmt::Debug for LpcacCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpcacCtrl")
            .field("dis_lpcac", &self.dis_lpcac())
            .field("clr_lpcac", &self.clr_lpcac())
            .field("frc_no_alloc", &self.frc_no_alloc())
            .field("dis_lpcac_wtbf", &self.dis_lpcac_wtbf())
            .field("lim_lpcac_wtbf", &self.lim_lpcac_wtbf())
            .field("lpcac_xom", &self.lpcac_xom())
            .field("lpcac_mem_req", &self.lpcac_mem_req())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpcacCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LpcacCtrl {{ dis_lpcac: {:?}, clr_lpcac: {:?}, frc_no_alloc: {:?}, dis_lpcac_wtbf: {=bool:?}, lim_lpcac_wtbf: {=bool:?}, lpcac_xom: {=bool:?}, lpcac_mem_req: {=bool:?} }}",
            self.dis_lpcac(),
            self.clr_lpcac(),
            self.frc_no_alloc(),
            self.dis_lpcac_wtbf(),
            self.lim_lpcac_wtbf(),
            self.lpcac_xom(),
            self.lpcac_mem_req()
        )
    }
}
#[doc = "NMI Source Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmisrc(pub u32);
impl Nmisrc {
    #[doc = "The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for CPU0, if enabled by NMIENCPU0."]
    #[must_use]
    #[inline(always)]
    pub const fn irqcpu0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for CPU0, if enabled by NMIENCPU0."]
    #[inline(always)]
    pub const fn set_irqcpu0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Enables the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
    #[must_use]
    #[inline(always)]
    pub const fn nmiencpu0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
    #[inline(always)]
    pub const fn set_nmiencpu0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Nmisrc {
    #[inline(always)]
    fn default() -> Nmisrc {
        Nmisrc(0)
    }
}
impl core::fmt::Debug for Nmisrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nmisrc")
            .field("irqcpu0", &self.irqcpu0())
            .field("nmiencpu0", &self.nmiencpu0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nmisrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Nmisrc {{ irqcpu0: {=u8:?}, nmiencpu0: {=bool:?} }}",
            self.irqcpu0(),
            self.nmiencpu0()
        )
    }
}
#[doc = "NVM Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NvmCtrl(pub u32);
impl NvmCtrl {
    #[doc = "Flash speculation control."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_flash_spec(&self) -> DisFlashSpec {
        let val = (self.0 >> 0usize) & 0x01;
        DisFlashSpec::from_bits(val as u8)
    }
    #[doc = "Flash speculation control."]
    #[inline(always)]
    pub const fn set_dis_flash_spec(&mut self, val: DisFlashSpec) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Flash data speculation control."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_data_spec(&self) -> DisDataSpec {
        let val = (self.0 >> 1usize) & 0x01;
        DisDataSpec::from_bits(val as u8)
    }
    #[doc = "Flash data speculation control."]
    #[inline(always)]
    pub const fn set_dis_data_spec(&mut self, val: DisDataSpec) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "FLASH stall on busy control."]
    #[must_use]
    #[inline(always)]
    pub const fn flash_stall_en(&self) -> FlashStallEn {
        let val = (self.0 >> 10usize) & 0x01;
        FlashStallEn::from_bits(val as u8)
    }
    #[doc = "FLASH stall on busy control."]
    #[inline(always)]
    pub const fn set_flash_stall_en(&mut self, val: FlashStallEn) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Bus error on data multi-bit ECC error control Set this field to 0 if you want to enable flash speculative."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_mbecc_err_inst(&self) -> DisMbeccErrInst {
        let val = (self.0 >> 16usize) & 0x01;
        DisMbeccErrInst::from_bits(val as u8)
    }
    #[doc = "Bus error on data multi-bit ECC error control Set this field to 0 if you want to enable flash speculative."]
    #[inline(always)]
    pub const fn set_dis_mbecc_err_inst(&mut self, val: DisMbeccErrInst) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Bus error on data multi-bit ECC error control Set this field to 0 if you want to enable flash speculative."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_mbecc_err_data(&self) -> DisMbeccErrData {
        let val = (self.0 >> 17usize) & 0x01;
        DisMbeccErrData::from_bits(val as u8)
    }
    #[doc = "Bus error on data multi-bit ECC error control Set this field to 0 if you want to enable flash speculative."]
    #[inline(always)]
    pub const fn set_dis_mbecc_err_data(&mut self, val: DisMbeccErrData) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
}
impl Default for NvmCtrl {
    #[inline(always)]
    fn default() -> NvmCtrl {
        NvmCtrl(0)
    }
}
impl core::fmt::Debug for NvmCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NvmCtrl")
            .field("dis_flash_spec", &self.dis_flash_spec())
            .field("dis_data_spec", &self.dis_data_spec())
            .field("flash_stall_en", &self.flash_stall_en())
            .field("dis_mbecc_err_inst", &self.dis_mbecc_err_inst())
            .field("dis_mbecc_err_data", &self.dis_mbecc_err_data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NvmCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NvmCtrl {{ dis_flash_spec: {:?}, dis_data_spec: {:?}, flash_stall_en: {:?}, dis_mbecc_err_inst: {:?}, dis_mbecc_err_data: {:?} }}",
            self.dis_flash_spec(),
            self.dis_data_spec(),
            self.flash_stall_en(),
            self.dis_mbecc_err_inst(),
            self.dis_mbecc_err_data()
        )
    }
}
#[doc = "PLL1_CLK_DIV Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll1clkdiv(pub u32);
impl Pll1clkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> Pll1clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        Pll1clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: Pll1clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> Pll1clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        Pll1clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: Pll1clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> Pll1clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        Pll1clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: Pll1clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pll1clkdiv {
    #[inline(always)]
    fn default() -> Pll1clkdiv {
        Pll1clkdiv(0)
    }
}
impl core::fmt::Debug for Pll1clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pll1clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pll1clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pll1clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "Protect Level Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Protlvl(pub u32);
impl Protlvl {
    #[doc = "Control privileged access of EIM, ERM, Flexcan, MBC, SCG, DMA, ROMCP and Flexspi."]
    #[must_use]
    #[inline(always)]
    pub const fn priv_(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control privileged access of EIM, ERM, Flexcan, MBC, SCG, DMA, ROMCP and Flexspi."]
    #[inline(always)]
    pub const fn set_priv_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control write access to Nonsecure MPU memory regions."]
    #[must_use]
    #[inline(always)]
    pub const fn locknsmpu(&self) -> Locknsmpu {
        let val = (self.0 >> 16usize) & 0x01;
        Locknsmpu::from_bits(val as u8)
    }
    #[doc = "Control write access to Nonsecure MPU memory regions."]
    #[inline(always)]
    pub const fn set_locknsmpu(&mut self, val: Locknsmpu) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "This 1-bit field provides a mechanism to limit writes to this register to protect its contents. Once set, this bit remains asserted until a system reset."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> ProtlvlLock {
        let val = (self.0 >> 31usize) & 0x01;
        ProtlvlLock::from_bits(val as u8)
    }
    #[doc = "This 1-bit field provides a mechanism to limit writes to this register to protect its contents. Once set, this bit remains asserted until a system reset."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: ProtlvlLock) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Protlvl {
    #[inline(always)]
    fn default() -> Protlvl {
        Protlvl(0)
    }
}
impl core::fmt::Debug for Protlvl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Protlvl")
            .field("priv_", &self.priv_())
            .field("locknsmpu", &self.locknsmpu())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Protlvl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Protlvl {{ priv_: {=bool:?}, locknsmpu: {:?}, lock: {:?} }}",
            self.priv_(),
            self.locknsmpu(),
            self.lock()
        )
    }
}
#[doc = "Controls Shared RAM Integration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamCaspCtrl(pub u32);
impl RamCaspCtrl {
    #[doc = "Controls RAM access for RAMA1 and RAMA2."]
    #[must_use]
    #[inline(always)]
    pub const fn interleave(&self) -> Interleave {
        let val = (self.0 >> 0usize) & 0x01;
        Interleave::from_bits(val as u8)
    }
    #[doc = "Controls RAM access for RAMA1 and RAMA2."]
    #[inline(always)]
    pub const fn set_interleave(&mut self, val: Interleave) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Request EZH memories."]
    #[must_use]
    #[inline(always)]
    pub const fn casp_req(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Request EZH memories."]
    #[inline(always)]
    pub const fn set_casp_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for RamCaspCtrl {
    #[inline(always)]
    fn default() -> RamCaspCtrl {
        RamCaspCtrl(0)
    }
}
impl core::fmt::Debug for RamCaspCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamCaspCtrl")
            .field("interleave", &self.interleave())
            .field("casp_req", &self.casp_req())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamCaspCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamCaspCtrl {{ interleave: {:?}, casp_req: {=bool:?} }}",
            self.interleave(),
            self.casp_req()
        )
    }
}
#[doc = "RAM Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamCtrl(pub u32);
impl RamCtrl {
    #[doc = "RAMA ECC enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rama_ecc_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA ECC enable."]
    #[inline(always)]
    pub const fn set_rama_ecc_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RAMA bank clock gating control, only avaiable when RAMA_ECC_ENABLE = 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rama_cg_override(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA bank clock gating control, only avaiable when RAMA_ECC_ENABLE = 0."]
    #[inline(always)]
    pub const fn set_rama_cg_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "RAMX bank clock gating control."]
    #[must_use]
    #[inline(always)]
    pub const fn ramx_cg_override(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "RAMX bank clock gating control."]
    #[inline(always)]
    pub const fn set_ramx_cg_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "RAMB bank clock gating control."]
    #[must_use]
    #[inline(always)]
    pub const fn ramb_cg_override(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "RAMB bank clock gating control."]
    #[inline(always)]
    pub const fn set_ramb_cg_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for RamCtrl {
    #[inline(always)]
    fn default() -> RamCtrl {
        RamCtrl(0)
    }
}
impl core::fmt::Debug for RamCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamCtrl")
            .field("rama_ecc_enable", &self.rama_ecc_enable())
            .field("rama_cg_override", &self.rama_cg_override())
            .field("ramx_cg_override", &self.ramx_cg_override())
            .field("ramb_cg_override", &self.ramb_cg_override())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamCtrl {{ rama_ecc_enable: {=bool:?}, rama_cg_override: {=bool:?}, ramx_cg_override: {=bool:?}, ramb_cg_override: {=bool:?} }}",
            self.rama_ecc_enable(),
            self.rama_cg_override(),
            self.ramx_cg_override(),
            self.ramb_cg_override()
        )
    }
}
#[doc = "AHB Matrix Remap Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Remap(pub u32);
impl Remap {
    #[doc = "RAMX0 address remap for CPU System bus."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_sbus(&self) -> RemapCpu0Sbus {
        let val = (self.0 >> 2usize) & 0x03;
        RemapCpu0Sbus::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for CPU System bus."]
    #[inline(always)]
    pub const fn set_cpu0_sbus(&mut self, val: RemapCpu0Sbus) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "RAMX0 address remap for SmartDMA I-BUS."]
    #[must_use]
    #[inline(always)]
    pub const fn smart_dma_i(&self) -> SmartDmaI {
        let val = (self.0 >> 4usize) & 0x03;
        SmartDmaI::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for SmartDMA I-BUS."]
    #[inline(always)]
    pub const fn set_smart_dma_i(&mut self, val: SmartDmaI) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "RAMX0 address remap for SmartDMA D-BUS."]
    #[must_use]
    #[inline(always)]
    pub const fn smart_dma_d(&self) -> SmartDmaD {
        let val = (self.0 >> 6usize) & 0x03;
        SmartDmaD::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for SmartDMA D-BUS."]
    #[inline(always)]
    pub const fn set_smart_dma_d(&mut self, val: SmartDmaD) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "RAMX0 address remap for DMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> RemapDma0 {
        let val = (self.0 >> 8usize) & 0x03;
        RemapDma0::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for DMA0."]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: RemapDma0) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "RAMX0 address remap for DMA1."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1(&self) -> RemapDma1 {
        let val = (self.0 >> 10usize) & 0x03;
        RemapDma1::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for DMA1."]
    #[inline(always)]
    pub const fn set_dma1(&mut self, val: RemapDma1) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "RAMX0 address remap for PKC_ELS."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc_els(&self) -> RemapPkcEls {
        let val = (self.0 >> 12usize) & 0x03;
        RemapPkcEls::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for PKC_ELS."]
    #[inline(always)]
    pub const fn set_pkc_els(&mut self, val: RemapPkcEls) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "RAMX0 address remap for ESPI."]
    #[must_use]
    #[inline(always)]
    pub const fn coolflux_y_espi(&self) -> RemapCoolfluxYEspi {
        let val = (self.0 >> 20usize) & 0x03;
        RemapCoolfluxYEspi::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for ESPI."]
    #[inline(always)]
    pub const fn set_coolflux_y_espi(&mut self, val: RemapCoolfluxYEspi) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "RAMX0 address remap for ENET."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_fs_enet(&self) -> RemapUsbFsEnet {
        let val = (self.0 >> 24usize) & 0x03;
        RemapUsbFsEnet::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for ENET."]
    #[inline(always)]
    pub const fn set_usb_fs_enet(&mut self, val: RemapUsbFsEnet) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "RAMX0 address remap for USB-HS."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_hs(&self) -> RemapUsbHs {
        let val = (self.0 >> 26usize) & 0x03;
        RemapUsbHs::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for USB-HS."]
    #[inline(always)]
    pub const fn set_usb_hs(&mut self, val: RemapUsbHs) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "This 1-bit field provides a mechanism to limit writes to this register to protect its contents. Once set, this bit remains asserted until a system reset."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> RemapLock {
        let val = (self.0 >> 31usize) & 0x01;
        RemapLock::from_bits(val as u8)
    }
    #[doc = "This 1-bit field provides a mechanism to limit writes to this register to protect its contents. Once set, this bit remains asserted until a system reset."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: RemapLock) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Remap {
    #[inline(always)]
    fn default() -> Remap {
        Remap(0)
    }
}
impl core::fmt::Debug for Remap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Remap")
            .field("cpu0_sbus", &self.cpu0_sbus())
            .field("smart_dma_i", &self.smart_dma_i())
            .field("smart_dma_d", &self.smart_dma_d())
            .field("dma0", &self.dma0())
            .field("dma1", &self.dma1())
            .field("pkc_els", &self.pkc_els())
            .field("coolflux_y_espi", &self.coolflux_y_espi())
            .field("usb_fs_enet", &self.usb_fs_enet())
            .field("usb_hs", &self.usb_hs())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Remap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Remap {{ cpu0_sbus: {:?}, smart_dma_i: {:?}, smart_dma_d: {:?}, dma0: {:?}, dma1: {:?}, pkc_els: {:?}, coolflux_y_espi: {:?}, usb_fs_enet: {:?}, usb_hs: {:?}, lock: {:?} }}",
            self.cpu0_sbus(),
            self.smart_dma_i(),
            self.smart_dma_d(),
            self.dma0(),
            self.dma1(),
            self.pkc_els(),
            self.coolflux_y_espi(),
            self.usb_fs_enet(),
            self.usb_hs(),
            self.lock()
        )
    }
}
#[doc = "SLOW_CLK Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slowclkdiv(pub u32);
impl Slowclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> SlowclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        SlowclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: SlowclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> SlowclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        SlowclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: SlowclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> SlowclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        SlowclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: SlowclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Slowclkdiv {
    #[inline(always)]
    fn default() -> Slowclkdiv {
        Slowclkdiv(0)
    }
}
impl core::fmt::Debug for Slowclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Slowclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Slowclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Slowclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "SmartDMA Interrupt Hijack."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmartDmaint(pub u32);
impl SmartDmaint {
    #[doc = "SmartDMA hijack NVIC IRQ2."]
    #[must_use]
    #[inline(always)]
    pub const fn int0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ2."]
    #[inline(always)]
    pub const fn set_int0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ23."]
    #[must_use]
    #[inline(always)]
    pub const fn int1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ23."]
    #[inline(always)]
    pub const fn set_int1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ26."]
    #[must_use]
    #[inline(always)]
    pub const fn int2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ26."]
    #[inline(always)]
    pub const fn set_int2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ27."]
    #[must_use]
    #[inline(always)]
    pub const fn int3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ27."]
    #[inline(always)]
    pub const fn set_int3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ28."]
    #[must_use]
    #[inline(always)]
    pub const fn int4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ28."]
    #[inline(always)]
    pub const fn set_int4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ29."]
    #[must_use]
    #[inline(always)]
    pub const fn int5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ29."]
    #[inline(always)]
    pub const fn set_int5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ31."]
    #[must_use]
    #[inline(always)]
    pub const fn int6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ31."]
    #[inline(always)]
    pub const fn set_int6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ32."]
    #[must_use]
    #[inline(always)]
    pub const fn int7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ32."]
    #[inline(always)]
    pub const fn set_int7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ33."]
    #[must_use]
    #[inline(always)]
    pub const fn int8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ33."]
    #[inline(always)]
    pub const fn set_int8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ34."]
    #[must_use]
    #[inline(always)]
    pub const fn int9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ34."]
    #[inline(always)]
    pub const fn set_int9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ39."]
    #[must_use]
    #[inline(always)]
    pub const fn int11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ39."]
    #[inline(always)]
    pub const fn set_int11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ40."]
    #[must_use]
    #[inline(always)]
    pub const fn int12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ40."]
    #[inline(always)]
    pub const fn set_int12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ41."]
    #[must_use]
    #[inline(always)]
    pub const fn int13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ41."]
    #[inline(always)]
    pub const fn set_int13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ59."]
    #[must_use]
    #[inline(always)]
    pub const fn int14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ59."]
    #[inline(always)]
    pub const fn set_int14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ62."]
    #[must_use]
    #[inline(always)]
    pub const fn int15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ62."]
    #[inline(always)]
    pub const fn set_int15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ64."]
    #[must_use]
    #[inline(always)]
    pub const fn int16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ64."]
    #[inline(always)]
    pub const fn set_int16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ71."]
    #[must_use]
    #[inline(always)]
    pub const fn int17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ71."]
    #[inline(always)]
    pub const fn set_int17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ72."]
    #[must_use]
    #[inline(always)]
    pub const fn int18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ72."]
    #[inline(always)]
    pub const fn set_int18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ73."]
    #[must_use]
    #[inline(always)]
    pub const fn int19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ73."]
    #[inline(always)]
    pub const fn set_int19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ74."]
    #[must_use]
    #[inline(always)]
    pub const fn int20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ74."]
    #[inline(always)]
    pub const fn set_int20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ75."]
    #[must_use]
    #[inline(always)]
    pub const fn int21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ75."]
    #[inline(always)]
    pub const fn set_int21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ103."]
    #[must_use]
    #[inline(always)]
    pub const fn int22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ103."]
    #[inline(always)]
    pub const fn set_int22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ104."]
    #[must_use]
    #[inline(always)]
    pub const fn int23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ104."]
    #[inline(always)]
    pub const fn set_int23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for SmartDmaint {
    #[inline(always)]
    fn default() -> SmartDmaint {
        SmartDmaint(0)
    }
}
impl core::fmt::Debug for SmartDmaint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmartDmaint")
            .field("int0", &self.int0())
            .field("int1", &self.int1())
            .field("int2", &self.int2())
            .field("int3", &self.int3())
            .field("int4", &self.int4())
            .field("int5", &self.int5())
            .field("int6", &self.int6())
            .field("int7", &self.int7())
            .field("int8", &self.int8())
            .field("int9", &self.int9())
            .field("int11", &self.int11())
            .field("int12", &self.int12())
            .field("int13", &self.int13())
            .field("int14", &self.int14())
            .field("int15", &self.int15())
            .field("int16", &self.int16())
            .field("int17", &self.int17())
            .field("int18", &self.int18())
            .field("int19", &self.int19())
            .field("int20", &self.int20())
            .field("int21", &self.int21())
            .field("int22", &self.int22())
            .field("int23", &self.int23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmartDmaint {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmartDmaint {{ int0: {=bool:?}, int1: {=bool:?}, int2: {=bool:?}, int3: {=bool:?}, int4: {=bool:?}, int5: {=bool:?}, int6: {=bool:?}, int7: {=bool:?}, int8: {=bool:?}, int9: {=bool:?}, int11: {=bool:?}, int12: {=bool:?}, int13: {=bool:?}, int14: {=bool:?}, int15: {=bool:?}, int16: {=bool:?}, int17: {=bool:?}, int18: {=bool:?}, int19: {=bool:?}, int20: {=bool:?}, int21: {=bool:?}, int22: {=bool:?}, int23: {=bool:?} }}",
            self.int0(),
            self.int1(),
            self.int2(),
            self.int3(),
            self.int4(),
            self.int5(),
            self.int6(),
            self.int7(),
            self.int8(),
            self.int9(),
            self.int11(),
            self.int12(),
            self.int13(),
            self.int14(),
            self.int15(),
            self.int16(),
            self.int17(),
            self.int18(),
            self.int19(),
            self.int20(),
            self.int21(),
            self.int22(),
            self.int23()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbclkdivUnstab {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl AhbclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> AhbclkdivUnstab {
        AhbclkdivUnstab::from_bits(val)
    }
}
impl From<AhbclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: AhbclkdivUnstab) -> u8 {
        AhbclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioCoolfluxYEspi {
    #[doc = "level 0."]
    LEVEL0 = 0x0,
    #[doc = "level 1."]
    LEVEL1 = 0x01,
    #[doc = "level 2."]
    LEVEL2 = 0x02,
    #[doc = "level 3."]
    LEVEL3 = 0x03,
}
impl AhbmatprioCoolfluxYEspi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioCoolfluxYEspi {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioCoolfluxYEspi {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioCoolfluxYEspi {
        AhbmatprioCoolfluxYEspi::from_bits(val)
    }
}
impl From<AhbmatprioCoolfluxYEspi> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioCoolfluxYEspi) -> u8 {
        AhbmatprioCoolfluxYEspi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioCpu0Sbus {
    #[doc = "level 0."]
    LEVEL0 = 0x0,
    #[doc = "level 1."]
    LEVEL1 = 0x01,
    #[doc = "level 2."]
    LEVEL2 = 0x02,
    #[doc = "level 3."]
    LEVEL3 = 0x03,
}
impl AhbmatprioCpu0Sbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioCpu0Sbus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioCpu0Sbus {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioCpu0Sbus {
        AhbmatprioCpu0Sbus::from_bits(val)
    }
}
impl From<AhbmatprioCpu0Sbus> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioCpu0Sbus) -> u8 {
        AhbmatprioCpu0Sbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioDma0 {
    #[doc = "level 0."]
    LEVEL0 = 0x0,
    #[doc = "level 1."]
    LEVEL1 = 0x01,
    #[doc = "level 2."]
    LEVEL2 = 0x02,
    #[doc = "level 3."]
    LEVEL3 = 0x03,
}
impl AhbmatprioDma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioDma0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioDma0 {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioDma0 {
        AhbmatprioDma0::from_bits(val)
    }
}
impl From<AhbmatprioDma0> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioDma0) -> u8 {
        AhbmatprioDma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioDma1 {
    #[doc = "level 0."]
    LEVEL0 = 0x0,
    #[doc = "level 1."]
    LEVEL1 = 0x01,
    #[doc = "level 2."]
    LEVEL2 = 0x02,
    #[doc = "level 3."]
    LEVEL3 = 0x03,
}
impl AhbmatprioDma1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioDma1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioDma1 {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioDma1 {
        AhbmatprioDma1::from_bits(val)
    }
}
impl From<AhbmatprioDma1> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioDma1) -> u8 {
        AhbmatprioDma1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioPkcEls {
    #[doc = "level 0."]
    LEVEL0 = 0x0,
    #[doc = "level 1."]
    LEVEL1 = 0x01,
    #[doc = "level 2."]
    LEVEL2 = 0x02,
    #[doc = "level 3."]
    LEVEL3 = 0x03,
}
impl AhbmatprioPkcEls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioPkcEls {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioPkcEls {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioPkcEls {
        AhbmatprioPkcEls::from_bits(val)
    }
}
impl From<AhbmatprioPkcEls> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioPkcEls) -> u8 {
        AhbmatprioPkcEls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioUsbFsEnet {
    #[doc = "level 0."]
    LEVEL0 = 0x0,
    #[doc = "level 1."]
    LEVEL1 = 0x01,
    #[doc = "level 2."]
    LEVEL2 = 0x02,
    #[doc = "level 3."]
    LEVEL3 = 0x03,
}
impl AhbmatprioUsbFsEnet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioUsbFsEnet {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioUsbFsEnet {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioUsbFsEnet {
        AhbmatprioUsbFsEnet::from_bits(val)
    }
}
impl From<AhbmatprioUsbFsEnet> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioUsbFsEnet) -> u8 {
        AhbmatprioUsbFsEnet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioUsbHs {
    #[doc = "level 0."]
    LEVEL0 = 0x0,
    #[doc = "level 1."]
    LEVEL1 = 0x01,
    #[doc = "level 2."]
    LEVEL2 = 0x02,
    #[doc = "level 3."]
    LEVEL3 = 0x03,
}
impl AhbmatprioUsbHs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioUsbHs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioUsbHs {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioUsbHs {
        AhbmatprioUsbHs::from_bits(val)
    }
}
impl From<AhbmatprioUsbHs> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioUsbHs) -> u8 {
        AhbmatprioUsbHs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BusclkdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stopped."]
    HALT = 0x01,
}
impl BusclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BusclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BusclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> BusclkdivHalt {
        BusclkdivHalt::from_bits(val)
    }
}
impl From<BusclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: BusclkdivHalt) -> u8 {
        BusclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BusclkdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl BusclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BusclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BusclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> BusclkdivReset {
        BusclkdivReset::from_bits(val)
    }
}
impl From<BusclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: BusclkdivReset) -> u8 {
        BusclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BusclkdivUnstab {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl BusclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BusclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BusclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> BusclkdivUnstab {
        BusclkdivUnstab::from_bits(val)
    }
}
impl From<BusclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: BusclkdivUnstab) -> u8 {
        BusclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClrLpcac {
    #[doc = "Unclears the cache."]
    ENABLE = 0x0,
    #[doc = "Clears the cache."]
    DISABLE = 0x01,
}
impl ClrLpcac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClrLpcac {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClrLpcac {
    #[inline(always)]
    fn from(val: u8) -> ClrLpcac {
        ClrLpcac::from_bits(val)
    }
}
impl From<ClrLpcac> for u8 {
    #[inline(always)]
    fn from(val: ClrLpcac) -> u8 {
        ClrLpcac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0Cbus {
    #[doc = "level 0."]
    LEVEL0 = 0x0,
    #[doc = "level 1."]
    LEVEL1 = 0x01,
    #[doc = "level 2."]
    LEVEL2 = 0x02,
    #[doc = "level 3."]
    LEVEL3 = 0x03,
}
impl Cpu0Cbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0Cbus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0Cbus {
    #[inline(always)]
    fn from(val: u8) -> Cpu0Cbus {
        Cpu0Cbus::from_bits(val)
    }
}
impl From<Cpu0Cbus> for u8 {
    #[inline(always)]
    fn from(val: Cpu0Cbus) -> u8 {
        Cpu0Cbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0lockup {
    #[doc = "CPU is not in lockup."]
    AWAKE = 0x0,
    #[doc = "CPU is in lockup."]
    SLEEPING = 0x01,
}
impl Cpu0lockup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0lockup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0lockup {
    #[inline(always)]
    fn from(val: u8) -> Cpu0lockup {
        Cpu0lockup::from_bits(val)
    }
}
impl From<Cpu0lockup> for u8 {
    #[inline(always)]
    fn from(val: Cpu0lockup) -> u8 {
        Cpu0lockup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0sleeping {
    #[doc = "CPU is not sleeping."]
    AWAKE = 0x0,
    #[doc = "CPU is sleeping."]
    SLEEPING = 0x01,
}
impl Cpu0sleeping {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0sleeping {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0sleeping {
    #[inline(always)]
    fn from(val: u8) -> Cpu0sleeping {
        Cpu0sleeping::from_bits(val)
    }
}
impl From<Cpu0sleeping> for u8 {
    #[inline(always)]
    fn from(val: Cpu0sleeping) -> u8 {
        Cpu0sleeping::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1CbusSmartDmaI {
    #[doc = "level 0."]
    LEVEL0 = 0x0,
    #[doc = "level 1."]
    LEVEL1 = 0x01,
    #[doc = "level 2."]
    LEVEL2 = 0x02,
    #[doc = "level 3."]
    LEVEL3 = 0x03,
}
impl Cpu1CbusSmartDmaI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1CbusSmartDmaI {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1CbusSmartDmaI {
    #[inline(always)]
    fn from(val: u8) -> Cpu1CbusSmartDmaI {
        Cpu1CbusSmartDmaI::from_bits(val)
    }
}
impl From<Cpu1CbusSmartDmaI> for u8 {
    #[inline(always)]
    fn from(val: Cpu1CbusSmartDmaI) -> u8 {
        Cpu1CbusSmartDmaI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1SbusSmartDmaD {
    #[doc = "level 0."]
    LEVEL0 = 0x0,
    #[doc = "level 1."]
    LEVEL1 = 0x01,
    #[doc = "level 2."]
    LEVEL2 = 0x02,
    #[doc = "level 3."]
    LEVEL3 = 0x03,
}
impl Cpu1SbusSmartDmaD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1SbusSmartDmaD {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1SbusSmartDmaD {
    #[inline(always)]
    fn from(val: u8) -> Cpu1SbusSmartDmaD {
        Cpu1SbusSmartDmaD::from_bits(val)
    }
}
impl From<Cpu1SbusSmartDmaD> for u8 {
    #[inline(always)]
    fn from(val: Cpu1SbusSmartDmaD) -> u8 {
        Cpu1SbusSmartDmaD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DeviceTypePkg {
    #[doc = "HLQFP."]
    HLQFP = 0x0,
    #[doc = "HTQFP."]
    HTQFP = 0x01,
    #[doc = "BGA."]
    BGA = 0x02,
    #[doc = "HDQFP."]
    HDQFP = 0x03,
    #[doc = "QFN."]
    QFN = 0x04,
    #[doc = "CSP."]
    CSP = 0x05,
    #[doc = "LQFP."]
    LQFP = 0x06,
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
impl DeviceTypePkg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DeviceTypePkg {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DeviceTypePkg {
    #[inline(always)]
    fn from(val: u8) -> DeviceTypePkg {
        DeviceTypePkg::from_bits(val)
    }
}
impl From<DeviceTypePkg> for u8 {
    #[inline(always)]
    fn from(val: DeviceTypePkg) -> u8 {
        DeviceTypePkg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DeviceTypeSec {
    #[doc = "Non Secure."]
    NON_SEC = 0x0,
    #[doc = "Secure."]
    SEC = 0x01,
}
impl DeviceTypeSec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DeviceTypeSec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DeviceTypeSec {
    #[inline(always)]
    fn from(val: u8) -> DeviceTypeSec {
        DeviceTypeSec::from_bits(val)
    }
}
impl From<DeviceTypeSec> for u8 {
    #[inline(always)]
    fn from(val: DeviceTypeSec) -> u8 {
        DeviceTypeSec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisDataSpec {
    #[doc = "Enables data speculation."]
    ENABLE = 0x0,
    #[doc = "Disables data speculation."]
    DISABLE = 0x01,
}
impl DisDataSpec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisDataSpec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisDataSpec {
    #[inline(always)]
    fn from(val: u8) -> DisDataSpec {
        DisDataSpec::from_bits(val)
    }
}
impl From<DisDataSpec> for u8 {
    #[inline(always)]
    fn from(val: DisDataSpec) -> u8 {
        DisDataSpec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisFlashSpec {
    #[doc = "Enables flash speculation."]
    ENABLE = 0x0,
    #[doc = "Disables flash speculation."]
    DISABLE = 0x01,
}
impl DisFlashSpec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisFlashSpec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisFlashSpec {
    #[inline(always)]
    fn from(val: u8) -> DisFlashSpec {
        DisFlashSpec::from_bits(val)
    }
}
impl From<DisFlashSpec> for u8 {
    #[inline(always)]
    fn from(val: DisFlashSpec) -> u8 {
        DisFlashSpec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisLpcac {
    #[doc = "Enabled."]
    ENABLE = 0x0,
    #[doc = "Disabled."]
    DISABLE = 0x01,
}
impl DisLpcac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisLpcac {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisLpcac {
    #[inline(always)]
    fn from(val: u8) -> DisLpcac {
        DisLpcac::from_bits(val)
    }
}
impl From<DisLpcac> for u8 {
    #[inline(always)]
    fn from(val: DisLpcac) -> u8 {
        DisLpcac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisMbeccErrData {
    #[doc = "Enables bus error on multi-bit ECC error for data."]
    ENABLE = 0x0,
    #[doc = "Disables bus error on multi-bit ECC error for data."]
    DISABLE = 0x01,
}
impl DisMbeccErrData {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisMbeccErrData {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisMbeccErrData {
    #[inline(always)]
    fn from(val: u8) -> DisMbeccErrData {
        DisMbeccErrData::from_bits(val)
    }
}
impl From<DisMbeccErrData> for u8 {
    #[inline(always)]
    fn from(val: DisMbeccErrData) -> u8 {
        DisMbeccErrData::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisMbeccErrInst {
    #[doc = "Enables bus error on multi-bit ECC error for instruction."]
    ENABLE = 0x0,
    #[doc = "Disables bus error on multi-bit ECC error for instruction."]
    DISABLE = 0x01,
}
impl DisMbeccErrInst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisMbeccErrInst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisMbeccErrInst {
    #[inline(always)]
    fn from(val: u8) -> DisMbeccErrInst {
        DisMbeccErrInst::from_bits(val)
    }
}
impl From<DisMbeccErrInst> for u8 {
    #[inline(always)]
    fn from(val: DisMbeccErrInst) -> u8 {
        DisMbeccErrInst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlashSize {
    #[doc = "32KB."]
    SIZE_32KB = 0x0,
    #[doc = "64KB."]
    SIZE_64KB = 0x01,
    #[doc = "128KB."]
    SIZE_128KB = 0x02,
    #[doc = "256KB."]
    SIZE_256KB = 0x03,
    #[doc = "512KB."]
    SIZE_512KB = 0x04,
    #[doc = "768KB."]
    SIZE_768KB = 0x05,
    #[doc = "1MB."]
    SIZE_1MB = 0x06,
    #[doc = "1.5MB."]
    SIZE_1P5MB = 0x07,
    #[doc = "2MB."]
    SIZE_2MB = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl FlashSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashSize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashSize {
    #[inline(always)]
    fn from(val: u8) -> FlashSize {
        FlashSize::from_bits(val)
    }
}
impl From<FlashSize> for u8 {
    #[inline(always)]
    fn from(val: FlashSize) -> u8 {
        FlashSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlashStallEn {
    #[doc = "No stall on FLASH busy."]
    ENABLE = 0x0,
    #[doc = "Stall on FLASH busy."]
    DISABLE = 0x01,
}
impl FlashStallEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashStallEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashStallEn {
    #[inline(always)]
    fn from(val: u8) -> FlashStallEn {
        FlashStallEn::from_bits(val)
    }
}
impl From<FlashStallEn> for u8 {
    #[inline(always)]
    fn from(val: FlashStallEn) -> u8 {
        FlashStallEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrcNoAlloc {
    #[doc = "Forces allocation."]
    ENABLE = 0x0,
    #[doc = "Forces no allocation."]
    DISABLE = 0x01,
}
impl FrcNoAlloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrcNoAlloc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrcNoAlloc {
    #[inline(always)]
    fn from(val: u8) -> FrcNoAlloc {
        FrcNoAlloc::from_bits(val)
    }
}
impl From<FrcNoAlloc> for u8 {
    #[inline(always)]
    fn from(val: FrcNoAlloc) -> u8 {
        FrcNoAlloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrohfdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stopped."]
    HALT = 0x01,
}
impl FrohfdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrohfdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrohfdivHalt {
    #[inline(always)]
    fn from(val: u8) -> FrohfdivHalt {
        FrohfdivHalt::from_bits(val)
    }
}
impl From<FrohfdivHalt> for u8 {
    #[inline(always)]
    fn from(val: FrohfdivHalt) -> u8 {
        FrohfdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrohfdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl FrohfdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrohfdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrohfdivReset {
    #[inline(always)]
    fn from(val: u8) -> FrohfdivReset {
        FrohfdivReset::from_bits(val)
    }
}
impl From<FrohfdivReset> for u8 {
    #[inline(always)]
    fn from(val: FrohfdivReset) -> u8 {
        FrohfdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrohfdivUnstab {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl FrohfdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrohfdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrohfdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> FrohfdivUnstab {
        FrohfdivUnstab::from_bits(val)
    }
}
impl From<FrohfdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: FrohfdivUnstab) -> u8 {
        FrohfdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrolfdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stopped."]
    HALT = 0x01,
}
impl FrolfdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrolfdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrolfdivHalt {
    #[inline(always)]
    fn from(val: u8) -> FrolfdivHalt {
        FrolfdivHalt::from_bits(val)
    }
}
impl From<FrolfdivHalt> for u8 {
    #[inline(always)]
    fn from(val: FrolfdivHalt) -> u8 {
        FrolfdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrolfdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl FrolfdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrolfdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrolfdivReset {
    #[inline(always)]
    fn from(val: u8) -> FrolfdivReset {
        FrolfdivReset::from_bits(val)
    }
}
impl From<FrolfdivReset> for u8 {
    #[inline(always)]
    fn from(val: FrolfdivReset) -> u8 {
        FrolfdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrolfdivUnstab {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl FrolfdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrolfdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrolfdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> FrolfdivUnstab {
        FrolfdivUnstab::from_bits(val)
    }
}
impl From<FrolfdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: FrolfdivUnstab) -> u8 {
        FrolfdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0SclFilt {
    #[doc = "Disabled filter function when I3C0 SDA_FILT=0b0000."]
    DISABLE = 0x0,
    #[doc = "Enable Spike filter on SCL input. Non_zero value means width of Glitch on SCL line to be filtered in number of half cycles of CLK_FLT."]
    FILT_CNT = 0x01,
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
impl I3c0SclFilt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0SclFilt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0SclFilt {
    #[inline(always)]
    fn from(val: u8) -> I3c0SclFilt {
        I3c0SclFilt::from_bits(val)
    }
}
impl From<I3c0SclFilt> for u8 {
    #[inline(always)]
    fn from(val: I3c0SclFilt) -> u8 {
        I3c0SclFilt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0SdaFilt {
    #[doc = "Disabled filter function when I3C0 SCL_FILT=0b0000."]
    DISABLE = 0x0,
    #[doc = "Enable Spike filter on SDA input. Non_zero value means width of Glitch on SDA line to be filtered in number of half cycles of CLK_FLT."]
    FILT_NUM = 0x01,
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
impl I3c0SdaFilt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0SdaFilt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0SdaFilt {
    #[inline(always)]
    fn from(val: u8) -> I3c0SdaFilt {
        I3c0SdaFilt::from_bits(val)
    }
}
impl From<I3c0SdaFilt> for u8 {
    #[inline(always)]
    fn from(val: I3c0SdaFilt) -> u8 {
        I3c0SdaFilt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1SclFilt {
    #[doc = "Disabled filter function when SDA_FILT=0b0000."]
    DISABLE = 0x0,
    #[doc = "Enable Spike filter on SCL input. Non_zero value means width of Glitch on SCL line to be filtered in number of half cycles of CLK_FLT."]
    FILT_NUM = 0x01,
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
impl I3c1SclFilt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1SclFilt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1SclFilt {
    #[inline(always)]
    fn from(val: u8) -> I3c1SclFilt {
        I3c1SclFilt::from_bits(val)
    }
}
impl From<I3c1SclFilt> for u8 {
    #[inline(always)]
    fn from(val: I3c1SclFilt) -> u8 {
        I3c1SclFilt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1SdaFilt {
    #[doc = "Disabled filter function when I3C1 SCL_FILT=0b0000."]
    DISABLE = 0x0,
    #[doc = "Enable Spike filter on SDA input. Non_zero value means width of Glitch on SDA line to be filtered in number of half cycles of CLK_FLT."]
    FILT_NUM = 0x01,
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
impl I3c1SdaFilt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1SdaFilt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1SdaFilt {
    #[inline(always)]
    fn from(val: u8) -> I3c1SdaFilt {
        I3c1SdaFilt::from_bits(val)
    }
}
impl From<I3c1SdaFilt> for u8 {
    #[inline(always)]
    fn from(val: I3c1SdaFilt) -> u8 {
        I3c1SdaFilt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c2SclFilt {
    #[doc = "Disabled filter function when I3C2 SDA_FILT=0b0000."]
    DISABLE = 0x0,
    #[doc = "Enable Spike filter on SCL input. Non_zero value means width of Glitch on SCL line to be filtered in number of half cycles of CLK_FLT."]
    FILT_NUM = 0x01,
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
impl I3c2SclFilt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c2SclFilt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c2SclFilt {
    #[inline(always)]
    fn from(val: u8) -> I3c2SclFilt {
        I3c2SclFilt::from_bits(val)
    }
}
impl From<I3c2SclFilt> for u8 {
    #[inline(always)]
    fn from(val: I3c2SclFilt) -> u8 {
        I3c2SclFilt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c2SdaFilt {
    #[doc = "Disabled filter function when I3C2 SCL_FILT=0b0000."]
    DISABLE = 0x0,
    #[doc = "Enable Spike filter on SDA input. Non_zero value means width of Glitch on SDA line to be filtered in number of half cycles of CLK_FLT."]
    FILT_NUM = 0x01,
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
impl I3c2SdaFilt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c2SdaFilt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c2SdaFilt {
    #[inline(always)]
    fn from(val: u8) -> I3c2SdaFilt {
        I3c2SdaFilt::from_bits(val)
    }
}
impl From<I3c2SdaFilt> for u8 {
    #[inline(always)]
    fn from(val: I3c2SdaFilt) -> u8 {
        I3c2SdaFilt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c3SclFilt {
    #[doc = "Disabled filter function when I3C3 SDA_FILT=0b0000."]
    DISABLE = 0x0,
    #[doc = "Enable Spike filter on SCL input. Non_zero value means width of Glitch on SCL line to be filtered in number of half cycles of CLK_FLT."]
    FILT_NUM = 0x01,
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
impl I3c3SclFilt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c3SclFilt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c3SclFilt {
    #[inline(always)]
    fn from(val: u8) -> I3c3SclFilt {
        I3c3SclFilt::from_bits(val)
    }
}
impl From<I3c3SclFilt> for u8 {
    #[inline(always)]
    fn from(val: I3c3SclFilt) -> u8 {
        I3c3SclFilt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c3SdaFilt {
    #[doc = "Disabled filter function when I3C3 SCL_FILT=0b0000."]
    DISABLE = 0x0,
    #[doc = "Enable Spike filter on SDA input. Non_zero value means width of Glitch on SDA line to be filtered in number of half cycles of CLK_FLT."]
    FILT_NUM = 0x01,
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
impl I3c3SdaFilt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c3SdaFilt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c3SdaFilt {
    #[inline(always)]
    fn from(val: u8) -> I3c3SdaFilt {
        I3c3SdaFilt::from_bits(val)
    }
}
impl From<I3c3SdaFilt> for u8 {
    #[inline(always)]
    fn from(val: I3c3SdaFilt) -> u8 {
        I3c3SdaFilt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interleave {
    #[doc = "RAM access is consecutive."]
    NORMAL = 0x0,
    #[doc = "RAM access is interleaved. This setting is need for PKC L0 memory access."]
    INTERLEAVE = 0x01,
}
impl Interleave {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Interleave {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Interleave {
    #[inline(always)]
    fn from(val: u8) -> Interleave {
        Interleave::from_bits(val)
    }
}
impl From<Interleave> for u8 {
    #[inline(always)]
    fn from(val: Interleave) -> u8 {
        Interleave::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Locknsmpu {
    #[doc = "Unlock these registers. privileged access to Nonsecure MPU memory regions is allowed."]
    ENABLE = 0x0,
    #[doc = "Disable writes to the MPU_CTRL_NS, MPU_RNR_NS, MPU_RBAR_NS, MPU_RLAR_NS, MPU_RBAR_A_NSn and MPU_RLAR_A_NSn. All writes to the registers are ignored."]
    DISABLE = 0x01,
}
impl Locknsmpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Locknsmpu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Locknsmpu {
    #[inline(always)]
    fn from(val: u8) -> Locknsmpu {
        Locknsmpu::from_bits(val)
    }
}
impl From<Locknsmpu> for u8 {
    #[inline(always)]
    fn from(val: Locknsmpu) -> u8 {
        Locknsmpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Noref {
    #[doc = "Reference clock is provided."]
    YES_REF = 0x0,
    #[doc = "No reference clock is provided."]
    NO_REF = 0x01,
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
pub enum PhyIntf {
    #[doc = "Selects MII PHY Interface."]
    MII = 0x0,
    #[doc = "Selects RMII PHY Interface."]
    RMII = 0x01,
}
impl PhyIntf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhyIntf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhyIntf {
    #[inline(always)]
    fn from(val: u8) -> PhyIntf {
        PhyIntf::from_bits(val)
    }
}
impl From<PhyIntf> for u8 {
    #[inline(always)]
    fn from(val: PhyIntf) -> u8 {
        PhyIntf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhySel {
    #[doc = "Selects external PHY."]
    PHY = 0x0,
    #[doc = "Selects on-chip 10BASE-T1S."]
    T1S = 0x01,
}
impl PhySel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhySel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhySel {
    #[inline(always)]
    fn from(val: u8) -> PhySel {
        PhySel::from_bits(val)
    }
}
impl From<PhySel> for u8 {
    #[inline(always)]
    fn from(val: PhySel) -> u8 {
        PhySel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clkdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stopped."]
    HALT = 0x01,
}
impl Pll1clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Pll1clkdivHalt {
        Pll1clkdivHalt::from_bits(val)
    }
}
impl From<Pll1clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Pll1clkdivHalt) -> u8 {
        Pll1clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clkdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl Pll1clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Pll1clkdivReset {
        Pll1clkdivReset::from_bits(val)
    }
}
impl From<Pll1clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Pll1clkdivReset) -> u8 {
        Pll1clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clkdivUnstab {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl Pll1clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Pll1clkdivUnstab {
        Pll1clkdivUnstab::from_bits(val)
    }
}
impl From<Pll1clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Pll1clkdivUnstab) -> u8 {
        Pll1clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProtlvlLock {
    #[doc = "This register is not locked and can be altered."]
    ENABLE = 0x0,
    #[doc = "This register is locked and cannot be altered until a system reset."]
    DISABLE = 0x01,
}
impl ProtlvlLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ProtlvlLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ProtlvlLock {
    #[inline(always)]
    fn from(val: u8) -> ProtlvlLock {
        ProtlvlLock::from_bits(val)
    }
}
impl From<ProtlvlLock> for u8 {
    #[inline(always)]
    fn from(val: ProtlvlLock) -> u8 {
        ProtlvlLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamSize {
    #[doc = "8KB."]
    SIZE_8KB = 0x0,
    #[doc = "16KB."]
    SIZE_16KB = 0x01,
    #[doc = "32KB."]
    SIZE_32KB = 0x02,
    #[doc = "64KB."]
    SIZE_64KB = 0x03,
    #[doc = "96KB."]
    SIZE_96KB = 0x04,
    #[doc = "128KB."]
    SIZE_128KB = 0x05,
    #[doc = "160KB."]
    SIZE_160KB = 0x06,
    #[doc = "192KB."]
    SIZE_192KB = 0x07,
    #[doc = "256KB."]
    SIZE_256KB = 0x08,
    #[doc = "288KB."]
    SIZE_288KB = 0x09,
    #[doc = "352KB."]
    SIZE_352KB = 0x0a,
    #[doc = "512KB."]
    SIZE_512KB = 0x0b,
    #[doc = "640KB."]
    SIZE_640KB = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl RamSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamSize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamSize {
    #[inline(always)]
    fn from(val: u8) -> RamSize {
        RamSize::from_bits(val)
    }
}
impl From<RamSize> for u8 {
    #[inline(always)]
    fn from(val: RamSize) -> u8 {
        RamSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RemapCoolfluxYEspi {
    #[doc = "RAMX0: alias space is disabled."]
    DISABLE = 0x0,
    #[doc = "RAMX0: same alias space as CPU0_SBUS."]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl RemapCoolfluxYEspi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RemapCoolfluxYEspi {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RemapCoolfluxYEspi {
    #[inline(always)]
    fn from(val: u8) -> RemapCoolfluxYEspi {
        RemapCoolfluxYEspi::from_bits(val)
    }
}
impl From<RemapCoolfluxYEspi> for u8 {
    #[inline(always)]
    fn from(val: RemapCoolfluxYEspi) -> u8 {
        RemapCoolfluxYEspi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RemapCpu0Sbus {
    #[doc = "RAMX0: alias space is disabled."]
    DISABLE = 0x0,
    #[doc = "RAMX0: alias space is enabled. It's linear address space from bottom of system ram. The start address is 0x20000000 + (system ram size - RAMX size)*1024."]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl RemapCpu0Sbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RemapCpu0Sbus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RemapCpu0Sbus {
    #[inline(always)]
    fn from(val: u8) -> RemapCpu0Sbus {
        RemapCpu0Sbus::from_bits(val)
    }
}
impl From<RemapCpu0Sbus> for u8 {
    #[inline(always)]
    fn from(val: RemapCpu0Sbus) -> u8 {
        RemapCpu0Sbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RemapDma0 {
    #[doc = "RAMX0: alias space is disabled."]
    DISABLE = 0x0,
    #[doc = "RAMX0: same alias space as CPU0_SBUS."]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl RemapDma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RemapDma0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RemapDma0 {
    #[inline(always)]
    fn from(val: u8) -> RemapDma0 {
        RemapDma0::from_bits(val)
    }
}
impl From<RemapDma0> for u8 {
    #[inline(always)]
    fn from(val: RemapDma0) -> u8 {
        RemapDma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RemapDma1 {
    #[doc = "RAMX0: alias space is disabled."]
    DISABLE = 0x0,
    #[doc = "RAMX0: same alias space as CPU0_SBUS."]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl RemapDma1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RemapDma1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RemapDma1 {
    #[inline(always)]
    fn from(val: u8) -> RemapDma1 {
        RemapDma1::from_bits(val)
    }
}
impl From<RemapDma1> for u8 {
    #[inline(always)]
    fn from(val: RemapDma1) -> u8 {
        RemapDma1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RemapLock {
    #[doc = "This register is not locked and can be altered."]
    ENABLE = 0x0,
    #[doc = "This register is locked and cannot be altered until a system reset."]
    DISABLE = 0x01,
}
impl RemapLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RemapLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RemapLock {
    #[inline(always)]
    fn from(val: u8) -> RemapLock {
        RemapLock::from_bits(val)
    }
}
impl From<RemapLock> for u8 {
    #[inline(always)]
    fn from(val: RemapLock) -> u8 {
        RemapLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RemapPkcEls {
    #[doc = "RAMX0: alias space is disabled."]
    DISABLE = 0x0,
    #[doc = "RAMX0: same alias space as CPU0_SBUS."]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl RemapPkcEls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RemapPkcEls {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RemapPkcEls {
    #[inline(always)]
    fn from(val: u8) -> RemapPkcEls {
        RemapPkcEls::from_bits(val)
    }
}
impl From<RemapPkcEls> for u8 {
    #[inline(always)]
    fn from(val: RemapPkcEls) -> u8 {
        RemapPkcEls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RemapUsbFsEnet {
    #[doc = "RAMX0: alias space is disabled."]
    DISABLE = 0x0,
    #[doc = "RAMX0: same alias space as CPU0_SBUS."]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl RemapUsbFsEnet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RemapUsbFsEnet {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RemapUsbFsEnet {
    #[inline(always)]
    fn from(val: u8) -> RemapUsbFsEnet {
        RemapUsbFsEnet::from_bits(val)
    }
}
impl From<RemapUsbFsEnet> for u8 {
    #[inline(always)]
    fn from(val: RemapUsbFsEnet) -> u8 {
        RemapUsbFsEnet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RemapUsbHs {
    #[doc = "RAMX0: alias space is disabled."]
    DISABLE = 0x0,
    #[doc = "RAMX0: same alias space as CPU0_SBUS."]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl RemapUsbHs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RemapUsbHs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RemapUsbHs {
    #[inline(always)]
    fn from(val: u8) -> RemapUsbHs {
        RemapUsbHs::from_bits(val)
    }
}
impl From<RemapUsbHs> for u8 {
    #[inline(always)]
    fn from(val: RemapUsbHs) -> u8 {
        RemapUsbHs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Security {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Secure version. (All values other than 1010b represent the secure version.)."]
    SEC = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Non secure version."]
    NON_SEC = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Security {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Security {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Security {
    #[inline(always)]
    fn from(val: u8) -> Security {
        Security::from_bits(val)
    }
}
impl From<Security> for u8 {
    #[inline(always)]
    fn from(val: Security) -> u8 {
        Security::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Skew {
    #[doc = "TENMS value is exact."]
    EXACT = 0x0,
    #[doc = "TENMS value is not exact or not given."]
    INEXACT = 0x01,
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
pub enum SlowclkdivHalt {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stopped."]
    HALT = 0x01,
}
impl SlowclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SlowclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SlowclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> SlowclkdivHalt {
        SlowclkdivHalt::from_bits(val)
    }
}
impl From<SlowclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: SlowclkdivHalt) -> u8 {
        SlowclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SlowclkdivReset {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl SlowclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SlowclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SlowclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> SlowclkdivReset {
        SlowclkdivReset::from_bits(val)
    }
}
impl From<SlowclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: SlowclkdivReset) -> u8 {
        SlowclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SlowclkdivUnstab {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl SlowclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SlowclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SlowclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> SlowclkdivUnstab {
        SlowclkdivUnstab::from_bits(val)
    }
}
impl From<SlowclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: SlowclkdivUnstab) -> u8 {
        SlowclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmartDmaD {
    #[doc = "RAMX0: alias space is disabled."]
    DISABLE = 0x0,
    #[doc = "RAMX0: same alias space as CPU0_SBUS."]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SmartDmaD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmartDmaD {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmartDmaD {
    #[inline(always)]
    fn from(val: u8) -> SmartDmaD {
        SmartDmaD::from_bits(val)
    }
}
impl From<SmartDmaD> for u8 {
    #[inline(always)]
    fn from(val: SmartDmaD) -> u8 {
        SmartDmaD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmartDmaI {
    #[doc = "RAMX0: alias space is disabled."]
    DISABLE = 0x0,
    #[doc = "RAMX0: same alias space as CPU0_SBUS."]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SmartDmaI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmartDmaI {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmartDmaI {
    #[inline(always)]
    fn from(val: u8) -> SmartDmaI {
        SmartDmaI::from_bits(val)
    }
}
impl From<SmartDmaI> for u8 {
    #[inline(always)]
    fn from(val: SmartDmaI) -> u8 {
        SmartDmaI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Unlock {
    #[doc = "Updates are allowed to all clock configuration registers."]
    ENABLE = 0x0,
    #[doc = "Freezes all clock configuration registers update."]
    FREEZE = 0x01,
}
impl Unlock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Unlock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Unlock {
    #[inline(always)]
    fn from(val: u8) -> Unlock {
        Unlock::from_bits(val)
    }
}
impl From<Unlock> for u8 {
    #[inline(always)]
    fn from(val: Unlock) -> u8 {
        Unlock::to_bits(val)
    }
}
