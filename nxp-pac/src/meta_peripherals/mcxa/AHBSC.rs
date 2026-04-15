#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (859f02b 2026-04-15))"]
#[doc = "AHBSC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbsc {
    ptr: *mut u8,
}
unsafe impl Send for Ahbsc {}
unsafe impl Sync for Ahbsc {}
impl Ahbsc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "AHB Slave Port 0 Rule."]
    #[inline(always)]
    pub const fn ahb_slave_port_p0_slave_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<AhbSlavePortP0SlaveRule, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _)
        }
    }
    #[doc = "Flash Memory Rule."]
    #[inline(always)]
    pub const fn flash00_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Flash00MemRule, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 4usize) as _)
        }
    }
    #[doc = "Flash Memory Rule."]
    #[inline(always)]
    pub const fn flash01_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<Flash01MemRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Flash Memory Rule."]
    #[inline(always)]
    pub const fn flash02_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<Flash02MemRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "AHB Slave Port 1 Rule."]
    #[inline(always)]
    pub const fn ahb_slave_port_p1_slave_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<AhbSlavePortP1SlaveRule, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _)
        }
    }
    #[doc = "ROM Memory Rule."]
    #[inline(always)]
    pub const fn rom_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<RomMemRule, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize + n * 4usize) as _)
        }
    }
    #[doc = "AHB Slave Port 2 Rule."]
    #[inline(always)]
    pub const fn ahb_slave_port_p2_slave_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<AhbSlavePortP2SlaveRule, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 4usize) as _)
        }
    }
    #[doc = "RAMX Memory Rule."]
    #[inline(always)]
    pub const fn ramx_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<RamxMemRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "AHB Slave Port 3 Rule."]
    #[inline(always)]
    pub const fn ahb_slave_port_p3_slave_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<AhbSlavePortP3SlaveRule, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _)
        }
    }
    #[doc = "RAMA Memory Rule."]
    #[inline(always)]
    pub const fn rama_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<RamMemRule, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize + n * 4usize) as _)
        }
    }
    #[doc = "AHB Slave Port 4 Rule."]
    #[inline(always)]
    pub const fn ahb_slave_port_p4_slave_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<AhbSlavePortP4SlaveRule, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _)
        }
    }
    #[doc = "RAMB Memory Rule."]
    #[inline(always)]
    pub const fn ramb_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<RamMemRule, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize + n * 4usize) as _)
        }
    }
    #[doc = "AHB Peripheral Slave Port 5 Slave Rule 0."]
    #[inline(always)]
    pub const fn ahb_peripheral_slave_port_p5_slave_rule0(
        self,
    ) -> crate::pac::common::Reg<AhbPeripheralSlavePortP5SlaveRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "AHB Peripheral Slave Port 5 Slave Rule 1."]
    #[inline(always)]
    pub const fn ahb_peripheral_slave_port_p5_slave_rule1(
        self,
    ) -> crate::pac::common::Reg<AhbPeripheralSlavePortP5SlaveRule1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "APB Bridge Group 0 Memory Rule 0."]
    #[inline(always)]
    pub const fn apb_peripheral_group0_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<ApbPeripheralGroup0MemRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "APB Bridge Group 0 Memory Rule 1."]
    #[inline(always)]
    pub const fn apb_peripheral_group0_mem_rule1(
        self,
    ) -> crate::pac::common::Reg<ApbPeripheralGroup0MemRule1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "APB Bridge Group 0 Rule 2."]
    #[inline(always)]
    pub const fn apb_peripheral_group0_mem_rule2(
        self,
    ) -> crate::pac::common::Reg<u32, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "APB Bridge Group 0 Memory Rule 3."]
    #[inline(always)]
    pub const fn apb_peripheral_group0_mem_rule3(
        self,
    ) -> crate::pac::common::Reg<u32, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "AIPS Bridge Group 0 Memory Rule 0."]
    #[inline(always)]
    pub const fn aips_bridge_group0_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup0MemRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "AIPS Bridge Group 0 Memory Rule 1."]
    #[inline(always)]
    pub const fn aips_bridge_group0_mem_rule1(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup0MemRule1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "AIPS Bridge Group 0 Memory Rule 2."]
    #[inline(always)]
    pub const fn aips_bridge_group0_mem_rule2(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup0MemRule2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "AIPS Bridge Group 0 Memory Rule 3."]
    #[inline(always)]
    pub const fn aips_bridge_group0_mem_rule3(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup0MemRule3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Memory Rule 0."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2MemRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Memory Rule 1."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule1(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2MemRule1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Memory Rule 2."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule2(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2MemRule2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Memory Rule 3."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule3(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2MemRule3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "AIPS Bridge Group 3 Rule 0."]
    #[inline(always)]
    pub const fn aips_bridge_group3_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup3MemRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "AIPS Bridge Group 3 Memory Rule 1."]
    #[inline(always)]
    pub const fn aips_bridge_group3_mem_rule1(
        self,
    ) -> crate::pac::common::Reg<u32, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "AIPS Bridge Group 3 Rule 2."]
    #[inline(always)]
    pub const fn aips_bridge_group3_mem_rule2(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup3MemRule2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "AIPS Bridge Group 3 Rule 3."]
    #[inline(always)]
    pub const fn aips_bridge_group3_mem_rule3(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup3MemRule3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "AHB Secure Control Peripheral Rule 0."]
    #[inline(always)]
    pub const fn ahb_secure_ctrl_peripheral_rule0(
        self,
    ) -> crate::pac::common::Reg<AhbSecureCtrlPeripheralRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "AHB Slave Port 6 Rule."]
    #[inline(always)]
    pub const fn ahb_slave_port_p6_slave_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<AhbSlavePortP6SlaveRule, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize + n * 4usize) as _)
        }
    }
    #[doc = "AON Domain Peripheral Rule 0."]
    #[inline(always)]
    pub const fn aon_domain_peripheral_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<AonDomainPeripheralMemRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "AON Domain Peripheral Rule 1."]
    #[inline(always)]
    pub const fn aon_domain_peripheral_mem_rule1(
        self,
    ) -> crate::pac::common::Reg<AonDomainPeripheralMemRule1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "AON Domain Peripheral Rule 2."]
    #[inline(always)]
    pub const fn aon_domain_peripheral_mem_rule2(
        self,
    ) -> crate::pac::common::Reg<AonDomainPeripheralMemRule2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "AON Domain Peripheral Rule 3."]
    #[inline(always)]
    pub const fn aon_domain_peripheral_mem_rule3(
        self,
    ) -> crate::pac::common::Reg<AonDomainPeripheralMemRule3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "AHB Slave Port 6 Rule."]
    #[inline(always)]
    pub const fn ahb_slave_port_p7_slave_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<AhbSlavePortP7SlaveRule, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize + n * 4usize) as _)
        }
    }
    #[doc = "AON Domain SRAM Memory Rule."]
    #[inline(always)]
    pub const fn aon_domain_sram_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<AonDomainSramMemRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Security Violation Address."]
    #[inline(always)]
    pub const fn sec_vio_addr(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<SecVioAddr, crate::pac::common::R> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e00usize + n * 4usize) as _)
        }
    }
    #[doc = "Security Violation Miscellaneous Information at Address."]
    #[inline(always)]
    pub const fn sec_vio_misc_info(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<SecVioMiscInfo, crate::pac::common::R> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e80usize + n * 4usize) as _)
        }
    }
    #[doc = "Security Violation Info Validity for Address."]
    #[inline(always)]
    pub const fn sec_vio_info_valid(
        self,
    ) -> crate::pac::common::Reg<SecVioInfoValid, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f00usize) as _) }
    }
    #[doc = "Secure general purpose registers."]
    #[inline(always)]
    pub const fn sec_gp_reg(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<SecGpReg, crate::pac::common::RW> {
        assert!(n < 10usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f80usize + n * 4usize) as _)
        }
    }
    #[doc = "Master Secure Level."]
    #[inline(always)]
    pub const fn master_sec_level(
        self,
    ) -> crate::pac::common::Reg<MasterSecLevel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd0usize) as _) }
    }
    #[doc = "Master Secure Level."]
    #[inline(always)]
    pub const fn master_sec_anti_pol_reg(
        self,
    ) -> crate::pac::common::Reg<MasterSecAntiPolReg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd4usize) as _) }
    }
    #[doc = "Miscellaneous CPU0 Control Signals."]
    #[inline(always)]
    pub const fn cpu0_lock_reg(
        self,
    ) -> crate::pac::common::Reg<Cpu0LockReg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fecusize) as _) }
    }
    #[doc = "Secure Control Duplicate."]
    #[inline(always)]
    pub const fn misc_ctrl_dp_reg(
        self,
    ) -> crate::pac::common::Reg<MiscCtrlDpReg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
    #[doc = "Secure Control."]
    #[inline(always)]
    pub const fn misc_ctrl_reg(
        self,
    ) -> crate::pac::common::Reg<MiscCtrlReg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
#[doc = "AHB Peripheral Slave Port 5 Slave Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheralSlavePortP5SlaveRule0(pub u32);
impl AhbPeripheralSlavePortP5SlaveRule0 {
    #[doc = "AIPS4 slaves."]
    #[must_use]
    #[inline(always)]
    pub const fn aips4_slaves(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "AIPS4 slaves."]
    #[inline(always)]
    pub const fn set_aips4_slaves(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "CDOG0."]
    #[must_use]
    #[inline(always)]
    pub const fn cdog0(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CDOG0."]
    #[inline(always)]
    pub const fn set_cdog0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "DEBUG_MAILBOX."]
    #[must_use]
    #[inline(always)]
    pub const fn debug_mailbox(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DEBUG_MAILBOX."]
    #[inline(always)]
    pub const fn set_debug_mailbox(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "GPIO1."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "GPIO1."]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AhbPeripheralSlavePortP5SlaveRule0 {
    #[inline(always)]
    fn default() -> AhbPeripheralSlavePortP5SlaveRule0 {
        AhbPeripheralSlavePortP5SlaveRule0(0)
    }
}
impl core::fmt::Debug for AhbPeripheralSlavePortP5SlaveRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheralSlavePortP5SlaveRule0")
            .field("aips4_slaves", &self.aips4_slaves())
            .field("cdog0", &self.cdog0())
            .field("debug_mailbox", &self.debug_mailbox())
            .field("gpio1", &self.gpio1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheralSlavePortP5SlaveRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheralSlavePortP5SlaveRule0 {{ aips4_slaves: {:?}, cdog0: {:?}, debug_mailbox: {:?}, gpio1: {:?} }}",
            self.aips4_slaves(),
            self.cdog0(),
            self.debug_mailbox(),
            self.gpio1()
        )
    }
}
#[doc = "AHB Peripheral Slave Port 5 Slave Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheralSlavePortP5SlaveRule1(pub u32);
impl AhbPeripheralSlavePortP5SlaveRule1 {
    #[doc = "GPIO2."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "GPIO2."]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "GPIO3."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "GPIO3."]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CDOG1."]
    #[must_use]
    #[inline(always)]
    pub const fn cdog1(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CDOG1."]
    #[inline(always)]
    pub const fn set_cdog1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
}
impl Default for AhbPeripheralSlavePortP5SlaveRule1 {
    #[inline(always)]
    fn default() -> AhbPeripheralSlavePortP5SlaveRule1 {
        AhbPeripheralSlavePortP5SlaveRule1(0)
    }
}
impl core::fmt::Debug for AhbPeripheralSlavePortP5SlaveRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheralSlavePortP5SlaveRule1")
            .field("gpio2", &self.gpio2())
            .field("gpio3", &self.gpio3())
            .field("cdog1", &self.cdog1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheralSlavePortP5SlaveRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheralSlavePortP5SlaveRule1 {{ gpio2: {:?}, gpio3: {:?}, cdog1: {:?} }}",
            self.gpio2(),
            self.gpio3(),
            self.cdog1()
        )
    }
}
#[doc = "AHB Secure Control Peripheral Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbSecureCtrlPeripheralRule0(pub u32);
impl AhbSecureCtrlPeripheralRule0 {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Rule {
        assert!(n < 4usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: Rule) {
        assert!(n < 4usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for AhbSecureCtrlPeripheralRule0 {
    #[inline(always)]
    fn default() -> AhbSecureCtrlPeripheralRule0 {
        AhbSecureCtrlPeripheralRule0(0)
    }
}
impl core::fmt::Debug for AhbSecureCtrlPeripheralRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbSecureCtrlPeripheralRule0")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbSecureCtrlPeripheralRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbSecureCtrlPeripheralRule0 {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize)
        )
    }
}
#[doc = "AHB Slave Port 0 Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbSlavePortP0SlaveRule(pub u32);
impl AhbSlavePortP0SlaveRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Rule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: Rule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for AhbSlavePortP0SlaveRule {
    #[inline(always)]
    fn default() -> AhbSlavePortP0SlaveRule {
        AhbSlavePortP0SlaveRule(0)
    }
}
impl core::fmt::Debug for AhbSlavePortP0SlaveRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbSlavePortP0SlaveRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbSlavePortP0SlaveRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbSlavePortP0SlaveRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "AHB Slave Port 1 Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbSlavePortP1SlaveRule(pub u32);
impl AhbSlavePortP1SlaveRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Rule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: Rule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for AhbSlavePortP1SlaveRule {
    #[inline(always)]
    fn default() -> AhbSlavePortP1SlaveRule {
        AhbSlavePortP1SlaveRule(0)
    }
}
impl core::fmt::Debug for AhbSlavePortP1SlaveRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbSlavePortP1SlaveRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbSlavePortP1SlaveRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbSlavePortP1SlaveRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "AHB Slave Port 2 Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbSlavePortP2SlaveRule(pub u32);
impl AhbSlavePortP2SlaveRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Rule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: Rule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for AhbSlavePortP2SlaveRule {
    #[inline(always)]
    fn default() -> AhbSlavePortP2SlaveRule {
        AhbSlavePortP2SlaveRule(0)
    }
}
impl core::fmt::Debug for AhbSlavePortP2SlaveRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbSlavePortP2SlaveRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbSlavePortP2SlaveRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbSlavePortP2SlaveRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "AHB Slave Port 3 Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbSlavePortP3SlaveRule(pub u32);
impl AhbSlavePortP3SlaveRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Rule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: Rule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for AhbSlavePortP3SlaveRule {
    #[inline(always)]
    fn default() -> AhbSlavePortP3SlaveRule {
        AhbSlavePortP3SlaveRule(0)
    }
}
impl core::fmt::Debug for AhbSlavePortP3SlaveRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbSlavePortP3SlaveRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbSlavePortP3SlaveRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbSlavePortP3SlaveRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "AHB Slave Port 4 Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbSlavePortP4SlaveRule(pub u32);
impl AhbSlavePortP4SlaveRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Rule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: Rule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for AhbSlavePortP4SlaveRule {
    #[inline(always)]
    fn default() -> AhbSlavePortP4SlaveRule {
        AhbSlavePortP4SlaveRule(0)
    }
}
impl core::fmt::Debug for AhbSlavePortP4SlaveRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbSlavePortP4SlaveRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbSlavePortP4SlaveRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbSlavePortP4SlaveRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "AHB Slave Port 6 Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbSlavePortP6SlaveRule(pub u32);
impl AhbSlavePortP6SlaveRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Rule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: Rule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for AhbSlavePortP6SlaveRule {
    #[inline(always)]
    fn default() -> AhbSlavePortP6SlaveRule {
        AhbSlavePortP6SlaveRule(0)
    }
}
impl core::fmt::Debug for AhbSlavePortP6SlaveRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbSlavePortP6SlaveRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbSlavePortP6SlaveRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbSlavePortP6SlaveRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "AHB Slave Port 6 Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbSlavePortP7SlaveRule(pub u32);
impl AhbSlavePortP7SlaveRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Rule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: Rule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for AhbSlavePortP7SlaveRule {
    #[inline(always)]
    fn default() -> AhbSlavePortP7SlaveRule {
        AhbSlavePortP7SlaveRule(0)
    }
}
impl core::fmt::Debug for AhbSlavePortP7SlaveRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbSlavePortP7SlaveRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbSlavePortP7SlaveRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbSlavePortP7SlaveRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "AIPS Bridge Group 0 Memory Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup0MemRule0(pub u32);
impl AipsBridgeGroup0MemRule0 {
    #[doc = "eDMA_0_MP."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma_0_mp(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "eDMA_0_MP."]
    #[inline(always)]
    pub const fn set_e_dma_0_mp(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "eDMA_0_TCD0."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma_0_tcd0(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "eDMA_0_TCD0."]
    #[inline(always)]
    pub const fn set_e_dma_0_tcd0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "eDMA_0_TCD1."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma_0_tcd1(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "eDMA_0_TCD1."]
    #[inline(always)]
    pub const fn set_e_dma_0_tcd1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "eDMA_0_TCD2."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma_0_tcd2(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "eDMA_0_TCD2."]
    #[inline(always)]
    pub const fn set_e_dma_0_tcd2(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "FLEXSPI0 Registers."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma_0_tcd3(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "FLEXSPI0 Registers."]
    #[inline(always)]
    pub const fn set_e_dma_0_tcd3(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "eDMA_0_TCD4 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma_0_tcd4(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "eDMA_0_TCD4 (Reserved)."]
    #[inline(always)]
    pub const fn set_e_dma_0_tcd4(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "eDMA_0_TCD5 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma_0_tcd5(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "eDMA_0_TCD5 (Reserved)."]
    #[inline(always)]
    pub const fn set_e_dma_0_tcd5(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "eDMA_0_TCD6 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma_0_tcd6(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "eDMA_0_TCD6 (Reserved)."]
    #[inline(always)]
    pub const fn set_e_dma_0_tcd6(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup0MemRule0 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup0MemRule0 {
        AipsBridgeGroup0MemRule0(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup0MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup0MemRule0")
            .field("e_dma_0_mp", &self.e_dma_0_mp())
            .field("e_dma_0_tcd0", &self.e_dma_0_tcd0())
            .field("e_dma_0_tcd1", &self.e_dma_0_tcd1())
            .field("e_dma_0_tcd2", &self.e_dma_0_tcd2())
            .field("e_dma_0_tcd3", &self.e_dma_0_tcd3())
            .field("e_dma_0_tcd4", &self.e_dma_0_tcd4())
            .field("e_dma_0_tcd5", &self.e_dma_0_tcd5())
            .field("e_dma_0_tcd6", &self.e_dma_0_tcd6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup0MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup0MemRule0 {{ e_dma_0_mp: {:?}, e_dma_0_tcd0: {:?}, e_dma_0_tcd1: {:?}, e_dma_0_tcd2: {:?}, e_dma_0_tcd3: {:?}, e_dma_0_tcd4: {:?}, e_dma_0_tcd5: {:?}, e_dma_0_tcd6: {:?} }}",
            self.e_dma_0_mp(),
            self.e_dma_0_tcd0(),
            self.e_dma_0_tcd1(),
            self.e_dma_0_tcd2(),
            self.e_dma_0_tcd3(),
            self.e_dma_0_tcd4(),
            self.e_dma_0_tcd5(),
            self.e_dma_0_tcd6()
        )
    }
}
#[doc = "AIPS Bridge Group 0 Memory Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup0MemRule1(pub u32);
impl AipsBridgeGroup0MemRule1 {
    #[doc = "eDMA_0_TCD7 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma_0_tcd7(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "eDMA_0_TCD7 (Reserved)."]
    #[inline(always)]
    pub const fn set_e_dma_0_tcd7(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "AOI0."]
    #[must_use]
    #[inline(always)]
    pub const fn aoi0(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "AOI0."]
    #[inline(always)]
    pub const fn set_aoi0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CRC0."]
    #[must_use]
    #[inline(always)]
    pub const fn crc0(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CRC0."]
    #[inline(always)]
    pub const fn set_crc0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CMC0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmc0(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CMC0."]
    #[inline(always)]
    pub const fn set_cmc0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "ERM0."]
    #[must_use]
    #[inline(always)]
    pub const fn erm0(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "ERM0."]
    #[inline(always)]
    pub const fn set_erm0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "MBC0."]
    #[must_use]
    #[inline(always)]
    pub const fn mbc0(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "MBC0."]
    #[inline(always)]
    pub const fn set_mbc0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "SCG0."]
    #[must_use]
    #[inline(always)]
    pub const fn scg0(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "SCG0."]
    #[inline(always)]
    pub const fn set_scg0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup0MemRule1 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup0MemRule1 {
        AipsBridgeGroup0MemRule1(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup0MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup0MemRule1")
            .field("e_dma_0_tcd7", &self.e_dma_0_tcd7())
            .field("aoi0", &self.aoi0())
            .field("crc0", &self.crc0())
            .field("cmc0", &self.cmc0())
            .field("erm0", &self.erm0())
            .field("mbc0", &self.mbc0())
            .field("scg0", &self.scg0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup0MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup0MemRule1 {{ e_dma_0_tcd7: {:?}, aoi0: {:?}, crc0: {:?}, cmc0: {:?}, erm0: {:?}, mbc0: {:?}, scg0: {:?} }}",
            self.e_dma_0_tcd7(),
            self.aoi0(),
            self.crc0(),
            self.cmc0(),
            self.erm0(),
            self.mbc0(),
            self.scg0()
        )
    }
}
#[doc = "AIPS Bridge Group 0 Memory Rule 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup0MemRule2(pub u32);
impl AipsBridgeGroup0MemRule2 {
    #[doc = "SYSCON."]
    #[must_use]
    #[inline(always)]
    pub const fn syscon(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "SYSCON."]
    #[inline(always)]
    pub const fn set_syscon(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "WUU0."]
    #[must_use]
    #[inline(always)]
    pub const fn wuu0(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "WUU0."]
    #[inline(always)]
    pub const fn set_wuu0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "FMC0."]
    #[must_use]
    #[inline(always)]
    pub const fn fmc0(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "FMC0."]
    #[inline(always)]
    pub const fn set_fmc0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "FMU0."]
    #[must_use]
    #[inline(always)]
    pub const fn fmu0(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "FMU0."]
    #[inline(always)]
    pub const fn set_fmu0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "FMU0_TST."]
    #[must_use]
    #[inline(always)]
    pub const fn fmu0_tst(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "FMU0_TST."]
    #[inline(always)]
    pub const fn set_fmu0_tst(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "BCANXL (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn bcanxl(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "BCANXL (Reserved)."]
    #[inline(always)]
    pub const fn set_bcanxl(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup0MemRule2 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup0MemRule2 {
        AipsBridgeGroup0MemRule2(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup0MemRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup0MemRule2")
            .field("syscon", &self.syscon())
            .field("wuu0", &self.wuu0())
            .field("fmc0", &self.fmc0())
            .field("fmu0", &self.fmu0())
            .field("fmu0_tst", &self.fmu0_tst())
            .field("bcanxl", &self.bcanxl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup0MemRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup0MemRule2 {{ syscon: {:?}, wuu0: {:?}, fmc0: {:?}, fmu0: {:?}, fmu0_tst: {:?}, bcanxl: {:?} }}",
            self.syscon(),
            self.wuu0(),
            self.fmc0(),
            self.fmu0(),
            self.fmu0_tst(),
            self.bcanxl()
        )
    }
}
#[doc = "AIPS Bridge Group 0 Memory Rule 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup0MemRule3(pub u32);
impl AipsBridgeGroup0MemRule3 {
    #[doc = "CANDMA (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn candma(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CANDMA (Reserved)."]
    #[inline(always)]
    pub const fn set_candma(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "FLEXIO (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "FLEXIO (Reserved)."]
    #[inline(always)]
    pub const fn set_flexio(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "LPI2C0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c0(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPI2C0."]
    #[inline(always)]
    pub const fn set_lpi2c0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LPI2C1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPI2C1."]
    #[inline(always)]
    pub const fn set_lpi2c1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "LPSPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi0(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPSPI0."]
    #[inline(always)]
    pub const fn set_lpspi0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "LPSPI1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPSPI1."]
    #[inline(always)]
    pub const fn set_lpspi1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "LPSPI2 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi2(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPSPI2 (Reserved)."]
    #[inline(always)]
    pub const fn set_lpspi2(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "LPUART0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart0(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPUART0."]
    #[inline(always)]
    pub const fn set_lpuart0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup0MemRule3 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup0MemRule3 {
        AipsBridgeGroup0MemRule3(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup0MemRule3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup0MemRule3")
            .field("candma", &self.candma())
            .field("flexio", &self.flexio())
            .field("lpi2c0", &self.lpi2c0())
            .field("lpi2c1", &self.lpi2c1())
            .field("lpspi0", &self.lpspi0())
            .field("lpspi1", &self.lpspi1())
            .field("lpspi2", &self.lpspi2())
            .field("lpuart0", &self.lpuart0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup0MemRule3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup0MemRule3 {{ candma: {:?}, flexio: {:?}, lpi2c0: {:?}, lpi2c1: {:?}, lpspi0: {:?}, lpspi1: {:?}, lpspi2: {:?}, lpuart0: {:?} }}",
            self.candma(),
            self.flexio(),
            self.lpi2c0(),
            self.lpi2c1(),
            self.lpspi0(),
            self.lpspi1(),
            self.lpspi2(),
            self.lpuart0()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Memory Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule0(pub u32);
impl AipsBridgeGroup2MemRule0 {
    #[doc = "LPUART1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPUART1."]
    #[inline(always)]
    pub const fn set_lpuart1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "LPUART2 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPUART2 (Reserved)."]
    #[inline(always)]
    pub const fn set_lpuart2(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "LPUART3 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPUART3 (Reserved)."]
    #[inline(always)]
    pub const fn set_lpuart3(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LPUART4 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPUART4 (Reserved)."]
    #[inline(always)]
    pub const fn set_lpuart4(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "USB_FS_OTG (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_fs_otg(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "USB_FS_OTG (Reserved)."]
    #[inline(always)]
    pub const fn set_usb_fs_otg(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "USB_FS_DCD (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_fs_dcd(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "USB_FS_DCD (Reserved)."]
    #[inline(always)]
    pub const fn set_usb_fs_dcd(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "USB_TYPEC_PD (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_typec_pd(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "USB_TYPEC_PD (Reserved)."]
    #[inline(always)]
    pub const fn set_usb_typec_pd(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "QUAD_DECODER0 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn quad_decoder0(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "QUAD_DECODER0 (Reserved)."]
    #[inline(always)]
    pub const fn set_quad_decoder0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2MemRule0 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule0 {
        AipsBridgeGroup2MemRule0(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule0")
            .field("lpuart1", &self.lpuart1())
            .field("lpuart2", &self.lpuart2())
            .field("lpuart3", &self.lpuart3())
            .field("lpuart4", &self.lpuart4())
            .field("usb_fs_otg", &self.usb_fs_otg())
            .field("usb_fs_dcd", &self.usb_fs_dcd())
            .field("usb_typec_pd", &self.usb_typec_pd())
            .field("quad_decoder0", &self.quad_decoder0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule0 {{ lpuart1: {:?}, lpuart2: {:?}, lpuart3: {:?}, lpuart4: {:?}, usb_fs_otg: {:?}, usb_fs_dcd: {:?}, usb_typec_pd: {:?}, quad_decoder0: {:?} }}",
            self.lpuart1(),
            self.lpuart2(),
            self.lpuart3(),
            self.lpuart4(),
            self.usb_fs_otg(),
            self.usb_fs_dcd(),
            self.usb_typec_pd(),
            self.quad_decoder0()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Memory Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule1(pub u32);
impl AipsBridgeGroup2MemRule1 {
    #[doc = "QUAD_DECODER1 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn quad_decoder1(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "QUAD_DECODER1 (Reserved)."]
    #[inline(always)]
    pub const fn set_quad_decoder1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "FLEXPWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm0(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "FLEXPWM0."]
    #[inline(always)]
    pub const fn set_flexpwm0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "FLEXPWM1 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn flexpwm1(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "FLEXPWM1 (Reserved)."]
    #[inline(always)]
    pub const fn set_flexpwm1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LPTMR0 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn lptmr0(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPTMR0 (Reserved)."]
    #[inline(always)]
    pub const fn set_lptmr0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "LPTMR1 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn lptmr1(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPTMR1 (Reserved)."]
    #[inline(always)]
    pub const fn set_lptmr1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "OSTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer0(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "OSTIMER0."]
    #[inline(always)]
    pub const fn set_ostimer0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "WAKE_TIMER (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn wake_timer(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "WAKE_TIMER (Reserved)."]
    #[inline(always)]
    pub const fn set_wake_timer(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "HSADC0."]
    #[must_use]
    #[inline(always)]
    pub const fn hsadc0(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "HSADC0."]
    #[inline(always)]
    pub const fn set_hsadc0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2MemRule1 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule1 {
        AipsBridgeGroup2MemRule1(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule1")
            .field("quad_decoder1", &self.quad_decoder1())
            .field("flexpwm0", &self.flexpwm0())
            .field("flexpwm1", &self.flexpwm1())
            .field("lptmr0", &self.lptmr0())
            .field("lptmr1", &self.lptmr1())
            .field("ostimer0", &self.ostimer0())
            .field("wake_timer", &self.wake_timer())
            .field("hsadc0", &self.hsadc0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule1 {{ quad_decoder1: {:?}, flexpwm0: {:?}, flexpwm1: {:?}, lptmr0: {:?}, lptmr1: {:?}, ostimer0: {:?}, wake_timer: {:?}, hsadc0: {:?} }}",
            self.quad_decoder1(),
            self.flexpwm0(),
            self.flexpwm1(),
            self.lptmr0(),
            self.lptmr1(),
            self.ostimer0(),
            self.wake_timer(),
            self.hsadc0()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Memory Rule 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule2(pub u32);
impl AipsBridgeGroup2MemRule2 {
    #[doc = "ADC1 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "ADC1 (Reserved)."]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp0(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CMP0."]
    #[inline(always)]
    pub const fn set_cmp0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CMP1 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp1(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CMP1 (Reserved)."]
    #[inline(always)]
    pub const fn set_cmp1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CMP2 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp2(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CMP2 (Reserved)."]
    #[inline(always)]
    pub const fn set_cmp2(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "DAC0 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DAC0 (Reserved)."]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "DAC1 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn dac1(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DAC1 (Reserved)."]
    #[inline(always)]
    pub const fn set_dac1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "DAC2 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn dac2(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DAC2 (Reserved)."]
    #[inline(always)]
    pub const fn set_dac2(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "OPAMP0 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp0(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "OPAMP0 (Reserved)."]
    #[inline(always)]
    pub const fn set_opamp0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2MemRule2 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule2 {
        AipsBridgeGroup2MemRule2(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule2")
            .field("adc1", &self.adc1())
            .field("cmp0", &self.cmp0())
            .field("cmp1", &self.cmp1())
            .field("cmp2", &self.cmp2())
            .field("dac0", &self.dac0())
            .field("dac1", &self.dac1())
            .field("dac2", &self.dac2())
            .field("opamp0", &self.opamp0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule2 {{ adc1: {:?}, cmp0: {:?}, cmp1: {:?}, cmp2: {:?}, dac0: {:?}, dac1: {:?}, dac2: {:?}, opamp0: {:?} }}",
            self.adc1(),
            self.cmp0(),
            self.cmp1(),
            self.cmp2(),
            self.dac0(),
            self.dac1(),
            self.dac2(),
            self.opamp0()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Memory Rule 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule3(pub u32);
impl AipsBridgeGroup2MemRule3 {
    #[doc = "OPAMP1 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp1(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "OPAMP1 (Reserved)."]
    #[inline(always)]
    pub const fn set_opamp1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "OPAMP2 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp2(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "OPAMP2 (Reserved)."]
    #[inline(always)]
    pub const fn set_opamp2(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "SD_ADC0 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn sd_adc0(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "SD_ADC0 (Reserved)."]
    #[inline(always)]
    pub const fn set_sd_adc0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "DGO_GPIO1_3 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn dgo_gpio1_3(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "DGO_GPIO1_3 (Reserved)."]
    #[inline(always)]
    pub const fn set_dgo_gpio1_3(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "PORT1."]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "PORT1."]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "PORT2."]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "PORT2."]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "PORT3."]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "PORT3."]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "PORT4 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "PORT4 (Reserved)."]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2MemRule3 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule3 {
        AipsBridgeGroup2MemRule3(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule3")
            .field("opamp1", &self.opamp1())
            .field("opamp2", &self.opamp2())
            .field("sd_adc0", &self.sd_adc0())
            .field("dgo_gpio1_3", &self.dgo_gpio1_3())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule3 {{ opamp1: {:?}, opamp2: {:?}, sd_adc0: {:?}, dgo_gpio1_3: {:?}, port1: {:?}, port2: {:?}, port3: {:?}, port4: {:?} }}",
            self.opamp1(),
            self.opamp2(),
            self.sd_adc0(),
            self.dgo_gpio1_3(),
            self.port1(),
            self.port2(),
            self.port3(),
            self.port4()
        )
    }
}
#[doc = "AIPS Bridge Group 3 Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup3MemRule0(pub u32);
impl AipsBridgeGroup3MemRule0 {
    #[doc = "PORT5 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn port5(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "PORT5 (Reserved)."]
    #[inline(always)]
    pub const fn set_port5(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "SAI (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn sai(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "SAI (Reserved)."]
    #[inline(always)]
    pub const fn set_sai(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "SEGMENT_LCD (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn segment_lcd(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "SEGMENT_LCD (Reserved)."]
    #[inline(always)]
    pub const fn set_segment_lcd(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "TSI (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn tsi(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "TSI (Reserved)."]
    #[inline(always)]
    pub const fn set_tsi(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "MU0."]
    #[must_use]
    #[inline(always)]
    pub const fn mu0(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "MU0."]
    #[inline(always)]
    pub const fn set_mu0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "MU1 (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn mu1(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "MU1 (Reserved)."]
    #[inline(always)]
    pub const fn set_mu1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
}
impl Default for AipsBridgeGroup3MemRule0 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup3MemRule0 {
        AipsBridgeGroup3MemRule0(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup3MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup3MemRule0")
            .field("port5", &self.port5())
            .field("sai", &self.sai())
            .field("segment_lcd", &self.segment_lcd())
            .field("tsi", &self.tsi())
            .field("mu0", &self.mu0())
            .field("mu1", &self.mu1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup3MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup3MemRule0 {{ port5: {:?}, sai: {:?}, segment_lcd: {:?}, tsi: {:?}, mu0: {:?}, mu1: {:?} }}",
            self.port5(),
            self.sai(),
            self.segment_lcd(),
            self.tsi(),
            self.mu0(),
            self.mu1()
        )
    }
}
#[doc = "AIPS Bridge Group 3 Rule 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup3MemRule2(pub u32);
impl AipsBridgeGroup3MemRule2 {
    #[doc = "PKC0."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc0(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "PKC0."]
    #[inline(always)]
    pub const fn set_pkc0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "SGI0."]
    #[must_use]
    #[inline(always)]
    pub const fn sgi0(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "SGI0."]
    #[inline(always)]
    pub const fn set_sgi0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "TRNG0."]
    #[must_use]
    #[inline(always)]
    pub const fn trng0(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "TRNG0."]
    #[inline(always)]
    pub const fn set_trng0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CAN0 RULE0."]
    #[must_use]
    #[inline(always)]
    pub const fn udf0(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CAN0 RULE0."]
    #[inline(always)]
    pub const fn set_udf0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "eDMA_1_MP."]
    #[must_use]
    #[inline(always)]
    pub const fn e_mda_1_mp(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "eDMA_1_MP."]
    #[inline(always)]
    pub const fn set_e_mda_1_mp(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup3MemRule2 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup3MemRule2 {
        AipsBridgeGroup3MemRule2(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup3MemRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup3MemRule2")
            .field("pkc0", &self.pkc0())
            .field("sgi0", &self.sgi0())
            .field("trng0", &self.trng0())
            .field("udf0", &self.udf0())
            .field("e_mda_1_mp", &self.e_mda_1_mp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup3MemRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup3MemRule2 {{ pkc0: {:?}, sgi0: {:?}, trng0: {:?}, udf0: {:?}, e_mda_1_mp: {:?} }}",
            self.pkc0(),
            self.sgi0(),
            self.trng0(),
            self.udf0(),
            self.e_mda_1_mp()
        )
    }
}
#[doc = "AIPS Bridge Group 3 Rule 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup3MemRule3(pub u32);
impl AipsBridgeGroup3MemRule3 {
    #[doc = "eDMA_1_TCD0."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma_1_tcd0(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "eDMA_1_TCD0."]
    #[inline(always)]
    pub const fn set_e_dma_1_tcd0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "eDMA_1_TCD1."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma_1_tcd1(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "eDMA_1_TCD1."]
    #[inline(always)]
    pub const fn set_e_dma_1_tcd1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "eDMA_1_TCD2."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma_1_tcd2(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "eDMA_1_TCD2."]
    #[inline(always)]
    pub const fn set_e_dma_1_tcd2(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "eDMA_1_TCD3."]
    #[must_use]
    #[inline(always)]
    pub const fn e_dma_1_tcd3(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "eDMA_1_TCD3."]
    #[inline(always)]
    pub const fn set_e_dma_1_tcd3(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for AipsBridgeGroup3MemRule3 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup3MemRule3 {
        AipsBridgeGroup3MemRule3(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup3MemRule3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup3MemRule3")
            .field("e_dma_1_tcd0", &self.e_dma_1_tcd0())
            .field("e_dma_1_tcd1", &self.e_dma_1_tcd1())
            .field("e_dma_1_tcd2", &self.e_dma_1_tcd2())
            .field("e_dma_1_tcd3", &self.e_dma_1_tcd3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup3MemRule3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup3MemRule3 {{ e_dma_1_tcd0: {:?}, e_dma_1_tcd1: {:?}, e_dma_1_tcd2: {:?}, e_dma_1_tcd3: {:?} }}",
            self.e_dma_1_tcd0(),
            self.e_dma_1_tcd1(),
            self.e_dma_1_tcd2(),
            self.e_dma_1_tcd3()
        )
    }
}
#[doc = "AON Domain Peripheral Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AonDomainPeripheralMemRule0(pub u32);
impl AonDomainPeripheralMemRule0 {
    #[doc = "LPI2C."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPI2C."]
    #[inline(always)]
    pub const fn set_lpi2c(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "LPI2C (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPI2C (Reserved)."]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "LPUART."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPUART."]
    #[inline(always)]
    pub const fn set_lpuart(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LPUART (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPUART (Reserved)."]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "MUB."]
    #[must_use]
    #[inline(always)]
    pub const fn mub(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "MUB."]
    #[inline(always)]
    pub const fn set_mub(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "PORT0."]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "PORT0."]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "ACMP."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "ACMP."]
    #[inline(always)]
    pub const fn set_acmp(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "ACMP (Reserved)."]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "ACMP (Reserved)."]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AonDomainPeripheralMemRule0 {
    #[inline(always)]
    fn default() -> AonDomainPeripheralMemRule0 {
        AonDomainPeripheralMemRule0(0)
    }
}
impl core::fmt::Debug for AonDomainPeripheralMemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AonDomainPeripheralMemRule0")
            .field("lpi2c", &self.lpi2c())
            .field("rule1", &self.rule1())
            .field("lpuart", &self.lpuart())
            .field("rule3", &self.rule3())
            .field("mub", &self.mub())
            .field("port0", &self.port0())
            .field("acmp", &self.acmp())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AonDomainPeripheralMemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AonDomainPeripheralMemRule0 {{ lpi2c: {:?}, rule1: {:?}, lpuart: {:?}, rule3: {:?}, mub: {:?}, port0: {:?}, acmp: {:?}, rule7: {:?} }}",
            self.lpi2c(),
            self.rule1(),
            self.lpuart(),
            self.rule3(),
            self.mub(),
            self.port0(),
            self.acmp(),
            self.rule7()
        )
    }
}
#[doc = "AON Domain Peripheral Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AonDomainPeripheralMemRule1(pub u32);
impl AonDomainPeripheralMemRule1 {
    #[doc = "LPTIMER."]
    #[must_use]
    #[inline(always)]
    pub const fn lptimer(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPTIMER."]
    #[inline(always)]
    pub const fn set_lptimer(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "QTMR32_0."]
    #[must_use]
    #[inline(always)]
    pub const fn qtmr32_0(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "QTMR32_0."]
    #[inline(always)]
    pub const fn set_qtmr32_0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "QTMR32_1."]
    #[must_use]
    #[inline(always)]
    pub const fn qtmr32_1(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "QTMR32_1."]
    #[inline(always)]
    pub const fn set_qtmr32_1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AonDomainPeripheralMemRule1 {
    #[inline(always)]
    fn default() -> AonDomainPeripheralMemRule1 {
        AonDomainPeripheralMemRule1(0)
    }
}
impl core::fmt::Debug for AonDomainPeripheralMemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AonDomainPeripheralMemRule1")
            .field("lptimer", &self.lptimer())
            .field("qtmr32_0", &self.qtmr32_0())
            .field("qtmr32_1", &self.qtmr32_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AonDomainPeripheralMemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AonDomainPeripheralMemRule1 {{ lptimer: {:?}, qtmr32_0: {:?}, qtmr32_1: {:?} }}",
            self.lptimer(),
            self.qtmr32_0(),
            self.qtmr32_1()
        )
    }
}
#[doc = "AON Domain Peripheral Rule 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AonDomainPeripheralMemRule2(pub u32);
impl AonDomainPeripheralMemRule2 {
    #[doc = "GP_GPR."]
    #[must_use]
    #[inline(always)]
    pub const fn gp_gpr(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "GP_GPR."]
    #[inline(always)]
    pub const fn set_gp_gpr(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "INPUTMUX."]
    #[must_use]
    #[inline(always)]
    pub const fn inputmux(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "INPUTMUX."]
    #[inline(always)]
    pub const fn set_inputmux(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "LPADC."]
    #[must_use]
    #[inline(always)]
    pub const fn lpadc(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPADC."]
    #[inline(always)]
    pub const fn set_lpadc(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "SGLCD0."]
    #[must_use]
    #[inline(always)]
    pub const fn sglcd(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "SGLCD0."]
    #[inline(always)]
    pub const fn set_sglcd(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "KPP."]
    #[must_use]
    #[inline(always)]
    pub const fn kpp(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "KPP."]
    #[inline(always)]
    pub const fn set_kpp(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "KPP."]
    #[must_use]
    #[inline(always)]
    pub const fn advc2(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "KPP."]
    #[inline(always)]
    pub const fn set_advc2(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AonDomainPeripheralMemRule2 {
    #[inline(always)]
    fn default() -> AonDomainPeripheralMemRule2 {
        AonDomainPeripheralMemRule2(0)
    }
}
impl core::fmt::Debug for AonDomainPeripheralMemRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AonDomainPeripheralMemRule2")
            .field("gp_gpr", &self.gp_gpr())
            .field("inputmux", &self.inputmux())
            .field("lpadc", &self.lpadc())
            .field("sglcd", &self.sglcd())
            .field("kpp", &self.kpp())
            .field("advc2", &self.advc2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AonDomainPeripheralMemRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AonDomainPeripheralMemRule2 {{ gp_gpr: {:?}, inputmux: {:?}, lpadc: {:?}, sglcd: {:?}, kpp: {:?}, advc2: {:?} }}",
            self.gp_gpr(),
            self.inputmux(),
            self.lpadc(),
            self.sglcd(),
            self.kpp(),
            self.advc2()
        )
    }
}
#[doc = "AON Domain Peripheral Rule 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AonDomainPeripheralMemRule3(pub u32);
impl AonDomainPeripheralMemRule3 {
    #[doc = "PMIC_CTRL."]
    #[must_use]
    #[inline(always)]
    pub const fn pmic_ctrl(&self) -> Rule {
        let val = (self.0 >> 0usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "PMIC_CTRL."]
    #[inline(always)]
    pub const fn set_pmic_ctrl(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CGU."]
    #[must_use]
    #[inline(always)]
    pub const fn cgu(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CGU."]
    #[inline(always)]
    pub const fn set_cgu(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "SMM."]
    #[must_use]
    #[inline(always)]
    pub const fn smm(&self) -> Rule {
        let val = (self.0 >> 8usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "SMM."]
    #[inline(always)]
    pub const fn set_smm(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LPCMP."]
    #[must_use]
    #[inline(always)]
    pub const fn lpcmp(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "LPCMP."]
    #[inline(always)]
    pub const fn set_lpcmp(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "AON_GPIO."]
    #[must_use]
    #[inline(always)]
    pub const fn aon_gpio(&self) -> Rule {
        let val = (self.0 >> 28usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "AON_GPIO."]
    #[inline(always)]
    pub const fn set_aon_gpio(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AonDomainPeripheralMemRule3 {
    #[inline(always)]
    fn default() -> AonDomainPeripheralMemRule3 {
        AonDomainPeripheralMemRule3(0)
    }
}
impl core::fmt::Debug for AonDomainPeripheralMemRule3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AonDomainPeripheralMemRule3")
            .field("pmic_ctrl", &self.pmic_ctrl())
            .field("cgu", &self.cgu())
            .field("smm", &self.smm())
            .field("lpcmp", &self.lpcmp())
            .field("aon_gpio", &self.aon_gpio())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AonDomainPeripheralMemRule3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AonDomainPeripheralMemRule3 {{ pmic_ctrl: {:?}, cgu: {:?}, smm: {:?}, lpcmp: {:?}, aon_gpio: {:?} }}",
            self.pmic_ctrl(),
            self.cgu(),
            self.smm(),
            self.lpcmp(),
            self.aon_gpio()
        )
    }
}
#[doc = "AON Domain SRAM Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AonDomainSramMemRule0(pub u32);
impl AonDomainSramMemRule0 {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Rule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: Rule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for AonDomainSramMemRule0 {
    #[inline(always)]
    fn default() -> AonDomainSramMemRule0 {
        AonDomainSramMemRule0(0)
    }
}
impl core::fmt::Debug for AonDomainSramMemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AonDomainSramMemRule0")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AonDomainSramMemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AonDomainSramMemRule0 {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "APB Bridge Group 0 Memory Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbPeripheralGroup0MemRule0(pub u32);
impl ApbPeripheralGroup0MemRule0 {
    #[doc = "INPUTMUX0."]
    #[must_use]
    #[inline(always)]
    pub const fn inputmux0(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "INPUTMUX0."]
    #[inline(always)]
    pub const fn set_inputmux0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CTIMER0."]
    #[inline(always)]
    pub const fn set_ctimer0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "CTIMER1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1(&self) -> Rule {
        let val = (self.0 >> 20usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CTIMER1."]
    #[inline(always)]
    pub const fn set_ctimer1(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "CTIMER2."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2(&self) -> Rule {
        let val = (self.0 >> 24usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "CTIMER2."]
    #[inline(always)]
    pub const fn set_ctimer2(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for ApbPeripheralGroup0MemRule0 {
    #[inline(always)]
    fn default() -> ApbPeripheralGroup0MemRule0 {
        ApbPeripheralGroup0MemRule0(0)
    }
}
impl core::fmt::Debug for ApbPeripheralGroup0MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbPeripheralGroup0MemRule0")
            .field("inputmux0", &self.inputmux0())
            .field("ctimer0", &self.ctimer0())
            .field("ctimer1", &self.ctimer1())
            .field("ctimer2", &self.ctimer2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbPeripheralGroup0MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbPeripheralGroup0MemRule0 {{ inputmux0: {:?}, ctimer0: {:?}, ctimer1: {:?}, ctimer2: {:?} }}",
            self.inputmux0(),
            self.ctimer0(),
            self.ctimer1(),
            self.ctimer2()
        )
    }
}
#[doc = "APB Bridge Group 0 Memory Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbPeripheralGroup0MemRule1(pub u32);
impl ApbPeripheralGroup0MemRule1 {
    #[doc = "FREQME00."]
    #[must_use]
    #[inline(always)]
    pub const fn freqme00(&self) -> Rule {
        let val = (self.0 >> 4usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "FREQME00."]
    #[inline(always)]
    pub const fn set_freqme00(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "UTICK0."]
    #[must_use]
    #[inline(always)]
    pub const fn utick0(&self) -> Rule {
        let val = (self.0 >> 12usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "UTICK0."]
    #[inline(always)]
    pub const fn set_utick0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "WWDT0."]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> Rule {
        let val = (self.0 >> 16usize) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "WWDT0."]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: Rule) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
}
impl Default for ApbPeripheralGroup0MemRule1 {
    #[inline(always)]
    fn default() -> ApbPeripheralGroup0MemRule1 {
        ApbPeripheralGroup0MemRule1(0)
    }
}
impl core::fmt::Debug for ApbPeripheralGroup0MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbPeripheralGroup0MemRule1")
            .field("freqme00", &self.freqme00())
            .field("utick0", &self.utick0())
            .field("wwdt0", &self.wwdt0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbPeripheralGroup0MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbPeripheralGroup0MemRule1 {{ freqme00: {:?}, utick0: {:?}, wwdt0: {:?} }}",
            self.freqme00(),
            self.utick0(),
            self.wwdt0()
        )
    }
}
#[doc = "Miscellaneous CPU0 Control Signals."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu0LockReg(pub u32);
impl Cpu0LockReg {
    #[doc = "LOCK_NS_VTOR."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ns_vtor(&self) -> LockNsVtor {
        let val = (self.0 >> 0usize) & 0x03;
        LockNsVtor::from_bits(val as u8)
    }
    #[doc = "LOCK_NS_VTOR."]
    #[inline(always)]
    pub const fn set_lock_ns_vtor(&mut self, val: LockNsVtor) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "LOCK_NS_MPU."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ns_mpu(&self) -> LockNsMpu {
        let val = (self.0 >> 2usize) & 0x03;
        LockNsMpu::from_bits(val as u8)
    }
    #[doc = "LOCK_NS_MPU."]
    #[inline(always)]
    pub const fn set_lock_ns_mpu(&mut self, val: LockNsMpu) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "LOCK_S_VTAIRCR."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_s_vtaircr(&self) -> LockSVtaircr {
        let val = (self.0 >> 4usize) & 0x03;
        LockSVtaircr::from_bits(val as u8)
    }
    #[doc = "LOCK_S_VTAIRCR."]
    #[inline(always)]
    pub const fn set_lock_s_vtaircr(&mut self, val: LockSVtaircr) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "LOCK_S_MPU."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_s_mpu(&self) -> LockSMpu {
        let val = (self.0 >> 6usize) & 0x03;
        LockSMpu::from_bits(val as u8)
    }
    #[doc = "LOCK_S_MPU."]
    #[inline(always)]
    pub const fn set_lock_s_mpu(&mut self, val: LockSMpu) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "LOCK_SAU."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_sau(&self) -> LockSau {
        let val = (self.0 >> 8usize) & 0x03;
        LockSau::from_bits(val as u8)
    }
    #[doc = "LOCK_SAU."]
    #[inline(always)]
    pub const fn set_lock_sau(&mut self, val: LockSau) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
}
impl Default for Cpu0LockReg {
    #[inline(always)]
    fn default() -> Cpu0LockReg {
        Cpu0LockReg(0)
    }
}
impl core::fmt::Debug for Cpu0LockReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpu0LockReg")
            .field("lock_ns_vtor", &self.lock_ns_vtor())
            .field("lock_ns_mpu", &self.lock_ns_mpu())
            .field("lock_s_vtaircr", &self.lock_s_vtaircr())
            .field("lock_s_mpu", &self.lock_s_mpu())
            .field("lock_sau", &self.lock_sau())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpu0LockReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpu0LockReg {{ lock_ns_vtor: {:?}, lock_ns_mpu: {:?}, lock_s_vtaircr: {:?}, lock_s_mpu: {:?}, lock_sau: {:?} }}",
            self.lock_ns_vtor(),
            self.lock_ns_mpu(),
            self.lock_s_vtaircr(),
            self.lock_s_mpu(),
            self.lock_sau()
        )
    }
}
#[doc = "Flash Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash00MemRule(pub u32);
impl Flash00MemRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Rule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: Rule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Flash00MemRule {
    #[inline(always)]
    fn default() -> Flash00MemRule {
        Flash00MemRule(0)
    }
}
impl core::fmt::Debug for Flash00MemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flash00MemRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flash00MemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flash00MemRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "Flash Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash01MemRule0(pub u32);
impl Flash01MemRule0 {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Rule {
        assert!(n < 4usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: Rule) {
        assert!(n < 4usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Flash01MemRule0 {
    #[inline(always)]
    fn default() -> Flash01MemRule0 {
        Flash01MemRule0(0)
    }
}
impl core::fmt::Debug for Flash01MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flash01MemRule0")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flash01MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flash01MemRule0 {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize)
        )
    }
}
#[doc = "Flash Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash02MemRule0(pub u32);
impl Flash02MemRule0 {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Rule {
        assert!(n < 4usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: Rule) {
        assert!(n < 4usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Flash02MemRule0 {
    #[inline(always)]
    fn default() -> Flash02MemRule0 {
        Flash02MemRule0(0)
    }
}
impl core::fmt::Debug for Flash02MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flash02MemRule0")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flash02MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flash02MemRule0 {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize)
        )
    }
}
#[doc = "Master Secure Level."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MasterSecAntiPolReg(pub u32);
impl MasterSecAntiPolReg {
    #[doc = "DMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> MasterSec {
        let val = (self.0 >> 2usize) & 0x03;
        MasterSec::from_bits(val as u8)
    }
    #[doc = "DMA0."]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: MasterSec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "PKC."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc(&self) -> MasterSec {
        let val = (self.0 >> 4usize) & 0x03;
        MasterSec::from_bits(val as u8)
    }
    #[doc = "PKC."]
    #[inline(always)]
    pub const fn set_pkc(&mut self, val: MasterSec) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "DMA1."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1(&self) -> MasterSec {
        let val = (self.0 >> 6usize) & 0x03;
        MasterSec::from_bits(val as u8)
    }
    #[doc = "DMA1."]
    #[inline(always)]
    pub const fn set_dma1(&mut self, val: MasterSec) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
}
impl Default for MasterSecAntiPolReg {
    #[inline(always)]
    fn default() -> MasterSecAntiPolReg {
        MasterSecAntiPolReg(0)
    }
}
impl core::fmt::Debug for MasterSecAntiPolReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MasterSecAntiPolReg")
            .field("dma0", &self.dma0())
            .field("pkc", &self.pkc())
            .field("dma1", &self.dma1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MasterSecAntiPolReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MasterSecAntiPolReg {{ dma0: {:?}, pkc: {:?}, dma1: {:?} }}",
            self.dma0(),
            self.pkc(),
            self.dma1()
        )
    }
}
#[doc = "Master Secure Level."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MasterSecLevel(pub u32);
impl MasterSecLevel {
    #[doc = "DMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> MasterSec {
        let val = (self.0 >> 2usize) & 0x03;
        MasterSec::from_bits(val as u8)
    }
    #[doc = "DMA0."]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: MasterSec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "PKC."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc(&self) -> MasterSec {
        let val = (self.0 >> 4usize) & 0x03;
        MasterSec::from_bits(val as u8)
    }
    #[doc = "PKC."]
    #[inline(always)]
    pub const fn set_pkc(&mut self, val: MasterSec) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "DMA1."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1(&self) -> MasterSec {
        let val = (self.0 >> 6usize) & 0x03;
        MasterSec::from_bits(val as u8)
    }
    #[doc = "DMA1."]
    #[inline(always)]
    pub const fn set_dma1(&mut self, val: MasterSec) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
}
impl Default for MasterSecLevel {
    #[inline(always)]
    fn default() -> MasterSecLevel {
        MasterSecLevel(0)
    }
}
impl core::fmt::Debug for MasterSecLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MasterSecLevel")
            .field("dma0", &self.dma0())
            .field("pkc", &self.pkc())
            .field("dma1", &self.dma1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MasterSecLevel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MasterSecLevel {{ dma0: {:?}, pkc: {:?}, dma1: {:?} }}",
            self.dma0(),
            self.pkc(),
            self.dma1()
        )
    }
}
#[doc = "Secure Control Duplicate."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiscCtrlDpReg(pub u32);
impl MiscCtrlDpReg {
    #[doc = "Write Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn write_lock(&self) -> MiscCtrlRegWriteLock {
        let val = (self.0 >> 0usize) & 0x03;
        MiscCtrlRegWriteLock::from_bits(val as u8)
    }
    #[doc = "Write Lock."]
    #[inline(always)]
    pub const fn set_write_lock(&mut self, val: MiscCtrlRegWriteLock) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Enable Secure Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_secure_checking(&self) -> MiscCtrlEnable {
        let val = (self.0 >> 2usize) & 0x03;
        MiscCtrlEnable::from_bits(val as u8)
    }
    #[doc = "Enable Secure Checking."]
    #[inline(always)]
    pub const fn set_enable_secure_checking(&mut self, val: MiscCtrlEnable) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable Secure Privilege Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_s_priv_check(&self) -> MiscCtrlEnable {
        let val = (self.0 >> 4usize) & 0x03;
        MiscCtrlEnable::from_bits(val as u8)
    }
    #[doc = "Enable Secure Privilege Checking."]
    #[inline(always)]
    pub const fn set_enable_s_priv_check(&mut self, val: MiscCtrlEnable) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Enable Non-Secure Privilege Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ns_priv_check(&self) -> MiscCtrlEnable {
        let val = (self.0 >> 6usize) & 0x03;
        MiscCtrlEnable::from_bits(val as u8)
    }
    #[doc = "Enable Non-Secure Privilege Checking."]
    #[inline(always)]
    pub const fn set_enable_ns_priv_check(&mut self, val: MiscCtrlEnable) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Disable Violation Abort."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_violation_abort(&self) -> MiscCtrlRegDisableViolationAbort {
        let val = (self.0 >> 8usize) & 0x03;
        MiscCtrlRegDisableViolationAbort::from_bits(val as u8)
    }
    #[doc = "Disable Violation Abort."]
    #[inline(always)]
    pub const fn set_disable_violation_abort(&mut self, val: MiscCtrlRegDisableViolationAbort) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Disable Strict Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_strict_mode(&self) -> MiscCtrlRegDisableStrictMode {
        let val = (self.0 >> 10usize) & 0x03;
        MiscCtrlRegDisableStrictMode::from_bits(val as u8)
    }
    #[doc = "Disable Strict Mode."]
    #[inline(always)]
    pub const fn set_disable_strict_mode(&mut self, val: MiscCtrlRegDisableStrictMode) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "IDAU All Non-Secure."]
    #[must_use]
    #[inline(always)]
    pub const fn idau_all_ns(&self) -> MiscCtrlRegIdauAllNs {
        let val = (self.0 >> 14usize) & 0x03;
        MiscCtrlRegIdauAllNs::from_bits(val as u8)
    }
    #[doc = "IDAU All Non-Secure."]
    #[inline(always)]
    pub const fn set_idau_all_ns(&mut self, val: MiscCtrlRegIdauAllNs) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for MiscCtrlDpReg {
    #[inline(always)]
    fn default() -> MiscCtrlDpReg {
        MiscCtrlDpReg(0)
    }
}
impl core::fmt::Debug for MiscCtrlDpReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiscCtrlDpReg")
            .field("write_lock", &self.write_lock())
            .field("enable_secure_checking", &self.enable_secure_checking())
            .field("enable_s_priv_check", &self.enable_s_priv_check())
            .field("enable_ns_priv_check", &self.enable_ns_priv_check())
            .field("disable_violation_abort", &self.disable_violation_abort())
            .field("disable_strict_mode", &self.disable_strict_mode())
            .field("idau_all_ns", &self.idau_all_ns())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiscCtrlDpReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MiscCtrlDpReg {{ write_lock: {:?}, enable_secure_checking: {:?}, enable_s_priv_check: {:?}, enable_ns_priv_check: {:?}, disable_violation_abort: {:?}, disable_strict_mode: {:?}, idau_all_ns: {:?} }}",
            self.write_lock(),
            self.enable_secure_checking(),
            self.enable_s_priv_check(),
            self.enable_ns_priv_check(),
            self.disable_violation_abort(),
            self.disable_strict_mode(),
            self.idau_all_ns()
        )
    }
}
#[doc = "Secure Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiscCtrlReg(pub u32);
impl MiscCtrlReg {
    #[doc = "Write Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn write_lock(&self) -> MiscCtrlRegWriteLock {
        let val = (self.0 >> 0usize) & 0x03;
        MiscCtrlRegWriteLock::from_bits(val as u8)
    }
    #[doc = "Write Lock."]
    #[inline(always)]
    pub const fn set_write_lock(&mut self, val: MiscCtrlRegWriteLock) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Enable Secure Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_secure_checking(&self) -> MiscCtrlEnable {
        let val = (self.0 >> 2usize) & 0x03;
        MiscCtrlEnable::from_bits(val as u8)
    }
    #[doc = "Enable Secure Checking."]
    #[inline(always)]
    pub const fn set_enable_secure_checking(&mut self, val: MiscCtrlEnable) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable Secure Privilege Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_s_priv_check(&self) -> MiscCtrlEnable {
        let val = (self.0 >> 4usize) & 0x03;
        MiscCtrlEnable::from_bits(val as u8)
    }
    #[doc = "Enable Secure Privilege Checking."]
    #[inline(always)]
    pub const fn set_enable_s_priv_check(&mut self, val: MiscCtrlEnable) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Enable Non-Secure Privilege Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ns_priv_check(&self) -> MiscCtrlEnable {
        let val = (self.0 >> 6usize) & 0x03;
        MiscCtrlEnable::from_bits(val as u8)
    }
    #[doc = "Enable Non-Secure Privilege Checking."]
    #[inline(always)]
    pub const fn set_enable_ns_priv_check(&mut self, val: MiscCtrlEnable) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Disable Violation Abort."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_violation_abort(&self) -> MiscCtrlRegDisableViolationAbort {
        let val = (self.0 >> 8usize) & 0x03;
        MiscCtrlRegDisableViolationAbort::from_bits(val as u8)
    }
    #[doc = "Disable Violation Abort."]
    #[inline(always)]
    pub const fn set_disable_violation_abort(&mut self, val: MiscCtrlRegDisableViolationAbort) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Disable Strict Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_strict_mode(&self) -> MiscCtrlRegDisableStrictMode {
        let val = (self.0 >> 10usize) & 0x03;
        MiscCtrlRegDisableStrictMode::from_bits(val as u8)
    }
    #[doc = "Disable Strict Mode."]
    #[inline(always)]
    pub const fn set_disable_strict_mode(&mut self, val: MiscCtrlRegDisableStrictMode) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "IDAU All Non-Secure."]
    #[must_use]
    #[inline(always)]
    pub const fn idau_all_ns(&self) -> MiscCtrlRegIdauAllNs {
        let val = (self.0 >> 14usize) & 0x03;
        MiscCtrlRegIdauAllNs::from_bits(val as u8)
    }
    #[doc = "IDAU All Non-Secure."]
    #[inline(always)]
    pub const fn set_idau_all_ns(&mut self, val: MiscCtrlRegIdauAllNs) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for MiscCtrlReg {
    #[inline(always)]
    fn default() -> MiscCtrlReg {
        MiscCtrlReg(0)
    }
}
impl core::fmt::Debug for MiscCtrlReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiscCtrlReg")
            .field("write_lock", &self.write_lock())
            .field("enable_secure_checking", &self.enable_secure_checking())
            .field("enable_s_priv_check", &self.enable_s_priv_check())
            .field("enable_ns_priv_check", &self.enable_ns_priv_check())
            .field("disable_violation_abort", &self.disable_violation_abort())
            .field("disable_strict_mode", &self.disable_strict_mode())
            .field("idau_all_ns", &self.idau_all_ns())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiscCtrlReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MiscCtrlReg {{ write_lock: {:?}, enable_secure_checking: {:?}, enable_s_priv_check: {:?}, enable_ns_priv_check: {:?}, disable_violation_abort: {:?}, disable_strict_mode: {:?}, idau_all_ns: {:?} }}",
            self.write_lock(),
            self.enable_secure_checking(),
            self.enable_s_priv_check(),
            self.enable_ns_priv_check(),
            self.disable_violation_abort(),
            self.disable_strict_mode(),
            self.idau_all_ns()
        )
    }
}
#[doc = "RAMA Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamMemRule(pub u32);
impl RamMemRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Rule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: Rule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for RamMemRule {
    #[inline(always)]
    fn default() -> RamMemRule {
        RamMemRule(0)
    }
}
impl core::fmt::Debug for RamMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamMemRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamMemRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "RAMX Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamxMemRule0(pub u32);
impl RamxMemRule0 {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Rule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: Rule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for RamxMemRule0 {
    #[inline(always)]
    fn default() -> RamxMemRule0 {
        RamxMemRule0(0)
    }
}
impl core::fmt::Debug for RamxMemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamxMemRule0")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamxMemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamxMemRule0 {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "ROM Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RomMemRule(pub u32);
impl RomMemRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule(&self, n: usize) -> Rule {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Rule::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule(&mut self, n: usize, val: Rule) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for RomMemRule {
    #[inline(always)]
    fn default() -> RomMemRule {
        RomMemRule(0)
    }
}
impl core::fmt::Debug for RomMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RomMemRule")
            .field("rule[0]", &self.rule(0usize))
            .field("rule[1]", &self.rule(1usize))
            .field("rule[2]", &self.rule(2usize))
            .field("rule[3]", &self.rule(3usize))
            .field("rule[4]", &self.rule(4usize))
            .field("rule[5]", &self.rule(5usize))
            .field("rule[6]", &self.rule(6usize))
            .field("rule[7]", &self.rule(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RomMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RomMemRule {{ rule[0]: {:?}, rule[1]: {:?}, rule[2]: {:?}, rule[3]: {:?}, rule[4]: {:?}, rule[5]: {:?}, rule[6]: {:?}, rule[7]: {:?} }}",
            self.rule(0usize),
            self.rule(1usize),
            self.rule(2usize),
            self.rule(3usize),
            self.rule(4usize),
            self.rule(5usize),
            self.rule(6usize),
            self.rule(7usize)
        )
    }
}
#[doc = "Secure general purpose registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpReg(pub u32);
impl SecGpReg {
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_ipd_req(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma_ipd_req(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for SecGpReg {
    #[inline(always)]
    fn default() -> SecGpReg {
        SecGpReg(0)
    }
}
impl core::fmt::Debug for SecGpReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpReg")
            .field("dma_ipd_req[0]", &self.dma_ipd_req(0usize))
            .field("dma_ipd_req[1]", &self.dma_ipd_req(1usize))
            .field("dma_ipd_req[2]", &self.dma_ipd_req(2usize))
            .field("dma_ipd_req[3]", &self.dma_ipd_req(3usize))
            .field("dma_ipd_req[4]", &self.dma_ipd_req(4usize))
            .field("dma_ipd_req[5]", &self.dma_ipd_req(5usize))
            .field("dma_ipd_req[6]", &self.dma_ipd_req(6usize))
            .field("dma_ipd_req[7]", &self.dma_ipd_req(7usize))
            .field("dma_ipd_req[8]", &self.dma_ipd_req(8usize))
            .field("dma_ipd_req[9]", &self.dma_ipd_req(9usize))
            .field("dma_ipd_req[10]", &self.dma_ipd_req(10usize))
            .field("dma_ipd_req[11]", &self.dma_ipd_req(11usize))
            .field("dma_ipd_req[12]", &self.dma_ipd_req(12usize))
            .field("dma_ipd_req[13]", &self.dma_ipd_req(13usize))
            .field("dma_ipd_req[14]", &self.dma_ipd_req(14usize))
            .field("dma_ipd_req[15]", &self.dma_ipd_req(15usize))
            .field("dma_ipd_req[16]", &self.dma_ipd_req(16usize))
            .field("dma_ipd_req[17]", &self.dma_ipd_req(17usize))
            .field("dma_ipd_req[18]", &self.dma_ipd_req(18usize))
            .field("dma_ipd_req[19]", &self.dma_ipd_req(19usize))
            .field("dma_ipd_req[20]", &self.dma_ipd_req(20usize))
            .field("dma_ipd_req[21]", &self.dma_ipd_req(21usize))
            .field("dma_ipd_req[22]", &self.dma_ipd_req(22usize))
            .field("dma_ipd_req[23]", &self.dma_ipd_req(23usize))
            .field("dma_ipd_req[24]", &self.dma_ipd_req(24usize))
            .field("dma_ipd_req[25]", &self.dma_ipd_req(25usize))
            .field("dma_ipd_req[26]", &self.dma_ipd_req(26usize))
            .field("dma_ipd_req[27]", &self.dma_ipd_req(27usize))
            .field("dma_ipd_req[28]", &self.dma_ipd_req(28usize))
            .field("dma_ipd_req[29]", &self.dma_ipd_req(29usize))
            .field("dma_ipd_req[30]", &self.dma_ipd_req(30usize))
            .field("dma_ipd_req[31]", &self.dma_ipd_req(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpReg {{ dma_ipd_req[0]: {=bool:?}, dma_ipd_req[1]: {=bool:?}, dma_ipd_req[2]: {=bool:?}, dma_ipd_req[3]: {=bool:?}, dma_ipd_req[4]: {=bool:?}, dma_ipd_req[5]: {=bool:?}, dma_ipd_req[6]: {=bool:?}, dma_ipd_req[7]: {=bool:?}, dma_ipd_req[8]: {=bool:?}, dma_ipd_req[9]: {=bool:?}, dma_ipd_req[10]: {=bool:?}, dma_ipd_req[11]: {=bool:?}, dma_ipd_req[12]: {=bool:?}, dma_ipd_req[13]: {=bool:?}, dma_ipd_req[14]: {=bool:?}, dma_ipd_req[15]: {=bool:?}, dma_ipd_req[16]: {=bool:?}, dma_ipd_req[17]: {=bool:?}, dma_ipd_req[18]: {=bool:?}, dma_ipd_req[19]: {=bool:?}, dma_ipd_req[20]: {=bool:?}, dma_ipd_req[21]: {=bool:?}, dma_ipd_req[22]: {=bool:?}, dma_ipd_req[23]: {=bool:?}, dma_ipd_req[24]: {=bool:?}, dma_ipd_req[25]: {=bool:?}, dma_ipd_req[26]: {=bool:?}, dma_ipd_req[27]: {=bool:?}, dma_ipd_req[28]: {=bool:?}, dma_ipd_req[29]: {=bool:?}, dma_ipd_req[30]: {=bool:?}, dma_ipd_req[31]: {=bool:?} }}",
            self.dma_ipd_req(0usize),
            self.dma_ipd_req(1usize),
            self.dma_ipd_req(2usize),
            self.dma_ipd_req(3usize),
            self.dma_ipd_req(4usize),
            self.dma_ipd_req(5usize),
            self.dma_ipd_req(6usize),
            self.dma_ipd_req(7usize),
            self.dma_ipd_req(8usize),
            self.dma_ipd_req(9usize),
            self.dma_ipd_req(10usize),
            self.dma_ipd_req(11usize),
            self.dma_ipd_req(12usize),
            self.dma_ipd_req(13usize),
            self.dma_ipd_req(14usize),
            self.dma_ipd_req(15usize),
            self.dma_ipd_req(16usize),
            self.dma_ipd_req(17usize),
            self.dma_ipd_req(18usize),
            self.dma_ipd_req(19usize),
            self.dma_ipd_req(20usize),
            self.dma_ipd_req(21usize),
            self.dma_ipd_req(22usize),
            self.dma_ipd_req(23usize),
            self.dma_ipd_req(24usize),
            self.dma_ipd_req(25usize),
            self.dma_ipd_req(26usize),
            self.dma_ipd_req(27usize),
            self.dma_ipd_req(28usize),
            self.dma_ipd_req(29usize),
            self.dma_ipd_req(30usize),
            self.dma_ipd_req(31usize)
        )
    }
}
#[doc = "Security Violation Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecVioAddr(pub u32);
impl SecVioAddr {
    #[doc = "Security violation address for AHB layer a reset value 0."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Security violation address for AHB layer a reset value 0."]
    #[inline(always)]
    pub const fn set_sec_vio_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SecVioAddr {
    #[inline(always)]
    fn default() -> SecVioAddr {
        SecVioAddr(0)
    }
}
impl core::fmt::Debug for SecVioAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecVioAddr")
            .field("sec_vio_addr", &self.sec_vio_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecVioAddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecVioAddr {{ sec_vio_addr: {=u32:?} }}",
            self.sec_vio_addr()
        )
    }
}
#[doc = "Security Violation Info Validity for Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecVioInfoValid(pub u32);
impl SecVioInfoValid {
    #[doc = "Violation information valid flag for AHB port 0."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 0."]
    #[inline(always)]
    pub const fn set_vio_info_valid(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for SecVioInfoValid {
    #[inline(always)]
    fn default() -> SecVioInfoValid {
        SecVioInfoValid(0)
    }
}
impl core::fmt::Debug for SecVioInfoValid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecVioInfoValid")
            .field("vio_info_valid[0]", &self.vio_info_valid(0usize))
            .field("vio_info_valid[1]", &self.vio_info_valid(1usize))
            .field("vio_info_valid[2]", &self.vio_info_valid(2usize))
            .field("vio_info_valid[3]", &self.vio_info_valid(3usize))
            .field("vio_info_valid[4]", &self.vio_info_valid(4usize))
            .field("vio_info_valid[5]", &self.vio_info_valid(5usize))
            .field("vio_info_valid[6]", &self.vio_info_valid(6usize))
            .field("vio_info_valid[7]", &self.vio_info_valid(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecVioInfoValid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecVioInfoValid {{ vio_info_valid[0]: {=bool:?}, vio_info_valid[1]: {=bool:?}, vio_info_valid[2]: {=bool:?}, vio_info_valid[3]: {=bool:?}, vio_info_valid[4]: {=bool:?}, vio_info_valid[5]: {=bool:?}, vio_info_valid[6]: {=bool:?}, vio_info_valid[7]: {=bool:?} }}",
            self.vio_info_valid(0usize),
            self.vio_info_valid(1usize),
            self.vio_info_valid(2usize),
            self.vio_info_valid(3usize),
            self.vio_info_valid(4usize),
            self.vio_info_valid(5usize),
            self.vio_info_valid(6usize),
            self.vio_info_valid(7usize)
        )
    }
}
#[doc = "Security Violation Miscellaneous Information at Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecVioMiscInfo(pub u32);
impl SecVioMiscInfo {
    #[doc = "Security violation access read/write indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_write(&self) -> SecVioInfoWrite {
        let val = (self.0 >> 0usize) & 0x01;
        SecVioInfoWrite::from_bits(val as u8)
    }
    #[doc = "Security violation access read/write indicator."]
    #[inline(always)]
    pub const fn set_sec_vio_info_write(&mut self, val: SecVioInfoWrite) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Security Violation Info Data Access."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_data_access(&self) -> SecVioInfoDataAccess {
        let val = (self.0 >> 1usize) & 0x01;
        SecVioInfoDataAccess::from_bits(val as u8)
    }
    #[doc = "Security Violation Info Data Access."]
    #[inline(always)]
    pub const fn set_sec_vio_info_data_access(&mut self, val: SecVioInfoDataAccess) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Security Violation Info Master Security Level."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_master_sec_level(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Security Violation Info Master Security Level."]
    #[inline(always)]
    pub const fn set_sec_vio_info_master_sec_level(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Security violation master number."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_master(&self) -> SecVioInfoMaster {
        let val = (self.0 >> 8usize) & 0x1f;
        SecVioInfoMaster::from_bits(val as u8)
    }
    #[doc = "Security violation master number."]
    #[inline(always)]
    pub const fn set_sec_vio_info_master(&mut self, val: SecVioInfoMaster) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
}
impl Default for SecVioMiscInfo {
    #[inline(always)]
    fn default() -> SecVioMiscInfo {
        SecVioMiscInfo(0)
    }
}
impl core::fmt::Debug for SecVioMiscInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecVioMiscInfo")
            .field("sec_vio_info_write", &self.sec_vio_info_write())
            .field("sec_vio_info_data_access", &self.sec_vio_info_data_access())
            .field(
                "sec_vio_info_master_sec_level",
                &self.sec_vio_info_master_sec_level(),
            )
            .field("sec_vio_info_master", &self.sec_vio_info_master())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecVioMiscInfo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecVioMiscInfo {{ sec_vio_info_write: {:?}, sec_vio_info_data_access: {:?}, sec_vio_info_master_sec_level: {=u8:?}, sec_vio_info_master: {:?} }}",
            self.sec_vio_info_write(),
            self.sec_vio_info_data_access(),
            self.sec_vio_info_master_sec_level(),
            self.sec_vio_info_master()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockNsMpu {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCK_NS_MPU is 1."]
    LockNsMpuEq1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_NS_MPU is 0."]
    LockNsMpuEq0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockNsMpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockNsMpu {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockNsMpu {
    #[inline(always)]
    fn from(val: u8) -> LockNsMpu {
        LockNsMpu::from_bits(val)
    }
}
impl From<LockNsMpu> for u8 {
    #[inline(always)]
    fn from(val: LockNsMpu) -> u8 {
        LockNsMpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockNsVtor {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCKNSVTOR is 1."]
    LockNsVtorEq1 = 0x01,
    #[doc = "CM33 (CPU0) LOCKNSVTOR is 0."]
    LockNsVtorEq0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockNsVtor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockNsVtor {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockNsVtor {
    #[inline(always)]
    fn from(val: u8) -> LockNsVtor {
        LockNsVtor::from_bits(val)
    }
}
impl From<LockNsVtor> for u8 {
    #[inline(always)]
    fn from(val: LockNsVtor) -> u8 {
        LockNsVtor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockSMpu {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCK_S_MPU is 1."]
    LockSMpuEq1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_S_MPU is 0."]
    LockSMpuEq0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockSMpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockSMpu {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockSMpu {
    #[inline(always)]
    fn from(val: u8) -> LockSMpu {
        LockSMpu::from_bits(val)
    }
}
impl From<LockSMpu> for u8 {
    #[inline(always)]
    fn from(val: LockSMpu) -> u8 {
        LockSMpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockSVtaircr {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCK_S_VTAIRCR is 1."]
    LockSVtaircrEq1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_S_VTAIRCR is 0."]
    LockSVtaircrEq0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockSVtaircr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockSVtaircr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockSVtaircr {
    #[inline(always)]
    fn from(val: u8) -> LockSVtaircr {
        LockSVtaircr::from_bits(val)
    }
}
impl From<LockSVtaircr> for u8 {
    #[inline(always)]
    fn from(val: LockSVtaircr) -> u8 {
        LockSVtaircr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockSau {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCK_SAU is 1."]
    LockSauEq1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_SAU is 0."]
    LockSauEq0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockSau {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockSau {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockSau {
    #[inline(always)]
    fn from(val: u8) -> LockSau {
        LockSau::from_bits(val)
    }
}
impl From<LockSau> for u8 {
    #[inline(always)]
    fn from(val: LockSau) -> u8 {
        LockSau::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSec {
    #[doc = "Secure and privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Secure and non-privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Non-secure and privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Non-secure and non-privileged Master."]
    SecurePrivMaster = 0x03,
}
impl MasterSec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSec {
    #[inline(always)]
    fn from(val: u8) -> MasterSec {
        MasterSec::from_bits(val)
    }
}
impl From<MasterSec> for u8 {
    #[inline(always)]
    fn from(val: MasterSec) -> u8 {
        MasterSec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlEnable {
    _RESERVED_0 = 0x0,
    #[doc = "Enables the privilege checking of non-secure mode access."]
    Enabled = 0x01,
    #[doc = "Disables the privilege checking of non-secure mode access."]
    Disabled = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlEnable {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlEnable {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlEnable {
        MiscCtrlEnable::from_bits(val)
    }
}
impl From<MiscCtrlEnable> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlEnable) -> u8 {
        MiscCtrlEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegDisableStrictMode {
    _RESERVED_0 = 0x0,
    #[doc = "Master strict mode is disabled and can access memories and peripherals at the same level or below that level."]
    Ahbtm = 0x01,
    #[doc = "Master strict mode is enabled and can access memories and peripherals at same level only."]
    Ahbsm1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegDisableStrictMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegDisableStrictMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegDisableStrictMode {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegDisableStrictMode {
        MiscCtrlRegDisableStrictMode::from_bits(val)
    }
}
impl From<MiscCtrlRegDisableStrictMode> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegDisableStrictMode) -> u8 {
        MiscCtrlRegDisableStrictMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegDisableViolationAbort {
    _RESERVED_0 = 0x0,
    #[doc = "The violation detected by the secure checker will not cause an abort, but a secure_violation_irq (interrupt request) will still be asserted and serviced by ISR."]
    NoAbort = 0x01,
    #[doc = "The violation detected by the secure checker will cause an abort."]
    Abort = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegDisableViolationAbort {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegDisableViolationAbort {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegDisableViolationAbort {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegDisableViolationAbort {
        MiscCtrlRegDisableViolationAbort::from_bits(val)
    }
}
impl From<MiscCtrlRegDisableViolationAbort> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegDisableViolationAbort) -> u8 {
        MiscCtrlRegDisableViolationAbort::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegIdauAllNs {
    _RESERVED_0 = 0x0,
    #[doc = "IDAU is disabled, which means that all memories are attributed as non-secure memory."]
    Disabled = 0x01,
    #[doc = "IDAU is enabled (restrictive mode)."]
    Enabled = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegIdauAllNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegIdauAllNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegIdauAllNs {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegIdauAllNs {
        MiscCtrlRegIdauAllNs::from_bits(val)
    }
}
impl From<MiscCtrlRegIdauAllNs> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegIdauAllNs) -> u8 {
        MiscCtrlRegIdauAllNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegWriteLock {
    _RESERVED_0 = 0x0,
    #[doc = "Writes to this register and to the Memory and Peripheral RULE registers are not allowed."]
    Locked = 0x01,
    #[doc = "Writes to this register and to the Memory and Peripheral RULE registers are allowed."]
    NotLocked = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegWriteLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegWriteLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegWriteLock {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegWriteLock {
        MiscCtrlRegWriteLock::from_bits(val)
    }
}
impl From<MiscCtrlRegWriteLock> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegWriteLock) -> u8 {
        MiscCtrlRegWriteLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rule {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NonsecureNonprivUserAllowed = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NonsecurePrivUserAllowed = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SecureNonprivUserAllowed = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SecurePrivUserAllowed = 0x03,
}
impl Rule {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rule {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rule {
    #[inline(always)]
    fn from(val: u8) -> Rule {
        Rule::from_bits(val)
    }
}
impl From<Rule> for u8 {
    #[inline(always)]
    fn from(val: Rule) -> u8 {
        Rule::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecVioInfoDataAccess {
    #[doc = "Code."]
    Code = 0x0,
    #[doc = "Data."]
    Data = 0x01,
}
impl SecVioInfoDataAccess {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecVioInfoDataAccess {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecVioInfoDataAccess {
    #[inline(always)]
    fn from(val: u8) -> SecVioInfoDataAccess {
        SecVioInfoDataAccess::from_bits(val)
    }
}
impl From<SecVioInfoDataAccess> for u8 {
    #[inline(always)]
    fn from(val: SecVioInfoDataAccess) -> u8 {
        SecVioInfoDataAccess::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecVioInfoMaster {
    #[doc = "M33 Code."]
    Cpu0Code = 0x0,
    #[doc = "M33 System."]
    Cpu0Sys = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "DMA0."]
    Dma0 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "DMA1."]
    Dma1 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "PKC."]
    Pkc = 0x0c,
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
}
impl SecVioInfoMaster {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecVioInfoMaster {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecVioInfoMaster {
    #[inline(always)]
    fn from(val: u8) -> SecVioInfoMaster {
        SecVioInfoMaster::from_bits(val)
    }
}
impl From<SecVioInfoMaster> for u8 {
    #[inline(always)]
    fn from(val: SecVioInfoMaster) -> u8 {
        SecVioInfoMaster::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecVioInfoWrite {
    #[doc = "Read access."]
    Read = 0x0,
    #[doc = "Write access."]
    Write = 0x01,
}
impl SecVioInfoWrite {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecVioInfoWrite {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecVioInfoWrite {
    #[inline(always)]
    fn from(val: u8) -> SecVioInfoWrite {
        SecVioInfoWrite::from_bits(val)
    }
}
impl From<SecVioInfoWrite> for u8 {
    #[inline(always)]
    fn from(val: SecVioInfoWrite) -> u8 {
        SecVioInfoWrite::to_bits(val)
    }
}
