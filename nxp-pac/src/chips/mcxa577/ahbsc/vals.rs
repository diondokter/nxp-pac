#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockNsMpu {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCK_NS_MPU is 1"]
    LOCK_NS_MPU_EQ_1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_NS_MPU is 0"]
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
    #[doc = "CM33 (CPU0) LOCKNSVTOR is 1"]
    LOCK_NS_VTOR_EQ_1 = 0x01,
    #[doc = "CM33 (CPU0) LOCKNSVTOR is 0"]
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
    #[doc = "CM33 (CPU0) LOCK_S_MPU is 1"]
    LOCK_S_MPU_EQ_1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_S_MPU is 0"]
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
    #[doc = "CM33 (CPU0) LOCK_S_VTAIRCR is 1"]
    LOCK_S_VTAIRCR_EQ_1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_S_VTAIRCR is 0"]
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
    #[doc = "CM33 (CPU0) LOCK_SAU is 1"]
    LOCK_SAU_EQ_1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_SAU is 0"]
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
pub enum MasterSec {
    #[doc = "Non-secure and non-privileged Master"]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master"]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master"]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master"]
    SECURE_PRIV_MASTER = 0x03,
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
    ENABLED = 0x01,
    #[doc = "Disables the privilege checking of non-secure mode access."]
    DISABLED = 0x02,
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
    #[doc = "Master can access memories and peripherals at the same level or below that level."]
    AHBTM = 0x01,
    #[doc = "Master can access memories and peripherals at same level only"]
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
pub enum MiscCtrlRegIdauAllNs {
    _RESERVED_0 = 0x0,
    #[doc = "IDAU is disabled, which means that all memories are attributed as non-secure memory."]
    DISABLED = 0x01,
    #[doc = "IDAU is enabled (restrictive mode)"]
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
    #[doc = "Writes to this register and to the Memory and Peripheral RULE registers are not allowed"]
    LOCKED = 0x01,
    #[doc = "Writes to this register and to the Memory and Peripheral RULE registers are allowed"]
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
pub enum Rule {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
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
    #[doc = "Code"]
    CODE = 0x0,
    #[doc = "Data"]
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
    #[doc = "CM33 Code"]
    CPU0_CODE = 0x0,
    #[doc = "CM33 System"]
    CPU0_SYS = 0x01,
    #[doc = "SMARTDMA Instruction"]
    SDMA_INSTR = 0x02,
    #[doc = "SMARTDMA Data"]
    SDMA_DATA = 0x03,
    #[doc = "eDMA1"]
    E_DMA1 = 0x04,
    #[doc = "eDMA0"]
    E_DMA0 = 0x05,
    #[doc = "USB HS"]
    USB_HS = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "eSPI"]
    ESPI = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "PKC"]
    PKC = 0x0c,
    #[doc = "Ethernet"]
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
    #[doc = "Read access"]
    READ = 0x0,
    #[doc = "Write access"]
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
