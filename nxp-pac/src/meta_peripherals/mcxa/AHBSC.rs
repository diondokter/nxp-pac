#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6a8c2aa 2026-01-27))"]
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
    pub const fn flash01_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Flash01MemRule, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _)
        }
    }
    #[doc = "Flash IFR0 Rule register."]
    #[inline(always)]
    pub const fn flash02_mem_rule(
        self,
    ) -> crate::pac::common::Reg<Flash02MemRule, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Flash Memory Rule."]
    #[inline(always)]
    pub const fn flash03_mem_rule(
        self,
    ) -> crate::pac::common::Reg<Flash03MemRule, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "ROM Memory Rule."]
    #[inline(always)]
    pub const fn rom_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<RomMemRule, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 4usize) as _)
        }
    }
    #[doc = "RAMX Memory Rule."]
    #[inline(always)]
    pub const fn ramx_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<RamxMemRule, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _)
        }
    }
    #[doc = "RAMA Memory Rule 0."]
    #[inline(always)]
    pub const fn rama_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<RamaMemRule, crate::pac::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _)
        }
    }
    #[doc = "RAMB Memory Rule 0."]
    #[inline(always)]
    pub const fn ramb_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<RambMemRule, crate::pac::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize + n * 4usize) as _)
        }
    }
    #[doc = "AHB Slave Port 5 Rule Register."]
    #[inline(always)]
    pub const fn ahb_slave_port_p5_slave_rule0(
        self,
    ) -> crate::pac::common::Reg<AhbSlavePortP5SlaveRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "APB Bridge Group 0 Memory Rule Register 0."]
    #[inline(always)]
    pub const fn apb_peripheral_group0_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<ApbPeripheralGroup0MemRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "APB Bridge Group 0 Memory Rule Register 1."]
    #[inline(always)]
    pub const fn apb_peripheral_group0_mem_rule1(
        self,
    ) -> crate::pac::common::Reg<ApbPeripheralGroup0MemRule1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "AIPS Bridge Group 0 Memory Rule 0."]
    #[inline(always)]
    pub const fn aips_bridge_group0_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup0MemRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "AIPS Bridge Group 0 Memory Rule 1."]
    #[inline(always)]
    pub const fn aips_bridge_group0_mem_rule1(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup0MemRule1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "AIPS Bridge Group 1 Memory Rule 0."]
    #[inline(always)]
    pub const fn aips_bridge_group1_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup1MemRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "AIPS Bridge Group 1 Memory Rule 1."]
    #[inline(always)]
    pub const fn aips_bridge_group1_mem_rule1(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup1MemRule1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "AHB Secure Control Peripheral Rule."]
    #[inline(always)]
    pub const fn ahb_secure_ctrl_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<AhbSecureCtrlMemRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "AHB Peripheral 0 Memory Rule 1."]
    #[inline(always)]
    pub const fn ahb_peripheral0_mem_rule1(
        self,
    ) -> crate::pac::common::Reg<AhbPeripheral0MemRule1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "AHB Peripheral 0 Memory Rule 2."]
    #[inline(always)]
    pub const fn ahb_peripheral0_mem_rule2(
        self,
    ) -> crate::pac::common::Reg<AhbPeripheral0MemRule2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "AHB Peripheral 0 Memory Rule 3."]
    #[inline(always)]
    pub const fn ahb_peripheral0_mem_rule3(
        self,
    ) -> crate::pac::common::Reg<AhbPeripheral0MemRule3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "AHB Peripheral 0 Memory Rule 4."]
    #[inline(always)]
    pub const fn ahb_peripheral0_mem_rule4(
        self,
    ) -> crate::pac::common::Reg<AhbPeripheral0MemRule4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "AHB Peripheral 0 Memory Rule 5."]
    #[inline(always)]
    pub const fn ahb_peripheral0_mem_rule5(
        self,
    ) -> crate::pac::common::Reg<AhbPeripheral0MemRule5, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 0."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule0(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2MemRule0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 1."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule1(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2MemRule1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 2."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule2(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2MemRule2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 3."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule3(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2MemRule3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 4."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule4(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2MemRule4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 5."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule5(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2MemRule5, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 6."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule6(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2MemRule6, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 7."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule7(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2MemRule7, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 8."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule8(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2MemRule8, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 9."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule9(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2MemRule9, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 10."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule10(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2MemRule10, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 11."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule11(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2MemRule11, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 12."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule12(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2MemRule12, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 13."]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule13(
        self,
    ) -> crate::pac::common::Reg<AipsBridgeGroup2MemRule13, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "FLEXSPI0 Region 0 Memory Rule."]
    #[inline(always)]
    pub const fn flexspi0_region0_mem_rule(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Flexspi0Region0MemRule, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize + n * 4usize) as _)
        }
    }
    #[doc = "Array of registers: FLEXSPI0_REGION_MEM_RULE."]
    #[inline(always)]
    pub const fn flexspi0_region1_6_mem_rule(self, n: usize) -> Flexspi0Region16MemRule {
        assert!(n < 6usize);
        unsafe {
            Flexspi0Region16MemRule::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 16usize) as _)
        }
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
    pub const fn sec_gp_reg0(self) -> crate::pac::common::Reg<SecGpReg0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f80usize) as _) }
    }
    #[doc = "Secure general purpose registers."]
    #[inline(always)]
    pub const fn sec_gp_reg1(self) -> crate::pac::common::Reg<SecGpReg1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f84usize) as _) }
    }
    #[doc = "Secure general purpose registers."]
    #[inline(always)]
    pub const fn sec_gp_reg2(self) -> crate::pac::common::Reg<SecGpReg2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f88usize) as _) }
    }
    #[doc = "Secure general purpose registers."]
    #[inline(always)]
    pub const fn sec_gp_reg3(self) -> crate::pac::common::Reg<SecGpReg3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f8cusize) as _) }
    }
    #[doc = "Secure general purpose registers."]
    #[inline(always)]
    pub const fn sec_gp_reg4(self) -> crate::pac::common::Reg<SecGpReg4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f90usize) as _) }
    }
    #[doc = "Secure general purpose registers."]
    #[inline(always)]
    pub const fn sec_gp_reg5(self) -> crate::pac::common::Reg<SecGpReg5, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f94usize) as _) }
    }
    #[doc = "Secure general purpose registers."]
    #[inline(always)]
    pub const fn sec_gp_reg6(self) -> crate::pac::common::Reg<SecGpReg6, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f98usize) as _) }
    }
    #[doc = "Secure general purpose registers."]
    #[inline(always)]
    pub const fn sec_gp_reg7(self) -> crate::pac::common::Reg<SecGpReg7, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f9cusize) as _) }
    }
    #[doc = "Secure general purpose registers."]
    #[inline(always)]
    pub const fn sec_gp_reg8(self) -> crate::pac::common::Reg<SecGpReg8, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa0usize) as _) }
    }
    #[doc = "Secure general purpose registers."]
    #[inline(always)]
    pub const fn sec_gp_reg9(self) -> crate::pac::common::Reg<SecGpReg9, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa4usize) as _) }
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
#[doc = "Array of registers: FLEXSPI0_REGION_MEM_RULE."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi0Region16MemRule {
    ptr: *mut u8,
}
unsafe impl Send for Flexspi0Region16MemRule {}
unsafe impl Sync for Flexspi0Region16MemRule {}
impl Flexspi0Region16MemRule {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "FLEXSPI0 Region index Memory Rule."]
    #[inline(always)]
    pub const fn flexspi0_region_mem_rule(
        self,
    ) -> crate::pac::common::Reg<Flexspi0RegionMemRule, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
#[doc = "AHB Peripheral 0 Memory Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral0MemRule1(pub u32);
impl AhbPeripheral0MemRule1 {
    #[doc = "GPIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> Gpio0 {
        let val = (self.0 >> 0usize) & 0x03;
        Gpio0::from_bits(val as u8)
    }
    #[doc = "GPIO0."]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: Gpio0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "GPIO0 ALIAS."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0_alias(&self) -> Gpio0Alias {
        let val = (self.0 >> 4usize) & 0x03;
        Gpio0Alias::from_bits(val as u8)
    }
    #[doc = "GPIO0 ALIAS."]
    #[inline(always)]
    pub const fn set_gpio0_alias(&mut self, val: Gpio0Alias) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for AhbPeripheral0MemRule1 {
    #[inline(always)]
    fn default() -> AhbPeripheral0MemRule1 {
        AhbPeripheral0MemRule1(0)
    }
}
impl core::fmt::Debug for AhbPeripheral0MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral0MemRule1")
            .field("gpio0", &self.gpio0())
            .field("gpio0_alias", &self.gpio0_alias())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral0MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral0MemRule1 {{ gpio0: {:?}, gpio0_alias: {:?} }}",
            self.gpio0(),
            self.gpio0_alias()
        )
    }
}
#[doc = "AHB Peripheral 0 Memory Rule 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral0MemRule2(pub u32);
impl AhbPeripheral0MemRule2 {
    #[doc = "GPIO1."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> Gpio1 {
        let val = (self.0 >> 0usize) & 0x03;
        Gpio1::from_bits(val as u8)
    }
    #[doc = "GPIO1."]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: Gpio1) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "GPIO1 ALIAS."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1_alias(&self) -> Gpio1Alias {
        let val = (self.0 >> 4usize) & 0x03;
        Gpio1Alias::from_bits(val as u8)
    }
    #[doc = "GPIO1 ALIAS."]
    #[inline(always)]
    pub const fn set_gpio1_alias(&mut self, val: Gpio1Alias) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for AhbPeripheral0MemRule2 {
    #[inline(always)]
    fn default() -> AhbPeripheral0MemRule2 {
        AhbPeripheral0MemRule2(0)
    }
}
impl core::fmt::Debug for AhbPeripheral0MemRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral0MemRule2")
            .field("gpio1", &self.gpio1())
            .field("gpio1_alias", &self.gpio1_alias())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral0MemRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral0MemRule2 {{ gpio1: {:?}, gpio1_alias: {:?} }}",
            self.gpio1(),
            self.gpio1_alias()
        )
    }
}
#[doc = "AHB Peripheral 0 Memory Rule 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral0MemRule3(pub u32);
impl AhbPeripheral0MemRule3 {
    #[doc = "GPIO2."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> Gpio2 {
        let val = (self.0 >> 0usize) & 0x03;
        Gpio2::from_bits(val as u8)
    }
    #[doc = "GPIO2."]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: Gpio2) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "GPIO2 ALIAS."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2_alias(&self) -> Gpio2Alias {
        let val = (self.0 >> 4usize) & 0x03;
        Gpio2Alias::from_bits(val as u8)
    }
    #[doc = "GPIO2 ALIAS."]
    #[inline(always)]
    pub const fn set_gpio2_alias(&mut self, val: Gpio2Alias) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for AhbPeripheral0MemRule3 {
    #[inline(always)]
    fn default() -> AhbPeripheral0MemRule3 {
        AhbPeripheral0MemRule3(0)
    }
}
impl core::fmt::Debug for AhbPeripheral0MemRule3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral0MemRule3")
            .field("gpio2", &self.gpio2())
            .field("gpio2_alias", &self.gpio2_alias())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral0MemRule3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral0MemRule3 {{ gpio2: {:?}, gpio2_alias: {:?} }}",
            self.gpio2(),
            self.gpio2_alias()
        )
    }
}
#[doc = "AHB Peripheral 0 Memory Rule 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral0MemRule4(pub u32);
impl AhbPeripheral0MemRule4 {
    #[doc = "GPIO3."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> Gpio3 {
        let val = (self.0 >> 0usize) & 0x03;
        Gpio3::from_bits(val as u8)
    }
    #[doc = "GPIO3."]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: Gpio3) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "GPIO3 ALIAS."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3_alias(&self) -> Gpio3Alias {
        let val = (self.0 >> 4usize) & 0x03;
        Gpio3Alias::from_bits(val as u8)
    }
    #[doc = "GPIO3 ALIAS."]
    #[inline(always)]
    pub const fn set_gpio3_alias(&mut self, val: Gpio3Alias) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for AhbPeripheral0MemRule4 {
    #[inline(always)]
    fn default() -> AhbPeripheral0MemRule4 {
        AhbPeripheral0MemRule4(0)
    }
}
impl core::fmt::Debug for AhbPeripheral0MemRule4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral0MemRule4")
            .field("gpio3", &self.gpio3())
            .field("gpio3_alias", &self.gpio3_alias())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral0MemRule4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral0MemRule4 {{ gpio3: {:?}, gpio3_alias: {:?} }}",
            self.gpio3(),
            self.gpio3_alias()
        )
    }
}
#[doc = "AHB Peripheral 0 Memory Rule 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral0MemRule5(pub u32);
impl AhbPeripheral0MemRule5 {
    #[doc = "GPIO4."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4(&self) -> Gpio4 {
        let val = (self.0 >> 0usize) & 0x03;
        Gpio4::from_bits(val as u8)
    }
    #[doc = "GPIO4."]
    #[inline(always)]
    pub const fn set_gpio4(&mut self, val: Gpio4) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "GPIO4 ALIAS."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4_alias(&self) -> Gpio4Alias {
        let val = (self.0 >> 4usize) & 0x03;
        Gpio4Alias::from_bits(val as u8)
    }
    #[doc = "GPIO4 ALIAS."]
    #[inline(always)]
    pub const fn set_gpio4_alias(&mut self, val: Gpio4Alias) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for AhbPeripheral0MemRule5 {
    #[inline(always)]
    fn default() -> AhbPeripheral0MemRule5 {
        AhbPeripheral0MemRule5(0)
    }
}
impl core::fmt::Debug for AhbPeripheral0MemRule5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral0MemRule5")
            .field("gpio4", &self.gpio4())
            .field("gpio4_alias", &self.gpio4_alias())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral0MemRule5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral0MemRule5 {{ gpio4: {:?}, gpio4_alias: {:?} }}",
            self.gpio4(),
            self.gpio4_alias()
        )
    }
}
#[doc = "AHB Secure Control Peripheral Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbSecureCtrlMemRule0(pub u32);
impl AhbSecureCtrlMemRule0 {
    #[doc = "AHBSC."]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> AhbSecureCtrlMemRule0Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        AhbSecureCtrlMemRule0Rule0::from_bits(val as u8)
    }
    #[doc = "AHBSC."]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: AhbSecureCtrlMemRule0Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "AHBSC ALIAS0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> AhbSecureCtrlMemRule0Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        AhbSecureCtrlMemRule0Rule1::from_bits(val as u8)
    }
    #[doc = "AHBSC ALIAS0."]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: AhbSecureCtrlMemRule0Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "AHBSC ALIAS1."]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> AhbSecureCtrlMemRule0Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        AhbSecureCtrlMemRule0Rule2::from_bits(val as u8)
    }
    #[doc = "AHBSC ALIAS1."]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: AhbSecureCtrlMemRule0Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "AHBSC ALIAS2."]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> AhbSecureCtrlMemRule0Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        AhbSecureCtrlMemRule0Rule3::from_bits(val as u8)
    }
    #[doc = "AHBSC ALIAS2."]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: AhbSecureCtrlMemRule0Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for AhbSecureCtrlMemRule0 {
    #[inline(always)]
    fn default() -> AhbSecureCtrlMemRule0 {
        AhbSecureCtrlMemRule0(0)
    }
}
impl core::fmt::Debug for AhbSecureCtrlMemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbSecureCtrlMemRule0")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbSecureCtrlMemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbSecureCtrlMemRule0 {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3()
        )
    }
}
#[doc = "AHB Slave Port 5 Rule Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbSlavePortP5SlaveRule0(pub u32);
impl AhbSlavePortP5SlaveRule0 {
    #[doc = "CDOG0."]
    #[must_use]
    #[inline(always)]
    pub const fn cdog0(&self) -> Cdog0 {
        let val = (self.0 >> 12usize) & 0x03;
        Cdog0::from_bits(val as u8)
    }
    #[doc = "CDOG0."]
    #[inline(always)]
    pub const fn set_cdog0(&mut self, val: Cdog0) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "CDOG1."]
    #[must_use]
    #[inline(always)]
    pub const fn cdog1(&self) -> Cdog1 {
        let val = (self.0 >> 16usize) & 0x03;
        Cdog1::from_bits(val as u8)
    }
    #[doc = "CDOG1."]
    #[inline(always)]
    pub const fn set_cdog1(&mut self, val: Cdog1) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "DEBUG_MAILBOX."]
    #[must_use]
    #[inline(always)]
    pub const fn debug_mailbox(&self) -> DebugMailbox {
        let val = (self.0 >> 20usize) & 0x03;
        DebugMailbox::from_bits(val as u8)
    }
    #[doc = "DEBUG_MAILBOX."]
    #[inline(always)]
    pub const fn set_debug_mailbox(&mut self, val: DebugMailbox) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6."]
    #[must_use]
    #[inline(always)]
    pub const fn mau0(&self) -> Mau0 {
        let val = (self.0 >> 24usize) & 0x03;
        Mau0::from_bits(val as u8)
    }
    #[doc = "Rule 6."]
    #[inline(always)]
    pub const fn set_mau0(&mut self, val: Mau0) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for AhbSlavePortP5SlaveRule0 {
    #[inline(always)]
    fn default() -> AhbSlavePortP5SlaveRule0 {
        AhbSlavePortP5SlaveRule0(0)
    }
}
impl core::fmt::Debug for AhbSlavePortP5SlaveRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbSlavePortP5SlaveRule0")
            .field("cdog0", &self.cdog0())
            .field("cdog1", &self.cdog1())
            .field("debug_mailbox", &self.debug_mailbox())
            .field("mau0", &self.mau0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbSlavePortP5SlaveRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbSlavePortP5SlaveRule0 {{ cdog0: {:?}, cdog1: {:?}, debug_mailbox: {:?}, mau0: {:?} }}",
            self.cdog0(),
            self.cdog1(),
            self.debug_mailbox(),
            self.mau0()
        )
    }
}
#[doc = "AIPS Bridge Group 0 Memory Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup0MemRule0(pub u32);
impl AipsBridgeGroup0MemRule0 {
    #[doc = "EWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn ewm0(&self) -> Ewm0 {
        let val = (self.0 >> 0usize) & 0x03;
        Ewm0::from_bits(val as u8)
    }
    #[doc = "EWM0."]
    #[inline(always)]
    pub const fn set_ewm0(&mut self, val: Ewm0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "ROMCP."]
    #[must_use]
    #[inline(always)]
    pub const fn romcp(&self) -> Romcp {
        let val = (self.0 >> 4usize) & 0x03;
        Romcp::from_bits(val as u8)
    }
    #[doc = "ROMCP."]
    #[inline(always)]
    pub const fn set_romcp(&mut self, val: Romcp) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "PKC0."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc0(&self) -> Pkc0 {
        let val = (self.0 >> 8usize) & 0x03;
        Pkc0::from_bits(val as u8)
    }
    #[doc = "PKC0."]
    #[inline(always)]
    pub const fn set_pkc0(&mut self, val: Pkc0) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "DMA_1_MP."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_1_mp(&self) -> Dma1Mp {
        let val = (self.0 >> 12usize) & 0x03;
        Dma1Mp::from_bits(val as u8)
    }
    #[doc = "DMA_1_MP."]
    #[inline(always)]
    pub const fn set_dma_1_mp(&mut self, val: Dma1Mp) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "DMA_1_CH0."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_1_ch0(&self) -> Dma1Ch0 {
        let val = (self.0 >> 16usize) & 0x03;
        Dma1Ch0::from_bits(val as u8)
    }
    #[doc = "DMA_1_CH0."]
    #[inline(always)]
    pub const fn set_dma_1_ch0(&mut self, val: Dma1Ch0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "DMA_1_CH1."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_1_ch1(&self) -> Dma1Ch1 {
        let val = (self.0 >> 20usize) & 0x03;
        Dma1Ch1::from_bits(val as u8)
    }
    #[doc = "DMA_1_CH1."]
    #[inline(always)]
    pub const fn set_dma_1_ch1(&mut self, val: Dma1Ch1) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "DMA_1_CH2."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_1_ch2(&self) -> Dma1Ch2 {
        let val = (self.0 >> 24usize) & 0x03;
        Dma1Ch2::from_bits(val as u8)
    }
    #[doc = "DMA_1_CH2."]
    #[inline(always)]
    pub const fn set_dma_1_ch2(&mut self, val: Dma1Ch2) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "DMA_1_CH3."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_1_ch3(&self) -> Dma1Ch3 {
        let val = (self.0 >> 28usize) & 0x03;
        Dma1Ch3::from_bits(val as u8)
    }
    #[doc = "DMA_1_CH3."]
    #[inline(always)]
    pub const fn set_dma_1_ch3(&mut self, val: Dma1Ch3) {
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
            .field("ewm0", &self.ewm0())
            .field("romcp", &self.romcp())
            .field("pkc0", &self.pkc0())
            .field("dma_1_mp", &self.dma_1_mp())
            .field("dma_1_ch0", &self.dma_1_ch0())
            .field("dma_1_ch1", &self.dma_1_ch1())
            .field("dma_1_ch2", &self.dma_1_ch2())
            .field("dma_1_ch3", &self.dma_1_ch3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup0MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup0MemRule0 {{ ewm0: {:?}, romcp: {:?}, pkc0: {:?}, dma_1_mp: {:?}, dma_1_ch0: {:?}, dma_1_ch1: {:?}, dma_1_ch2: {:?}, dma_1_ch3: {:?} }}",
            self.ewm0(),
            self.romcp(),
            self.pkc0(),
            self.dma_1_mp(),
            self.dma_1_ch0(),
            self.dma_1_ch1(),
            self.dma_1_ch2(),
            self.dma_1_ch3()
        )
    }
}
#[doc = "AIPS Bridge Group 0 Memory Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup0MemRule1(pub u32);
impl AipsBridgeGroup0MemRule1 {
    #[doc = "ENET0_0."]
    #[must_use]
    #[inline(always)]
    pub const fn enet0_0(&self) -> Enet00 {
        let val = (self.0 >> 16usize) & 0x03;
        Enet00::from_bits(val as u8)
    }
    #[doc = "ENET0_0."]
    #[inline(always)]
    pub const fn set_enet0_0(&mut self, val: Enet00) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "ENET0_1."]
    #[must_use]
    #[inline(always)]
    pub const fn enet0_1(&self) -> Enet01 {
        let val = (self.0 >> 20usize) & 0x03;
        Enet01::from_bits(val as u8)
    }
    #[doc = "ENET0_1."]
    #[inline(always)]
    pub const fn set_enet0_1(&mut self, val: Enet01) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "eSPI."]
    #[must_use]
    #[inline(always)]
    pub const fn e_spi(&self) -> ESpi {
        let val = (self.0 >> 28usize) & 0x03;
        ESpi::from_bits(val as u8)
    }
    #[doc = "eSPI."]
    #[inline(always)]
    pub const fn set_e_spi(&mut self, val: ESpi) {
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
            .field("enet0_0", &self.enet0_0())
            .field("enet0_1", &self.enet0_1())
            .field("e_spi", &self.e_spi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup0MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup0MemRule1 {{ enet0_0: {:?}, enet0_1: {:?}, e_spi: {:?} }}",
            self.enet0_0(),
            self.enet0_1(),
            self.e_spi()
        )
    }
}
#[doc = "AIPS Bridge Group 1 Memory Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup1MemRule0(pub u32);
impl AipsBridgeGroup1MemRule0 {
    #[doc = "FLEXSPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi0(&self) -> Flexspi0 {
        let val = (self.0 >> 0usize) & 0x03;
        Flexspi0::from_bits(val as u8)
    }
    #[doc = "FLEXSPI0."]
    #[inline(always)]
    pub const fn set_flexspi0(&mut self, val: Flexspi0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "LPSPI2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi2(&self) -> Lpspi2 {
        let val = (self.0 >> 4usize) & 0x03;
        Lpspi2::from_bits(val as u8)
    }
    #[doc = "LPSPI2."]
    #[inline(always)]
    pub const fn set_lpspi2(&mut self, val: Lpspi2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "LPSPI3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi3(&self) -> Lpspi3 {
        let val = (self.0 >> 8usize) & 0x03;
        Lpspi3::from_bits(val as u8)
    }
    #[doc = "LPSPI3."]
    #[inline(always)]
    pub const fn set_lpspi3(&mut self, val: Lpspi3) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LPSPI4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi4(&self) -> Lpspi4 {
        let val = (self.0 >> 12usize) & 0x03;
        Lpspi4::from_bits(val as u8)
    }
    #[doc = "LPSPI4."]
    #[inline(always)]
    pub const fn set_lpspi4(&mut self, val: Lpspi4) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "LPSPI5."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi5(&self) -> Lpspi5 {
        let val = (self.0 >> 16usize) & 0x03;
        Lpspi5::from_bits(val as u8)
    }
    #[doc = "LPSPI5."]
    #[inline(always)]
    pub const fn set_lpspi5(&mut self, val: Lpspi5) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
}
impl Default for AipsBridgeGroup1MemRule0 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup1MemRule0 {
        AipsBridgeGroup1MemRule0(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup1MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup1MemRule0")
            .field("flexspi0", &self.flexspi0())
            .field("lpspi2", &self.lpspi2())
            .field("lpspi3", &self.lpspi3())
            .field("lpspi4", &self.lpspi4())
            .field("lpspi5", &self.lpspi5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup1MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup1MemRule0 {{ flexspi0: {:?}, lpspi2: {:?}, lpspi3: {:?}, lpspi4: {:?}, lpspi5: {:?} }}",
            self.flexspi0(),
            self.lpspi2(),
            self.lpspi3(),
            self.lpspi4(),
            self.lpspi5()
        )
    }
}
#[doc = "AIPS Bridge Group 1 Memory Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup1MemRule1(pub u32);
impl AipsBridgeGroup1MemRule1 {
    #[doc = "SPI_FILETER0."]
    #[must_use]
    #[inline(always)]
    pub const fn spi_fileter0(&self) -> SpiFileter0 {
        let val = (self.0 >> 16usize) & 0x03;
        SpiFileter0::from_bits(val as u8)
    }
    #[doc = "SPI_FILETER0."]
    #[inline(always)]
    pub const fn set_spi_fileter0(&mut self, val: SpiFileter0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "10BASE_T1S0."]
    #[must_use]
    #[inline(always)]
    pub const fn t1s0(&self) -> T1s0 {
        let val = (self.0 >> 20usize) & 0x03;
        T1s0::from_bits(val as u8)
    }
    #[doc = "10BASE_T1S0."]
    #[inline(always)]
    pub const fn set_t1s0(&mut self, val: T1s0) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "USB1."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1(&self) -> AipsBridgeGroup1MemRule1Usb1 {
        let val = (self.0 >> 24usize) & 0x03;
        AipsBridgeGroup1MemRule1Usb1::from_bits(val as u8)
    }
    #[doc = "USB1."]
    #[inline(always)]
    pub const fn set_usb1(&mut self, val: AipsBridgeGroup1MemRule1Usb1) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "USB1_PHY."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1_phy(&self) -> Usb1Phy {
        let val = (self.0 >> 28usize) & 0x03;
        Usb1Phy::from_bits(val as u8)
    }
    #[doc = "USB1_PHY."]
    #[inline(always)]
    pub const fn set_usb1_phy(&mut self, val: Usb1Phy) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup1MemRule1 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup1MemRule1 {
        AipsBridgeGroup1MemRule1(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup1MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup1MemRule1")
            .field("spi_fileter0", &self.spi_fileter0())
            .field("t1s0", &self.t1s0())
            .field("usb1", &self.usb1())
            .field("usb1_phy", &self.usb1_phy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup1MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup1MemRule1 {{ spi_fileter0: {:?}, t1s0: {:?}, usb1: {:?}, usb1_phy: {:?} }}",
            self.spi_fileter0(),
            self.t1s0(),
            self.usb1(),
            self.usb1_phy()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule0(pub u32);
impl AipsBridgeGroup2MemRule0 {
    #[doc = "DMA_0_MP."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_mp(&self) -> Dma0Mp {
        let val = (self.0 >> 0usize) & 0x03;
        Dma0Mp::from_bits(val as u8)
    }
    #[doc = "DMA_0_MP."]
    #[inline(always)]
    pub const fn set_dma_0_mp(&mut self, val: Dma0Mp) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "DMA_0_CH0."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch0(&self) -> Dma0Ch0 {
        let val = (self.0 >> 4usize) & 0x03;
        Dma0Ch0::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH0."]
    #[inline(always)]
    pub const fn set_dma_0_ch0(&mut self, val: Dma0Ch0) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "DMA_0_CH1."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch1(&self) -> Dma0Ch1 {
        let val = (self.0 >> 8usize) & 0x03;
        Dma0Ch1::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH1."]
    #[inline(always)]
    pub const fn set_dma_0_ch1(&mut self, val: Dma0Ch1) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "DMA_0_CH2."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch2(&self) -> Dma0Ch2 {
        let val = (self.0 >> 12usize) & 0x03;
        Dma0Ch2::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH2."]
    #[inline(always)]
    pub const fn set_dma_0_ch2(&mut self, val: Dma0Ch2) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "DMA_0_CH3."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch3(&self) -> Dma0Ch3 {
        let val = (self.0 >> 16usize) & 0x03;
        Dma0Ch3::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH3."]
    #[inline(always)]
    pub const fn set_dma_0_ch3(&mut self, val: Dma0Ch3) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "DMA_0_CH4."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch4(&self) -> Dma0Ch4 {
        let val = (self.0 >> 20usize) & 0x03;
        Dma0Ch4::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH4."]
    #[inline(always)]
    pub const fn set_dma_0_ch4(&mut self, val: Dma0Ch4) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "DMA_0_CH5."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch5(&self) -> Dma0Ch5 {
        let val = (self.0 >> 24usize) & 0x03;
        Dma0Ch5::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH5."]
    #[inline(always)]
    pub const fn set_dma_0_ch5(&mut self, val: Dma0Ch5) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "DMA_0_CH6."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch6(&self) -> Dma0Ch6 {
        let val = (self.0 >> 28usize) & 0x03;
        Dma0Ch6::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH6."]
    #[inline(always)]
    pub const fn set_dma_0_ch6(&mut self, val: Dma0Ch6) {
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
            .field("dma_0_mp", &self.dma_0_mp())
            .field("dma_0_ch0", &self.dma_0_ch0())
            .field("dma_0_ch1", &self.dma_0_ch1())
            .field("dma_0_ch2", &self.dma_0_ch2())
            .field("dma_0_ch3", &self.dma_0_ch3())
            .field("dma_0_ch4", &self.dma_0_ch4())
            .field("dma_0_ch5", &self.dma_0_ch5())
            .field("dma_0_ch6", &self.dma_0_ch6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule0 {{ dma_0_mp: {:?}, dma_0_ch0: {:?}, dma_0_ch1: {:?}, dma_0_ch2: {:?}, dma_0_ch3: {:?}, dma_0_ch4: {:?}, dma_0_ch5: {:?}, dma_0_ch6: {:?} }}",
            self.dma_0_mp(),
            self.dma_0_ch0(),
            self.dma_0_ch1(),
            self.dma_0_ch2(),
            self.dma_0_ch3(),
            self.dma_0_ch4(),
            self.dma_0_ch5(),
            self.dma_0_ch6()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule1(pub u32);
impl AipsBridgeGroup2MemRule1 {
    #[doc = "DMA_0_CH7."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch7(&self) -> Dma0Ch7 {
        let val = (self.0 >> 0usize) & 0x03;
        Dma0Ch7::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH7."]
    #[inline(always)]
    pub const fn set_dma_0_ch7(&mut self, val: Dma0Ch7) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "DMA_0_CH8."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch8(&self) -> Dma0Ch8 {
        let val = (self.0 >> 4usize) & 0x03;
        Dma0Ch8::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH8."]
    #[inline(always)]
    pub const fn set_dma_0_ch8(&mut self, val: Dma0Ch8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "DMA_0_CH9."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch9(&self) -> Dma0Ch9 {
        let val = (self.0 >> 8usize) & 0x03;
        Dma0Ch9::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH9."]
    #[inline(always)]
    pub const fn set_dma_0_ch9(&mut self, val: Dma0Ch9) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "DMA_0_CH10."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch10(&self) -> Dma0Ch10 {
        let val = (self.0 >> 12usize) & 0x03;
        Dma0Ch10::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH10."]
    #[inline(always)]
    pub const fn set_dma_0_ch10(&mut self, val: Dma0Ch10) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "DMA_0_CH11."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch11(&self) -> Dma0Ch11 {
        let val = (self.0 >> 16usize) & 0x03;
        Dma0Ch11::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH11."]
    #[inline(always)]
    pub const fn set_dma_0_ch11(&mut self, val: Dma0Ch11) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
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
            .field("dma_0_ch7", &self.dma_0_ch7())
            .field("dma_0_ch8", &self.dma_0_ch8())
            .field("dma_0_ch9", &self.dma_0_ch9())
            .field("dma_0_ch10", &self.dma_0_ch10())
            .field("dma_0_ch11", &self.dma_0_ch11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule1 {{ dma_0_ch7: {:?}, dma_0_ch8: {:?}, dma_0_ch9: {:?}, dma_0_ch10: {:?}, dma_0_ch11: {:?} }}",
            self.dma_0_ch7(),
            self.dma_0_ch8(),
            self.dma_0_ch9(),
            self.dma_0_ch10(),
            self.dma_0_ch11()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 10."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule10(pub u32);
impl AipsBridgeGroup2MemRule10 {
    #[doc = "CAN1 Region 0."]
    #[must_use]
    #[inline(always)]
    pub const fn can1_region0(&self) -> Can1Region0 {
        let val = (self.0 >> 0usize) & 0x03;
        Can1Region0::from_bits(val as u8)
    }
    #[doc = "CAN1 Region 0."]
    #[inline(always)]
    pub const fn set_can1_region0(&mut self, val: Can1Region0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CAN1 Region 1."]
    #[must_use]
    #[inline(always)]
    pub const fn can1_region1(&self) -> Can1Region1 {
        let val = (self.0 >> 4usize) & 0x03;
        Can1Region1::from_bits(val as u8)
    }
    #[doc = "CAN1 Region 1."]
    #[inline(always)]
    pub const fn set_can1_region1(&mut self, val: Can1Region1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CAN1 Region 2."]
    #[must_use]
    #[inline(always)]
    pub const fn can1_region2(&self) -> Can1Region2 {
        let val = (self.0 >> 8usize) & 0x03;
        Can1Region2::from_bits(val as u8)
    }
    #[doc = "CAN1 Region 2."]
    #[inline(always)]
    pub const fn set_can1_region2(&mut self, val: Can1Region2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CAN1 Region 3."]
    #[must_use]
    #[inline(always)]
    pub const fn can1_region3(&self) -> Can1Region3 {
        let val = (self.0 >> 12usize) & 0x03;
        Can1Region3::from_bits(val as u8)
    }
    #[doc = "CAN1 Region 3."]
    #[inline(always)]
    pub const fn set_can1_region3(&mut self, val: Can1Region3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "LPI2C2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2(&self) -> Lpi2c2 {
        let val = (self.0 >> 16usize) & 0x03;
        Lpi2c2::from_bits(val as u8)
    }
    #[doc = "LPI2C2."]
    #[inline(always)]
    pub const fn set_lpi2c2(&mut self, val: Lpi2c2) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "LPI2C3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3(&self) -> Lpi2c3 {
        let val = (self.0 >> 20usize) & 0x03;
        Lpi2c3::from_bits(val as u8)
    }
    #[doc = "LPI2C3."]
    #[inline(always)]
    pub const fn set_lpi2c3(&mut self, val: Lpi2c3) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "LPI2C4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c4(&self) -> Lpi2c4 {
        let val = (self.0 >> 24usize) & 0x03;
        Lpi2c4::from_bits(val as u8)
    }
    #[doc = "LPI2C4."]
    #[inline(always)]
    pub const fn set_lpi2c4(&mut self, val: Lpi2c4) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for AipsBridgeGroup2MemRule10 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule10 {
        AipsBridgeGroup2MemRule10(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule10")
            .field("can1_region0", &self.can1_region0())
            .field("can1_region1", &self.can1_region1())
            .field("can1_region2", &self.can1_region2())
            .field("can1_region3", &self.can1_region3())
            .field("lpi2c2", &self.lpi2c2())
            .field("lpi2c3", &self.lpi2c3())
            .field("lpi2c4", &self.lpi2c4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule10 {{ can1_region0: {:?}, can1_region1: {:?}, can1_region2: {:?}, can1_region3: {:?}, lpi2c2: {:?}, lpi2c3: {:?}, lpi2c4: {:?} }}",
            self.can1_region0(),
            self.can1_region1(),
            self.can1_region2(),
            self.can1_region3(),
            self.lpi2c2(),
            self.lpi2c3(),
            self.lpi2c4()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 11."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule11(pub u32);
impl AipsBridgeGroup2MemRule11 {
    #[doc = "LPUART5."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart5(&self) -> Lpuart5 {
        let val = (self.0 >> 8usize) & 0x03;
        Lpuart5::from_bits(val as u8)
    }
    #[doc = "LPUART5."]
    #[inline(always)]
    pub const fn set_lpuart5(&mut self, val: Lpuart5) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "I3C3."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c3(&self) -> I3c3 {
        let val = (self.0 >> 24usize) & 0x03;
        I3c3::from_bits(val as u8)
    }
    #[doc = "I3C3."]
    #[inline(always)]
    pub const fn set_i3c3(&mut self, val: I3c3) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "GPIO5."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio5(&self) -> Gpio5 {
        let val = (self.0 >> 28usize) & 0x03;
        Gpio5::from_bits(val as u8)
    }
    #[doc = "GPIO5."]
    #[inline(always)]
    pub const fn set_gpio5(&mut self, val: Gpio5) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2MemRule11 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule11 {
        AipsBridgeGroup2MemRule11(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule11")
            .field("lpuart5", &self.lpuart5())
            .field("i3c3", &self.i3c3())
            .field("gpio5", &self.gpio5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule11 {{ lpuart5: {:?}, i3c3: {:?}, gpio5: {:?} }}",
            self.lpuart5(),
            self.i3c3(),
            self.gpio5()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 12."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule12(pub u32);
impl AipsBridgeGroup2MemRule12 {
    #[doc = "GPIO5_ALIAS."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio5_alias(&self) -> Gpio5Alias {
        let val = (self.0 >> 0usize) & 0x03;
        Gpio5Alias::from_bits(val as u8)
    }
    #[doc = "GPIO5_ALIAS."]
    #[inline(always)]
    pub const fn set_gpio5_alias(&mut self, val: Gpio5Alias) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "PORT5."]
    #[must_use]
    #[inline(always)]
    pub const fn port5(&self) -> Port5 {
        let val = (self.0 >> 12usize) & 0x03;
        Port5::from_bits(val as u8)
    }
    #[doc = "PORT5."]
    #[inline(always)]
    pub const fn set_port5(&mut self, val: Port5) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "DGDET0."]
    #[must_use]
    #[inline(always)]
    pub const fn dgdet0(&self) -> Dgdet0 {
        let val = (self.0 >> 20usize) & 0x03;
        Dgdet0::from_bits(val as u8)
    }
    #[doc = "DGDET0."]
    #[inline(always)]
    pub const fn set_dgdet0(&mut self, val: Dgdet0) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "ITRC0."]
    #[must_use]
    #[inline(always)]
    pub const fn itrc0(&self) -> Itrc0 {
        let val = (self.0 >> 28usize) & 0x03;
        Itrc0::from_bits(val as u8)
    }
    #[doc = "ITRC0."]
    #[inline(always)]
    pub const fn set_itrc0(&mut self, val: Itrc0) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2MemRule12 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule12 {
        AipsBridgeGroup2MemRule12(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule12")
            .field("gpio5_alias", &self.gpio5_alias())
            .field("port5", &self.port5())
            .field("dgdet0", &self.dgdet0())
            .field("itrc0", &self.itrc0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule12 {{ gpio5_alias: {:?}, port5: {:?}, dgdet0: {:?}, itrc0: {:?} }}",
            self.gpio5_alias(),
            self.port5(),
            self.dgdet0(),
            self.itrc0()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 13."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule13(pub u32);
impl AipsBridgeGroup2MemRule13 {
    #[doc = "GLIKEY0."]
    #[must_use]
    #[inline(always)]
    pub const fn glikey0(&self) -> Glikey0 {
        let val = (self.0 >> 0usize) & 0x03;
        Glikey0::from_bits(val as u8)
    }
    #[doc = "GLIKEY0."]
    #[inline(always)]
    pub const fn set_glikey0(&mut self, val: Glikey0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "TDET0."]
    #[must_use]
    #[inline(always)]
    pub const fn tdet0(&self) -> Tdet0 {
        let val = (self.0 >> 4usize) & 0x03;
        Tdet0::from_bits(val as u8)
    }
    #[doc = "TDET0."]
    #[inline(always)]
    pub const fn set_tdet0(&mut self, val: Tdet0) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "SECCON."]
    #[must_use]
    #[inline(always)]
    pub const fn seccon(&self) -> Seccon {
        let val = (self.0 >> 8usize) & 0x03;
        Seccon::from_bits(val as u8)
    }
    #[doc = "SECCON."]
    #[inline(always)]
    pub const fn set_seccon(&mut self, val: Seccon) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "SGI0."]
    #[must_use]
    #[inline(always)]
    pub const fn sgi0(&self) -> Sgi0 {
        let val = (self.0 >> 12usize) & 0x03;
        Sgi0::from_bits(val as u8)
    }
    #[doc = "SGI0."]
    #[inline(always)]
    pub const fn set_sgi0(&mut self, val: Sgi0) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "TRNG0."]
    #[must_use]
    #[inline(always)]
    pub const fn trng0(&self) -> Trng0 {
        let val = (self.0 >> 16usize) & 0x03;
        Trng0::from_bits(val as u8)
    }
    #[doc = "TRNG0."]
    #[inline(always)]
    pub const fn set_trng0(&mut self, val: Trng0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "UDF0."]
    #[must_use]
    #[inline(always)]
    pub const fn udf0(&self) -> Udf0 {
        let val = (self.0 >> 20usize) & 0x03;
        Udf0::from_bits(val as u8)
    }
    #[doc = "UDF0."]
    #[inline(always)]
    pub const fn set_udf0(&mut self, val: Udf0) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "RTC0."]
    #[must_use]
    #[inline(always)]
    pub const fn rtc0(&self) -> Rtc0 {
        let val = (self.0 >> 24usize) & 0x03;
        Rtc0::from_bits(val as u8)
    }
    #[doc = "RTC0."]
    #[inline(always)]
    pub const fn set_rtc0(&mut self, val: Rtc0) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for AipsBridgeGroup2MemRule13 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule13 {
        AipsBridgeGroup2MemRule13(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule13")
            .field("glikey0", &self.glikey0())
            .field("tdet0", &self.tdet0())
            .field("seccon", &self.seccon())
            .field("sgi0", &self.sgi0())
            .field("trng0", &self.trng0())
            .field("udf0", &self.udf0())
            .field("rtc0", &self.rtc0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule13 {{ glikey0: {:?}, tdet0: {:?}, seccon: {:?}, sgi0: {:?}, trng0: {:?}, udf0: {:?}, rtc0: {:?} }}",
            self.glikey0(),
            self.tdet0(),
            self.seccon(),
            self.sgi0(),
            self.trng0(),
            self.udf0(),
            self.rtc0()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule2(pub u32);
impl AipsBridgeGroup2MemRule2 {
    #[doc = "SYSCON."]
    #[must_use]
    #[inline(always)]
    pub const fn syscon(&self) -> Syscon {
        let val = (self.0 >> 4usize) & 0x03;
        Syscon::from_bits(val as u8)
    }
    #[doc = "SYSCON."]
    #[inline(always)]
    pub const fn set_syscon(&mut self, val: Syscon) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "WUU."]
    #[must_use]
    #[inline(always)]
    pub const fn wuu(&self) -> Wuu {
        let val = (self.0 >> 8usize) & 0x03;
        Wuu::from_bits(val as u8)
    }
    #[doc = "WUU."]
    #[inline(always)]
    pub const fn set_wuu(&mut self, val: Wuu) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "VBAT."]
    #[must_use]
    #[inline(always)]
    pub const fn vbat(&self) -> Vbat {
        let val = (self.0 >> 12usize) & 0x03;
        Vbat::from_bits(val as u8)
    }
    #[doc = "VBAT."]
    #[inline(always)]
    pub const fn set_vbat(&mut self, val: Vbat) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "FMC."]
    #[must_use]
    #[inline(always)]
    pub const fn fmc(&self) -> Fmc {
        let val = (self.0 >> 16usize) & 0x03;
        Fmc::from_bits(val as u8)
    }
    #[doc = "FMC."]
    #[inline(always)]
    pub const fn set_fmc(&mut self, val: Fmc) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "FMU."]
    #[must_use]
    #[inline(always)]
    pub const fn fmu(&self) -> Fmu {
        let val = (self.0 >> 20usize) & 0x03;
        Fmu::from_bits(val as u8)
    }
    #[doc = "FMU."]
    #[inline(always)]
    pub const fn set_fmu(&mut self, val: Fmu) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
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
            .field("syscon", &self.syscon())
            .field("wuu", &self.wuu())
            .field("vbat", &self.vbat())
            .field("fmc", &self.fmc())
            .field("fmu", &self.fmu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule2 {{ syscon: {:?}, wuu: {:?}, vbat: {:?}, fmc: {:?}, fmu: {:?} }}",
            self.syscon(),
            self.wuu(),
            self.vbat(),
            self.fmc(),
            self.fmu()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule3(pub u32);
impl AipsBridgeGroup2MemRule3 {
    #[doc = "FLEXIO."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio(&self) -> Flexio {
        let val = (self.0 >> 4usize) & 0x03;
        Flexio::from_bits(val as u8)
    }
    #[doc = "FLEXIO."]
    #[inline(always)]
    pub const fn set_flexio(&mut self, val: Flexio) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "LPI2C0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c0(&self) -> Lpi2c0 {
        let val = (self.0 >> 8usize) & 0x03;
        Lpi2c0::from_bits(val as u8)
    }
    #[doc = "LPI2C0."]
    #[inline(always)]
    pub const fn set_lpi2c0(&mut self, val: Lpi2c0) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LPI2C1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1(&self) -> Lpi2c1 {
        let val = (self.0 >> 12usize) & 0x03;
        Lpi2c1::from_bits(val as u8)
    }
    #[doc = "LPI2C1."]
    #[inline(always)]
    pub const fn set_lpi2c1(&mut self, val: Lpi2c1) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "LPSPI0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi0(&self) -> Lpspi0 {
        let val = (self.0 >> 16usize) & 0x03;
        Lpspi0::from_bits(val as u8)
    }
    #[doc = "LPSPI0."]
    #[inline(always)]
    pub const fn set_lpspi0(&mut self, val: Lpspi0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "LPSPI1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1(&self) -> Lpspi1 {
        let val = (self.0 >> 20usize) & 0x03;
        Lpspi1::from_bits(val as u8)
    }
    #[doc = "LPSPI1."]
    #[inline(always)]
    pub const fn set_lpspi1(&mut self, val: Lpspi1) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "I3C2."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c2(&self) -> I3c2 {
        let val = (self.0 >> 24usize) & 0x03;
        I3c2::from_bits(val as u8)
    }
    #[doc = "I3C2."]
    #[inline(always)]
    pub const fn set_i3c2(&mut self, val: I3c2) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "LPUART0."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart0(&self) -> Lpuart0 {
        let val = (self.0 >> 28usize) & 0x03;
        Lpuart0::from_bits(val as u8)
    }
    #[doc = "LPUART0."]
    #[inline(always)]
    pub const fn set_lpuart0(&mut self, val: Lpuart0) {
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
            .field("flexio", &self.flexio())
            .field("lpi2c0", &self.lpi2c0())
            .field("lpi2c1", &self.lpi2c1())
            .field("lpspi0", &self.lpspi0())
            .field("lpspi1", &self.lpspi1())
            .field("i3c2", &self.i3c2())
            .field("lpuart0", &self.lpuart0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule3 {{ flexio: {:?}, lpi2c0: {:?}, lpi2c1: {:?}, lpspi0: {:?}, lpspi1: {:?}, i3c2: {:?}, lpuart0: {:?} }}",
            self.flexio(),
            self.lpi2c0(),
            self.lpi2c1(),
            self.lpspi0(),
            self.lpspi1(),
            self.i3c2(),
            self.lpuart0()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule4(pub u32);
impl AipsBridgeGroup2MemRule4 {
    #[doc = "LPUART1."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1(&self) -> Lpuart1 {
        let val = (self.0 >> 0usize) & 0x03;
        Lpuart1::from_bits(val as u8)
    }
    #[doc = "LPUART1."]
    #[inline(always)]
    pub const fn set_lpuart1(&mut self, val: Lpuart1) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "LPUART2."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2(&self) -> Lpuart2 {
        let val = (self.0 >> 4usize) & 0x03;
        Lpuart2::from_bits(val as u8)
    }
    #[doc = "LPUART2."]
    #[inline(always)]
    pub const fn set_lpuart2(&mut self, val: Lpuart2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "LPUART3."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3(&self) -> Lpuart3 {
        let val = (self.0 >> 8usize) & 0x03;
        Lpuart3::from_bits(val as u8)
    }
    #[doc = "LPUART3."]
    #[inline(always)]
    pub const fn set_lpuart3(&mut self, val: Lpuart3) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LPUART4."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4(&self) -> Lpuart4 {
        let val = (self.0 >> 12usize) & 0x03;
        Lpuart4::from_bits(val as u8)
    }
    #[doc = "LPUART4."]
    #[inline(always)]
    pub const fn set_lpuart4(&mut self, val: Lpuart4) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for AipsBridgeGroup2MemRule4 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule4 {
        AipsBridgeGroup2MemRule4(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule4")
            .field("lpuart1", &self.lpuart1())
            .field("lpuart2", &self.lpuart2())
            .field("lpuart3", &self.lpuart3())
            .field("lpuart4", &self.lpuart4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule4 {{ lpuart1: {:?}, lpuart2: {:?}, lpuart3: {:?}, lpuart4: {:?} }}",
            self.lpuart1(),
            self.lpuart2(),
            self.lpuart3(),
            self.lpuart4()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule5(pub u32);
impl AipsBridgeGroup2MemRule5 {
    #[doc = "LPTMR."]
    #[must_use]
    #[inline(always)]
    pub const fn lptmr(&self) -> Lptmr {
        let val = (self.0 >> 12usize) & 0x03;
        Lptmr::from_bits(val as u8)
    }
    #[doc = "LPTMR."]
    #[inline(always)]
    pub const fn set_lptmr(&mut self, val: Lptmr) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "OSTIMER."]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer(&self) -> Ostimer {
        let val = (self.0 >> 20usize) & 0x03;
        Ostimer::from_bits(val as u8)
    }
    #[doc = "OSTIMER."]
    #[inline(always)]
    pub const fn set_ostimer(&mut self, val: Ostimer) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "WAKE_TIMER."]
    #[must_use]
    #[inline(always)]
    pub const fn wake_timer(&self) -> WakeTimer {
        let val = (self.0 >> 24usize) & 0x03;
        WakeTimer::from_bits(val as u8)
    }
    #[doc = "WAKE_TIMER."]
    #[inline(always)]
    pub const fn set_wake_timer(&mut self, val: WakeTimer) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "ADC0."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> Adc0 {
        let val = (self.0 >> 28usize) & 0x03;
        Adc0::from_bits(val as u8)
    }
    #[doc = "ADC0."]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: Adc0) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2MemRule5 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule5 {
        AipsBridgeGroup2MemRule5(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule5")
            .field("lptmr", &self.lptmr())
            .field("ostimer", &self.ostimer())
            .field("wake_timer", &self.wake_timer())
            .field("adc0", &self.adc0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule5 {{ lptmr: {:?}, ostimer: {:?}, wake_timer: {:?}, adc0: {:?} }}",
            self.lptmr(),
            self.ostimer(),
            self.wake_timer(),
            self.adc0()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 6."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule6(pub u32);
impl AipsBridgeGroup2MemRule6 {
    #[doc = "ADC1."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> Adc1 {
        let val = (self.0 >> 0usize) & 0x03;
        Adc1::from_bits(val as u8)
    }
    #[doc = "ADC1."]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: Adc1) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CMP0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp0(&self) -> Cmp0 {
        let val = (self.0 >> 4usize) & 0x03;
        Cmp0::from_bits(val as u8)
    }
    #[doc = "CMP0."]
    #[inline(always)]
    pub const fn set_cmp0(&mut self, val: Cmp0) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "DAC0."]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> Dac0 {
        let val = (self.0 >> 16usize) & 0x03;
        Dac0::from_bits(val as u8)
    }
    #[doc = "DAC0."]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: Dac0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "DAC1."]
    #[must_use]
    #[inline(always)]
    pub const fn dac1(&self) -> Dac1 {
        let val = (self.0 >> 20usize) & 0x03;
        Dac1::from_bits(val as u8)
    }
    #[doc = "DAC1."]
    #[inline(always)]
    pub const fn set_dac1(&mut self, val: Dac1) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
}
impl Default for AipsBridgeGroup2MemRule6 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule6 {
        AipsBridgeGroup2MemRule6(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule6")
            .field("adc1", &self.adc1())
            .field("cmp0", &self.cmp0())
            .field("dac0", &self.dac0())
            .field("dac1", &self.dac1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule6 {{ adc1: {:?}, cmp0: {:?}, dac0: {:?}, dac1: {:?} }}",
            self.adc1(),
            self.cmp0(),
            self.dac0(),
            self.dac1()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 7."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule7(pub u32);
impl AipsBridgeGroup2MemRule7 {
    #[doc = "VREF0."]
    #[must_use]
    #[inline(always)]
    pub const fn vref0(&self) -> Vref0 {
        let val = (self.0 >> 12usize) & 0x03;
        Vref0::from_bits(val as u8)
    }
    #[doc = "VREF0."]
    #[inline(always)]
    pub const fn set_vref0(&mut self, val: Vref0) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "PORT0."]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> Port0 {
        let val = (self.0 >> 16usize) & 0x03;
        Port0::from_bits(val as u8)
    }
    #[doc = "PORT0."]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: Port0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "PORT1."]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> Port1 {
        let val = (self.0 >> 20usize) & 0x03;
        Port1::from_bits(val as u8)
    }
    #[doc = "PORT1."]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: Port1) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "PORT2."]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> Port2 {
        let val = (self.0 >> 24usize) & 0x03;
        Port2::from_bits(val as u8)
    }
    #[doc = "PORT2."]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: Port2) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "PORT3."]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> Port3 {
        let val = (self.0 >> 28usize) & 0x03;
        Port3::from_bits(val as u8)
    }
    #[doc = "PORT3."]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: Port3) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2MemRule7 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule7 {
        AipsBridgeGroup2MemRule7(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule7")
            .field("vref0", &self.vref0())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule7 {{ vref0: {:?}, port0: {:?}, port1: {:?}, port2: {:?}, port3: {:?} }}",
            self.vref0(),
            self.port0(),
            self.port1(),
            self.port2(),
            self.port3()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 8."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule8(pub u32);
impl AipsBridgeGroup2MemRule8 {
    #[doc = "PORT4."]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> Port4 {
        let val = (self.0 >> 0usize) & 0x03;
        Port4::from_bits(val as u8)
    }
    #[doc = "PORT4."]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: Port4) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "TSI0."]
    #[must_use]
    #[inline(always)]
    pub const fn tsi0(&self) -> Tsi0 {
        let val = (self.0 >> 12usize) & 0x03;
        Tsi0::from_bits(val as u8)
    }
    #[doc = "TSI0."]
    #[inline(always)]
    pub const fn set_tsi0(&mut self, val: Tsi0) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "AOI0."]
    #[must_use]
    #[inline(always)]
    pub const fn aoi0(&self) -> Aoi0 {
        let val = (self.0 >> 16usize) & 0x03;
        Aoi0::from_bits(val as u8)
    }
    #[doc = "AOI0."]
    #[inline(always)]
    pub const fn set_aoi0(&mut self, val: Aoi0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "CRC0."]
    #[must_use]
    #[inline(always)]
    pub const fn crc0(&self) -> Crc0 {
        let val = (self.0 >> 20usize) & 0x03;
        Crc0::from_bits(val as u8)
    }
    #[doc = "CRC0."]
    #[inline(always)]
    pub const fn set_crc0(&mut self, val: Crc0) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "CMC."]
    #[must_use]
    #[inline(always)]
    pub const fn cmc(&self) -> Cmc {
        let val = (self.0 >> 24usize) & 0x03;
        Cmc::from_bits(val as u8)
    }
    #[doc = "CMC."]
    #[inline(always)]
    pub const fn set_cmc(&mut self, val: Cmc) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "EIM."]
    #[must_use]
    #[inline(always)]
    pub const fn eim(&self) -> Eim {
        let val = (self.0 >> 28usize) & 0x03;
        Eim::from_bits(val as u8)
    }
    #[doc = "EIM."]
    #[inline(always)]
    pub const fn set_eim(&mut self, val: Eim) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2MemRule8 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule8 {
        AipsBridgeGroup2MemRule8(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule8")
            .field("port4", &self.port4())
            .field("tsi0", &self.tsi0())
            .field("aoi0", &self.aoi0())
            .field("crc0", &self.crc0())
            .field("cmc", &self.cmc())
            .field("eim", &self.eim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule8 {{ port4: {:?}, tsi0: {:?}, aoi0: {:?}, crc0: {:?}, cmc: {:?}, eim: {:?} }}",
            self.port4(),
            self.tsi0(),
            self.aoi0(),
            self.crc0(),
            self.cmc(),
            self.eim()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 9."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule9(pub u32);
impl AipsBridgeGroup2MemRule9 {
    #[doc = "ERM."]
    #[must_use]
    #[inline(always)]
    pub const fn erm(&self) -> Erm {
        let val = (self.0 >> 0usize) & 0x03;
        Erm::from_bits(val as u8)
    }
    #[doc = "ERM."]
    #[inline(always)]
    pub const fn set_erm(&mut self, val: Erm) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "MBC."]
    #[must_use]
    #[inline(always)]
    pub const fn mbc(&self) -> Mbc {
        let val = (self.0 >> 4usize) & 0x03;
        Mbc::from_bits(val as u8)
    }
    #[doc = "MBC."]
    #[inline(always)]
    pub const fn set_mbc(&mut self, val: Mbc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "SCG."]
    #[must_use]
    #[inline(always)]
    pub const fn scg(&self) -> Scg {
        let val = (self.0 >> 8usize) & 0x03;
        Scg::from_bits(val as u8)
    }
    #[doc = "SCG."]
    #[inline(always)]
    pub const fn set_scg(&mut self, val: Scg) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "SPC."]
    #[must_use]
    #[inline(always)]
    pub const fn spc(&self) -> Spc {
        let val = (self.0 >> 12usize) & 0x03;
        Spc::from_bits(val as u8)
    }
    #[doc = "SPC."]
    #[inline(always)]
    pub const fn set_spc(&mut self, val: Spc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "CAN0 Region 0."]
    #[must_use]
    #[inline(always)]
    pub const fn can0_region0(&self) -> Can0Region0 {
        let val = (self.0 >> 16usize) & 0x03;
        Can0Region0::from_bits(val as u8)
    }
    #[doc = "CAN0 Region 0."]
    #[inline(always)]
    pub const fn set_can0_region0(&mut self, val: Can0Region0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "CAN0 Region 1."]
    #[must_use]
    #[inline(always)]
    pub const fn can0_region1(&self) -> Can0Region1 {
        let val = (self.0 >> 20usize) & 0x03;
        Can0Region1::from_bits(val as u8)
    }
    #[doc = "CAN0 Region 1."]
    #[inline(always)]
    pub const fn set_can0_region1(&mut self, val: Can0Region1) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "CAN0 Region 2."]
    #[must_use]
    #[inline(always)]
    pub const fn can0_region2(&self) -> Can0Region2 {
        let val = (self.0 >> 24usize) & 0x03;
        Can0Region2::from_bits(val as u8)
    }
    #[doc = "CAN0 Region 2."]
    #[inline(always)]
    pub const fn set_can0_region2(&mut self, val: Can0Region2) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "CAN0 Region 3."]
    #[must_use]
    #[inline(always)]
    pub const fn can0_region3(&self) -> Can0Region3 {
        let val = (self.0 >> 28usize) & 0x03;
        Can0Region3::from_bits(val as u8)
    }
    #[doc = "CAN0 Region 3."]
    #[inline(always)]
    pub const fn set_can0_region3(&mut self, val: Can0Region3) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2MemRule9 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule9 {
        AipsBridgeGroup2MemRule9(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule9")
            .field("erm", &self.erm())
            .field("mbc", &self.mbc())
            .field("scg", &self.scg())
            .field("spc", &self.spc())
            .field("can0_region0", &self.can0_region0())
            .field("can0_region1", &self.can0_region1())
            .field("can0_region2", &self.can0_region2())
            .field("can0_region3", &self.can0_region3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule9 {{ erm: {:?}, mbc: {:?}, scg: {:?}, spc: {:?}, can0_region0: {:?}, can0_region1: {:?}, can0_region2: {:?}, can0_region3: {:?} }}",
            self.erm(),
            self.mbc(),
            self.scg(),
            self.spc(),
            self.can0_region0(),
            self.can0_region1(),
            self.can0_region2(),
            self.can0_region3()
        )
    }
}
#[doc = "APB Bridge Group 0 Memory Rule Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbPeripheralGroup0MemRule0(pub u32);
impl ApbPeripheralGroup0MemRule0 {
    #[doc = "INPUTMUX."]
    #[must_use]
    #[inline(always)]
    pub const fn inputmux(&self) -> Inputmux {
        let val = (self.0 >> 4usize) & 0x03;
        Inputmux::from_bits(val as u8)
    }
    #[doc = "INPUTMUX."]
    #[inline(always)]
    pub const fn set_inputmux(&mut self, val: Inputmux) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "I3C0."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> I3c0 {
        let val = (self.0 >> 8usize) & 0x03;
        I3c0::from_bits(val as u8)
    }
    #[doc = "I3C0."]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: I3c0) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "I3C1."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c1(&self) -> I3c1 {
        let val = (self.0 >> 12usize) & 0x03;
        I3c1::from_bits(val as u8)
    }
    #[doc = "I3C1."]
    #[inline(always)]
    pub const fn set_i3c1(&mut self, val: I3c1) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "CTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0(&self) -> Ctimer0 {
        let val = (self.0 >> 16usize) & 0x03;
        Ctimer0::from_bits(val as u8)
    }
    #[doc = "CTIMER0."]
    #[inline(always)]
    pub const fn set_ctimer0(&mut self, val: Ctimer0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "CTIMER1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1(&self) -> Ctimer1 {
        let val = (self.0 >> 20usize) & 0x03;
        Ctimer1::from_bits(val as u8)
    }
    #[doc = "CTIMER1."]
    #[inline(always)]
    pub const fn set_ctimer1(&mut self, val: Ctimer1) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "CTIMER2."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2(&self) -> Ctimer2 {
        let val = (self.0 >> 24usize) & 0x03;
        Ctimer2::from_bits(val as u8)
    }
    #[doc = "CTIMER2."]
    #[inline(always)]
    pub const fn set_ctimer2(&mut self, val: Ctimer2) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "CTIMER3."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer3(&self) -> Ctimer3 {
        let val = (self.0 >> 28usize) & 0x03;
        Ctimer3::from_bits(val as u8)
    }
    #[doc = "CTIMER3."]
    #[inline(always)]
    pub const fn set_ctimer3(&mut self, val: Ctimer3) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
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
            .field("inputmux", &self.inputmux())
            .field("i3c0", &self.i3c0())
            .field("i3c1", &self.i3c1())
            .field("ctimer0", &self.ctimer0())
            .field("ctimer1", &self.ctimer1())
            .field("ctimer2", &self.ctimer2())
            .field("ctimer3", &self.ctimer3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbPeripheralGroup0MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbPeripheralGroup0MemRule0 {{ inputmux: {:?}, i3c0: {:?}, i3c1: {:?}, ctimer0: {:?}, ctimer1: {:?}, ctimer2: {:?}, ctimer3: {:?} }}",
            self.inputmux(),
            self.i3c0(),
            self.i3c1(),
            self.ctimer0(),
            self.ctimer1(),
            self.ctimer2(),
            self.ctimer3()
        )
    }
}
#[doc = "APB Bridge Group 0 Memory Rule Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbPeripheralGroup0MemRule1(pub u32);
impl ApbPeripheralGroup0MemRule1 {
    #[doc = "CTIMER4."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4(&self) -> Ctimer4 {
        let val = (self.0 >> 0usize) & 0x03;
        Ctimer4::from_bits(val as u8)
    }
    #[doc = "CTIMER4."]
    #[inline(always)]
    pub const fn set_ctimer4(&mut self, val: Ctimer4) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "FREQME."]
    #[must_use]
    #[inline(always)]
    pub const fn freqme(&self) -> Freqme {
        let val = (self.0 >> 4usize) & 0x03;
        Freqme::from_bits(val as u8)
    }
    #[doc = "FREQME."]
    #[inline(always)]
    pub const fn set_freqme(&mut self, val: Freqme) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "UTCIK0."]
    #[must_use]
    #[inline(always)]
    pub const fn utick(&self) -> Utick {
        let val = (self.0 >> 12usize) & 0x03;
        Utick::from_bits(val as u8)
    }
    #[doc = "UTCIK0."]
    #[inline(always)]
    pub const fn set_utick(&mut self, val: Utick) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "WWDT0."]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> Wwdt0 {
        let val = (self.0 >> 16usize) & 0x03;
        Wwdt0::from_bits(val as u8)
    }
    #[doc = "WWDT0."]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: Wwdt0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "WWDT1."]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt1(&self) -> Wwdt1 {
        let val = (self.0 >> 20usize) & 0x03;
        Wwdt1::from_bits(val as u8)
    }
    #[doc = "WWDT1."]
    #[inline(always)]
    pub const fn set_wwdt1(&mut self, val: Wwdt1) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "SmartDMA."]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma(&self) -> ApbPeripheralGroup0MemRule1Smartdma {
        let val = (self.0 >> 24usize) & 0x03;
        ApbPeripheralGroup0MemRule1Smartdma::from_bits(val as u8)
    }
    #[doc = "SmartDMA."]
    #[inline(always)]
    pub const fn set_smartdma(&mut self, val: ApbPeripheralGroup0MemRule1Smartdma) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
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
            .field("ctimer4", &self.ctimer4())
            .field("freqme", &self.freqme())
            .field("utick", &self.utick())
            .field("wwdt0", &self.wwdt0())
            .field("wwdt1", &self.wwdt1())
            .field("smartdma", &self.smartdma())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbPeripheralGroup0MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbPeripheralGroup0MemRule1 {{ ctimer4: {:?}, freqme: {:?}, utick: {:?}, wwdt0: {:?}, wwdt1: {:?}, smartdma: {:?} }}",
            self.ctimer4(),
            self.freqme(),
            self.utick(),
            self.wwdt0(),
            self.wwdt1(),
            self.smartdma()
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
    pub const fn rule0(&self) -> Flash00MemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        Flash00MemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: Flash00MemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> Flash00MemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        Flash00MemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1."]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: Flash00MemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2."]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> Flash00MemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        Flash00MemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2."]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: Flash00MemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3."]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> Flash00MemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        Flash00MemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3."]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: Flash00MemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4."]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> Flash00MemRuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        Flash00MemRuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4."]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: Flash00MemRuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5."]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> Flash00MemRuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        Flash00MemRuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5."]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: Flash00MemRuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6."]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> Flash00MemRuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        Flash00MemRuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6."]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: Flash00MemRuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7."]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> Flash00MemRuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        Flash00MemRuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7."]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: Flash00MemRuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
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
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flash00MemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flash00MemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "Flash Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash01MemRule(pub u32);
impl Flash01MemRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> Flash01MemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        Flash01MemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: Flash01MemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> Flash01MemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        Flash01MemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1."]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: Flash01MemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2."]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> Flash01MemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        Flash01MemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2."]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: Flash01MemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3."]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> Flash01MemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        Flash01MemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3."]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: Flash01MemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4."]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> Flash01MemRuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        Flash01MemRuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4."]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: Flash01MemRuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5."]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> Flash01MemRuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        Flash01MemRuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5."]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: Flash01MemRuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6."]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> Flash01MemRuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        Flash01MemRuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6."]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: Flash01MemRuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7."]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> Flash01MemRuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        Flash01MemRuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7."]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: Flash01MemRuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Flash01MemRule {
    #[inline(always)]
    fn default() -> Flash01MemRule {
        Flash01MemRule(0)
    }
}
impl core::fmt::Debug for Flash01MemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flash01MemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flash01MemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flash01MemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "Flash IFR0 Rule register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash02MemRule(pub u32);
impl Flash02MemRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> Flash02MemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        Flash02MemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: Flash02MemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> Flash02MemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        Flash02MemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1."]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: Flash02MemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2."]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> Flash02MemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        Flash02MemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2."]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: Flash02MemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3."]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> Flash02MemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        Flash02MemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3."]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: Flash02MemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for Flash02MemRule {
    #[inline(always)]
    fn default() -> Flash02MemRule {
        Flash02MemRule(0)
    }
}
impl core::fmt::Debug for Flash02MemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flash02MemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flash02MemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flash02MemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3()
        )
    }
}
#[doc = "Flash Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash03MemRule(pub u32);
impl Flash03MemRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> Flash03MemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        Flash03MemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: Flash03MemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> Flash03MemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        Flash03MemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1."]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: Flash03MemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2."]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> Flash03MemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        Flash03MemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2."]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: Flash03MemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3."]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> Flash03MemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        Flash03MemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3."]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: Flash03MemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for Flash03MemRule {
    #[inline(always)]
    fn default() -> Flash03MemRule {
        Flash03MemRule(0)
    }
}
impl core::fmt::Debug for Flash03MemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flash03MemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flash03MemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flash03MemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3()
        )
    }
}
#[doc = "FLEXSPI0 Region 0 Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi0Region0MemRule(pub u32);
impl Flexspi0Region0MemRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> Flexspi0Region0MemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        Flexspi0Region0MemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: Flexspi0Region0MemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> Flexspi0Region0MemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        Flexspi0Region0MemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1."]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: Flexspi0Region0MemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2."]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> Flexspi0Region0MemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        Flexspi0Region0MemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2."]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: Flexspi0Region0MemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3."]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> Flexspi0Region0MemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        Flexspi0Region0MemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3."]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: Flexspi0Region0MemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4."]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> Flexspi0Region0MemRuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        Flexspi0Region0MemRuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4."]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: Flexspi0Region0MemRuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5."]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> Flexspi0Region0MemRuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        Flexspi0Region0MemRuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5."]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: Flexspi0Region0MemRuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6."]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> Flexspi0Region0MemRuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        Flexspi0Region0MemRuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6."]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: Flexspi0Region0MemRuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7."]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> Flexspi0Region0MemRuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        Flexspi0Region0MemRuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7."]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: Flexspi0Region0MemRuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Flexspi0Region0MemRule {
    #[inline(always)]
    fn default() -> Flexspi0Region0MemRule {
        Flexspi0Region0MemRule(0)
    }
}
impl core::fmt::Debug for Flexspi0Region0MemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi0Region0MemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi0Region0MemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexspi0Region0MemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "FLEXSPI0 Region index Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi0RegionMemRule(pub u32);
impl Flexspi0RegionMemRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> Flexspi0RegionMemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        Flexspi0RegionMemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: Flexspi0RegionMemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> Flexspi0RegionMemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        Flexspi0RegionMemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1."]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: Flexspi0RegionMemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2."]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> Flexspi0RegionMemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        Flexspi0RegionMemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2."]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: Flexspi0RegionMemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3."]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> Flexspi0RegionMemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        Flexspi0RegionMemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3."]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: Flexspi0RegionMemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for Flexspi0RegionMemRule {
    #[inline(always)]
    fn default() -> Flexspi0RegionMemRule {
        Flexspi0RegionMemRule(0)
    }
}
impl core::fmt::Debug for Flexspi0RegionMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi0RegionMemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi0RegionMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexspi0RegionMemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3()
        )
    }
}
#[doc = "Master Secure Level."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MasterSecAntiPolReg(pub u32);
impl MasterSecAntiPolReg {
    #[doc = "SMARTDMA Data."]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma(&self) -> MasterSecAntiPolRegSmartdma {
        let val = (self.0 >> 4usize) & 0x03;
        MasterSecAntiPolRegSmartdma::from_bits(val as u8)
    }
    #[doc = "SMARTDMA Data."]
    #[inline(always)]
    pub const fn set_smartdma(&mut self, val: MasterSecAntiPolRegSmartdma) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "eDMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> MasterSecAntiPolRegDma0 {
        let val = (self.0 >> 6usize) & 0x03;
        MasterSecAntiPolRegDma0::from_bits(val as u8)
    }
    #[doc = "eDMA0."]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: MasterSecAntiPolRegDma0) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "eDMA1."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1(&self) -> MasterSecAntiPolRegDma1 {
        let val = (self.0 >> 8usize) & 0x03;
        MasterSecAntiPolRegDma1::from_bits(val as u8)
    }
    #[doc = "eDMA1."]
    #[inline(always)]
    pub const fn set_dma1(&mut self, val: MasterSecAntiPolRegDma1) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "PKC."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc(&self) -> MasterSecAntiPolRegPkc {
        let val = (self.0 >> 10usize) & 0x03;
        MasterSecAntiPolRegPkc::from_bits(val as u8)
    }
    #[doc = "PKC."]
    #[inline(always)]
    pub const fn set_pkc(&mut self, val: MasterSecAntiPolRegPkc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "ENET0."]
    #[must_use]
    #[inline(always)]
    pub const fn enet0(&self) -> MasterSecAntiPolRegEnet0 {
        let val = (self.0 >> 24usize) & 0x03;
        MasterSecAntiPolRegEnet0::from_bits(val as u8)
    }
    #[doc = "ENET0."]
    #[inline(always)]
    pub const fn set_enet0(&mut self, val: MasterSecAntiPolRegEnet0) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "USB1."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1(&self) -> MasterSecAntiPolRegUsb1 {
        let val = (self.0 >> 26usize) & 0x03;
        MasterSecAntiPolRegUsb1::from_bits(val as u8)
    }
    #[doc = "USB1."]
    #[inline(always)]
    pub const fn set_usb1(&mut self, val: MasterSecAntiPolRegUsb1) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
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
            .field("smartdma", &self.smartdma())
            .field("dma0", &self.dma0())
            .field("dma1", &self.dma1())
            .field("pkc", &self.pkc())
            .field("enet0", &self.enet0())
            .field("usb1", &self.usb1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MasterSecAntiPolReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MasterSecAntiPolReg {{ smartdma: {:?}, dma0: {:?}, dma1: {:?}, pkc: {:?}, enet0: {:?}, usb1: {:?} }}",
            self.smartdma(),
            self.dma0(),
            self.dma1(),
            self.pkc(),
            self.enet0(),
            self.usb1()
        )
    }
}
#[doc = "Master Secure Level."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MasterSecLevel(pub u32);
impl MasterSecLevel {
    #[doc = "SMARTDMA Data."]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma(&self) -> MasterSecLevelSmartdma {
        let val = (self.0 >> 4usize) & 0x03;
        MasterSecLevelSmartdma::from_bits(val as u8)
    }
    #[doc = "SMARTDMA Data."]
    #[inline(always)]
    pub const fn set_smartdma(&mut self, val: MasterSecLevelSmartdma) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "eDMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> MasterSecLevelDma0 {
        let val = (self.0 >> 6usize) & 0x03;
        MasterSecLevelDma0::from_bits(val as u8)
    }
    #[doc = "eDMA0."]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: MasterSecLevelDma0) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "eDMA1."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1(&self) -> MasterSecLevelDma1 {
        let val = (self.0 >> 8usize) & 0x03;
        MasterSecLevelDma1::from_bits(val as u8)
    }
    #[doc = "eDMA1."]
    #[inline(always)]
    pub const fn set_dma1(&mut self, val: MasterSecLevelDma1) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "PKC."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc(&self) -> MasterSecLevelPkc {
        let val = (self.0 >> 10usize) & 0x03;
        MasterSecLevelPkc::from_bits(val as u8)
    }
    #[doc = "PKC."]
    #[inline(always)]
    pub const fn set_pkc(&mut self, val: MasterSecLevelPkc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "ENET0."]
    #[must_use]
    #[inline(always)]
    pub const fn enet0(&self) -> MasterSecLevelEnet0 {
        let val = (self.0 >> 24usize) & 0x03;
        MasterSecLevelEnet0::from_bits(val as u8)
    }
    #[doc = "ENET0."]
    #[inline(always)]
    pub const fn set_enet0(&mut self, val: MasterSecLevelEnet0) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "USB1."]
    #[must_use]
    #[inline(always)]
    pub const fn usb1(&self) -> MasterSecLevelUsb1 {
        let val = (self.0 >> 26usize) & 0x03;
        MasterSecLevelUsb1::from_bits(val as u8)
    }
    #[doc = "USB1."]
    #[inline(always)]
    pub const fn set_usb1(&mut self, val: MasterSecLevelUsb1) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
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
            .field("smartdma", &self.smartdma())
            .field("dma0", &self.dma0())
            .field("dma1", &self.dma1())
            .field("pkc", &self.pkc())
            .field("enet0", &self.enet0())
            .field("usb1", &self.usb1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MasterSecLevel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MasterSecLevel {{ smartdma: {:?}, dma0: {:?}, dma1: {:?}, pkc: {:?}, enet0: {:?}, usb1: {:?} }}",
            self.smartdma(),
            self.dma0(),
            self.dma1(),
            self.pkc(),
            self.enet0(),
            self.usb1()
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
    pub const fn write_lock(&self) -> MiscCtrlDpRegWriteLock {
        let val = (self.0 >> 0usize) & 0x03;
        MiscCtrlDpRegWriteLock::from_bits(val as u8)
    }
    #[doc = "Write Lock."]
    #[inline(always)]
    pub const fn set_write_lock(&mut self, val: MiscCtrlDpRegWriteLock) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Enable Secure Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_secure_checking(&self) -> MiscCtrlDpRegEnableSecureChecking {
        let val = (self.0 >> 2usize) & 0x03;
        MiscCtrlDpRegEnableSecureChecking::from_bits(val as u8)
    }
    #[doc = "Enable Secure Checking."]
    #[inline(always)]
    pub const fn set_enable_secure_checking(&mut self, val: MiscCtrlDpRegEnableSecureChecking) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable Secure Privilege Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_s_priv_check(&self) -> MiscCtrlDpRegEnableSPrivCheck {
        let val = (self.0 >> 4usize) & 0x03;
        MiscCtrlDpRegEnableSPrivCheck::from_bits(val as u8)
    }
    #[doc = "Enable Secure Privilege Checking."]
    #[inline(always)]
    pub const fn set_enable_s_priv_check(&mut self, val: MiscCtrlDpRegEnableSPrivCheck) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Enable Non-Secure Privilege Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ns_priv_check(&self) -> MiscCtrlDpRegEnableNsPrivCheck {
        let val = (self.0 >> 6usize) & 0x03;
        MiscCtrlDpRegEnableNsPrivCheck::from_bits(val as u8)
    }
    #[doc = "Enable Non-Secure Privilege Checking."]
    #[inline(always)]
    pub const fn set_enable_ns_priv_check(&mut self, val: MiscCtrlDpRegEnableNsPrivCheck) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Disable Violation Abort."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_violation_abort(&self) -> MiscCtrlDpRegDisableViolationAbort {
        let val = (self.0 >> 8usize) & 0x03;
        MiscCtrlDpRegDisableViolationAbort::from_bits(val as u8)
    }
    #[doc = "Disable Violation Abort."]
    #[inline(always)]
    pub const fn set_disable_violation_abort(&mut self, val: MiscCtrlDpRegDisableViolationAbort) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Disable Strict Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_strict_mode(&self) -> MiscCtrlDpRegDisableStrictMode {
        let val = (self.0 >> 10usize) & 0x03;
        MiscCtrlDpRegDisableStrictMode::from_bits(val as u8)
    }
    #[doc = "Disable Strict Mode."]
    #[inline(always)]
    pub const fn set_disable_strict_mode(&mut self, val: MiscCtrlDpRegDisableStrictMode) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "IDAU All Non-Secure."]
    #[must_use]
    #[inline(always)]
    pub const fn idau_all_ns(&self) -> MiscCtrlDpRegIdauAllNs {
        let val = (self.0 >> 14usize) & 0x03;
        MiscCtrlDpRegIdauAllNs::from_bits(val as u8)
    }
    #[doc = "IDAU All Non-Secure."]
    #[inline(always)]
    pub const fn set_idau_all_ns(&mut self, val: MiscCtrlDpRegIdauAllNs) {
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
    pub const fn enable_secure_checking(&self) -> MiscCtrlRegEnableSecureChecking {
        let val = (self.0 >> 2usize) & 0x03;
        MiscCtrlRegEnableSecureChecking::from_bits(val as u8)
    }
    #[doc = "Enable Secure Checking."]
    #[inline(always)]
    pub const fn set_enable_secure_checking(&mut self, val: MiscCtrlRegEnableSecureChecking) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable Secure Privilege Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_s_priv_check(&self) -> MiscCtrlRegEnableSPrivCheck {
        let val = (self.0 >> 4usize) & 0x03;
        MiscCtrlRegEnableSPrivCheck::from_bits(val as u8)
    }
    #[doc = "Enable Secure Privilege Checking."]
    #[inline(always)]
    pub const fn set_enable_s_priv_check(&mut self, val: MiscCtrlRegEnableSPrivCheck) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Enable Non-Secure Privilege Checking."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ns_priv_check(&self) -> MiscCtrlRegEnableNsPrivCheck {
        let val = (self.0 >> 6usize) & 0x03;
        MiscCtrlRegEnableNsPrivCheck::from_bits(val as u8)
    }
    #[doc = "Enable Non-Secure Privilege Checking."]
    #[inline(always)]
    pub const fn set_enable_ns_priv_check(&mut self, val: MiscCtrlRegEnableNsPrivCheck) {
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
#[doc = "RAMA Memory Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamaMemRule(pub u32);
impl RamaMemRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> RamaMemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        RamaMemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: RamaMemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> RamaMemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        RamaMemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1."]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: RamaMemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2."]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> RamaMemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        RamaMemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2."]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: RamaMemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3."]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> RamaMemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        RamaMemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3."]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: RamaMemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4."]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> RamaMemRuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        RamaMemRuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4."]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: RamaMemRuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5."]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> RamaMemRuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        RamaMemRuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5."]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: RamaMemRuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6."]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> RamaMemRuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        RamaMemRuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6."]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: RamaMemRuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7."]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> RamaMemRuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        RamaMemRuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7."]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: RamaMemRuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for RamaMemRule {
    #[inline(always)]
    fn default() -> RamaMemRule {
        RamaMemRule(0)
    }
}
impl core::fmt::Debug for RamaMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamaMemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamaMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamaMemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "RAMB Memory Rule 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RambMemRule(pub u32);
impl RambMemRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> RambMemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        RambMemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: RambMemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> RambMemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        RambMemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1."]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: RambMemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2."]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> RambMemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        RambMemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2."]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: RambMemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3."]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> RambMemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        RambMemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3."]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: RambMemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4."]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> RambMemRuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        RambMemRuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4."]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: RambMemRuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5."]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> RambMemRuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        RambMemRuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5."]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: RambMemRuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6."]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> RambMemRuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        RambMemRuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6."]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: RambMemRuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7."]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> RambMemRuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        RambMemRuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7."]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: RambMemRuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for RambMemRule {
    #[inline(always)]
    fn default() -> RambMemRule {
        RambMemRule(0)
    }
}
impl core::fmt::Debug for RambMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RambMemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RambMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RambMemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "RAMX Memory Rule."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamxMemRule(pub u32);
impl RamxMemRule {
    #[doc = "Rule 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> RamxMemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        RamxMemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: RamxMemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> RamxMemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        RamxMemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1."]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: RamxMemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2."]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> RamxMemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        RamxMemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2."]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: RamxMemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3."]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> RamxMemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        RamxMemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3."]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: RamxMemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4."]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> RamxMemRuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        RamxMemRuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4."]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: RamxMemRuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5."]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> RamxMemRuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        RamxMemRuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5."]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: RamxMemRuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6."]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> RamxMemRuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        RamxMemRuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6."]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: RamxMemRuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7."]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> RamxMemRuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        RamxMemRuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7."]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: RamxMemRuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for RamxMemRule {
    #[inline(always)]
    fn default() -> RamxMemRule {
        RamxMemRule(0)
    }
}
impl core::fmt::Debug for RamxMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamxMemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamxMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamxMemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
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
    pub const fn rule0(&self) -> RomMemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        RomMemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0."]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: RomMemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> RomMemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        RomMemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1."]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: RomMemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2."]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> RomMemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        RomMemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2."]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: RomMemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3."]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> RomMemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        RomMemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3."]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: RomMemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4."]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> RomMemRuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        RomMemRuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4."]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: RomMemRuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5."]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> RomMemRuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        RomMemRuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5."]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: RomMemRuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6."]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> RomMemRuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        RomMemRuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6."]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: RomMemRuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7."]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> RomMemRuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        RomMemRuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7."]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: RomMemRuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
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
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RomMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RomMemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "Secure general purpose registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpReg0(pub u32);
impl SecGpReg0 {
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpReg0 {
    #[inline(always)]
    fn default() -> SecGpReg0 {
        SecGpReg0(0)
    }
}
impl core::fmt::Debug for SecGpReg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpReg0")
            .field("dma0_ipd_req_0", &self.dma0_ipd_req_0())
            .field("dma0_ipd_req_1", &self.dma0_ipd_req_1())
            .field("dma0_ipd_req_2", &self.dma0_ipd_req_2())
            .field("dma0_ipd_req_3", &self.dma0_ipd_req_3())
            .field("dma0_ipd_req_4", &self.dma0_ipd_req_4())
            .field("dma0_ipd_req_5", &self.dma0_ipd_req_5())
            .field("dma0_ipd_req_6", &self.dma0_ipd_req_6())
            .field("dma0_ipd_req_7", &self.dma0_ipd_req_7())
            .field("dma0_ipd_req_8", &self.dma0_ipd_req_8())
            .field("dma0_ipd_req_9", &self.dma0_ipd_req_9())
            .field("dma0_ipd_req_10", &self.dma0_ipd_req_10())
            .field("dma0_ipd_req_11", &self.dma0_ipd_req_11())
            .field("dma0_ipd_req_12", &self.dma0_ipd_req_12())
            .field("dma0_ipd_req_13", &self.dma0_ipd_req_13())
            .field("dma0_ipd_req_14", &self.dma0_ipd_req_14())
            .field("dma0_ipd_req_15", &self.dma0_ipd_req_15())
            .field("dma0_ipd_req_16", &self.dma0_ipd_req_16())
            .field("dma0_ipd_req_17", &self.dma0_ipd_req_17())
            .field("dma0_ipd_req_18", &self.dma0_ipd_req_18())
            .field("dma0_ipd_req_19", &self.dma0_ipd_req_19())
            .field("dma0_ipd_req_20", &self.dma0_ipd_req_20())
            .field("dma0_ipd_req_21", &self.dma0_ipd_req_21())
            .field("dma0_ipd_req_22", &self.dma0_ipd_req_22())
            .field("dma0_ipd_req_23", &self.dma0_ipd_req_23())
            .field("dma0_ipd_req_24", &self.dma0_ipd_req_24())
            .field("dma0_ipd_req_25", &self.dma0_ipd_req_25())
            .field("dma0_ipd_req_26", &self.dma0_ipd_req_26())
            .field("dma0_ipd_req_27", &self.dma0_ipd_req_27())
            .field("dma0_ipd_req_28", &self.dma0_ipd_req_28())
            .field("dma0_ipd_req_29", &self.dma0_ipd_req_29())
            .field("dma0_ipd_req_30", &self.dma0_ipd_req_30())
            .field("dma0_ipd_req_31", &self.dma0_ipd_req_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpReg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpReg0 {{ dma0_ipd_req_0: {=bool:?}, dma0_ipd_req_1: {=bool:?}, dma0_ipd_req_2: {=bool:?}, dma0_ipd_req_3: {=bool:?}, dma0_ipd_req_4: {=bool:?}, dma0_ipd_req_5: {=bool:?}, dma0_ipd_req_6: {=bool:?}, dma0_ipd_req_7: {=bool:?}, dma0_ipd_req_8: {=bool:?}, dma0_ipd_req_9: {=bool:?}, dma0_ipd_req_10: {=bool:?}, dma0_ipd_req_11: {=bool:?}, dma0_ipd_req_12: {=bool:?}, dma0_ipd_req_13: {=bool:?}, dma0_ipd_req_14: {=bool:?}, dma0_ipd_req_15: {=bool:?}, dma0_ipd_req_16: {=bool:?}, dma0_ipd_req_17: {=bool:?}, dma0_ipd_req_18: {=bool:?}, dma0_ipd_req_19: {=bool:?}, dma0_ipd_req_20: {=bool:?}, dma0_ipd_req_21: {=bool:?}, dma0_ipd_req_22: {=bool:?}, dma0_ipd_req_23: {=bool:?}, dma0_ipd_req_24: {=bool:?}, dma0_ipd_req_25: {=bool:?}, dma0_ipd_req_26: {=bool:?}, dma0_ipd_req_27: {=bool:?}, dma0_ipd_req_28: {=bool:?}, dma0_ipd_req_29: {=bool:?}, dma0_ipd_req_30: {=bool:?}, dma0_ipd_req_31: {=bool:?} }}",
            self.dma0_ipd_req_0(),
            self.dma0_ipd_req_1(),
            self.dma0_ipd_req_2(),
            self.dma0_ipd_req_3(),
            self.dma0_ipd_req_4(),
            self.dma0_ipd_req_5(),
            self.dma0_ipd_req_6(),
            self.dma0_ipd_req_7(),
            self.dma0_ipd_req_8(),
            self.dma0_ipd_req_9(),
            self.dma0_ipd_req_10(),
            self.dma0_ipd_req_11(),
            self.dma0_ipd_req_12(),
            self.dma0_ipd_req_13(),
            self.dma0_ipd_req_14(),
            self.dma0_ipd_req_15(),
            self.dma0_ipd_req_16(),
            self.dma0_ipd_req_17(),
            self.dma0_ipd_req_18(),
            self.dma0_ipd_req_19(),
            self.dma0_ipd_req_20(),
            self.dma0_ipd_req_21(),
            self.dma0_ipd_req_22(),
            self.dma0_ipd_req_23(),
            self.dma0_ipd_req_24(),
            self.dma0_ipd_req_25(),
            self.dma0_ipd_req_26(),
            self.dma0_ipd_req_27(),
            self.dma0_ipd_req_28(),
            self.dma0_ipd_req_29(),
            self.dma0_ipd_req_30(),
            self.dma0_ipd_req_31()
        )
    }
}
#[doc = "Secure general purpose registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpReg1(pub u32);
impl SecGpReg1 {
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpReg1 {
    #[inline(always)]
    fn default() -> SecGpReg1 {
        SecGpReg1(0)
    }
}
impl core::fmt::Debug for SecGpReg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpReg1")
            .field("dma0_ipd_req_0", &self.dma0_ipd_req_0())
            .field("dma0_ipd_req_1", &self.dma0_ipd_req_1())
            .field("dma0_ipd_req_2", &self.dma0_ipd_req_2())
            .field("dma0_ipd_req_3", &self.dma0_ipd_req_3())
            .field("dma0_ipd_req_4", &self.dma0_ipd_req_4())
            .field("dma0_ipd_req_5", &self.dma0_ipd_req_5())
            .field("dma0_ipd_req_6", &self.dma0_ipd_req_6())
            .field("dma0_ipd_req_7", &self.dma0_ipd_req_7())
            .field("dma0_ipd_req_8", &self.dma0_ipd_req_8())
            .field("dma0_ipd_req_9", &self.dma0_ipd_req_9())
            .field("dma0_ipd_req_10", &self.dma0_ipd_req_10())
            .field("dma0_ipd_req_11", &self.dma0_ipd_req_11())
            .field("dma0_ipd_req_12", &self.dma0_ipd_req_12())
            .field("dma0_ipd_req_13", &self.dma0_ipd_req_13())
            .field("dma0_ipd_req_14", &self.dma0_ipd_req_14())
            .field("dma0_ipd_req_15", &self.dma0_ipd_req_15())
            .field("dma0_ipd_req_16", &self.dma0_ipd_req_16())
            .field("dma0_ipd_req_17", &self.dma0_ipd_req_17())
            .field("dma0_ipd_req_18", &self.dma0_ipd_req_18())
            .field("dma0_ipd_req_19", &self.dma0_ipd_req_19())
            .field("dma0_ipd_req_20", &self.dma0_ipd_req_20())
            .field("dma0_ipd_req_21", &self.dma0_ipd_req_21())
            .field("dma0_ipd_req_22", &self.dma0_ipd_req_22())
            .field("dma0_ipd_req_23", &self.dma0_ipd_req_23())
            .field("dma0_ipd_req_24", &self.dma0_ipd_req_24())
            .field("dma0_ipd_req_25", &self.dma0_ipd_req_25())
            .field("dma0_ipd_req_26", &self.dma0_ipd_req_26())
            .field("dma0_ipd_req_27", &self.dma0_ipd_req_27())
            .field("dma0_ipd_req_28", &self.dma0_ipd_req_28())
            .field("dma0_ipd_req_29", &self.dma0_ipd_req_29())
            .field("dma0_ipd_req_30", &self.dma0_ipd_req_30())
            .field("dma0_ipd_req_31", &self.dma0_ipd_req_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpReg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpReg1 {{ dma0_ipd_req_0: {=bool:?}, dma0_ipd_req_1: {=bool:?}, dma0_ipd_req_2: {=bool:?}, dma0_ipd_req_3: {=bool:?}, dma0_ipd_req_4: {=bool:?}, dma0_ipd_req_5: {=bool:?}, dma0_ipd_req_6: {=bool:?}, dma0_ipd_req_7: {=bool:?}, dma0_ipd_req_8: {=bool:?}, dma0_ipd_req_9: {=bool:?}, dma0_ipd_req_10: {=bool:?}, dma0_ipd_req_11: {=bool:?}, dma0_ipd_req_12: {=bool:?}, dma0_ipd_req_13: {=bool:?}, dma0_ipd_req_14: {=bool:?}, dma0_ipd_req_15: {=bool:?}, dma0_ipd_req_16: {=bool:?}, dma0_ipd_req_17: {=bool:?}, dma0_ipd_req_18: {=bool:?}, dma0_ipd_req_19: {=bool:?}, dma0_ipd_req_20: {=bool:?}, dma0_ipd_req_21: {=bool:?}, dma0_ipd_req_22: {=bool:?}, dma0_ipd_req_23: {=bool:?}, dma0_ipd_req_24: {=bool:?}, dma0_ipd_req_25: {=bool:?}, dma0_ipd_req_26: {=bool:?}, dma0_ipd_req_27: {=bool:?}, dma0_ipd_req_28: {=bool:?}, dma0_ipd_req_29: {=bool:?}, dma0_ipd_req_30: {=bool:?}, dma0_ipd_req_31: {=bool:?} }}",
            self.dma0_ipd_req_0(),
            self.dma0_ipd_req_1(),
            self.dma0_ipd_req_2(),
            self.dma0_ipd_req_3(),
            self.dma0_ipd_req_4(),
            self.dma0_ipd_req_5(),
            self.dma0_ipd_req_6(),
            self.dma0_ipd_req_7(),
            self.dma0_ipd_req_8(),
            self.dma0_ipd_req_9(),
            self.dma0_ipd_req_10(),
            self.dma0_ipd_req_11(),
            self.dma0_ipd_req_12(),
            self.dma0_ipd_req_13(),
            self.dma0_ipd_req_14(),
            self.dma0_ipd_req_15(),
            self.dma0_ipd_req_16(),
            self.dma0_ipd_req_17(),
            self.dma0_ipd_req_18(),
            self.dma0_ipd_req_19(),
            self.dma0_ipd_req_20(),
            self.dma0_ipd_req_21(),
            self.dma0_ipd_req_22(),
            self.dma0_ipd_req_23(),
            self.dma0_ipd_req_24(),
            self.dma0_ipd_req_25(),
            self.dma0_ipd_req_26(),
            self.dma0_ipd_req_27(),
            self.dma0_ipd_req_28(),
            self.dma0_ipd_req_29(),
            self.dma0_ipd_req_30(),
            self.dma0_ipd_req_31()
        )
    }
}
#[doc = "Secure general purpose registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpReg2(pub u32);
impl SecGpReg2 {
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpReg2 {
    #[inline(always)]
    fn default() -> SecGpReg2 {
        SecGpReg2(0)
    }
}
impl core::fmt::Debug for SecGpReg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpReg2")
            .field("dma0_ipd_req_0", &self.dma0_ipd_req_0())
            .field("dma0_ipd_req_1", &self.dma0_ipd_req_1())
            .field("dma0_ipd_req_2", &self.dma0_ipd_req_2())
            .field("dma0_ipd_req_3", &self.dma0_ipd_req_3())
            .field("dma0_ipd_req_4", &self.dma0_ipd_req_4())
            .field("dma0_ipd_req_5", &self.dma0_ipd_req_5())
            .field("dma0_ipd_req_6", &self.dma0_ipd_req_6())
            .field("dma0_ipd_req_7", &self.dma0_ipd_req_7())
            .field("dma0_ipd_req_8", &self.dma0_ipd_req_8())
            .field("dma0_ipd_req_9", &self.dma0_ipd_req_9())
            .field("dma0_ipd_req_10", &self.dma0_ipd_req_10())
            .field("dma0_ipd_req_11", &self.dma0_ipd_req_11())
            .field("dma0_ipd_req_12", &self.dma0_ipd_req_12())
            .field("dma0_ipd_req_13", &self.dma0_ipd_req_13())
            .field("dma0_ipd_req_14", &self.dma0_ipd_req_14())
            .field("dma0_ipd_req_15", &self.dma0_ipd_req_15())
            .field("dma0_ipd_req_16", &self.dma0_ipd_req_16())
            .field("dma0_ipd_req_17", &self.dma0_ipd_req_17())
            .field("dma0_ipd_req_18", &self.dma0_ipd_req_18())
            .field("dma0_ipd_req_19", &self.dma0_ipd_req_19())
            .field("dma0_ipd_req_20", &self.dma0_ipd_req_20())
            .field("dma0_ipd_req_21", &self.dma0_ipd_req_21())
            .field("dma0_ipd_req_22", &self.dma0_ipd_req_22())
            .field("dma0_ipd_req_23", &self.dma0_ipd_req_23())
            .field("dma0_ipd_req_24", &self.dma0_ipd_req_24())
            .field("dma0_ipd_req_25", &self.dma0_ipd_req_25())
            .field("dma0_ipd_req_26", &self.dma0_ipd_req_26())
            .field("dma0_ipd_req_27", &self.dma0_ipd_req_27())
            .field("dma0_ipd_req_28", &self.dma0_ipd_req_28())
            .field("dma0_ipd_req_29", &self.dma0_ipd_req_29())
            .field("dma0_ipd_req_30", &self.dma0_ipd_req_30())
            .field("dma0_ipd_req_31", &self.dma0_ipd_req_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpReg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpReg2 {{ dma0_ipd_req_0: {=bool:?}, dma0_ipd_req_1: {=bool:?}, dma0_ipd_req_2: {=bool:?}, dma0_ipd_req_3: {=bool:?}, dma0_ipd_req_4: {=bool:?}, dma0_ipd_req_5: {=bool:?}, dma0_ipd_req_6: {=bool:?}, dma0_ipd_req_7: {=bool:?}, dma0_ipd_req_8: {=bool:?}, dma0_ipd_req_9: {=bool:?}, dma0_ipd_req_10: {=bool:?}, dma0_ipd_req_11: {=bool:?}, dma0_ipd_req_12: {=bool:?}, dma0_ipd_req_13: {=bool:?}, dma0_ipd_req_14: {=bool:?}, dma0_ipd_req_15: {=bool:?}, dma0_ipd_req_16: {=bool:?}, dma0_ipd_req_17: {=bool:?}, dma0_ipd_req_18: {=bool:?}, dma0_ipd_req_19: {=bool:?}, dma0_ipd_req_20: {=bool:?}, dma0_ipd_req_21: {=bool:?}, dma0_ipd_req_22: {=bool:?}, dma0_ipd_req_23: {=bool:?}, dma0_ipd_req_24: {=bool:?}, dma0_ipd_req_25: {=bool:?}, dma0_ipd_req_26: {=bool:?}, dma0_ipd_req_27: {=bool:?}, dma0_ipd_req_28: {=bool:?}, dma0_ipd_req_29: {=bool:?}, dma0_ipd_req_30: {=bool:?}, dma0_ipd_req_31: {=bool:?} }}",
            self.dma0_ipd_req_0(),
            self.dma0_ipd_req_1(),
            self.dma0_ipd_req_2(),
            self.dma0_ipd_req_3(),
            self.dma0_ipd_req_4(),
            self.dma0_ipd_req_5(),
            self.dma0_ipd_req_6(),
            self.dma0_ipd_req_7(),
            self.dma0_ipd_req_8(),
            self.dma0_ipd_req_9(),
            self.dma0_ipd_req_10(),
            self.dma0_ipd_req_11(),
            self.dma0_ipd_req_12(),
            self.dma0_ipd_req_13(),
            self.dma0_ipd_req_14(),
            self.dma0_ipd_req_15(),
            self.dma0_ipd_req_16(),
            self.dma0_ipd_req_17(),
            self.dma0_ipd_req_18(),
            self.dma0_ipd_req_19(),
            self.dma0_ipd_req_20(),
            self.dma0_ipd_req_21(),
            self.dma0_ipd_req_22(),
            self.dma0_ipd_req_23(),
            self.dma0_ipd_req_24(),
            self.dma0_ipd_req_25(),
            self.dma0_ipd_req_26(),
            self.dma0_ipd_req_27(),
            self.dma0_ipd_req_28(),
            self.dma0_ipd_req_29(),
            self.dma0_ipd_req_30(),
            self.dma0_ipd_req_31()
        )
    }
}
#[doc = "Secure general purpose registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpReg3(pub u32);
impl SecGpReg3 {
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpReg3 {
    #[inline(always)]
    fn default() -> SecGpReg3 {
        SecGpReg3(0)
    }
}
impl core::fmt::Debug for SecGpReg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpReg3")
            .field("dma0_ipd_req_0", &self.dma0_ipd_req_0())
            .field("dma0_ipd_req_1", &self.dma0_ipd_req_1())
            .field("dma0_ipd_req_2", &self.dma0_ipd_req_2())
            .field("dma0_ipd_req_3", &self.dma0_ipd_req_3())
            .field("dma0_ipd_req_4", &self.dma0_ipd_req_4())
            .field("dma0_ipd_req_5", &self.dma0_ipd_req_5())
            .field("dma0_ipd_req_6", &self.dma0_ipd_req_6())
            .field("dma0_ipd_req_7", &self.dma0_ipd_req_7())
            .field("dma0_ipd_req_8", &self.dma0_ipd_req_8())
            .field("dma0_ipd_req_9", &self.dma0_ipd_req_9())
            .field("dma0_ipd_req_10", &self.dma0_ipd_req_10())
            .field("dma0_ipd_req_11", &self.dma0_ipd_req_11())
            .field("dma0_ipd_req_12", &self.dma0_ipd_req_12())
            .field("dma0_ipd_req_13", &self.dma0_ipd_req_13())
            .field("dma0_ipd_req_14", &self.dma0_ipd_req_14())
            .field("dma0_ipd_req_15", &self.dma0_ipd_req_15())
            .field("dma0_ipd_req_16", &self.dma0_ipd_req_16())
            .field("dma0_ipd_req_17", &self.dma0_ipd_req_17())
            .field("dma0_ipd_req_18", &self.dma0_ipd_req_18())
            .field("dma0_ipd_req_19", &self.dma0_ipd_req_19())
            .field("dma0_ipd_req_20", &self.dma0_ipd_req_20())
            .field("dma0_ipd_req_21", &self.dma0_ipd_req_21())
            .field("dma0_ipd_req_22", &self.dma0_ipd_req_22())
            .field("dma0_ipd_req_23", &self.dma0_ipd_req_23())
            .field("dma0_ipd_req_24", &self.dma0_ipd_req_24())
            .field("dma0_ipd_req_25", &self.dma0_ipd_req_25())
            .field("dma0_ipd_req_26", &self.dma0_ipd_req_26())
            .field("dma0_ipd_req_27", &self.dma0_ipd_req_27())
            .field("dma0_ipd_req_28", &self.dma0_ipd_req_28())
            .field("dma0_ipd_req_29", &self.dma0_ipd_req_29())
            .field("dma0_ipd_req_30", &self.dma0_ipd_req_30())
            .field("dma0_ipd_req_31", &self.dma0_ipd_req_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpReg3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpReg3 {{ dma0_ipd_req_0: {=bool:?}, dma0_ipd_req_1: {=bool:?}, dma0_ipd_req_2: {=bool:?}, dma0_ipd_req_3: {=bool:?}, dma0_ipd_req_4: {=bool:?}, dma0_ipd_req_5: {=bool:?}, dma0_ipd_req_6: {=bool:?}, dma0_ipd_req_7: {=bool:?}, dma0_ipd_req_8: {=bool:?}, dma0_ipd_req_9: {=bool:?}, dma0_ipd_req_10: {=bool:?}, dma0_ipd_req_11: {=bool:?}, dma0_ipd_req_12: {=bool:?}, dma0_ipd_req_13: {=bool:?}, dma0_ipd_req_14: {=bool:?}, dma0_ipd_req_15: {=bool:?}, dma0_ipd_req_16: {=bool:?}, dma0_ipd_req_17: {=bool:?}, dma0_ipd_req_18: {=bool:?}, dma0_ipd_req_19: {=bool:?}, dma0_ipd_req_20: {=bool:?}, dma0_ipd_req_21: {=bool:?}, dma0_ipd_req_22: {=bool:?}, dma0_ipd_req_23: {=bool:?}, dma0_ipd_req_24: {=bool:?}, dma0_ipd_req_25: {=bool:?}, dma0_ipd_req_26: {=bool:?}, dma0_ipd_req_27: {=bool:?}, dma0_ipd_req_28: {=bool:?}, dma0_ipd_req_29: {=bool:?}, dma0_ipd_req_30: {=bool:?}, dma0_ipd_req_31: {=bool:?} }}",
            self.dma0_ipd_req_0(),
            self.dma0_ipd_req_1(),
            self.dma0_ipd_req_2(),
            self.dma0_ipd_req_3(),
            self.dma0_ipd_req_4(),
            self.dma0_ipd_req_5(),
            self.dma0_ipd_req_6(),
            self.dma0_ipd_req_7(),
            self.dma0_ipd_req_8(),
            self.dma0_ipd_req_9(),
            self.dma0_ipd_req_10(),
            self.dma0_ipd_req_11(),
            self.dma0_ipd_req_12(),
            self.dma0_ipd_req_13(),
            self.dma0_ipd_req_14(),
            self.dma0_ipd_req_15(),
            self.dma0_ipd_req_16(),
            self.dma0_ipd_req_17(),
            self.dma0_ipd_req_18(),
            self.dma0_ipd_req_19(),
            self.dma0_ipd_req_20(),
            self.dma0_ipd_req_21(),
            self.dma0_ipd_req_22(),
            self.dma0_ipd_req_23(),
            self.dma0_ipd_req_24(),
            self.dma0_ipd_req_25(),
            self.dma0_ipd_req_26(),
            self.dma0_ipd_req_27(),
            self.dma0_ipd_req_28(),
            self.dma0_ipd_req_29(),
            self.dma0_ipd_req_30(),
            self.dma0_ipd_req_31()
        )
    }
}
#[doc = "Secure general purpose registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpReg4(pub u32);
impl SecGpReg4 {
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA0 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpReg4 {
    #[inline(always)]
    fn default() -> SecGpReg4 {
        SecGpReg4(0)
    }
}
impl core::fmt::Debug for SecGpReg4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpReg4")
            .field("dma0_ipd_req_0", &self.dma0_ipd_req_0())
            .field("dma0_ipd_req_1", &self.dma0_ipd_req_1())
            .field("dma0_ipd_req_2", &self.dma0_ipd_req_2())
            .field("dma0_ipd_req_3", &self.dma0_ipd_req_3())
            .field("dma0_ipd_req_4", &self.dma0_ipd_req_4())
            .field("dma0_ipd_req_5", &self.dma0_ipd_req_5())
            .field("dma0_ipd_req_6", &self.dma0_ipd_req_6())
            .field("dma0_ipd_req_7", &self.dma0_ipd_req_7())
            .field("dma0_ipd_req_8", &self.dma0_ipd_req_8())
            .field("dma0_ipd_req_9", &self.dma0_ipd_req_9())
            .field("dma0_ipd_req_10", &self.dma0_ipd_req_10())
            .field("dma0_ipd_req_11", &self.dma0_ipd_req_11())
            .field("dma0_ipd_req_12", &self.dma0_ipd_req_12())
            .field("dma0_ipd_req_13", &self.dma0_ipd_req_13())
            .field("dma0_ipd_req_14", &self.dma0_ipd_req_14())
            .field("dma0_ipd_req_15", &self.dma0_ipd_req_15())
            .field("dma0_ipd_req_16", &self.dma0_ipd_req_16())
            .field("dma0_ipd_req_17", &self.dma0_ipd_req_17())
            .field("dma0_ipd_req_18", &self.dma0_ipd_req_18())
            .field("dma0_ipd_req_19", &self.dma0_ipd_req_19())
            .field("dma0_ipd_req_20", &self.dma0_ipd_req_20())
            .field("dma0_ipd_req_21", &self.dma0_ipd_req_21())
            .field("dma0_ipd_req_22", &self.dma0_ipd_req_22())
            .field("dma0_ipd_req_23", &self.dma0_ipd_req_23())
            .field("dma0_ipd_req_24", &self.dma0_ipd_req_24())
            .field("dma0_ipd_req_25", &self.dma0_ipd_req_25())
            .field("dma0_ipd_req_26", &self.dma0_ipd_req_26())
            .field("dma0_ipd_req_27", &self.dma0_ipd_req_27())
            .field("dma0_ipd_req_28", &self.dma0_ipd_req_28())
            .field("dma0_ipd_req_29", &self.dma0_ipd_req_29())
            .field("dma0_ipd_req_30", &self.dma0_ipd_req_30())
            .field("dma0_ipd_req_31", &self.dma0_ipd_req_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpReg4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpReg4 {{ dma0_ipd_req_0: {=bool:?}, dma0_ipd_req_1: {=bool:?}, dma0_ipd_req_2: {=bool:?}, dma0_ipd_req_3: {=bool:?}, dma0_ipd_req_4: {=bool:?}, dma0_ipd_req_5: {=bool:?}, dma0_ipd_req_6: {=bool:?}, dma0_ipd_req_7: {=bool:?}, dma0_ipd_req_8: {=bool:?}, dma0_ipd_req_9: {=bool:?}, dma0_ipd_req_10: {=bool:?}, dma0_ipd_req_11: {=bool:?}, dma0_ipd_req_12: {=bool:?}, dma0_ipd_req_13: {=bool:?}, dma0_ipd_req_14: {=bool:?}, dma0_ipd_req_15: {=bool:?}, dma0_ipd_req_16: {=bool:?}, dma0_ipd_req_17: {=bool:?}, dma0_ipd_req_18: {=bool:?}, dma0_ipd_req_19: {=bool:?}, dma0_ipd_req_20: {=bool:?}, dma0_ipd_req_21: {=bool:?}, dma0_ipd_req_22: {=bool:?}, dma0_ipd_req_23: {=bool:?}, dma0_ipd_req_24: {=bool:?}, dma0_ipd_req_25: {=bool:?}, dma0_ipd_req_26: {=bool:?}, dma0_ipd_req_27: {=bool:?}, dma0_ipd_req_28: {=bool:?}, dma0_ipd_req_29: {=bool:?}, dma0_ipd_req_30: {=bool:?}, dma0_ipd_req_31: {=bool:?} }}",
            self.dma0_ipd_req_0(),
            self.dma0_ipd_req_1(),
            self.dma0_ipd_req_2(),
            self.dma0_ipd_req_3(),
            self.dma0_ipd_req_4(),
            self.dma0_ipd_req_5(),
            self.dma0_ipd_req_6(),
            self.dma0_ipd_req_7(),
            self.dma0_ipd_req_8(),
            self.dma0_ipd_req_9(),
            self.dma0_ipd_req_10(),
            self.dma0_ipd_req_11(),
            self.dma0_ipd_req_12(),
            self.dma0_ipd_req_13(),
            self.dma0_ipd_req_14(),
            self.dma0_ipd_req_15(),
            self.dma0_ipd_req_16(),
            self.dma0_ipd_req_17(),
            self.dma0_ipd_req_18(),
            self.dma0_ipd_req_19(),
            self.dma0_ipd_req_20(),
            self.dma0_ipd_req_21(),
            self.dma0_ipd_req_22(),
            self.dma0_ipd_req_23(),
            self.dma0_ipd_req_24(),
            self.dma0_ipd_req_25(),
            self.dma0_ipd_req_26(),
            self.dma0_ipd_req_27(),
            self.dma0_ipd_req_28(),
            self.dma0_ipd_req_29(),
            self.dma0_ipd_req_30(),
            self.dma0_ipd_req_31()
        )
    }
}
#[doc = "Secure general purpose registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpReg5(pub u32);
impl SecGpReg5 {
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpReg5 {
    #[inline(always)]
    fn default() -> SecGpReg5 {
        SecGpReg5(0)
    }
}
impl core::fmt::Debug for SecGpReg5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpReg5")
            .field("dma1_ipd_req_0", &self.dma1_ipd_req_0())
            .field("dma1_ipd_req_1", &self.dma1_ipd_req_1())
            .field("dma1_ipd_req_2", &self.dma1_ipd_req_2())
            .field("dma1_ipd_req_3", &self.dma1_ipd_req_3())
            .field("dma1_ipd_req_4", &self.dma1_ipd_req_4())
            .field("dma1_ipd_req_5", &self.dma1_ipd_req_5())
            .field("dma1_ipd_req_6", &self.dma1_ipd_req_6())
            .field("dma1_ipd_req_7", &self.dma1_ipd_req_7())
            .field("dma1_ipd_req_8", &self.dma1_ipd_req_8())
            .field("dma1_ipd_req_9", &self.dma1_ipd_req_9())
            .field("dma1_ipd_req_10", &self.dma1_ipd_req_10())
            .field("dma1_ipd_req_11", &self.dma1_ipd_req_11())
            .field("dma1_ipd_req_12", &self.dma1_ipd_req_12())
            .field("dma1_ipd_req_13", &self.dma1_ipd_req_13())
            .field("dma1_ipd_req_14", &self.dma1_ipd_req_14())
            .field("dma1_ipd_req_15", &self.dma1_ipd_req_15())
            .field("dma1_ipd_req_16", &self.dma1_ipd_req_16())
            .field("dma1_ipd_req_17", &self.dma1_ipd_req_17())
            .field("dma1_ipd_req_18", &self.dma1_ipd_req_18())
            .field("dma1_ipd_req_19", &self.dma1_ipd_req_19())
            .field("dma1_ipd_req_20", &self.dma1_ipd_req_20())
            .field("dma1_ipd_req_21", &self.dma1_ipd_req_21())
            .field("dma1_ipd_req_22", &self.dma1_ipd_req_22())
            .field("dma1_ipd_req_23", &self.dma1_ipd_req_23())
            .field("dma1_ipd_req_24", &self.dma1_ipd_req_24())
            .field("dma1_ipd_req_25", &self.dma1_ipd_req_25())
            .field("dma1_ipd_req_26", &self.dma1_ipd_req_26())
            .field("dma1_ipd_req_27", &self.dma1_ipd_req_27())
            .field("dma1_ipd_req_28", &self.dma1_ipd_req_28())
            .field("dma1_ipd_req_29", &self.dma1_ipd_req_29())
            .field("dma1_ipd_req_30", &self.dma1_ipd_req_30())
            .field("dma1_ipd_req_31", &self.dma1_ipd_req_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpReg5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpReg5 {{ dma1_ipd_req_0: {=bool:?}, dma1_ipd_req_1: {=bool:?}, dma1_ipd_req_2: {=bool:?}, dma1_ipd_req_3: {=bool:?}, dma1_ipd_req_4: {=bool:?}, dma1_ipd_req_5: {=bool:?}, dma1_ipd_req_6: {=bool:?}, dma1_ipd_req_7: {=bool:?}, dma1_ipd_req_8: {=bool:?}, dma1_ipd_req_9: {=bool:?}, dma1_ipd_req_10: {=bool:?}, dma1_ipd_req_11: {=bool:?}, dma1_ipd_req_12: {=bool:?}, dma1_ipd_req_13: {=bool:?}, dma1_ipd_req_14: {=bool:?}, dma1_ipd_req_15: {=bool:?}, dma1_ipd_req_16: {=bool:?}, dma1_ipd_req_17: {=bool:?}, dma1_ipd_req_18: {=bool:?}, dma1_ipd_req_19: {=bool:?}, dma1_ipd_req_20: {=bool:?}, dma1_ipd_req_21: {=bool:?}, dma1_ipd_req_22: {=bool:?}, dma1_ipd_req_23: {=bool:?}, dma1_ipd_req_24: {=bool:?}, dma1_ipd_req_25: {=bool:?}, dma1_ipd_req_26: {=bool:?}, dma1_ipd_req_27: {=bool:?}, dma1_ipd_req_28: {=bool:?}, dma1_ipd_req_29: {=bool:?}, dma1_ipd_req_30: {=bool:?}, dma1_ipd_req_31: {=bool:?} }}",
            self.dma1_ipd_req_0(),
            self.dma1_ipd_req_1(),
            self.dma1_ipd_req_2(),
            self.dma1_ipd_req_3(),
            self.dma1_ipd_req_4(),
            self.dma1_ipd_req_5(),
            self.dma1_ipd_req_6(),
            self.dma1_ipd_req_7(),
            self.dma1_ipd_req_8(),
            self.dma1_ipd_req_9(),
            self.dma1_ipd_req_10(),
            self.dma1_ipd_req_11(),
            self.dma1_ipd_req_12(),
            self.dma1_ipd_req_13(),
            self.dma1_ipd_req_14(),
            self.dma1_ipd_req_15(),
            self.dma1_ipd_req_16(),
            self.dma1_ipd_req_17(),
            self.dma1_ipd_req_18(),
            self.dma1_ipd_req_19(),
            self.dma1_ipd_req_20(),
            self.dma1_ipd_req_21(),
            self.dma1_ipd_req_22(),
            self.dma1_ipd_req_23(),
            self.dma1_ipd_req_24(),
            self.dma1_ipd_req_25(),
            self.dma1_ipd_req_26(),
            self.dma1_ipd_req_27(),
            self.dma1_ipd_req_28(),
            self.dma1_ipd_req_29(),
            self.dma1_ipd_req_30(),
            self.dma1_ipd_req_31()
        )
    }
}
#[doc = "Secure general purpose registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpReg6(pub u32);
impl SecGpReg6 {
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpReg6 {
    #[inline(always)]
    fn default() -> SecGpReg6 {
        SecGpReg6(0)
    }
}
impl core::fmt::Debug for SecGpReg6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpReg6")
            .field("dma1_ipd_req_0", &self.dma1_ipd_req_0())
            .field("dma1_ipd_req_1", &self.dma1_ipd_req_1())
            .field("dma1_ipd_req_2", &self.dma1_ipd_req_2())
            .field("dma1_ipd_req_3", &self.dma1_ipd_req_3())
            .field("dma1_ipd_req_4", &self.dma1_ipd_req_4())
            .field("dma1_ipd_req_5", &self.dma1_ipd_req_5())
            .field("dma1_ipd_req_6", &self.dma1_ipd_req_6())
            .field("dma1_ipd_req_7", &self.dma1_ipd_req_7())
            .field("dma1_ipd_req_8", &self.dma1_ipd_req_8())
            .field("dma1_ipd_req_9", &self.dma1_ipd_req_9())
            .field("dma1_ipd_req_10", &self.dma1_ipd_req_10())
            .field("dma1_ipd_req_11", &self.dma1_ipd_req_11())
            .field("dma1_ipd_req_12", &self.dma1_ipd_req_12())
            .field("dma1_ipd_req_13", &self.dma1_ipd_req_13())
            .field("dma1_ipd_req_14", &self.dma1_ipd_req_14())
            .field("dma1_ipd_req_15", &self.dma1_ipd_req_15())
            .field("dma1_ipd_req_16", &self.dma1_ipd_req_16())
            .field("dma1_ipd_req_17", &self.dma1_ipd_req_17())
            .field("dma1_ipd_req_18", &self.dma1_ipd_req_18())
            .field("dma1_ipd_req_19", &self.dma1_ipd_req_19())
            .field("dma1_ipd_req_20", &self.dma1_ipd_req_20())
            .field("dma1_ipd_req_21", &self.dma1_ipd_req_21())
            .field("dma1_ipd_req_22", &self.dma1_ipd_req_22())
            .field("dma1_ipd_req_23", &self.dma1_ipd_req_23())
            .field("dma1_ipd_req_24", &self.dma1_ipd_req_24())
            .field("dma1_ipd_req_25", &self.dma1_ipd_req_25())
            .field("dma1_ipd_req_26", &self.dma1_ipd_req_26())
            .field("dma1_ipd_req_27", &self.dma1_ipd_req_27())
            .field("dma1_ipd_req_28", &self.dma1_ipd_req_28())
            .field("dma1_ipd_req_29", &self.dma1_ipd_req_29())
            .field("dma1_ipd_req_30", &self.dma1_ipd_req_30())
            .field("dma1_ipd_req_31", &self.dma1_ipd_req_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpReg6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpReg6 {{ dma1_ipd_req_0: {=bool:?}, dma1_ipd_req_1: {=bool:?}, dma1_ipd_req_2: {=bool:?}, dma1_ipd_req_3: {=bool:?}, dma1_ipd_req_4: {=bool:?}, dma1_ipd_req_5: {=bool:?}, dma1_ipd_req_6: {=bool:?}, dma1_ipd_req_7: {=bool:?}, dma1_ipd_req_8: {=bool:?}, dma1_ipd_req_9: {=bool:?}, dma1_ipd_req_10: {=bool:?}, dma1_ipd_req_11: {=bool:?}, dma1_ipd_req_12: {=bool:?}, dma1_ipd_req_13: {=bool:?}, dma1_ipd_req_14: {=bool:?}, dma1_ipd_req_15: {=bool:?}, dma1_ipd_req_16: {=bool:?}, dma1_ipd_req_17: {=bool:?}, dma1_ipd_req_18: {=bool:?}, dma1_ipd_req_19: {=bool:?}, dma1_ipd_req_20: {=bool:?}, dma1_ipd_req_21: {=bool:?}, dma1_ipd_req_22: {=bool:?}, dma1_ipd_req_23: {=bool:?}, dma1_ipd_req_24: {=bool:?}, dma1_ipd_req_25: {=bool:?}, dma1_ipd_req_26: {=bool:?}, dma1_ipd_req_27: {=bool:?}, dma1_ipd_req_28: {=bool:?}, dma1_ipd_req_29: {=bool:?}, dma1_ipd_req_30: {=bool:?}, dma1_ipd_req_31: {=bool:?} }}",
            self.dma1_ipd_req_0(),
            self.dma1_ipd_req_1(),
            self.dma1_ipd_req_2(),
            self.dma1_ipd_req_3(),
            self.dma1_ipd_req_4(),
            self.dma1_ipd_req_5(),
            self.dma1_ipd_req_6(),
            self.dma1_ipd_req_7(),
            self.dma1_ipd_req_8(),
            self.dma1_ipd_req_9(),
            self.dma1_ipd_req_10(),
            self.dma1_ipd_req_11(),
            self.dma1_ipd_req_12(),
            self.dma1_ipd_req_13(),
            self.dma1_ipd_req_14(),
            self.dma1_ipd_req_15(),
            self.dma1_ipd_req_16(),
            self.dma1_ipd_req_17(),
            self.dma1_ipd_req_18(),
            self.dma1_ipd_req_19(),
            self.dma1_ipd_req_20(),
            self.dma1_ipd_req_21(),
            self.dma1_ipd_req_22(),
            self.dma1_ipd_req_23(),
            self.dma1_ipd_req_24(),
            self.dma1_ipd_req_25(),
            self.dma1_ipd_req_26(),
            self.dma1_ipd_req_27(),
            self.dma1_ipd_req_28(),
            self.dma1_ipd_req_29(),
            self.dma1_ipd_req_30(),
            self.dma1_ipd_req_31()
        )
    }
}
#[doc = "Secure general purpose registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpReg7(pub u32);
impl SecGpReg7 {
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpReg7 {
    #[inline(always)]
    fn default() -> SecGpReg7 {
        SecGpReg7(0)
    }
}
impl core::fmt::Debug for SecGpReg7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpReg7")
            .field("dma1_ipd_req_0", &self.dma1_ipd_req_0())
            .field("dma1_ipd_req_1", &self.dma1_ipd_req_1())
            .field("dma1_ipd_req_2", &self.dma1_ipd_req_2())
            .field("dma1_ipd_req_3", &self.dma1_ipd_req_3())
            .field("dma1_ipd_req_4", &self.dma1_ipd_req_4())
            .field("dma1_ipd_req_5", &self.dma1_ipd_req_5())
            .field("dma1_ipd_req_6", &self.dma1_ipd_req_6())
            .field("dma1_ipd_req_7", &self.dma1_ipd_req_7())
            .field("dma1_ipd_req_8", &self.dma1_ipd_req_8())
            .field("dma1_ipd_req_9", &self.dma1_ipd_req_9())
            .field("dma1_ipd_req_10", &self.dma1_ipd_req_10())
            .field("dma1_ipd_req_11", &self.dma1_ipd_req_11())
            .field("dma1_ipd_req_12", &self.dma1_ipd_req_12())
            .field("dma1_ipd_req_13", &self.dma1_ipd_req_13())
            .field("dma1_ipd_req_14", &self.dma1_ipd_req_14())
            .field("dma1_ipd_req_15", &self.dma1_ipd_req_15())
            .field("dma1_ipd_req_16", &self.dma1_ipd_req_16())
            .field("dma1_ipd_req_17", &self.dma1_ipd_req_17())
            .field("dma1_ipd_req_18", &self.dma1_ipd_req_18())
            .field("dma1_ipd_req_19", &self.dma1_ipd_req_19())
            .field("dma1_ipd_req_20", &self.dma1_ipd_req_20())
            .field("dma1_ipd_req_21", &self.dma1_ipd_req_21())
            .field("dma1_ipd_req_22", &self.dma1_ipd_req_22())
            .field("dma1_ipd_req_23", &self.dma1_ipd_req_23())
            .field("dma1_ipd_req_24", &self.dma1_ipd_req_24())
            .field("dma1_ipd_req_25", &self.dma1_ipd_req_25())
            .field("dma1_ipd_req_26", &self.dma1_ipd_req_26())
            .field("dma1_ipd_req_27", &self.dma1_ipd_req_27())
            .field("dma1_ipd_req_28", &self.dma1_ipd_req_28())
            .field("dma1_ipd_req_29", &self.dma1_ipd_req_29())
            .field("dma1_ipd_req_30", &self.dma1_ipd_req_30())
            .field("dma1_ipd_req_31", &self.dma1_ipd_req_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpReg7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpReg7 {{ dma1_ipd_req_0: {=bool:?}, dma1_ipd_req_1: {=bool:?}, dma1_ipd_req_2: {=bool:?}, dma1_ipd_req_3: {=bool:?}, dma1_ipd_req_4: {=bool:?}, dma1_ipd_req_5: {=bool:?}, dma1_ipd_req_6: {=bool:?}, dma1_ipd_req_7: {=bool:?}, dma1_ipd_req_8: {=bool:?}, dma1_ipd_req_9: {=bool:?}, dma1_ipd_req_10: {=bool:?}, dma1_ipd_req_11: {=bool:?}, dma1_ipd_req_12: {=bool:?}, dma1_ipd_req_13: {=bool:?}, dma1_ipd_req_14: {=bool:?}, dma1_ipd_req_15: {=bool:?}, dma1_ipd_req_16: {=bool:?}, dma1_ipd_req_17: {=bool:?}, dma1_ipd_req_18: {=bool:?}, dma1_ipd_req_19: {=bool:?}, dma1_ipd_req_20: {=bool:?}, dma1_ipd_req_21: {=bool:?}, dma1_ipd_req_22: {=bool:?}, dma1_ipd_req_23: {=bool:?}, dma1_ipd_req_24: {=bool:?}, dma1_ipd_req_25: {=bool:?}, dma1_ipd_req_26: {=bool:?}, dma1_ipd_req_27: {=bool:?}, dma1_ipd_req_28: {=bool:?}, dma1_ipd_req_29: {=bool:?}, dma1_ipd_req_30: {=bool:?}, dma1_ipd_req_31: {=bool:?} }}",
            self.dma1_ipd_req_0(),
            self.dma1_ipd_req_1(),
            self.dma1_ipd_req_2(),
            self.dma1_ipd_req_3(),
            self.dma1_ipd_req_4(),
            self.dma1_ipd_req_5(),
            self.dma1_ipd_req_6(),
            self.dma1_ipd_req_7(),
            self.dma1_ipd_req_8(),
            self.dma1_ipd_req_9(),
            self.dma1_ipd_req_10(),
            self.dma1_ipd_req_11(),
            self.dma1_ipd_req_12(),
            self.dma1_ipd_req_13(),
            self.dma1_ipd_req_14(),
            self.dma1_ipd_req_15(),
            self.dma1_ipd_req_16(),
            self.dma1_ipd_req_17(),
            self.dma1_ipd_req_18(),
            self.dma1_ipd_req_19(),
            self.dma1_ipd_req_20(),
            self.dma1_ipd_req_21(),
            self.dma1_ipd_req_22(),
            self.dma1_ipd_req_23(),
            self.dma1_ipd_req_24(),
            self.dma1_ipd_req_25(),
            self.dma1_ipd_req_26(),
            self.dma1_ipd_req_27(),
            self.dma1_ipd_req_28(),
            self.dma1_ipd_req_29(),
            self.dma1_ipd_req_30(),
            self.dma1_ipd_req_31()
        )
    }
}
#[doc = "Secure general purpose registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpReg8(pub u32);
impl SecGpReg8 {
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpReg8 {
    #[inline(always)]
    fn default() -> SecGpReg8 {
        SecGpReg8(0)
    }
}
impl core::fmt::Debug for SecGpReg8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpReg8")
            .field("dma1_ipd_req_0", &self.dma1_ipd_req_0())
            .field("dma1_ipd_req_1", &self.dma1_ipd_req_1())
            .field("dma1_ipd_req_2", &self.dma1_ipd_req_2())
            .field("dma1_ipd_req_3", &self.dma1_ipd_req_3())
            .field("dma1_ipd_req_4", &self.dma1_ipd_req_4())
            .field("dma1_ipd_req_5", &self.dma1_ipd_req_5())
            .field("dma1_ipd_req_6", &self.dma1_ipd_req_6())
            .field("dma1_ipd_req_7", &self.dma1_ipd_req_7())
            .field("dma1_ipd_req_8", &self.dma1_ipd_req_8())
            .field("dma1_ipd_req_9", &self.dma1_ipd_req_9())
            .field("dma1_ipd_req_10", &self.dma1_ipd_req_10())
            .field("dma1_ipd_req_11", &self.dma1_ipd_req_11())
            .field("dma1_ipd_req_12", &self.dma1_ipd_req_12())
            .field("dma1_ipd_req_13", &self.dma1_ipd_req_13())
            .field("dma1_ipd_req_14", &self.dma1_ipd_req_14())
            .field("dma1_ipd_req_15", &self.dma1_ipd_req_15())
            .field("dma1_ipd_req_16", &self.dma1_ipd_req_16())
            .field("dma1_ipd_req_17", &self.dma1_ipd_req_17())
            .field("dma1_ipd_req_18", &self.dma1_ipd_req_18())
            .field("dma1_ipd_req_19", &self.dma1_ipd_req_19())
            .field("dma1_ipd_req_20", &self.dma1_ipd_req_20())
            .field("dma1_ipd_req_21", &self.dma1_ipd_req_21())
            .field("dma1_ipd_req_22", &self.dma1_ipd_req_22())
            .field("dma1_ipd_req_23", &self.dma1_ipd_req_23())
            .field("dma1_ipd_req_24", &self.dma1_ipd_req_24())
            .field("dma1_ipd_req_25", &self.dma1_ipd_req_25())
            .field("dma1_ipd_req_26", &self.dma1_ipd_req_26())
            .field("dma1_ipd_req_27", &self.dma1_ipd_req_27())
            .field("dma1_ipd_req_28", &self.dma1_ipd_req_28())
            .field("dma1_ipd_req_29", &self.dma1_ipd_req_29())
            .field("dma1_ipd_req_30", &self.dma1_ipd_req_30())
            .field("dma1_ipd_req_31", &self.dma1_ipd_req_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpReg8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpReg8 {{ dma1_ipd_req_0: {=bool:?}, dma1_ipd_req_1: {=bool:?}, dma1_ipd_req_2: {=bool:?}, dma1_ipd_req_3: {=bool:?}, dma1_ipd_req_4: {=bool:?}, dma1_ipd_req_5: {=bool:?}, dma1_ipd_req_6: {=bool:?}, dma1_ipd_req_7: {=bool:?}, dma1_ipd_req_8: {=bool:?}, dma1_ipd_req_9: {=bool:?}, dma1_ipd_req_10: {=bool:?}, dma1_ipd_req_11: {=bool:?}, dma1_ipd_req_12: {=bool:?}, dma1_ipd_req_13: {=bool:?}, dma1_ipd_req_14: {=bool:?}, dma1_ipd_req_15: {=bool:?}, dma1_ipd_req_16: {=bool:?}, dma1_ipd_req_17: {=bool:?}, dma1_ipd_req_18: {=bool:?}, dma1_ipd_req_19: {=bool:?}, dma1_ipd_req_20: {=bool:?}, dma1_ipd_req_21: {=bool:?}, dma1_ipd_req_22: {=bool:?}, dma1_ipd_req_23: {=bool:?}, dma1_ipd_req_24: {=bool:?}, dma1_ipd_req_25: {=bool:?}, dma1_ipd_req_26: {=bool:?}, dma1_ipd_req_27: {=bool:?}, dma1_ipd_req_28: {=bool:?}, dma1_ipd_req_29: {=bool:?}, dma1_ipd_req_30: {=bool:?}, dma1_ipd_req_31: {=bool:?} }}",
            self.dma1_ipd_req_0(),
            self.dma1_ipd_req_1(),
            self.dma1_ipd_req_2(),
            self.dma1_ipd_req_3(),
            self.dma1_ipd_req_4(),
            self.dma1_ipd_req_5(),
            self.dma1_ipd_req_6(),
            self.dma1_ipd_req_7(),
            self.dma1_ipd_req_8(),
            self.dma1_ipd_req_9(),
            self.dma1_ipd_req_10(),
            self.dma1_ipd_req_11(),
            self.dma1_ipd_req_12(),
            self.dma1_ipd_req_13(),
            self.dma1_ipd_req_14(),
            self.dma1_ipd_req_15(),
            self.dma1_ipd_req_16(),
            self.dma1_ipd_req_17(),
            self.dma1_ipd_req_18(),
            self.dma1_ipd_req_19(),
            self.dma1_ipd_req_20(),
            self.dma1_ipd_req_21(),
            self.dma1_ipd_req_22(),
            self.dma1_ipd_req_23(),
            self.dma1_ipd_req_24(),
            self.dma1_ipd_req_25(),
            self.dma1_ipd_req_26(),
            self.dma1_ipd_req_27(),
            self.dma1_ipd_req_28(),
            self.dma1_ipd_req_29(),
            self.dma1_ipd_req_30(),
            self.dma1_ipd_req_31()
        )
    }
}
#[doc = "Secure general purpose registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpReg9(pub u32);
impl SecGpReg9 {
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA1 IPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ."]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpReg9 {
    #[inline(always)]
    fn default() -> SecGpReg9 {
        SecGpReg9(0)
    }
}
impl core::fmt::Debug for SecGpReg9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpReg9")
            .field("dma1_ipd_req_0", &self.dma1_ipd_req_0())
            .field("dma1_ipd_req_1", &self.dma1_ipd_req_1())
            .field("dma1_ipd_req_2", &self.dma1_ipd_req_2())
            .field("dma1_ipd_req_3", &self.dma1_ipd_req_3())
            .field("dma1_ipd_req_4", &self.dma1_ipd_req_4())
            .field("dma1_ipd_req_5", &self.dma1_ipd_req_5())
            .field("dma1_ipd_req_6", &self.dma1_ipd_req_6())
            .field("dma1_ipd_req_7", &self.dma1_ipd_req_7())
            .field("dma1_ipd_req_8", &self.dma1_ipd_req_8())
            .field("dma1_ipd_req_9", &self.dma1_ipd_req_9())
            .field("dma1_ipd_req_10", &self.dma1_ipd_req_10())
            .field("dma1_ipd_req_11", &self.dma1_ipd_req_11())
            .field("dma1_ipd_req_12", &self.dma1_ipd_req_12())
            .field("dma1_ipd_req_13", &self.dma1_ipd_req_13())
            .field("dma1_ipd_req_14", &self.dma1_ipd_req_14())
            .field("dma1_ipd_req_15", &self.dma1_ipd_req_15())
            .field("dma1_ipd_req_16", &self.dma1_ipd_req_16())
            .field("dma1_ipd_req_17", &self.dma1_ipd_req_17())
            .field("dma1_ipd_req_18", &self.dma1_ipd_req_18())
            .field("dma1_ipd_req_19", &self.dma1_ipd_req_19())
            .field("dma1_ipd_req_20", &self.dma1_ipd_req_20())
            .field("dma1_ipd_req_21", &self.dma1_ipd_req_21())
            .field("dma1_ipd_req_22", &self.dma1_ipd_req_22())
            .field("dma1_ipd_req_23", &self.dma1_ipd_req_23())
            .field("dma1_ipd_req_24", &self.dma1_ipd_req_24())
            .field("dma1_ipd_req_25", &self.dma1_ipd_req_25())
            .field("dma1_ipd_req_26", &self.dma1_ipd_req_26())
            .field("dma1_ipd_req_27", &self.dma1_ipd_req_27())
            .field("dma1_ipd_req_28", &self.dma1_ipd_req_28())
            .field("dma1_ipd_req_29", &self.dma1_ipd_req_29())
            .field("dma1_ipd_req_30", &self.dma1_ipd_req_30())
            .field("dma1_ipd_req_31", &self.dma1_ipd_req_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpReg9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpReg9 {{ dma1_ipd_req_0: {=bool:?}, dma1_ipd_req_1: {=bool:?}, dma1_ipd_req_2: {=bool:?}, dma1_ipd_req_3: {=bool:?}, dma1_ipd_req_4: {=bool:?}, dma1_ipd_req_5: {=bool:?}, dma1_ipd_req_6: {=bool:?}, dma1_ipd_req_7: {=bool:?}, dma1_ipd_req_8: {=bool:?}, dma1_ipd_req_9: {=bool:?}, dma1_ipd_req_10: {=bool:?}, dma1_ipd_req_11: {=bool:?}, dma1_ipd_req_12: {=bool:?}, dma1_ipd_req_13: {=bool:?}, dma1_ipd_req_14: {=bool:?}, dma1_ipd_req_15: {=bool:?}, dma1_ipd_req_16: {=bool:?}, dma1_ipd_req_17: {=bool:?}, dma1_ipd_req_18: {=bool:?}, dma1_ipd_req_19: {=bool:?}, dma1_ipd_req_20: {=bool:?}, dma1_ipd_req_21: {=bool:?}, dma1_ipd_req_22: {=bool:?}, dma1_ipd_req_23: {=bool:?}, dma1_ipd_req_24: {=bool:?}, dma1_ipd_req_25: {=bool:?}, dma1_ipd_req_26: {=bool:?}, dma1_ipd_req_27: {=bool:?}, dma1_ipd_req_28: {=bool:?}, dma1_ipd_req_29: {=bool:?}, dma1_ipd_req_30: {=bool:?}, dma1_ipd_req_31: {=bool:?} }}",
            self.dma1_ipd_req_0(),
            self.dma1_ipd_req_1(),
            self.dma1_ipd_req_2(),
            self.dma1_ipd_req_3(),
            self.dma1_ipd_req_4(),
            self.dma1_ipd_req_5(),
            self.dma1_ipd_req_6(),
            self.dma1_ipd_req_7(),
            self.dma1_ipd_req_8(),
            self.dma1_ipd_req_9(),
            self.dma1_ipd_req_10(),
            self.dma1_ipd_req_11(),
            self.dma1_ipd_req_12(),
            self.dma1_ipd_req_13(),
            self.dma1_ipd_req_14(),
            self.dma1_ipd_req_15(),
            self.dma1_ipd_req_16(),
            self.dma1_ipd_req_17(),
            self.dma1_ipd_req_18(),
            self.dma1_ipd_req_19(),
            self.dma1_ipd_req_20(),
            self.dma1_ipd_req_21(),
            self.dma1_ipd_req_22(),
            self.dma1_ipd_req_23(),
            self.dma1_ipd_req_24(),
            self.dma1_ipd_req_25(),
            self.dma1_ipd_req_26(),
            self.dma1_ipd_req_27(),
            self.dma1_ipd_req_28(),
            self.dma1_ipd_req_29(),
            self.dma1_ipd_req_30(),
            self.dma1_ipd_req_31()
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
    pub const fn vio_info_valid0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 0."]
    #[inline(always)]
    pub const fn set_vio_info_valid0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Violation information valid flag for AHB port 1."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 1."]
    #[inline(always)]
    pub const fn set_vio_info_valid1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Violation information valid flag for AHB port 2."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 2."]
    #[inline(always)]
    pub const fn set_vio_info_valid2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Violation information valid flag for AHB port 3."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 3."]
    #[inline(always)]
    pub const fn set_vio_info_valid3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Violation information valid flag for AHB port 4."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 4."]
    #[inline(always)]
    pub const fn set_vio_info_valid4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Violation information valid flag for AHB port 5."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 5."]
    #[inline(always)]
    pub const fn set_vio_info_valid5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Violation information valid flag for AHB port 6."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 6."]
    #[inline(always)]
    pub const fn set_vio_info_valid6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Violation information valid flag for AHB port 7."]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 7."]
    #[inline(always)]
    pub const fn set_vio_info_valid7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
            .field("vio_info_valid0", &self.vio_info_valid0())
            .field("vio_info_valid1", &self.vio_info_valid1())
            .field("vio_info_valid2", &self.vio_info_valid2())
            .field("vio_info_valid3", &self.vio_info_valid3())
            .field("vio_info_valid4", &self.vio_info_valid4())
            .field("vio_info_valid5", &self.vio_info_valid5())
            .field("vio_info_valid6", &self.vio_info_valid6())
            .field("vio_info_valid7", &self.vio_info_valid7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecVioInfoValid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecVioInfoValid {{ vio_info_valid0: {=bool:?}, vio_info_valid1: {=bool:?}, vio_info_valid2: {=bool:?}, vio_info_valid3: {=bool:?}, vio_info_valid4: {=bool:?}, vio_info_valid5: {=bool:?}, vio_info_valid6: {=bool:?}, vio_info_valid7: {=bool:?} }}",
            self.vio_info_valid0(),
            self.vio_info_valid1(),
            self.vio_info_valid2(),
            self.vio_info_valid3(),
            self.vio_info_valid4(),
            self.vio_info_valid5(),
            self.vio_info_valid6(),
            self.vio_info_valid7()
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
pub enum Adc0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Adc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0 {
    #[inline(always)]
    fn from(val: u8) -> Adc0 {
        Adc0::from_bits(val)
    }
}
impl From<Adc0> for u8 {
    #[inline(always)]
    fn from(val: Adc0) -> u8 {
        Adc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Adc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1 {
    #[inline(always)]
    fn from(val: u8) -> Adc1 {
        Adc1::from_bits(val)
    }
}
impl From<Adc1> for u8 {
    #[inline(always)]
    fn from(val: Adc1) -> u8 {
        Adc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSecureCtrlMemRule0Rule0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl AhbSecureCtrlMemRule0Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSecureCtrlMemRule0Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSecureCtrlMemRule0Rule0 {
    #[inline(always)]
    fn from(val: u8) -> AhbSecureCtrlMemRule0Rule0 {
        AhbSecureCtrlMemRule0Rule0::from_bits(val)
    }
}
impl From<AhbSecureCtrlMemRule0Rule0> for u8 {
    #[inline(always)]
    fn from(val: AhbSecureCtrlMemRule0Rule0) -> u8 {
        AhbSecureCtrlMemRule0Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSecureCtrlMemRule0Rule1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl AhbSecureCtrlMemRule0Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSecureCtrlMemRule0Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSecureCtrlMemRule0Rule1 {
    #[inline(always)]
    fn from(val: u8) -> AhbSecureCtrlMemRule0Rule1 {
        AhbSecureCtrlMemRule0Rule1::from_bits(val)
    }
}
impl From<AhbSecureCtrlMemRule0Rule1> for u8 {
    #[inline(always)]
    fn from(val: AhbSecureCtrlMemRule0Rule1) -> u8 {
        AhbSecureCtrlMemRule0Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSecureCtrlMemRule0Rule2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl AhbSecureCtrlMemRule0Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSecureCtrlMemRule0Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSecureCtrlMemRule0Rule2 {
    #[inline(always)]
    fn from(val: u8) -> AhbSecureCtrlMemRule0Rule2 {
        AhbSecureCtrlMemRule0Rule2::from_bits(val)
    }
}
impl From<AhbSecureCtrlMemRule0Rule2> for u8 {
    #[inline(always)]
    fn from(val: AhbSecureCtrlMemRule0Rule2) -> u8 {
        AhbSecureCtrlMemRule0Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSecureCtrlMemRule0Rule3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl AhbSecureCtrlMemRule0Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSecureCtrlMemRule0Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSecureCtrlMemRule0Rule3 {
    #[inline(always)]
    fn from(val: u8) -> AhbSecureCtrlMemRule0Rule3 {
        AhbSecureCtrlMemRule0Rule3::from_bits(val)
    }
}
impl From<AhbSecureCtrlMemRule0Rule3> for u8 {
    #[inline(always)]
    fn from(val: AhbSecureCtrlMemRule0Rule3) -> u8 {
        AhbSecureCtrlMemRule0Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AipsBridgeGroup1MemRule1Usb1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl AipsBridgeGroup1MemRule1Usb1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AipsBridgeGroup1MemRule1Usb1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AipsBridgeGroup1MemRule1Usb1 {
    #[inline(always)]
    fn from(val: u8) -> AipsBridgeGroup1MemRule1Usb1 {
        AipsBridgeGroup1MemRule1Usb1::from_bits(val)
    }
}
impl From<AipsBridgeGroup1MemRule1Usb1> for u8 {
    #[inline(always)]
    fn from(val: AipsBridgeGroup1MemRule1Usb1) -> u8 {
        AipsBridgeGroup1MemRule1Usb1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aoi0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Aoi0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aoi0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aoi0 {
    #[inline(always)]
    fn from(val: u8) -> Aoi0 {
        Aoi0::from_bits(val)
    }
}
impl From<Aoi0> for u8 {
    #[inline(always)]
    fn from(val: Aoi0) -> u8 {
        Aoi0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApbPeripheralGroup0MemRule1Smartdma {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl ApbPeripheralGroup0MemRule1Smartdma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApbPeripheralGroup0MemRule1Smartdma {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApbPeripheralGroup0MemRule1Smartdma {
    #[inline(always)]
    fn from(val: u8) -> ApbPeripheralGroup0MemRule1Smartdma {
        ApbPeripheralGroup0MemRule1Smartdma::from_bits(val)
    }
}
impl From<ApbPeripheralGroup0MemRule1Smartdma> for u8 {
    #[inline(always)]
    fn from(val: ApbPeripheralGroup0MemRule1Smartdma) -> u8 {
        ApbPeripheralGroup0MemRule1Smartdma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can0Region0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Can0Region0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can0Region0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can0Region0 {
    #[inline(always)]
    fn from(val: u8) -> Can0Region0 {
        Can0Region0::from_bits(val)
    }
}
impl From<Can0Region0> for u8 {
    #[inline(always)]
    fn from(val: Can0Region0) -> u8 {
        Can0Region0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can0Region1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Can0Region1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can0Region1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can0Region1 {
    #[inline(always)]
    fn from(val: u8) -> Can0Region1 {
        Can0Region1::from_bits(val)
    }
}
impl From<Can0Region1> for u8 {
    #[inline(always)]
    fn from(val: Can0Region1) -> u8 {
        Can0Region1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can0Region2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Can0Region2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can0Region2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can0Region2 {
    #[inline(always)]
    fn from(val: u8) -> Can0Region2 {
        Can0Region2::from_bits(val)
    }
}
impl From<Can0Region2> for u8 {
    #[inline(always)]
    fn from(val: Can0Region2) -> u8 {
        Can0Region2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can0Region3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Can0Region3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can0Region3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can0Region3 {
    #[inline(always)]
    fn from(val: u8) -> Can0Region3 {
        Can0Region3::from_bits(val)
    }
}
impl From<Can0Region3> for u8 {
    #[inline(always)]
    fn from(val: Can0Region3) -> u8 {
        Can0Region3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can1Region0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Can1Region0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can1Region0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can1Region0 {
    #[inline(always)]
    fn from(val: u8) -> Can1Region0 {
        Can1Region0::from_bits(val)
    }
}
impl From<Can1Region0> for u8 {
    #[inline(always)]
    fn from(val: Can1Region0) -> u8 {
        Can1Region0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can1Region1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Can1Region1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can1Region1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can1Region1 {
    #[inline(always)]
    fn from(val: u8) -> Can1Region1 {
        Can1Region1::from_bits(val)
    }
}
impl From<Can1Region1> for u8 {
    #[inline(always)]
    fn from(val: Can1Region1) -> u8 {
        Can1Region1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can1Region2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Can1Region2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can1Region2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can1Region2 {
    #[inline(always)]
    fn from(val: u8) -> Can1Region2 {
        Can1Region2::from_bits(val)
    }
}
impl From<Can1Region2> for u8 {
    #[inline(always)]
    fn from(val: Can1Region2) -> u8 {
        Can1Region2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can1Region3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Can1Region3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can1Region3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can1Region3 {
    #[inline(always)]
    fn from(val: u8) -> Can1Region3 {
        Can1Region3::from_bits(val)
    }
}
impl From<Can1Region3> for u8 {
    #[inline(always)]
    fn from(val: Can1Region3) -> u8 {
        Can1Region3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cdog0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Cdog0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cdog0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cdog0 {
    #[inline(always)]
    fn from(val: u8) -> Cdog0 {
        Cdog0::from_bits(val)
    }
}
impl From<Cdog0> for u8 {
    #[inline(always)]
    fn from(val: Cdog0) -> u8 {
        Cdog0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cdog1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Cdog1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cdog1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cdog1 {
    #[inline(always)]
    fn from(val: u8) -> Cdog1 {
        Cdog1::from_bits(val)
    }
}
impl From<Cdog1> for u8 {
    #[inline(always)]
    fn from(val: Cdog1) -> u8 {
        Cdog1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmc {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Cmc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmc {
    #[inline(always)]
    fn from(val: u8) -> Cmc {
        Cmc::from_bits(val)
    }
}
impl From<Cmc> for u8 {
    #[inline(always)]
    fn from(val: Cmc) -> u8 {
        Cmc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Cmp0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0 {
    #[inline(always)]
    fn from(val: u8) -> Cmp0 {
        Cmp0::from_bits(val)
    }
}
impl From<Cmp0> for u8 {
    #[inline(always)]
    fn from(val: Cmp0) -> u8 {
        Cmp0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crc0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Crc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crc0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crc0 {
    #[inline(always)]
    fn from(val: u8) -> Crc0 {
        Crc0::from_bits(val)
    }
}
impl From<Crc0> for u8 {
    #[inline(always)]
    fn from(val: Crc0) -> u8 {
        Crc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ctimer0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer0 {
    #[inline(always)]
    fn from(val: u8) -> Ctimer0 {
        Ctimer0::from_bits(val)
    }
}
impl From<Ctimer0> for u8 {
    #[inline(always)]
    fn from(val: Ctimer0) -> u8 {
        Ctimer0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ctimer1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer1 {
    #[inline(always)]
    fn from(val: u8) -> Ctimer1 {
        Ctimer1::from_bits(val)
    }
}
impl From<Ctimer1> for u8 {
    #[inline(always)]
    fn from(val: Ctimer1) -> u8 {
        Ctimer1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ctimer2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer2 {
    #[inline(always)]
    fn from(val: u8) -> Ctimer2 {
        Ctimer2::from_bits(val)
    }
}
impl From<Ctimer2> for u8 {
    #[inline(always)]
    fn from(val: Ctimer2) -> u8 {
        Ctimer2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ctimer3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer3 {
    #[inline(always)]
    fn from(val: u8) -> Ctimer3 {
        Ctimer3::from_bits(val)
    }
}
impl From<Ctimer3> for u8 {
    #[inline(always)]
    fn from(val: Ctimer3) -> u8 {
        Ctimer3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ctimer4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer4 {
    #[inline(always)]
    fn from(val: u8) -> Ctimer4 {
        Ctimer4::from_bits(val)
    }
}
impl From<Ctimer4> for u8 {
    #[inline(always)]
    fn from(val: Ctimer4) -> u8 {
        Ctimer4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dac0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac0 {
    #[inline(always)]
    fn from(val: u8) -> Dac0 {
        Dac0::from_bits(val)
    }
}
impl From<Dac0> for u8 {
    #[inline(always)]
    fn from(val: Dac0) -> u8 {
        Dac0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dac1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac1 {
    #[inline(always)]
    fn from(val: u8) -> Dac1 {
        Dac1::from_bits(val)
    }
}
impl From<Dac1> for u8 {
    #[inline(always)]
    fn from(val: Dac1) -> u8 {
        Dac1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugMailbox {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl DebugMailbox {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugMailbox {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugMailbox {
    #[inline(always)]
    fn from(val: u8) -> DebugMailbox {
        DebugMailbox::from_bits(val)
    }
}
impl From<DebugMailbox> for u8 {
    #[inline(always)]
    fn from(val: DebugMailbox) -> u8 {
        DebugMailbox::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dgdet0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dgdet0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dgdet0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dgdet0 {
    #[inline(always)]
    fn from(val: u8) -> Dgdet0 {
        Dgdet0::from_bits(val)
    }
}
impl From<Dgdet0> for u8 {
    #[inline(always)]
    fn from(val: Dgdet0) -> u8 {
        Dgdet0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Ch0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Ch0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Ch0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Ch0 {
    #[inline(always)]
    fn from(val: u8) -> Dma0Ch0 {
        Dma0Ch0::from_bits(val)
    }
}
impl From<Dma0Ch0> for u8 {
    #[inline(always)]
    fn from(val: Dma0Ch0) -> u8 {
        Dma0Ch0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Ch1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Ch1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Ch1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Ch1 {
    #[inline(always)]
    fn from(val: u8) -> Dma0Ch1 {
        Dma0Ch1::from_bits(val)
    }
}
impl From<Dma0Ch1> for u8 {
    #[inline(always)]
    fn from(val: Dma0Ch1) -> u8 {
        Dma0Ch1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Ch10 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Ch10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Ch10 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Ch10 {
    #[inline(always)]
    fn from(val: u8) -> Dma0Ch10 {
        Dma0Ch10::from_bits(val)
    }
}
impl From<Dma0Ch10> for u8 {
    #[inline(always)]
    fn from(val: Dma0Ch10) -> u8 {
        Dma0Ch10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Ch11 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Ch11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Ch11 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Ch11 {
    #[inline(always)]
    fn from(val: u8) -> Dma0Ch11 {
        Dma0Ch11::from_bits(val)
    }
}
impl From<Dma0Ch11> for u8 {
    #[inline(always)]
    fn from(val: Dma0Ch11) -> u8 {
        Dma0Ch11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Ch2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Ch2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Ch2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Ch2 {
    #[inline(always)]
    fn from(val: u8) -> Dma0Ch2 {
        Dma0Ch2::from_bits(val)
    }
}
impl From<Dma0Ch2> for u8 {
    #[inline(always)]
    fn from(val: Dma0Ch2) -> u8 {
        Dma0Ch2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Ch3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Ch3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Ch3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Ch3 {
    #[inline(always)]
    fn from(val: u8) -> Dma0Ch3 {
        Dma0Ch3::from_bits(val)
    }
}
impl From<Dma0Ch3> for u8 {
    #[inline(always)]
    fn from(val: Dma0Ch3) -> u8 {
        Dma0Ch3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Ch4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Ch4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Ch4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Ch4 {
    #[inline(always)]
    fn from(val: u8) -> Dma0Ch4 {
        Dma0Ch4::from_bits(val)
    }
}
impl From<Dma0Ch4> for u8 {
    #[inline(always)]
    fn from(val: Dma0Ch4) -> u8 {
        Dma0Ch4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Ch5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Ch5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Ch5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Ch5 {
    #[inline(always)]
    fn from(val: u8) -> Dma0Ch5 {
        Dma0Ch5::from_bits(val)
    }
}
impl From<Dma0Ch5> for u8 {
    #[inline(always)]
    fn from(val: Dma0Ch5) -> u8 {
        Dma0Ch5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Ch6 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Ch6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Ch6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Ch6 {
    #[inline(always)]
    fn from(val: u8) -> Dma0Ch6 {
        Dma0Ch6::from_bits(val)
    }
}
impl From<Dma0Ch6> for u8 {
    #[inline(always)]
    fn from(val: Dma0Ch6) -> u8 {
        Dma0Ch6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Ch7 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Ch7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Ch7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Ch7 {
    #[inline(always)]
    fn from(val: u8) -> Dma0Ch7 {
        Dma0Ch7::from_bits(val)
    }
}
impl From<Dma0Ch7> for u8 {
    #[inline(always)]
    fn from(val: Dma0Ch7) -> u8 {
        Dma0Ch7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Ch8 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Ch8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Ch8 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Ch8 {
    #[inline(always)]
    fn from(val: u8) -> Dma0Ch8 {
        Dma0Ch8::from_bits(val)
    }
}
impl From<Dma0Ch8> for u8 {
    #[inline(always)]
    fn from(val: Dma0Ch8) -> u8 {
        Dma0Ch8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Ch9 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Ch9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Ch9 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Ch9 {
    #[inline(always)]
    fn from(val: u8) -> Dma0Ch9 {
        Dma0Ch9::from_bits(val)
    }
}
impl From<Dma0Ch9> for u8 {
    #[inline(always)]
    fn from(val: Dma0Ch9) -> u8 {
        Dma0Ch9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Mp {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Mp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Mp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Mp {
    #[inline(always)]
    fn from(val: u8) -> Dma0Mp {
        Dma0Mp::from_bits(val)
    }
}
impl From<Dma0Mp> for u8 {
    #[inline(always)]
    fn from(val: Dma0Mp) -> u8 {
        Dma0Mp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma1Ch0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma1Ch0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1Ch0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1Ch0 {
    #[inline(always)]
    fn from(val: u8) -> Dma1Ch0 {
        Dma1Ch0::from_bits(val)
    }
}
impl From<Dma1Ch0> for u8 {
    #[inline(always)]
    fn from(val: Dma1Ch0) -> u8 {
        Dma1Ch0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma1Ch1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma1Ch1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1Ch1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1Ch1 {
    #[inline(always)]
    fn from(val: u8) -> Dma1Ch1 {
        Dma1Ch1::from_bits(val)
    }
}
impl From<Dma1Ch1> for u8 {
    #[inline(always)]
    fn from(val: Dma1Ch1) -> u8 {
        Dma1Ch1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma1Ch2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma1Ch2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1Ch2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1Ch2 {
    #[inline(always)]
    fn from(val: u8) -> Dma1Ch2 {
        Dma1Ch2::from_bits(val)
    }
}
impl From<Dma1Ch2> for u8 {
    #[inline(always)]
    fn from(val: Dma1Ch2) -> u8 {
        Dma1Ch2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma1Ch3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma1Ch3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1Ch3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1Ch3 {
    #[inline(always)]
    fn from(val: u8) -> Dma1Ch3 {
        Dma1Ch3::from_bits(val)
    }
}
impl From<Dma1Ch3> for u8 {
    #[inline(always)]
    fn from(val: Dma1Ch3) -> u8 {
        Dma1Ch3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma1Mp {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma1Mp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1Mp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1Mp {
    #[inline(always)]
    fn from(val: u8) -> Dma1Mp {
        Dma1Mp::from_bits(val)
    }
}
impl From<Dma1Mp> for u8 {
    #[inline(always)]
    fn from(val: Dma1Mp) -> u8 {
        Dma1Mp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ESpi {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl ESpi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ESpi {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ESpi {
    #[inline(always)]
    fn from(val: u8) -> ESpi {
        ESpi::from_bits(val)
    }
}
impl From<ESpi> for u8 {
    #[inline(always)]
    fn from(val: ESpi) -> u8 {
        ESpi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eim {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Eim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eim {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eim {
    #[inline(always)]
    fn from(val: u8) -> Eim {
        Eim::from_bits(val)
    }
}
impl From<Eim> for u8 {
    #[inline(always)]
    fn from(val: Eim) -> u8 {
        Eim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet00 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Enet00 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet00 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet00 {
    #[inline(always)]
    fn from(val: u8) -> Enet00 {
        Enet00::from_bits(val)
    }
}
impl From<Enet00> for u8 {
    #[inline(always)]
    fn from(val: Enet00) -> u8 {
        Enet00::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet01 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Enet01 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet01 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet01 {
    #[inline(always)]
    fn from(val: u8) -> Enet01 {
        Enet01::from_bits(val)
    }
}
impl From<Enet01> for u8 {
    #[inline(always)]
    fn from(val: Enet01) -> u8 {
        Enet01::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Erm {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Erm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Erm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Erm {
    #[inline(always)]
    fn from(val: u8) -> Erm {
        Erm::from_bits(val)
    }
}
impl From<Erm> for u8 {
    #[inline(always)]
    fn from(val: Erm) -> u8 {
        Erm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ewm0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ewm0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ewm0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ewm0 {
    #[inline(always)]
    fn from(val: u8) -> Ewm0 {
        Ewm0::from_bits(val)
    }
}
impl From<Ewm0> for u8 {
    #[inline(always)]
    fn from(val: Ewm0) -> u8 {
        Ewm0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash00MemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule0 {
        Flash00MemRuleRule0::from_bits(val)
    }
}
impl From<Flash00MemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule0) -> u8 {
        Flash00MemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash00MemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule1 {
        Flash00MemRuleRule1::from_bits(val)
    }
}
impl From<Flash00MemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule1) -> u8 {
        Flash00MemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash00MemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule2 {
        Flash00MemRuleRule2::from_bits(val)
    }
}
impl From<Flash00MemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule2) -> u8 {
        Flash00MemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash00MemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule3 {
        Flash00MemRuleRule3::from_bits(val)
    }
}
impl From<Flash00MemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule3) -> u8 {
        Flash00MemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash00MemRuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule4 {
        Flash00MemRuleRule4::from_bits(val)
    }
}
impl From<Flash00MemRuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule4) -> u8 {
        Flash00MemRuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash00MemRuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule5 {
        Flash00MemRuleRule5::from_bits(val)
    }
}
impl From<Flash00MemRuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule5) -> u8 {
        Flash00MemRuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash00MemRuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule6 {
        Flash00MemRuleRule6::from_bits(val)
    }
}
impl From<Flash00MemRuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule6) -> u8 {
        Flash00MemRuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash00MemRuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule7 {
        Flash00MemRuleRule7::from_bits(val)
    }
}
impl From<Flash00MemRuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule7) -> u8 {
        Flash00MemRuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash01MemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash01MemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash01MemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash01MemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Flash01MemRuleRule0 {
        Flash01MemRuleRule0::from_bits(val)
    }
}
impl From<Flash01MemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Flash01MemRuleRule0) -> u8 {
        Flash01MemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash01MemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash01MemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash01MemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash01MemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Flash01MemRuleRule1 {
        Flash01MemRuleRule1::from_bits(val)
    }
}
impl From<Flash01MemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Flash01MemRuleRule1) -> u8 {
        Flash01MemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash01MemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash01MemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash01MemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash01MemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Flash01MemRuleRule2 {
        Flash01MemRuleRule2::from_bits(val)
    }
}
impl From<Flash01MemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Flash01MemRuleRule2) -> u8 {
        Flash01MemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash01MemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash01MemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash01MemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash01MemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Flash01MemRuleRule3 {
        Flash01MemRuleRule3::from_bits(val)
    }
}
impl From<Flash01MemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Flash01MemRuleRule3) -> u8 {
        Flash01MemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash01MemRuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash01MemRuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash01MemRuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash01MemRuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Flash01MemRuleRule4 {
        Flash01MemRuleRule4::from_bits(val)
    }
}
impl From<Flash01MemRuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Flash01MemRuleRule4) -> u8 {
        Flash01MemRuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash01MemRuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash01MemRuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash01MemRuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash01MemRuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Flash01MemRuleRule5 {
        Flash01MemRuleRule5::from_bits(val)
    }
}
impl From<Flash01MemRuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Flash01MemRuleRule5) -> u8 {
        Flash01MemRuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash01MemRuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash01MemRuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash01MemRuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash01MemRuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Flash01MemRuleRule6 {
        Flash01MemRuleRule6::from_bits(val)
    }
}
impl From<Flash01MemRuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Flash01MemRuleRule6) -> u8 {
        Flash01MemRuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash01MemRuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash01MemRuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash01MemRuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash01MemRuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Flash01MemRuleRule7 {
        Flash01MemRuleRule7::from_bits(val)
    }
}
impl From<Flash01MemRuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Flash01MemRuleRule7) -> u8 {
        Flash01MemRuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash02MemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash02MemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash02MemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash02MemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Flash02MemRuleRule0 {
        Flash02MemRuleRule0::from_bits(val)
    }
}
impl From<Flash02MemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Flash02MemRuleRule0) -> u8 {
        Flash02MemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash02MemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash02MemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash02MemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash02MemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Flash02MemRuleRule1 {
        Flash02MemRuleRule1::from_bits(val)
    }
}
impl From<Flash02MemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Flash02MemRuleRule1) -> u8 {
        Flash02MemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash02MemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash02MemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash02MemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash02MemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Flash02MemRuleRule2 {
        Flash02MemRuleRule2::from_bits(val)
    }
}
impl From<Flash02MemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Flash02MemRuleRule2) -> u8 {
        Flash02MemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash02MemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash02MemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash02MemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash02MemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Flash02MemRuleRule3 {
        Flash02MemRuleRule3::from_bits(val)
    }
}
impl From<Flash02MemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Flash02MemRuleRule3) -> u8 {
        Flash02MemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash03MemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash03MemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash03MemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash03MemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Flash03MemRuleRule0 {
        Flash03MemRuleRule0::from_bits(val)
    }
}
impl From<Flash03MemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Flash03MemRuleRule0) -> u8 {
        Flash03MemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash03MemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash03MemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash03MemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash03MemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Flash03MemRuleRule1 {
        Flash03MemRuleRule1::from_bits(val)
    }
}
impl From<Flash03MemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Flash03MemRuleRule1) -> u8 {
        Flash03MemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash03MemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash03MemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash03MemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash03MemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Flash03MemRuleRule2 {
        Flash03MemRuleRule2::from_bits(val)
    }
}
impl From<Flash03MemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Flash03MemRuleRule2) -> u8 {
        Flash03MemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash03MemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash03MemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash03MemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash03MemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Flash03MemRuleRule3 {
        Flash03MemRuleRule3::from_bits(val)
    }
}
impl From<Flash03MemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Flash03MemRuleRule3) -> u8 {
        Flash03MemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexio {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio {
    #[inline(always)]
    fn from(val: u8) -> Flexio {
        Flexio::from_bits(val)
    }
}
impl From<Flexio> for u8 {
    #[inline(always)]
    fn from(val: Flexio) -> u8 {
        Flexio::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0 {
        Flexspi0::from_bits(val)
    }
}
impl From<Flexspi0> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0) -> u8 {
        Flexspi0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0MemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region0MemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0MemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0MemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0MemRuleRule0 {
        Flexspi0Region0MemRuleRule0::from_bits(val)
    }
}
impl From<Flexspi0Region0MemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0MemRuleRule0) -> u8 {
        Flexspi0Region0MemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0MemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region0MemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0MemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0MemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0MemRuleRule1 {
        Flexspi0Region0MemRuleRule1::from_bits(val)
    }
}
impl From<Flexspi0Region0MemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0MemRuleRule1) -> u8 {
        Flexspi0Region0MemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0MemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region0MemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0MemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0MemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0MemRuleRule2 {
        Flexspi0Region0MemRuleRule2::from_bits(val)
    }
}
impl From<Flexspi0Region0MemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0MemRuleRule2) -> u8 {
        Flexspi0Region0MemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0MemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region0MemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0MemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0MemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0MemRuleRule3 {
        Flexspi0Region0MemRuleRule3::from_bits(val)
    }
}
impl From<Flexspi0Region0MemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0MemRuleRule3) -> u8 {
        Flexspi0Region0MemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0MemRuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region0MemRuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0MemRuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0MemRuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0MemRuleRule4 {
        Flexspi0Region0MemRuleRule4::from_bits(val)
    }
}
impl From<Flexspi0Region0MemRuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0MemRuleRule4) -> u8 {
        Flexspi0Region0MemRuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0MemRuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region0MemRuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0MemRuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0MemRuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0MemRuleRule5 {
        Flexspi0Region0MemRuleRule5::from_bits(val)
    }
}
impl From<Flexspi0Region0MemRuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0MemRuleRule5) -> u8 {
        Flexspi0Region0MemRuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0MemRuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region0MemRuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0MemRuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0MemRuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0MemRuleRule6 {
        Flexspi0Region0MemRuleRule6::from_bits(val)
    }
}
impl From<Flexspi0Region0MemRuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0MemRuleRule6) -> u8 {
        Flexspi0Region0MemRuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0MemRuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region0MemRuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0MemRuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0MemRuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0MemRuleRule7 {
        Flexspi0Region0MemRuleRule7::from_bits(val)
    }
}
impl From<Flexspi0Region0MemRuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0MemRuleRule7) -> u8 {
        Flexspi0Region0MemRuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0RegionMemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0RegionMemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0RegionMemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0RegionMemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0RegionMemRuleRule0 {
        Flexspi0RegionMemRuleRule0::from_bits(val)
    }
}
impl From<Flexspi0RegionMemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0RegionMemRuleRule0) -> u8 {
        Flexspi0RegionMemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0RegionMemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0RegionMemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0RegionMemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0RegionMemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0RegionMemRuleRule1 {
        Flexspi0RegionMemRuleRule1::from_bits(val)
    }
}
impl From<Flexspi0RegionMemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0RegionMemRuleRule1) -> u8 {
        Flexspi0RegionMemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0RegionMemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0RegionMemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0RegionMemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0RegionMemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0RegionMemRuleRule2 {
        Flexspi0RegionMemRuleRule2::from_bits(val)
    }
}
impl From<Flexspi0RegionMemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0RegionMemRuleRule2) -> u8 {
        Flexspi0RegionMemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0RegionMemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0RegionMemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0RegionMemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0RegionMemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0RegionMemRuleRule3 {
        Flexspi0RegionMemRuleRule3::from_bits(val)
    }
}
impl From<Flexspi0RegionMemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0RegionMemRuleRule3) -> u8 {
        Flexspi0RegionMemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fmc {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Fmc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fmc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fmc {
    #[inline(always)]
    fn from(val: u8) -> Fmc {
        Fmc::from_bits(val)
    }
}
impl From<Fmc> for u8 {
    #[inline(always)]
    fn from(val: Fmc) -> u8 {
        Fmc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fmu {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Fmu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fmu {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fmu {
    #[inline(always)]
    fn from(val: u8) -> Fmu {
        Fmu::from_bits(val)
    }
}
impl From<Fmu> for u8 {
    #[inline(always)]
    fn from(val: Fmu) -> u8 {
        Fmu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Freqme {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Freqme {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Freqme {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Freqme {
    #[inline(always)]
    fn from(val: u8) -> Freqme {
        Freqme::from_bits(val)
    }
}
impl From<Freqme> for u8 {
    #[inline(always)]
    fn from(val: Freqme) -> u8 {
        Freqme::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Glikey0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Glikey0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Glikey0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Glikey0 {
    #[inline(always)]
    fn from(val: u8) -> Glikey0 {
        Glikey0::from_bits(val)
    }
}
impl From<Glikey0> for u8 {
    #[inline(always)]
    fn from(val: Glikey0) -> u8 {
        Glikey0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Gpio0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio0 {
    #[inline(always)]
    fn from(val: u8) -> Gpio0 {
        Gpio0::from_bits(val)
    }
}
impl From<Gpio0> for u8 {
    #[inline(always)]
    fn from(val: Gpio0) -> u8 {
        Gpio0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio0Alias {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Gpio0Alias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio0Alias {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio0Alias {
    #[inline(always)]
    fn from(val: u8) -> Gpio0Alias {
        Gpio0Alias::from_bits(val)
    }
}
impl From<Gpio0Alias> for u8 {
    #[inline(always)]
    fn from(val: Gpio0Alias) -> u8 {
        Gpio0Alias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Gpio1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio1 {
    #[inline(always)]
    fn from(val: u8) -> Gpio1 {
        Gpio1::from_bits(val)
    }
}
impl From<Gpio1> for u8 {
    #[inline(always)]
    fn from(val: Gpio1) -> u8 {
        Gpio1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio1Alias {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Gpio1Alias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio1Alias {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio1Alias {
    #[inline(always)]
    fn from(val: u8) -> Gpio1Alias {
        Gpio1Alias::from_bits(val)
    }
}
impl From<Gpio1Alias> for u8 {
    #[inline(always)]
    fn from(val: Gpio1Alias) -> u8 {
        Gpio1Alias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Gpio2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio2 {
    #[inline(always)]
    fn from(val: u8) -> Gpio2 {
        Gpio2::from_bits(val)
    }
}
impl From<Gpio2> for u8 {
    #[inline(always)]
    fn from(val: Gpio2) -> u8 {
        Gpio2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio2Alias {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Gpio2Alias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio2Alias {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio2Alias {
    #[inline(always)]
    fn from(val: u8) -> Gpio2Alias {
        Gpio2Alias::from_bits(val)
    }
}
impl From<Gpio2Alias> for u8 {
    #[inline(always)]
    fn from(val: Gpio2Alias) -> u8 {
        Gpio2Alias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Gpio3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio3 {
    #[inline(always)]
    fn from(val: u8) -> Gpio3 {
        Gpio3::from_bits(val)
    }
}
impl From<Gpio3> for u8 {
    #[inline(always)]
    fn from(val: Gpio3) -> u8 {
        Gpio3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio3Alias {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Gpio3Alias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio3Alias {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio3Alias {
    #[inline(always)]
    fn from(val: u8) -> Gpio3Alias {
        Gpio3Alias::from_bits(val)
    }
}
impl From<Gpio3Alias> for u8 {
    #[inline(always)]
    fn from(val: Gpio3Alias) -> u8 {
        Gpio3Alias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Gpio4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio4 {
    #[inline(always)]
    fn from(val: u8) -> Gpio4 {
        Gpio4::from_bits(val)
    }
}
impl From<Gpio4> for u8 {
    #[inline(always)]
    fn from(val: Gpio4) -> u8 {
        Gpio4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio4Alias {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Gpio4Alias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio4Alias {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio4Alias {
    #[inline(always)]
    fn from(val: u8) -> Gpio4Alias {
        Gpio4Alias::from_bits(val)
    }
}
impl From<Gpio4Alias> for u8 {
    #[inline(always)]
    fn from(val: Gpio4Alias) -> u8 {
        Gpio4Alias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Gpio5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio5 {
    #[inline(always)]
    fn from(val: u8) -> Gpio5 {
        Gpio5::from_bits(val)
    }
}
impl From<Gpio5> for u8 {
    #[inline(always)]
    fn from(val: Gpio5) -> u8 {
        Gpio5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio5Alias {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Gpio5Alias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio5Alias {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio5Alias {
    #[inline(always)]
    fn from(val: u8) -> Gpio5Alias {
        Gpio5Alias::from_bits(val)
    }
}
impl From<Gpio5Alias> for u8 {
    #[inline(always)]
    fn from(val: Gpio5Alias) -> u8 {
        Gpio5Alias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl I3c0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0 {
    #[inline(always)]
    fn from(val: u8) -> I3c0 {
        I3c0::from_bits(val)
    }
}
impl From<I3c0> for u8 {
    #[inline(always)]
    fn from(val: I3c0) -> u8 {
        I3c0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl I3c1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1 {
    #[inline(always)]
    fn from(val: u8) -> I3c1 {
        I3c1::from_bits(val)
    }
}
impl From<I3c1> for u8 {
    #[inline(always)]
    fn from(val: I3c1) -> u8 {
        I3c1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl I3c2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c2 {
    #[inline(always)]
    fn from(val: u8) -> I3c2 {
        I3c2::from_bits(val)
    }
}
impl From<I3c2> for u8 {
    #[inline(always)]
    fn from(val: I3c2) -> u8 {
        I3c2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl I3c3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c3 {
    #[inline(always)]
    fn from(val: u8) -> I3c3 {
        I3c3::from_bits(val)
    }
}
impl From<I3c3> for u8 {
    #[inline(always)]
    fn from(val: I3c3) -> u8 {
        I3c3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inputmux {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Inputmux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inputmux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inputmux {
    #[inline(always)]
    fn from(val: u8) -> Inputmux {
        Inputmux::from_bits(val)
    }
}
impl From<Inputmux> for u8 {
    #[inline(always)]
    fn from(val: Inputmux) -> u8 {
        Inputmux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Itrc0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Itrc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Itrc0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Itrc0 {
    #[inline(always)]
    fn from(val: u8) -> Itrc0 {
        Itrc0::from_bits(val)
    }
}
impl From<Itrc0> for u8 {
    #[inline(always)]
    fn from(val: Itrc0) -> u8 {
        Itrc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockNsMpu {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCK_NS_MPU is 1."]
    LOCK_NS_MPU_EQ_1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_NS_MPU is 0."]
    LOCK_NS_MPU_EQ_0 = 0x02,
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
    LOCK_NS_VTOR_EQ_1 = 0x01,
    #[doc = "CM33 (CPU0) LOCKNSVTOR is 0."]
    LOCK_NS_VTOR_EQ_0 = 0x02,
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
    LOCK_S_MPU_EQ_1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_S_MPU is 0."]
    LOCK_S_MPU_EQ_0 = 0x02,
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
    LOCK_S_VTAIRCR_EQ_1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_S_VTAIRCR is 0."]
    LOCK_S_VTAIRCR_EQ_0 = 0x02,
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
    LOCK_SAU_EQ_1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_SAU is 0."]
    LOCK_SAU_EQ_0 = 0x02,
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
pub enum Lpi2c0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpi2c0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c0 {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c0 {
        Lpi2c0::from_bits(val)
    }
}
impl From<Lpi2c0> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c0) -> u8 {
        Lpi2c0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpi2c1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c1 {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c1 {
        Lpi2c1::from_bits(val)
    }
}
impl From<Lpi2c1> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c1) -> u8 {
        Lpi2c1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpi2c2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c2 {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c2 {
        Lpi2c2::from_bits(val)
    }
}
impl From<Lpi2c2> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c2) -> u8 {
        Lpi2c2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpi2c3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c3 {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c3 {
        Lpi2c3::from_bits(val)
    }
}
impl From<Lpi2c3> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c3) -> u8 {
        Lpi2c3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpi2c4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c4 {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c4 {
        Lpi2c4::from_bits(val)
    }
}
impl From<Lpi2c4> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c4) -> u8 {
        Lpi2c4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpspi0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi0 {
    #[inline(always)]
    fn from(val: u8) -> Lpspi0 {
        Lpspi0::from_bits(val)
    }
}
impl From<Lpspi0> for u8 {
    #[inline(always)]
    fn from(val: Lpspi0) -> u8 {
        Lpspi0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpspi1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi1 {
    #[inline(always)]
    fn from(val: u8) -> Lpspi1 {
        Lpspi1::from_bits(val)
    }
}
impl From<Lpspi1> for u8 {
    #[inline(always)]
    fn from(val: Lpspi1) -> u8 {
        Lpspi1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpspi2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi2 {
    #[inline(always)]
    fn from(val: u8) -> Lpspi2 {
        Lpspi2::from_bits(val)
    }
}
impl From<Lpspi2> for u8 {
    #[inline(always)]
    fn from(val: Lpspi2) -> u8 {
        Lpspi2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpspi3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi3 {
    #[inline(always)]
    fn from(val: u8) -> Lpspi3 {
        Lpspi3::from_bits(val)
    }
}
impl From<Lpspi3> for u8 {
    #[inline(always)]
    fn from(val: Lpspi3) -> u8 {
        Lpspi3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpspi4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi4 {
    #[inline(always)]
    fn from(val: u8) -> Lpspi4 {
        Lpspi4::from_bits(val)
    }
}
impl From<Lpspi4> for u8 {
    #[inline(always)]
    fn from(val: Lpspi4) -> u8 {
        Lpspi4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpspi5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi5 {
    #[inline(always)]
    fn from(val: u8) -> Lpspi5 {
        Lpspi5::from_bits(val)
    }
}
impl From<Lpspi5> for u8 {
    #[inline(always)]
    fn from(val: Lpspi5) -> u8 {
        Lpspi5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lptmr {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lptmr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lptmr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lptmr {
    #[inline(always)]
    fn from(val: u8) -> Lptmr {
        Lptmr::from_bits(val)
    }
}
impl From<Lptmr> for u8 {
    #[inline(always)]
    fn from(val: Lptmr) -> u8 {
        Lptmr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpuart0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart0 {
    #[inline(always)]
    fn from(val: u8) -> Lpuart0 {
        Lpuart0::from_bits(val)
    }
}
impl From<Lpuart0> for u8 {
    #[inline(always)]
    fn from(val: Lpuart0) -> u8 {
        Lpuart0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpuart1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart1 {
    #[inline(always)]
    fn from(val: u8) -> Lpuart1 {
        Lpuart1::from_bits(val)
    }
}
impl From<Lpuart1> for u8 {
    #[inline(always)]
    fn from(val: Lpuart1) -> u8 {
        Lpuart1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpuart2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart2 {
    #[inline(always)]
    fn from(val: u8) -> Lpuart2 {
        Lpuart2::from_bits(val)
    }
}
impl From<Lpuart2> for u8 {
    #[inline(always)]
    fn from(val: Lpuart2) -> u8 {
        Lpuart2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpuart3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart3 {
    #[inline(always)]
    fn from(val: u8) -> Lpuart3 {
        Lpuart3::from_bits(val)
    }
}
impl From<Lpuart3> for u8 {
    #[inline(always)]
    fn from(val: Lpuart3) -> u8 {
        Lpuart3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpuart4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart4 {
    #[inline(always)]
    fn from(val: u8) -> Lpuart4 {
        Lpuart4::from_bits(val)
    }
}
impl From<Lpuart4> for u8 {
    #[inline(always)]
    fn from(val: Lpuart4) -> u8 {
        Lpuart4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpuart5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart5 {
    #[inline(always)]
    fn from(val: u8) -> Lpuart5 {
        Lpuart5::from_bits(val)
    }
}
impl From<Lpuart5> for u8 {
    #[inline(always)]
    fn from(val: Lpuart5) -> u8 {
        Lpuart5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegDma0 {
    #[doc = "Non-secure and non-privileged Master."]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master."]
    SECURE_PRIV_MASTER = 0x03,
}
impl MasterSecAntiPolRegDma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegDma0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegDma0 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegDma0 {
        MasterSecAntiPolRegDma0::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegDma0> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegDma0) -> u8 {
        MasterSecAntiPolRegDma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegDma1 {
    #[doc = "Non-secure and non-privileged Master."]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master."]
    SECURE_PRIV_MASTER = 0x03,
}
impl MasterSecAntiPolRegDma1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegDma1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegDma1 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegDma1 {
        MasterSecAntiPolRegDma1::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegDma1> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegDma1) -> u8 {
        MasterSecAntiPolRegDma1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegEnet0 {
    #[doc = "Non-secure and non-privileged Master."]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master."]
    SECURE_PRIV_MASTER = 0x03,
}
impl MasterSecAntiPolRegEnet0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegEnet0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegEnet0 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegEnet0 {
        MasterSecAntiPolRegEnet0::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegEnet0> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegEnet0) -> u8 {
        MasterSecAntiPolRegEnet0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegPkc {
    #[doc = "Non-secure and non-privileged Master."]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master."]
    SECURE_PRIV_MASTER = 0x03,
}
impl MasterSecAntiPolRegPkc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegPkc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegPkc {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegPkc {
        MasterSecAntiPolRegPkc::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegPkc> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegPkc) -> u8 {
        MasterSecAntiPolRegPkc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegSmartdma {
    #[doc = "Non-secure and non-privileged Master."]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master."]
    SECURE_PRIV_MASTER = 0x03,
}
impl MasterSecAntiPolRegSmartdma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegSmartdma {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegSmartdma {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegSmartdma {
        MasterSecAntiPolRegSmartdma::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegSmartdma> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegSmartdma) -> u8 {
        MasterSecAntiPolRegSmartdma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegUsb1 {
    #[doc = "Non-secure and non-privileged Master."]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master."]
    SECURE_PRIV_MASTER = 0x03,
}
impl MasterSecAntiPolRegUsb1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegUsb1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegUsb1 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegUsb1 {
        MasterSecAntiPolRegUsb1::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegUsb1> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegUsb1) -> u8 {
        MasterSecAntiPolRegUsb1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelDma0 {
    #[doc = "Non-secure and non-privileged Master."]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master."]
    SECURE_PRIV_MASTER = 0x03,
}
impl MasterSecLevelDma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelDma0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelDma0 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelDma0 {
        MasterSecLevelDma0::from_bits(val)
    }
}
impl From<MasterSecLevelDma0> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelDma0) -> u8 {
        MasterSecLevelDma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelDma1 {
    #[doc = "Non-secure and non-privileged Master."]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master."]
    SECURE_PRIV_MASTER = 0x03,
}
impl MasterSecLevelDma1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelDma1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelDma1 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelDma1 {
        MasterSecLevelDma1::from_bits(val)
    }
}
impl From<MasterSecLevelDma1> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelDma1) -> u8 {
        MasterSecLevelDma1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelEnet0 {
    #[doc = "Non-secure and non-privileged Master."]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master."]
    SECURE_PRIV_MASTER = 0x03,
}
impl MasterSecLevelEnet0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelEnet0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelEnet0 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelEnet0 {
        MasterSecLevelEnet0::from_bits(val)
    }
}
impl From<MasterSecLevelEnet0> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelEnet0) -> u8 {
        MasterSecLevelEnet0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelPkc {
    #[doc = "Non-secure and non-privileged Master."]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master."]
    SECURE_PRIV_MASTER = 0x03,
}
impl MasterSecLevelPkc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelPkc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelPkc {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelPkc {
        MasterSecLevelPkc::from_bits(val)
    }
}
impl From<MasterSecLevelPkc> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelPkc) -> u8 {
        MasterSecLevelPkc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelSmartdma {
    #[doc = "Non-secure and non-privileged Master."]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master."]
    SECURE_PRIV_MASTER = 0x03,
}
impl MasterSecLevelSmartdma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelSmartdma {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelSmartdma {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelSmartdma {
        MasterSecLevelSmartdma::from_bits(val)
    }
}
impl From<MasterSecLevelSmartdma> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelSmartdma) -> u8 {
        MasterSecLevelSmartdma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelUsb1 {
    #[doc = "Non-secure and non-privileged Master."]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master."]
    SECURE_PRIV_MASTER = 0x03,
}
impl MasterSecLevelUsb1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelUsb1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelUsb1 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelUsb1 {
        MasterSecLevelUsb1::from_bits(val)
    }
}
impl From<MasterSecLevelUsb1> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelUsb1) -> u8 {
        MasterSecLevelUsb1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mau0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Mau0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mau0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mau0 {
    #[inline(always)]
    fn from(val: u8) -> Mau0 {
        Mau0::from_bits(val)
    }
}
impl From<Mau0> for u8 {
    #[inline(always)]
    fn from(val: Mau0) -> u8 {
        Mau0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Mbc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc {
    #[inline(always)]
    fn from(val: u8) -> Mbc {
        Mbc::from_bits(val)
    }
}
impl From<Mbc> for u8 {
    #[inline(always)]
    fn from(val: Mbc) -> u8 {
        Mbc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegDisableStrictMode {
    _RESERVED_0 = 0x0,
    #[doc = "Master can access memories and peripherals at the same level or below that level."]
    AHBTM = 0x01,
    #[doc = "Master can access memories and peripherals at same level only."]
    AHBSM1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegDisableStrictMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegDisableStrictMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegDisableStrictMode {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegDisableStrictMode {
        MiscCtrlDpRegDisableStrictMode::from_bits(val)
    }
}
impl From<MiscCtrlDpRegDisableStrictMode> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegDisableStrictMode) -> u8 {
        MiscCtrlDpRegDisableStrictMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegDisableViolationAbort {
    _RESERVED_0 = 0x0,
    #[doc = "The violation detected by the secure checker will not cause an abort, but a secure_violation_irq (interrupt request) will still be asserted and serviced by ISR."]
    NO_ABORT = 0x01,
    #[doc = "The violation detected by the secure checker will cause an abort."]
    ABORT = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegDisableViolationAbort {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegDisableViolationAbort {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegDisableViolationAbort {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegDisableViolationAbort {
        MiscCtrlDpRegDisableViolationAbort::from_bits(val)
    }
}
impl From<MiscCtrlDpRegDisableViolationAbort> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegDisableViolationAbort) -> u8 {
        MiscCtrlDpRegDisableViolationAbort::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegEnableNsPrivCheck {
    _RESERVED_0 = 0x0,
    #[doc = "Enables the privilege checking of non-secure mode access."]
    ENABLED = 0x01,
    #[doc = "Disables the privilege checking of non-secure mode access."]
    DISABLED = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegEnableNsPrivCheck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegEnableNsPrivCheck {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegEnableNsPrivCheck {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegEnableNsPrivCheck {
        MiscCtrlDpRegEnableNsPrivCheck::from_bits(val)
    }
}
impl From<MiscCtrlDpRegEnableNsPrivCheck> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegEnableNsPrivCheck) -> u8 {
        MiscCtrlDpRegEnableNsPrivCheck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegEnableSPrivCheck {
    _RESERVED_0 = 0x0,
    #[doc = "Enables the privilege checking of secure mode access."]
    ENABLED = 0x01,
    #[doc = "Disables the privilege checking of secure mode access."]
    DISABLED = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegEnableSPrivCheck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegEnableSPrivCheck {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegEnableSPrivCheck {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegEnableSPrivCheck {
        MiscCtrlDpRegEnableSPrivCheck::from_bits(val)
    }
}
impl From<MiscCtrlDpRegEnableSPrivCheck> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegEnableSPrivCheck) -> u8 {
        MiscCtrlDpRegEnableSPrivCheck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegEnableSecureChecking {
    _RESERVED_0 = 0x0,
    #[doc = "Enables secure checking. Violation can be detected when the security level of a transaction does not meet the security rule of the slave or memory to be accessed."]
    ENABLED = 0x01,
    #[doc = "Disables secure checking. Even if the security level of a transaction does not conform to the security rule of the slave or memory, it will not be detected as a violation."]
    DISABLED = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegEnableSecureChecking {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegEnableSecureChecking {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegEnableSecureChecking {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegEnableSecureChecking {
        MiscCtrlDpRegEnableSecureChecking::from_bits(val)
    }
}
impl From<MiscCtrlDpRegEnableSecureChecking> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegEnableSecureChecking) -> u8 {
        MiscCtrlDpRegEnableSecureChecking::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegIdauAllNs {
    _RESERVED_0 = 0x0,
    #[doc = "IDAU is disabled, which means that all memories are attributed as non-secure memory."]
    DISABLED = 0x01,
    #[doc = "IDAU is enabled (restrictive mode)."]
    ENABLED = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegIdauAllNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegIdauAllNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegIdauAllNs {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegIdauAllNs {
        MiscCtrlDpRegIdauAllNs::from_bits(val)
    }
}
impl From<MiscCtrlDpRegIdauAllNs> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegIdauAllNs) -> u8 {
        MiscCtrlDpRegIdauAllNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegWriteLock {
    _RESERVED_0 = 0x0,
    #[doc = "Writes to this register and to the Memory and Peripheral RULE registers are not allowed."]
    LOCKED = 0x01,
    #[doc = "Writes to this register and to the Memory and Peripheral RULE registers are allowed."]
    NOT_LOCKED = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegWriteLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegWriteLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegWriteLock {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegWriteLock {
        MiscCtrlDpRegWriteLock::from_bits(val)
    }
}
impl From<MiscCtrlDpRegWriteLock> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegWriteLock) -> u8 {
        MiscCtrlDpRegWriteLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegDisableStrictMode {
    _RESERVED_0 = 0x0,
    #[doc = "Master strict mode is disabled/off and can access memories and peripherals at the same level or below that level."]
    AHBTM = 0x01,
    #[doc = "Master strict mode is enabled/on and can access memories and peripherals at same level only."]
    AHBSM1 = 0x02,
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
    NO_ABORT = 0x01,
    #[doc = "The violation detected by the secure checker will cause an abort."]
    ABORT = 0x02,
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
pub enum MiscCtrlRegEnableNsPrivCheck {
    _RESERVED_0 = 0x0,
    #[doc = "Enables privilege checking of non-secure mode access."]
    ENABLED = 0x01,
    #[doc = "Disables privilege checking of non-secure mode access is disabled."]
    DISABLED = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegEnableNsPrivCheck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegEnableNsPrivCheck {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegEnableNsPrivCheck {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegEnableNsPrivCheck {
        MiscCtrlRegEnableNsPrivCheck::from_bits(val)
    }
}
impl From<MiscCtrlRegEnableNsPrivCheck> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegEnableNsPrivCheck) -> u8 {
        MiscCtrlRegEnableNsPrivCheck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegEnableSPrivCheck {
    _RESERVED_0 = 0x0,
    #[doc = "Enables privilege checking of secure mode access."]
    ENABLED = 0x01,
    #[doc = "Disables privilege checking of secure mode access."]
    DISABLED = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegEnableSPrivCheck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegEnableSPrivCheck {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegEnableSPrivCheck {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegEnableSPrivCheck {
        MiscCtrlRegEnableSPrivCheck::from_bits(val)
    }
}
impl From<MiscCtrlRegEnableSPrivCheck> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegEnableSPrivCheck) -> u8 {
        MiscCtrlRegEnableSPrivCheck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegEnableSecureChecking {
    _RESERVED_0 = 0x0,
    #[doc = "Enables secure checking. Violation can be detected when the security level of a transaction does not meet the security rule of the slave or memory to be accessed."]
    ENABLED = 0x01,
    #[doc = "Disables secure checking. Even if the security level of a transaction does not conform to the security rule of the slave or memory, it will not be detected as a violation."]
    DISABLED = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegEnableSecureChecking {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegEnableSecureChecking {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegEnableSecureChecking {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegEnableSecureChecking {
        MiscCtrlRegEnableSecureChecking::from_bits(val)
    }
}
impl From<MiscCtrlRegEnableSecureChecking> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegEnableSecureChecking) -> u8 {
        MiscCtrlRegEnableSecureChecking::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegIdauAllNs {
    _RESERVED_0 = 0x0,
    #[doc = "IDAU is disabled, which means that all memories are attributed as non-secure memory."]
    DISABLED = 0x01,
    #[doc = "IDAU is enabled (restrictive mode)."]
    ENABLED = 0x02,
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
    LOCKED = 0x01,
    #[doc = "Writes to this register and to the Memory and Peripheral RULE registers are allowed."]
    NOT_LOCKED = 0x02,
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
pub enum Ostimer {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ostimer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ostimer {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ostimer {
    #[inline(always)]
    fn from(val: u8) -> Ostimer {
        Ostimer::from_bits(val)
    }
}
impl From<Ostimer> for u8 {
    #[inline(always)]
    fn from(val: Ostimer) -> u8 {
        Ostimer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pkc0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Pkc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pkc0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pkc0 {
    #[inline(always)]
    fn from(val: u8) -> Pkc0 {
        Pkc0::from_bits(val)
    }
}
impl From<Pkc0> for u8 {
    #[inline(always)]
    fn from(val: Pkc0) -> u8 {
        Pkc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Port0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port0 {
    #[inline(always)]
    fn from(val: u8) -> Port0 {
        Port0::from_bits(val)
    }
}
impl From<Port0> for u8 {
    #[inline(always)]
    fn from(val: Port0) -> u8 {
        Port0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Port1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port1 {
    #[inline(always)]
    fn from(val: u8) -> Port1 {
        Port1::from_bits(val)
    }
}
impl From<Port1> for u8 {
    #[inline(always)]
    fn from(val: Port1) -> u8 {
        Port1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Port2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port2 {
    #[inline(always)]
    fn from(val: u8) -> Port2 {
        Port2::from_bits(val)
    }
}
impl From<Port2> for u8 {
    #[inline(always)]
    fn from(val: Port2) -> u8 {
        Port2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Port3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port3 {
    #[inline(always)]
    fn from(val: u8) -> Port3 {
        Port3::from_bits(val)
    }
}
impl From<Port3> for u8 {
    #[inline(always)]
    fn from(val: Port3) -> u8 {
        Port3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Port4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port4 {
    #[inline(always)]
    fn from(val: u8) -> Port4 {
        Port4::from_bits(val)
    }
}
impl From<Port4> for u8 {
    #[inline(always)]
    fn from(val: Port4) -> u8 {
        Port4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Port5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port5 {
    #[inline(always)]
    fn from(val: u8) -> Port5 {
        Port5::from_bits(val)
    }
}
impl From<Port5> for u8 {
    #[inline(always)]
    fn from(val: Port5) -> u8 {
        Port5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamaMemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule0 {
        RamaMemRuleRule0::from_bits(val)
    }
}
impl From<RamaMemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule0) -> u8 {
        RamaMemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamaMemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule1 {
        RamaMemRuleRule1::from_bits(val)
    }
}
impl From<RamaMemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule1) -> u8 {
        RamaMemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamaMemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule2 {
        RamaMemRuleRule2::from_bits(val)
    }
}
impl From<RamaMemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule2) -> u8 {
        RamaMemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamaMemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule3 {
        RamaMemRuleRule3::from_bits(val)
    }
}
impl From<RamaMemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule3) -> u8 {
        RamaMemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamaMemRuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule4 {
        RamaMemRuleRule4::from_bits(val)
    }
}
impl From<RamaMemRuleRule4> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule4) -> u8 {
        RamaMemRuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamaMemRuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule5 {
        RamaMemRuleRule5::from_bits(val)
    }
}
impl From<RamaMemRuleRule5> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule5) -> u8 {
        RamaMemRuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamaMemRuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule6 {
        RamaMemRuleRule6::from_bits(val)
    }
}
impl From<RamaMemRuleRule6> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule6) -> u8 {
        RamaMemRuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamaMemRuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule7 {
        RamaMemRuleRule7::from_bits(val)
    }
}
impl From<RamaMemRuleRule7> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule7) -> u8 {
        RamaMemRuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RambMemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule0 {
        RambMemRuleRule0::from_bits(val)
    }
}
impl From<RambMemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule0) -> u8 {
        RambMemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RambMemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule1 {
        RambMemRuleRule1::from_bits(val)
    }
}
impl From<RambMemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule1) -> u8 {
        RambMemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RambMemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule2 {
        RambMemRuleRule2::from_bits(val)
    }
}
impl From<RambMemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule2) -> u8 {
        RambMemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RambMemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule3 {
        RambMemRuleRule3::from_bits(val)
    }
}
impl From<RambMemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule3) -> u8 {
        RambMemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RambMemRuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule4 {
        RambMemRuleRule4::from_bits(val)
    }
}
impl From<RambMemRuleRule4> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule4) -> u8 {
        RambMemRuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RambMemRuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule5 {
        RambMemRuleRule5::from_bits(val)
    }
}
impl From<RambMemRuleRule5> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule5) -> u8 {
        RambMemRuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RambMemRuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule6 {
        RambMemRuleRule6::from_bits(val)
    }
}
impl From<RambMemRuleRule6> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule6) -> u8 {
        RambMemRuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RambMemRuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule7 {
        RambMemRuleRule7::from_bits(val)
    }
}
impl From<RambMemRuleRule7> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule7) -> u8 {
        RambMemRuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamxMemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule0 {
        RamxMemRuleRule0::from_bits(val)
    }
}
impl From<RamxMemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule0) -> u8 {
        RamxMemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamxMemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule1 {
        RamxMemRuleRule1::from_bits(val)
    }
}
impl From<RamxMemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule1) -> u8 {
        RamxMemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamxMemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule2 {
        RamxMemRuleRule2::from_bits(val)
    }
}
impl From<RamxMemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule2) -> u8 {
        RamxMemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamxMemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule3 {
        RamxMemRuleRule3::from_bits(val)
    }
}
impl From<RamxMemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule3) -> u8 {
        RamxMemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamxMemRuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule4 {
        RamxMemRuleRule4::from_bits(val)
    }
}
impl From<RamxMemRuleRule4> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule4) -> u8 {
        RamxMemRuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamxMemRuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule5 {
        RamxMemRuleRule5::from_bits(val)
    }
}
impl From<RamxMemRuleRule5> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule5) -> u8 {
        RamxMemRuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamxMemRuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule6 {
        RamxMemRuleRule6::from_bits(val)
    }
}
impl From<RamxMemRuleRule6> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule6) -> u8 {
        RamxMemRuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamxMemRuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule7 {
        RamxMemRuleRule7::from_bits(val)
    }
}
impl From<RamxMemRuleRule7> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule7) -> u8 {
        RamxMemRuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RomMemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule0 {
        RomMemRuleRule0::from_bits(val)
    }
}
impl From<RomMemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule0) -> u8 {
        RomMemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RomMemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule1 {
        RomMemRuleRule1::from_bits(val)
    }
}
impl From<RomMemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule1) -> u8 {
        RomMemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RomMemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule2 {
        RomMemRuleRule2::from_bits(val)
    }
}
impl From<RomMemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule2) -> u8 {
        RomMemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RomMemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule3 {
        RomMemRuleRule3::from_bits(val)
    }
}
impl From<RomMemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule3) -> u8 {
        RomMemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RomMemRuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule4 {
        RomMemRuleRule4::from_bits(val)
    }
}
impl From<RomMemRuleRule4> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule4) -> u8 {
        RomMemRuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RomMemRuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule5 {
        RomMemRuleRule5::from_bits(val)
    }
}
impl From<RomMemRuleRule5> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule5) -> u8 {
        RomMemRuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RomMemRuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule6 {
        RomMemRuleRule6::from_bits(val)
    }
}
impl From<RomMemRuleRule6> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule6) -> u8 {
        RomMemRuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RomMemRuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule7 {
        RomMemRuleRule7::from_bits(val)
    }
}
impl From<RomMemRuleRule7> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule7) -> u8 {
        RomMemRuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Romcp {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Romcp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Romcp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Romcp {
    #[inline(always)]
    fn from(val: u8) -> Romcp {
        Romcp::from_bits(val)
    }
}
impl From<Romcp> for u8 {
    #[inline(always)]
    fn from(val: Romcp) -> u8 {
        Romcp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rtc0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Rtc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtc0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtc0 {
    #[inline(always)]
    fn from(val: u8) -> Rtc0 {
        Rtc0::from_bits(val)
    }
}
impl From<Rtc0> for u8 {
    #[inline(always)]
    fn from(val: Rtc0) -> u8 {
        Rtc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scg {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Scg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scg {
    #[inline(always)]
    fn from(val: u8) -> Scg {
        Scg::from_bits(val)
    }
}
impl From<Scg> for u8 {
    #[inline(always)]
    fn from(val: Scg) -> u8 {
        Scg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecVioInfoDataAccess {
    #[doc = "Code."]
    CODE = 0x0,
    #[doc = "Data."]
    DATA = 0x01,
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
    #[doc = "CM33 Code."]
    CPU0_CODE = 0x0,
    #[doc = "CM33 System."]
    CPU0_SYS = 0x01,
    #[doc = "SMARTDMA Instruction."]
    SDMA_INSTR = 0x02,
    #[doc = "SMARTDMA Data."]
    SDMA_DATA = 0x03,
    #[doc = "eDMA1."]
    E_DMA1 = 0x04,
    #[doc = "eDMA0."]
    E_DMA0 = 0x05,
    #[doc = "USB HS."]
    USB_HS = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "eSPI."]
    ESPI = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "PKC."]
    PKC = 0x0c,
    #[doc = "Ethernet."]
    ENET = 0x0d,
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
    READ = 0x0,
    #[doc = "Write access."]
    WRITE = 0x01,
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Seccon {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Seccon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Seccon {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Seccon {
    #[inline(always)]
    fn from(val: u8) -> Seccon {
        Seccon::from_bits(val)
    }
}
impl From<Seccon> for u8 {
    #[inline(always)]
    fn from(val: Seccon) -> u8 {
        Seccon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sgi0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Sgi0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sgi0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sgi0 {
    #[inline(always)]
    fn from(val: u8) -> Sgi0 {
        Sgi0::from_bits(val)
    }
}
impl From<Sgi0> for u8 {
    #[inline(always)]
    fn from(val: Sgi0) -> u8 {
        Sgi0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spc {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Spc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spc {
    #[inline(always)]
    fn from(val: u8) -> Spc {
        Spc::from_bits(val)
    }
}
impl From<Spc> for u8 {
    #[inline(always)]
    fn from(val: Spc) -> u8 {
        Spc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpiFileter0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl SpiFileter0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpiFileter0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpiFileter0 {
    #[inline(always)]
    fn from(val: u8) -> SpiFileter0 {
        SpiFileter0::from_bits(val)
    }
}
impl From<SpiFileter0> for u8 {
    #[inline(always)]
    fn from(val: SpiFileter0) -> u8 {
        SpiFileter0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Syscon {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Syscon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Syscon {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Syscon {
    #[inline(always)]
    fn from(val: u8) -> Syscon {
        Syscon::from_bits(val)
    }
}
impl From<Syscon> for u8 {
    #[inline(always)]
    fn from(val: Syscon) -> u8 {
        Syscon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum T1s0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl T1s0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> T1s0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for T1s0 {
    #[inline(always)]
    fn from(val: u8) -> T1s0 {
        T1s0::from_bits(val)
    }
}
impl From<T1s0> for u8 {
    #[inline(always)]
    fn from(val: T1s0) -> u8 {
        T1s0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tdet0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Tdet0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tdet0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tdet0 {
    #[inline(always)]
    fn from(val: u8) -> Tdet0 {
        Tdet0::from_bits(val)
    }
}
impl From<Tdet0> for u8 {
    #[inline(always)]
    fn from(val: Tdet0) -> u8 {
        Tdet0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trng0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Trng0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trng0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trng0 {
    #[inline(always)]
    fn from(val: u8) -> Trng0 {
        Trng0::from_bits(val)
    }
}
impl From<Trng0> for u8 {
    #[inline(always)]
    fn from(val: Trng0) -> u8 {
        Trng0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsi0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Tsi0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsi0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsi0 {
    #[inline(always)]
    fn from(val: u8) -> Tsi0 {
        Tsi0::from_bits(val)
    }
}
impl From<Tsi0> for u8 {
    #[inline(always)]
    fn from(val: Tsi0) -> u8 {
        Tsi0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Udf0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Udf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Udf0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Udf0 {
    #[inline(always)]
    fn from(val: u8) -> Udf0 {
        Udf0::from_bits(val)
    }
}
impl From<Udf0> for u8 {
    #[inline(always)]
    fn from(val: Udf0) -> u8 {
        Udf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1Phy {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Usb1Phy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1Phy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1Phy {
    #[inline(always)]
    fn from(val: u8) -> Usb1Phy {
        Usb1Phy::from_bits(val)
    }
}
impl From<Usb1Phy> for u8 {
    #[inline(always)]
    fn from(val: Usb1Phy) -> u8 {
        Usb1Phy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Utick {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Utick {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Utick {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Utick {
    #[inline(always)]
    fn from(val: u8) -> Utick {
        Utick::from_bits(val)
    }
}
impl From<Utick> for u8 {
    #[inline(always)]
    fn from(val: Utick) -> u8 {
        Utick::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vbat {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Vbat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbat {
    #[inline(always)]
    fn from(val: u8) -> Vbat {
        Vbat::from_bits(val)
    }
}
impl From<Vbat> for u8 {
    #[inline(always)]
    fn from(val: Vbat) -> u8 {
        Vbat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vref0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Vref0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vref0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vref0 {
    #[inline(always)]
    fn from(val: u8) -> Vref0 {
        Vref0::from_bits(val)
    }
}
impl From<Vref0> for u8 {
    #[inline(always)]
    fn from(val: Vref0) -> u8 {
        Vref0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WakeTimer {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl WakeTimer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WakeTimer {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WakeTimer {
    #[inline(always)]
    fn from(val: u8) -> WakeTimer {
        WakeTimer::from_bits(val)
    }
}
impl From<WakeTimer> for u8 {
    #[inline(always)]
    fn from(val: WakeTimer) -> u8 {
        WakeTimer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuu {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Wuu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuu {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuu {
    #[inline(always)]
    fn from(val: u8) -> Wuu {
        Wuu::from_bits(val)
    }
}
impl From<Wuu> for u8 {
    #[inline(always)]
    fn from(val: Wuu) -> u8 {
        Wuu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wwdt0 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Wwdt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wwdt0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wwdt0 {
    #[inline(always)]
    fn from(val: u8) -> Wwdt0 {
        Wwdt0::from_bits(val)
    }
}
impl From<Wwdt0> for u8 {
    #[inline(always)]
    fn from(val: Wwdt0) -> u8 {
        Wwdt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wwdt1 {
    #[doc = "Non-secure and non-privilege user access allowed."]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed."]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed."]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed."]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Wwdt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wwdt1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wwdt1 {
    #[inline(always)]
    fn from(val: u8) -> Wwdt1 {
        Wwdt1::from_bits(val)
    }
}
impl From<Wwdt1> for u8 {
    #[inline(always)]
    fn from(val: Wwdt1) -> u8 {
        Wwdt1::to_bits(val)
    }
}
