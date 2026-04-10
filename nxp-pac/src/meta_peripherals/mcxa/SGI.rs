#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (2fd28c582599 2026-04-02))"]
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sgi {
    ptr: *mut u8,
}
unsafe impl Send for Sgi {}
unsafe impl Sync for Sgi {}
impl Sgi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Input Data register 0 - Word-3."]
    #[inline(always)]
    pub const fn sgi_datin0a(self) -> crate::pac::common::Reg<SgiDatin0a, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Input Data register 0 - Word-2."]
    #[inline(always)]
    pub const fn sgi_datin0b(self) -> crate::pac::common::Reg<SgiDatin0b, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Input Data register 0 - Word-1."]
    #[inline(always)]
    pub const fn sgi_datin0c(self) -> crate::pac::common::Reg<SgiDatin0c, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Input Data register 0 - Word-0."]
    #[inline(always)]
    pub const fn sgi_datin0d(self) -> crate::pac::common::Reg<SgiDatin0d, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "Input Data register 1 - Word-3."]
    #[inline(always)]
    pub const fn sgi_datin1a(self) -> crate::pac::common::Reg<SgiDatin1a, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Input Data register 1 - Word-2."]
    #[inline(always)]
    pub const fn sgi_datin1b(self) -> crate::pac::common::Reg<SgiDatin1b, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Input Data register 1 - Word-1."]
    #[inline(always)]
    pub const fn sgi_datin1c(self) -> crate::pac::common::Reg<SgiDatin1c, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "Input Data register 1 - Word-0."]
    #[inline(always)]
    pub const fn sgi_datin1d(self) -> crate::pac::common::Reg<SgiDatin1d, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "Input Data register 2 - Word-3."]
    #[inline(always)]
    pub const fn sgi_datin2a(self) -> crate::pac::common::Reg<SgiDatin2a, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "Input Data register 2 - Word-2."]
    #[inline(always)]
    pub const fn sgi_datin2b(self) -> crate::pac::common::Reg<SgiDatin2b, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "Input Data register 2 - Word-1."]
    #[inline(always)]
    pub const fn sgi_datin2c(self) -> crate::pac::common::Reg<SgiDatin2c, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "Input Data register 2 - Word-0."]
    #[inline(always)]
    pub const fn sgi_datin2d(self) -> crate::pac::common::Reg<SgiDatin2d, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "Input Data register 3 - Word-3."]
    #[inline(always)]
    pub const fn sgi_datin3a(self) -> crate::pac::common::Reg<SgiDatin3a, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "Input Data register 3 - Word-2."]
    #[inline(always)]
    pub const fn sgi_datin3b(self) -> crate::pac::common::Reg<SgiDatin3b, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "Input Data register 3 - Word-1."]
    #[inline(always)]
    pub const fn sgi_datin3c(self) -> crate::pac::common::Reg<SgiDatin3c, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "Input Data register 3 - Word-0."]
    #[inline(always)]
    pub const fn sgi_datin3d(self) -> crate::pac::common::Reg<SgiDatin3d, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "Input Key register 0 - Word-3."]
    #[inline(always)]
    pub const fn sgi_key0a(self) -> crate::pac::common::Reg<SgiKey0a, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "Input Key register 0 - Word-2."]
    #[inline(always)]
    pub const fn sgi_key0b(self) -> crate::pac::common::Reg<SgiKey0b, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "Input Key register 0 - Word-1."]
    #[inline(always)]
    pub const fn sgi_key0c(self) -> crate::pac::common::Reg<SgiKey0c, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "Input Key register 0 - Word-0."]
    #[inline(always)]
    pub const fn sgi_key0d(self) -> crate::pac::common::Reg<SgiKey0d, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "Input Key register 1 - Word-3."]
    #[inline(always)]
    pub const fn sgi_key1a(self) -> crate::pac::common::Reg<SgiKey1a, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "Input Key register 1 - Word-2."]
    #[inline(always)]
    pub const fn sgi_key1b(self) -> crate::pac::common::Reg<SgiKey1b, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "Input Key register 1 - Word-1."]
    #[inline(always)]
    pub const fn sgi_key1c(self) -> crate::pac::common::Reg<SgiKey1c, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "Input Key register 1 - Word-0."]
    #[inline(always)]
    pub const fn sgi_key1d(self) -> crate::pac::common::Reg<SgiKey1d, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
    #[doc = "Input Key register 2 - Word-3."]
    #[inline(always)]
    pub const fn sgi_key2a(self) -> crate::pac::common::Reg<SgiKey2a, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "Input Key register 2 - Word-2."]
    #[inline(always)]
    pub const fn sgi_key2b(self) -> crate::pac::common::Reg<SgiKey2b, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "Input Key register 2 - Word-1."]
    #[inline(always)]
    pub const fn sgi_key2c(self) -> crate::pac::common::Reg<SgiKey2c, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "Input Key register 2 - Word-0."]
    #[inline(always)]
    pub const fn sgi_key2d(self) -> crate::pac::common::Reg<SgiKey2d, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x026cusize) as _) }
    }
    #[doc = "Input Key register 3 - Word-3."]
    #[inline(always)]
    pub const fn sgi_key3a(self) -> crate::pac::common::Reg<SgiKey3a, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "Input Key register 3 - Word-2."]
    #[inline(always)]
    pub const fn sgi_key3b(self) -> crate::pac::common::Reg<SgiKey3b, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "Input Key register 3 - Word-1."]
    #[inline(always)]
    pub const fn sgi_key3c(self) -> crate::pac::common::Reg<SgiKey3c, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0278usize) as _) }
    }
    #[doc = "Input Key register 3 - Word-0."]
    #[inline(always)]
    pub const fn sgi_key3d(self) -> crate::pac::common::Reg<SgiKey3d, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x027cusize) as _) }
    }
    #[doc = "Input Key register 4 - Word-3."]
    #[inline(always)]
    pub const fn sgi_key4a(self) -> crate::pac::common::Reg<SgiKey4a, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "Input Key register 4 - Word-2."]
    #[inline(always)]
    pub const fn sgi_key4b(self) -> crate::pac::common::Reg<SgiKey4b, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "Input Key register 4 - Word-1."]
    #[inline(always)]
    pub const fn sgi_key4c(self) -> crate::pac::common::Reg<SgiKey4c, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "Input Key register 4 - Word-0."]
    #[inline(always)]
    pub const fn sgi_key4d(self) -> crate::pac::common::Reg<SgiKey4d, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x028cusize) as _) }
    }
    #[doc = "Input Key register 5 - Word-3."]
    #[inline(always)]
    pub const fn sgi_key5a(self) -> crate::pac::common::Reg<SgiKey5a, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    #[doc = "Input Key register 5 - Word-2."]
    #[inline(always)]
    pub const fn sgi_key5b(self) -> crate::pac::common::Reg<SgiKey5b, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0294usize) as _) }
    }
    #[doc = "Input Key register 5 - Word-1."]
    #[inline(always)]
    pub const fn sgi_key5c(self) -> crate::pac::common::Reg<SgiKey5c, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0298usize) as _) }
    }
    #[doc = "Input Key register 5 - Word-0."]
    #[inline(always)]
    pub const fn sgi_key5d(self) -> crate::pac::common::Reg<SgiKey5d, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x029cusize) as _) }
    }
    #[doc = "Input Key register 6 - Word-3."]
    #[inline(always)]
    pub const fn sgi_key6a(self) -> crate::pac::common::Reg<SgiKey6a, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize) as _) }
    }
    #[doc = "Input Key register 6 - Word-2."]
    #[inline(always)]
    pub const fn sgi_key6b(self) -> crate::pac::common::Reg<SgiKey6b, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize) as _) }
    }
    #[doc = "Input Key register 6 - Word-1."]
    #[inline(always)]
    pub const fn sgi_key6c(self) -> crate::pac::common::Reg<SgiKey6c, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
    #[doc = "Input Key register 6 - Word-0."]
    #[inline(always)]
    pub const fn sgi_key6d(self) -> crate::pac::common::Reg<SgiKey6d, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02acusize) as _) }
    }
    #[doc = "Input Key register 7 - Word-3."]
    #[inline(always)]
    pub const fn sgi_key7a(self) -> crate::pac::common::Reg<SgiKey7a, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize) as _) }
    }
    #[doc = "Input Key register 7 - Word-2."]
    #[inline(always)]
    pub const fn sgi_key7b(self) -> crate::pac::common::Reg<SgiKey7b, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b4usize) as _) }
    }
    #[doc = "Input Key register 7 - Word-1."]
    #[inline(always)]
    pub const fn sgi_key7c(self) -> crate::pac::common::Reg<SgiKey7c, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b8usize) as _) }
    }
    #[doc = "Input Key register 7 - Word-0."]
    #[inline(always)]
    pub const fn sgi_key7d(self) -> crate::pac::common::Reg<SgiKey7d, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02bcusize) as _) }
    }
    #[doc = "Output Data register - Word-3."]
    #[inline(always)]
    pub const fn sgi_datouta(self) -> crate::pac::common::Reg<SgiDatouta, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize) as _) }
    }
    #[doc = "Output Data register - Word-2."]
    #[inline(always)]
    pub const fn sgi_datoutb(self) -> crate::pac::common::Reg<SgiDatoutb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c4usize) as _) }
    }
    #[doc = "Output Data register - Word-1."]
    #[inline(always)]
    pub const fn sgi_datoutc(self) -> crate::pac::common::Reg<SgiDatoutc, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c8usize) as _) }
    }
    #[doc = "Output Data register - Word-0."]
    #[inline(always)]
    pub const fn sgi_datoutd(self) -> crate::pac::common::Reg<SgiDatoutd, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ccusize) as _) }
    }
    #[doc = "Status register."]
    #[inline(always)]
    pub const fn sgi_status(self) -> crate::pac::common::Reg<SgiStatus, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c00usize) as _) }
    }
    #[doc = "Calculation counter."]
    #[inline(always)]
    pub const fn sgi_count(self) -> crate::pac::common::Reg<SgiCount, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c04usize) as _) }
    }
    #[doc = "Key checksum register."]
    #[inline(always)]
    pub const fn sgi_keychk(self) -> crate::pac::common::Reg<SgiKeychk, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c08usize) as _) }
    }
    #[doc = "SGI Control register."]
    #[inline(always)]
    pub const fn sgi_ctrl(self) -> crate::pac::common::Reg<SgiCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d00usize) as _) }
    }
    #[doc = "SGI Control register 2."]
    #[inline(always)]
    pub const fn sgi_ctrl2(self) -> crate::pac::common::Reg<SgiCtrl2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d04usize) as _) }
    }
    #[doc = "Configuration of dummy controls."]
    #[inline(always)]
    pub const fn sgi_dummy_ctrl(
        self,
    ) -> crate::pac::common::Reg<SgiDummyCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d08usize) as _) }
    }
    #[doc = "SFRSEED register for SFRMASK feature."]
    #[inline(always)]
    pub const fn sgi_sfrseed(self) -> crate::pac::common::Reg<SgiSfrseed, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d10usize) as _) }
    }
    #[doc = "SHA Control Register."]
    #[inline(always)]
    pub const fn sgi_sha2_ctrl(
        self,
    ) -> crate::pac::common::Reg<SgiSha2Ctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d14usize) as _) }
    }
    #[doc = "SHA FIFO lower-bank low."]
    #[inline(always)]
    pub const fn sgi_sha_fifo(self) -> crate::pac::common::Reg<SgiShaFifo, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d18usize) as _) }
    }
    #[doc = "SHA Configuration Reg."]
    #[inline(always)]
    pub const fn sgi_config(self) -> crate::pac::common::Reg<SgiConfig, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d1cusize) as _) }
    }
    #[doc = "SHA Configuration 2 Reg."]
    #[inline(always)]
    pub const fn sgi_config2(self) -> crate::pac::common::Reg<SgiConfig2, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d20usize) as _) }
    }
    #[doc = "SGI Auto Mode Control register."]
    #[inline(always)]
    pub const fn sgi_auto_mode(
        self,
    ) -> crate::pac::common::Reg<SgiAutoMode, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d24usize) as _) }
    }
    #[doc = "SGI Auto Mode Control register."]
    #[inline(always)]
    pub const fn sgi_auto_dma_ctrl(
        self,
    ) -> crate::pac::common::Reg<SgiAutoDmaCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d28usize) as _) }
    }
    #[doc = "SGI internal PRNG SW seeding register."]
    #[inline(always)]
    pub const fn sgi_prng_sw_seed(
        self,
    ) -> crate::pac::common::Reg<SgiPrngSwSeed, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d30usize) as _) }
    }
    #[doc = "SGI Key Control SFR."]
    #[inline(always)]
    pub const fn sgi_key_ctrl(self) -> crate::pac::common::Reg<SgiKeyCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d40usize) as _) }
    }
    #[doc = "Wrapped key read SFR."]
    #[inline(always)]
    pub const fn sgi_key_wrap(self) -> crate::pac::common::Reg<SgiKeyWrap, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d50usize) as _) }
    }
    #[doc = "SGI Version."]
    #[inline(always)]
    pub const fn sgi_version(self) -> crate::pac::common::Reg<SgiVersion, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f08usize) as _) }
    }
    #[doc = "Access Error."]
    #[inline(always)]
    pub const fn sgi_access_err(
        self,
    ) -> crate::pac::common::Reg<SgiAccessErr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc0usize) as _) }
    }
    #[doc = "Clear Access Error."]
    #[inline(always)]
    pub const fn sgi_access_err_clr(
        self,
    ) -> crate::pac::common::Reg<SgiAccessErrClr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc4usize) as _) }
    }
    #[doc = "Interrupt status."]
    #[inline(always)]
    pub const fn sgi_int_status(
        self,
    ) -> crate::pac::common::Reg<SgiIntStatus, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe0usize) as _) }
    }
    #[doc = "Interrupt enable."]
    #[inline(always)]
    pub const fn sgi_int_enable(
        self,
    ) -> crate::pac::common::Reg<SgiIntEnable, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe4usize) as _) }
    }
    #[doc = "Interrupt status clear."]
    #[inline(always)]
    pub const fn sgi_int_status_clr(
        self,
    ) -> crate::pac::common::Reg<SgiIntStatusClr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe8usize) as _) }
    }
    #[doc = "Interrupt status set."]
    #[inline(always)]
    pub const fn sgi_int_status_set(
        self,
    ) -> crate::pac::common::Reg<SgiIntStatusSet, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fecusize) as _) }
    }
    #[doc = "Module ID."]
    #[inline(always)]
    pub const fn sgi_module_id(
        self,
    ) -> crate::pac::common::Reg<SgiModuleId, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
#[doc = "Access Error."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiAccessErr(pub u32);
impl SgiAccessErr {
    #[doc = "APB Error: address not available."]
    #[must_use]
    #[inline(always)]
    pub const fn apb_notav(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "APB Error: address not available."]
    #[inline(always)]
    pub const fn set_apb_notav(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APB Error: Wrong access mode."]
    #[must_use]
    #[inline(always)]
    pub const fn apb_wrgmd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "APB Error: Wrong access mode."]
    #[inline(always)]
    pub const fn set_apb_wrgmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "APB Master that triggered first APB error (APB_WRGMD or APB_NOTAV)."]
    #[must_use]
    #[inline(always)]
    pub const fn apb_master(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "APB Master that triggered first APB error (APB_WRGMD or APB_NOTAV)."]
    #[inline(always)]
    pub const fn set_apb_master(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for SgiAccessErr {
    #[inline(always)]
    fn default() -> SgiAccessErr {
        SgiAccessErr(0)
    }
}
impl core::fmt::Debug for SgiAccessErr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiAccessErr")
            .field("apb_notav", &self.apb_notav())
            .field("apb_wrgmd", &self.apb_wrgmd())
            .field("apb_master", &self.apb_master())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiAccessErr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SgiAccessErr {{ apb_notav: {=bool:?}, apb_wrgmd: {=bool:?}, apb_master: {=u8:?} }}",
            self.apb_notav(),
            self.apb_wrgmd(),
            self.apb_master()
        )
    }
}
#[doc = "Clear Access Error."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiAccessErrClr(pub u32);
impl SgiAccessErrClr {
    #[doc = "Write to reset SGI_ACCESS_ERR SFR."]
    #[must_use]
    #[inline(always)]
    pub const fn err_clr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write to reset SGI_ACCESS_ERR SFR."]
    #[inline(always)]
    pub const fn set_err_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for SgiAccessErrClr {
    #[inline(always)]
    fn default() -> SgiAccessErrClr {
        SgiAccessErrClr(0)
    }
}
impl core::fmt::Debug for SgiAccessErrClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiAccessErrClr")
            .field("err_clr", &self.err_clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiAccessErrClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SgiAccessErrClr {{ err_clr: {=bool:?} }}",
            self.err_clr()
        )
    }
}
#[doc = "SGI Auto Mode Control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiAutoDmaCtrl(pub u32);
impl SgiAutoDmaCtrl {
    #[doc = "Input FIFO DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ife(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Input FIFO DMA Enable."]
    #[inline(always)]
    pub const fn set_ife(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Ouput FIFO DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ofe(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Ouput FIFO DMA Enable."]
    #[inline(always)]
    pub const fn set_ofe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for SgiAutoDmaCtrl {
    #[inline(always)]
    fn default() -> SgiAutoDmaCtrl {
        SgiAutoDmaCtrl(0)
    }
}
impl core::fmt::Debug for SgiAutoDmaCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiAutoDmaCtrl")
            .field("ife", &self.ife())
            .field("ofe", &self.ofe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiAutoDmaCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SgiAutoDmaCtrl {{ ife: {=bool:?}, ofe: {=bool:?} }}",
            self.ife(),
            self.ofe()
        )
    }
}
#[doc = "SGI Auto Mode Control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiAutoMode(pub u32);
impl SgiAutoMode {
    #[doc = "auto_start_en."]
    #[must_use]
    #[inline(always)]
    pub const fn auto_mode_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "auto_start_en."]
    #[inline(always)]
    pub const fn set_auto_mode_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "auto_mode_stop."]
    #[must_use]
    #[inline(always)]
    pub const fn auto_mode_stop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "auto_mode_stop."]
    #[inline(always)]
    pub const fn set_auto_mode_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CTR increment mode."]
    #[must_use]
    #[inline(always)]
    pub const fn incr_mode(&self) -> IncrMode {
        let val = (self.0 >> 4usize) & 0x03;
        IncrMode::from_bits(val as u8)
    }
    #[doc = "CTR increment mode."]
    #[inline(always)]
    pub const fn set_incr_mode(&mut self, val: IncrMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Auto mode of operation."]
    #[must_use]
    #[inline(always)]
    pub const fn cmd(&self) -> Cmd {
        let val = (self.0 >> 8usize) & 0xff;
        Cmd::from_bits(val as u8)
    }
    #[doc = "Auto mode of operation."]
    #[inline(always)]
    pub const fn set_cmd(&mut self, val: Cmd) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
}
impl Default for SgiAutoMode {
    #[inline(always)]
    fn default() -> SgiAutoMode {
        SgiAutoMode(0)
    }
}
impl core::fmt::Debug for SgiAutoMode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiAutoMode")
            .field("auto_mode_en", &self.auto_mode_en())
            .field("auto_mode_stop", &self.auto_mode_stop())
            .field("incr_mode", &self.incr_mode())
            .field("cmd", &self.cmd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiAutoMode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SgiAutoMode {{ auto_mode_en: {=bool:?}, auto_mode_stop: {=bool:?}, incr_mode: {:?}, cmd: {:?} }}",
            self.auto_mode_en(),
            self.auto_mode_stop(),
            self.incr_mode(),
            self.cmd()
        )
    }
}
#[doc = "SHA Configuration Reg."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiConfig(pub u32);
impl SgiConfig {
    #[doc = "SGI Diversified for 'ROW'."]
    #[must_use]
    #[inline(always)]
    pub const fn row(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SGI Diversified for 'ROW'."]
    #[inline(always)]
    pub const fn set_row(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SGI Diversified for 'CHINA'."]
    #[must_use]
    #[inline(always)]
    pub const fn china(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SGI Diversified for 'CHINA'."]
    #[inline(always)]
    pub const fn set_china(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SGI Diversified for 'CC'."]
    #[must_use]
    #[inline(always)]
    pub const fn cc(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SGI Diversified for 'CC'."]
    #[inline(always)]
    pub const fn set_cc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "HAS AES."]
    #[must_use]
    #[inline(always)]
    pub const fn has_aes(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "HAS AES."]
    #[inline(always)]
    pub const fn set_has_aes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "HAS DES."]
    #[must_use]
    #[inline(always)]
    pub const fn has_des(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "HAS DES."]
    #[inline(always)]
    pub const fn set_has_des(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "HAS SHA."]
    #[must_use]
    #[inline(always)]
    pub const fn has_sha(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "HAS SHA."]
    #[inline(always)]
    pub const fn set_has_sha(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "HAS MOVEM."]
    #[must_use]
    #[inline(always)]
    pub const fn has_movem(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "HAS MOVEM."]
    #[inline(always)]
    pub const fn set_has_movem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "HAS CMAC."]
    #[must_use]
    #[inline(always)]
    pub const fn has_cmac(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "HAS CMAC."]
    #[inline(always)]
    pub const fn set_has_cmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "HAS GFMUL."]
    #[must_use]
    #[inline(always)]
    pub const fn has_gfmul(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "HAS GFMUL."]
    #[inline(always)]
    pub const fn set_has_gfmul(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "HAS INTERNAL PRNG."]
    #[must_use]
    #[inline(always)]
    pub const fn internal_prng(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "HAS INTERNAL PRNG."]
    #[inline(always)]
    pub const fn set_internal_prng(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "HAS KEY DIGEST."]
    #[must_use]
    #[inline(always)]
    pub const fn key_digest(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "HAS KEY DIGEST."]
    #[inline(always)]
    pub const fn set_key_digest(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "0 - COUNT=16, 1 - COUNT=32."]
    #[must_use]
    #[inline(always)]
    pub const fn count_size(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "0 - COUNT=16, 1 - COUNT=32."]
    #[inline(always)]
    pub const fn set_count_size(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "HAS FA protection."]
    #[must_use]
    #[inline(always)]
    pub const fn fa(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "HAS FA protection."]
    #[inline(always)]
    pub const fn set_fa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "0 - BUS_WIDTH=16, 1 - BUS_WIDTH=32."]
    #[must_use]
    #[inline(always)]
    pub const fn bus_width(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "0 - BUS_WIDTH=16, 1 - BUS_WIDTH=32."]
    #[inline(always)]
    pub const fn set_bus_width(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "NUMBER OF DATIN REGBANKS."]
    #[must_use]
    #[inline(always)]
    pub const fn num_datin(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "NUMBER OF DATIN REGBANKS."]
    #[inline(always)]
    pub const fn set_num_datin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "NUMBER OR KEY REGBANKS."]
    #[must_use]
    #[inline(always)]
    pub const fn num_key(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "NUMBER OR KEY REGBANKS."]
    #[inline(always)]
    pub const fn set_num_key(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "DATIN to KERNEL End-to-end EDC is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn edc(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DATIN to KERNEL End-to-end EDC is enabled."]
    #[inline(always)]
    pub const fn set_edc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "HAS SHA-256 ONLY."]
    #[must_use]
    #[inline(always)]
    pub const fn sha_256_only(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "HAS SHA-256 ONLY."]
    #[inline(always)]
    pub const fn set_sha_256_only(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "ID_CFG_SGI_SPB_SUPPORT is set."]
    #[must_use]
    #[inline(always)]
    pub const fn spb_support(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "ID_CFG_SGI_SPB_SUPPORT is set."]
    #[inline(always)]
    pub const fn set_spb_support(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "ID_CFG_SGI_SPB_MASKING is set."]
    #[must_use]
    #[inline(always)]
    pub const fn spb_masking(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "ID_CFG_SGI_SPB_MASKING is set."]
    #[inline(always)]
    pub const fn set_spb_masking(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "ID_CFG_SGI_USE_SFR_SW_MASK is set."]
    #[must_use]
    #[inline(always)]
    pub const fn sfr_sw_mask(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "ID_CFG_SGI_USE_SFR_SW_MASK is set."]
    #[inline(always)]
    pub const fn set_sfr_sw_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for SgiConfig {
    #[inline(always)]
    fn default() -> SgiConfig {
        SgiConfig(0)
    }
}
impl core::fmt::Debug for SgiConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiConfig")
            .field("row", &self.row())
            .field("china", &self.china())
            .field("cc", &self.cc())
            .field("has_aes", &self.has_aes())
            .field("has_des", &self.has_des())
            .field("has_sha", &self.has_sha())
            .field("has_movem", &self.has_movem())
            .field("has_cmac", &self.has_cmac())
            .field("has_gfmul", &self.has_gfmul())
            .field("internal_prng", &self.internal_prng())
            .field("key_digest", &self.key_digest())
            .field("count_size", &self.count_size())
            .field("fa", &self.fa())
            .field("bus_width", &self.bus_width())
            .field("num_datin", &self.num_datin())
            .field("num_key", &self.num_key())
            .field("edc", &self.edc())
            .field("sha_256_only", &self.sha_256_only())
            .field("spb_support", &self.spb_support())
            .field("spb_masking", &self.spb_masking())
            .field("sfr_sw_mask", &self.sfr_sw_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiConfig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SgiConfig {{ row: {=bool:?}, china: {=bool:?}, cc: {=bool:?}, has_aes: {=bool:?}, has_des: {=bool:?}, has_sha: {=bool:?}, has_movem: {=bool:?}, has_cmac: {=bool:?}, has_gfmul: {=bool:?}, internal_prng: {=bool:?}, key_digest: {=bool:?}, count_size: {=bool:?}, fa: {=bool:?}, bus_width: {=bool:?}, num_datin: {=u8:?}, num_key: {=u8:?}, edc: {=bool:?}, sha_256_only: {=bool:?}, spb_support: {=bool:?}, spb_masking: {=bool:?}, sfr_sw_mask: {=bool:?} }}",
            self.row(),
            self.china(),
            self.cc(),
            self.has_aes(),
            self.has_des(),
            self.has_sha(),
            self.has_movem(),
            self.has_cmac(),
            self.has_gfmul(),
            self.internal_prng(),
            self.key_digest(),
            self.count_size(),
            self.fa(),
            self.bus_width(),
            self.num_datin(),
            self.num_key(),
            self.edc(),
            self.sha_256_only(),
            self.spb_support(),
            self.spb_masking(),
            self.sfr_sw_mask()
        )
    }
}
#[doc = "SHA Configuration 2 Reg."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiConfig2(pub u32);
impl SgiConfig2 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn aes_used(&self) -> AesUsed {
        let val = (self.0 >> 0usize) & 0x0f;
        AesUsed::from_bits(val as u8)
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_aes_used(&mut self, val: AesUsed) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Number of AES sboxes."]
    #[must_use]
    #[inline(always)]
    pub const fn aes_num_sboxes(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of AES sboxes."]
    #[inline(always)]
    pub const fn set_aes_num_sboxes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
    }
    #[doc = "Indicates which AES key size has been selected."]
    #[must_use]
    #[inline(always)]
    pub const fn aes_keysize(&self) -> AesKeysize {
        let val = (self.0 >> 9usize) & 0x03;
        AesKeysize::from_bits(val as u8)
    }
    #[doc = "Indicates which AES key size has been selected."]
    #[inline(always)]
    pub const fn set_aes_keysize(&mut self, val: AesKeysize) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn des_used(&self) -> DesUsed {
        let val = (self.0 >> 16usize) & 0x0f;
        DesUsed::from_bits(val as u8)
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_des_used(&mut self, val: DesUsed) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Number of DES sboxes."]
    #[must_use]
    #[inline(always)]
    pub const fn des_num_sboxes(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of DES sboxes."]
    #[inline(always)]
    pub const fn set_des_num_sboxes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
    }
}
impl Default for SgiConfig2 {
    #[inline(always)]
    fn default() -> SgiConfig2 {
        SgiConfig2(0)
    }
}
impl core::fmt::Debug for SgiConfig2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiConfig2")
            .field("aes_used", &self.aes_used())
            .field("aes_num_sboxes", &self.aes_num_sboxes())
            .field("aes_keysize", &self.aes_keysize())
            .field("des_used", &self.des_used())
            .field("des_num_sboxes", &self.des_num_sboxes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiConfig2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SgiConfig2 {{ aes_used: {:?}, aes_num_sboxes: {=u8:?}, aes_keysize: {:?}, des_used: {:?}, des_num_sboxes: {=u8:?} }}",
            self.aes_used(),
            self.aes_num_sboxes(),
            self.aes_keysize(),
            self.des_used(),
            self.des_num_sboxes()
        )
    }
}
#[doc = "Calculation counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiCount(pub u32);
impl SgiCount {
    #[doc = "Calculation counter, incremented with each calculation start."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Calculation counter, incremented with each calculation start."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for SgiCount {
    #[inline(always)]
    fn default() -> SgiCount {
        SgiCount(0)
    }
}
impl core::fmt::Debug for SgiCount {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiCount")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiCount {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiCount {{ count: {=u16:?} }}", self.count())
    }
}
#[doc = "SGI Control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiCtrl(pub u32);
impl SgiCtrl {
    #[doc = "Start crypto operation."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> Start {
        let val = (self.0 >> 0usize) & 0x01;
        Start::from_bits(val as u8)
    }
    #[doc = "Start crypto operation."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: Start) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Sets Cipher direction(AES and DES)."]
    #[must_use]
    #[inline(always)]
    pub const fn decrypt(&self) -> Decrypt {
        let val = (self.0 >> 1usize) & 0x01;
        Decrypt::from_bits(val as u8)
    }
    #[doc = "Sets Cipher direction(AES and DES)."]
    #[inline(always)]
    pub const fn set_decrypt(&mut self, val: Decrypt) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Sets AES key size."]
    #[must_use]
    #[inline(always)]
    pub const fn aeskeysz(&self) -> Aeskeysz {
        let val = (self.0 >> 2usize) & 0x03;
        Aeskeysz::from_bits(val as u8)
    }
    #[doc = "Sets AES key size."]
    #[inline(always)]
    pub const fn set_aeskeysz(&mut self, val: Aeskeysz) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Sets 'Crypto Operation' type."]
    #[must_use]
    #[inline(always)]
    pub const fn crypto_op(&self) -> CryptoOp {
        let val = (self.0 >> 4usize) & 0x07;
        CryptoOp::from_bits(val as u8)
    }
    #[doc = "Sets 'Crypto Operation' type."]
    #[inline(always)]
    pub const fn set_crypto_op(&mut self, val: CryptoOp) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn insel(&self) -> Insel {
        let val = (self.0 >> 7usize) & 0x0f;
        Insel::from_bits(val as u8)
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_insel(&mut self, val: Insel) {
        self.0 = (self.0 & !(0x0f << 7usize)) | (((val.to_bits() as u32) & 0x0f) << 7usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn outsel(&self) -> Outsel {
        let val = (self.0 >> 11usize) & 0x07;
        Outsel::from_bits(val as u8)
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_outsel(&mut self, val: Outsel) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
    }
    #[doc = "Kernels data out options."]
    #[must_use]
    #[inline(always)]
    pub const fn datout_res(&self) -> DatoutRes {
        let val = (self.0 >> 14usize) & 0x03;
        DatoutRes::from_bits(val as u8)
    }
    #[doc = "Kernels data out options."]
    #[inline(always)]
    pub const fn set_datout_res(&mut self, val: DatoutRes) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "AES Kernel Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn aes_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "AES Kernel Enable."]
    #[inline(always)]
    pub const fn set_aes_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DES Kernel Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn des_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DES Kernel Enable."]
    #[inline(always)]
    pub const fn set_des_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "GFMUL Kernel Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gcm_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "GFMUL Kernel Enable."]
    #[inline(always)]
    pub const fn set_gcm_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "PRNG Enable (only if SGI has internal PRNG)."]
    #[must_use]
    #[inline(always)]
    pub const fn prng_en(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "PRNG Enable (only if SGI has internal PRNG)."]
    #[inline(always)]
    pub const fn set_prng_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Input key selection."]
    #[must_use]
    #[inline(always)]
    pub const fn inkeysel(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x1f;
        val as u8
    }
    #[doc = "Input key selection."]
    #[inline(always)]
    pub const fn set_inkeysel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
    }
    #[doc = "Triple-DES Key Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn tdeskey(&self) -> Tdeskey {
        let val = (self.0 >> 25usize) & 0x01;
        Tdeskey::from_bits(val as u8)
    }
    #[doc = "Triple-DES Key Configuration."]
    #[inline(always)]
    pub const fn set_tdeskey(&mut self, val: Tdeskey) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "AES No decryption key scheduleThis bit is only supported for select configurations of the SGI."]
    #[must_use]
    #[inline(always)]
    pub const fn aes_no_kl(&self) -> AesNoKl {
        let val = (self.0 >> 26usize) & 0x01;
        AesNoKl::from_bits(val as u8)
    }
    #[doc = "AES No decryption key scheduleThis bit is only supported for select configurations of the SGI."]
    #[inline(always)]
    pub const fn set_aes_no_kl(&mut self, val: AesNoKl) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "AES Dual Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn aes_sel(&self) -> AesSel {
        let val = (self.0 >> 27usize) & 0x01;
        AesSel::from_bits(val as u8)
    }
    #[doc = "AES Dual Selection."]
    #[inline(always)]
    pub const fn set_aes_sel(&mut self, val: AesSel) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for SgiCtrl {
    #[inline(always)]
    fn default() -> SgiCtrl {
        SgiCtrl(0)
    }
}
impl core::fmt::Debug for SgiCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiCtrl")
            .field("start", &self.start())
            .field("decrypt", &self.decrypt())
            .field("aeskeysz", &self.aeskeysz())
            .field("crypto_op", &self.crypto_op())
            .field("insel", &self.insel())
            .field("outsel", &self.outsel())
            .field("datout_res", &self.datout_res())
            .field("aes_en", &self.aes_en())
            .field("des_en", &self.des_en())
            .field("gcm_en", &self.gcm_en())
            .field("prng_en", &self.prng_en())
            .field("inkeysel", &self.inkeysel())
            .field("tdeskey", &self.tdeskey())
            .field("aes_no_kl", &self.aes_no_kl())
            .field("aes_sel", &self.aes_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SgiCtrl {{ start: {:?}, decrypt: {:?}, aeskeysz: {:?}, crypto_op: {:?}, insel: {:?}, outsel: {:?}, datout_res: {:?}, aes_en: {=bool:?}, des_en: {=bool:?}, gcm_en: {=bool:?}, prng_en: {=bool:?}, inkeysel: {=u8:?}, tdeskey: {:?}, aes_no_kl: {:?}, aes_sel: {:?} }}",
            self.start(),
            self.decrypt(),
            self.aeskeysz(),
            self.crypto_op(),
            self.insel(),
            self.outsel(),
            self.datout_res(),
            self.aes_en(),
            self.des_en(),
            self.gcm_en(),
            self.prng_en(),
            self.inkeysel(),
            self.tdeskey(),
            self.aes_no_kl(),
            self.aes_sel()
        )
    }
}
#[doc = "SGI Control register 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiCtrl2(pub u32);
impl SgiCtrl2 {
    #[doc = "Start Full SGI Flush."]
    #[must_use]
    #[inline(always)]
    pub const fn flush(&self) -> Flush {
        let val = (self.0 >> 0usize) & 0x01;
        Flush::from_bits(val as u8)
    }
    #[doc = "Start Full SGI Flush."]
    #[inline(always)]
    pub const fn set_flush(&mut self, val: Flush) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Start KEY register-bank Flush."]
    #[must_use]
    #[inline(always)]
    pub const fn key_flush(&self) -> KeyFlush {
        let val = (self.0 >> 1usize) & 0x01;
        KeyFlush::from_bits(val as u8)
    }
    #[doc = "Start KEY register-bank Flush."]
    #[inline(always)]
    pub const fn set_key_flush(&mut self, val: KeyFlush) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Start DATIN register-bank Flush."]
    #[must_use]
    #[inline(always)]
    pub const fn datin_flush(&self) -> DatinFlush {
        let val = (self.0 >> 2usize) & 0x01;
        DatinFlush::from_bits(val as u8)
    }
    #[doc = "Start DATIN register-bank Flush."]
    #[inline(always)]
    pub const fn set_datin_flush(&mut self, val: DatinFlush) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Increment(Triggered by SFR write)."]
    #[must_use]
    #[inline(always)]
    pub const fn incr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Increment(Triggered by SFR write)."]
    #[inline(always)]
    pub const fn set_incr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write-XOR control."]
    #[must_use]
    #[inline(always)]
    pub const fn xorwr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write-XOR control."]
    #[inline(always)]
    pub const fn set_xorwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Flush Write control."]
    #[must_use]
    #[inline(always)]
    pub const fn flushwr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Flush Write control."]
    #[inline(always)]
    pub const fn set_flushwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Increment Carry-In control."]
    #[must_use]
    #[inline(always)]
    pub const fn incr_cin(&self) -> IncrCin {
        let val = (self.0 >> 6usize) & 0x01;
        IncrCin::from_bits(val as u8)
    }
    #[doc = "Increment Carry-In control."]
    #[inline(always)]
    pub const fn set_incr_cin(&mut self, val: IncrCin) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "SFRMASK Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn smasken(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SFRMASK Enable."]
    #[inline(always)]
    pub const fn set_smasken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SFRSEED increment control."]
    #[must_use]
    #[inline(always)]
    pub const fn smaskstep(&self) -> Smaskstep {
        let val = (self.0 >> 9usize) & 0x01;
        Smaskstep::from_bits(val as u8)
    }
    #[doc = "SFRSEED increment control."]
    #[inline(always)]
    pub const fn set_smaskstep(&mut self, val: Smaskstep) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "SFRMASK MASK control."]
    #[must_use]
    #[inline(always)]
    pub const fn smasksw(&self) -> Smasksw {
        let val = (self.0 >> 10usize) & 0x01;
        Smasksw::from_bits(val as u8)
    }
    #[doc = "SFRMASK MASK control."]
    #[inline(always)]
    pub const fn set_smasksw(&mut self, val: Smasksw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "4-bit optional input for MOVEM feature."]
    #[must_use]
    #[inline(always)]
    pub const fn movem(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "4-bit optional input for MOVEM feature."]
    #[inline(always)]
    pub const fn set_movem(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Selects key registers to be updated when rkey=1."]
    #[must_use]
    #[inline(always)]
    pub const fn keyres(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Selects key registers to be updated when rkey=1."]
    #[inline(always)]
    pub const fn set_keyres(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Crypto result location."]
    #[must_use]
    #[inline(always)]
    pub const fn rkey(&self) -> Rkey {
        let val = (self.0 >> 21usize) & 0x01;
        Rkey::from_bits(val as u8)
    }
    #[doc = "Crypto result location."]
    #[inline(always)]
    pub const fn set_rkey(&mut self, val: Rkey) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Byte order of regbank read/write data."]
    #[must_use]
    #[inline(always)]
    pub const fn bytes_order(&self) -> BytesOrder {
        let val = (self.0 >> 22usize) & 0x01;
        BytesOrder::from_bits(val as u8)
    }
    #[doc = "Byte order of regbank read/write data."]
    #[inline(always)]
    pub const fn set_bytes_order(&mut self, val: BytesOrder) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "GCM INXOR."]
    #[must_use]
    #[inline(always)]
    pub const fn gcm_inxor(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "GCM INXOR."]
    #[inline(always)]
    pub const fn set_gcm_inxor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for SgiCtrl2 {
    #[inline(always)]
    fn default() -> SgiCtrl2 {
        SgiCtrl2(0)
    }
}
impl core::fmt::Debug for SgiCtrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiCtrl2")
            .field("flush", &self.flush())
            .field("key_flush", &self.key_flush())
            .field("datin_flush", &self.datin_flush())
            .field("incr", &self.incr())
            .field("xorwr", &self.xorwr())
            .field("flushwr", &self.flushwr())
            .field("incr_cin", &self.incr_cin())
            .field("smasken", &self.smasken())
            .field("smaskstep", &self.smaskstep())
            .field("smasksw", &self.smasksw())
            .field("movem", &self.movem())
            .field("keyres", &self.keyres())
            .field("rkey", &self.rkey())
            .field("bytes_order", &self.bytes_order())
            .field("gcm_inxor", &self.gcm_inxor())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiCtrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SgiCtrl2 {{ flush: {:?}, key_flush: {:?}, datin_flush: {:?}, incr: {=bool:?}, xorwr: {=bool:?}, flushwr: {=bool:?}, incr_cin: {:?}, smasken: {=bool:?}, smaskstep: {:?}, smasksw: {:?}, movem: {=u8:?}, keyres: {=u8:?}, rkey: {:?}, bytes_order: {:?}, gcm_inxor: {=bool:?} }}",
            self.flush(),
            self.key_flush(),
            self.datin_flush(),
            self.incr(),
            self.xorwr(),
            self.flushwr(),
            self.incr_cin(),
            self.smasken(),
            self.smaskstep(),
            self.smasksw(),
            self.movem(),
            self.keyres(),
            self.rkey(),
            self.bytes_order(),
            self.gcm_inxor()
        )
    }
}
#[doc = "Input Data register 0 - Word-3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiDatin0a(pub u32);
impl SgiDatin0a {
    #[doc = "Input Data register."]
    #[must_use]
    #[inline(always)]
    pub const fn datin0a(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Data register."]
    #[inline(always)]
    pub const fn set_datin0a(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiDatin0a {
    #[inline(always)]
    fn default() -> SgiDatin0a {
        SgiDatin0a(0)
    }
}
impl core::fmt::Debug for SgiDatin0a {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiDatin0a")
            .field("datin0a", &self.datin0a())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiDatin0a {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiDatin0a {{ datin0a: {=u32:?} }}", self.datin0a())
    }
}
#[doc = "Input Data register 0 - Word-2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiDatin0b(pub u32);
impl SgiDatin0b {
    #[doc = "Input Data register."]
    #[must_use]
    #[inline(always)]
    pub const fn datin0b(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Data register."]
    #[inline(always)]
    pub const fn set_datin0b(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiDatin0b {
    #[inline(always)]
    fn default() -> SgiDatin0b {
        SgiDatin0b(0)
    }
}
impl core::fmt::Debug for SgiDatin0b {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiDatin0b")
            .field("datin0b", &self.datin0b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiDatin0b {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiDatin0b {{ datin0b: {=u32:?} }}", self.datin0b())
    }
}
#[doc = "Input Data register 0 - Word-1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiDatin0c(pub u32);
impl SgiDatin0c {
    #[doc = "Input Data register."]
    #[must_use]
    #[inline(always)]
    pub const fn datin0c(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Data register."]
    #[inline(always)]
    pub const fn set_datin0c(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiDatin0c {
    #[inline(always)]
    fn default() -> SgiDatin0c {
        SgiDatin0c(0)
    }
}
impl core::fmt::Debug for SgiDatin0c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiDatin0c")
            .field("datin0c", &self.datin0c())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiDatin0c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiDatin0c {{ datin0c: {=u32:?} }}", self.datin0c())
    }
}
#[doc = "Input Data register 0 - Word-0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiDatin0d(pub u32);
impl SgiDatin0d {
    #[doc = "Input Data register."]
    #[must_use]
    #[inline(always)]
    pub const fn datin0d(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Data register."]
    #[inline(always)]
    pub const fn set_datin0d(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiDatin0d {
    #[inline(always)]
    fn default() -> SgiDatin0d {
        SgiDatin0d(0)
    }
}
impl core::fmt::Debug for SgiDatin0d {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiDatin0d")
            .field("datin0d", &self.datin0d())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiDatin0d {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiDatin0d {{ datin0d: {=u32:?} }}", self.datin0d())
    }
}
#[doc = "Input Data register 1 - Word-3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiDatin1a(pub u32);
impl SgiDatin1a {
    #[doc = "Input Data register."]
    #[must_use]
    #[inline(always)]
    pub const fn datin1a(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Data register."]
    #[inline(always)]
    pub const fn set_datin1a(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiDatin1a {
    #[inline(always)]
    fn default() -> SgiDatin1a {
        SgiDatin1a(0)
    }
}
impl core::fmt::Debug for SgiDatin1a {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiDatin1a")
            .field("datin1a", &self.datin1a())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiDatin1a {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiDatin1a {{ datin1a: {=u32:?} }}", self.datin1a())
    }
}
#[doc = "Input Data register 1 - Word-2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiDatin1b(pub u32);
impl SgiDatin1b {
    #[doc = "Input Data register."]
    #[must_use]
    #[inline(always)]
    pub const fn datin1b(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Data register."]
    #[inline(always)]
    pub const fn set_datin1b(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiDatin1b {
    #[inline(always)]
    fn default() -> SgiDatin1b {
        SgiDatin1b(0)
    }
}
impl core::fmt::Debug for SgiDatin1b {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiDatin1b")
            .field("datin1b", &self.datin1b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiDatin1b {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiDatin1b {{ datin1b: {=u32:?} }}", self.datin1b())
    }
}
#[doc = "Input Data register 1 - Word-1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiDatin1c(pub u32);
impl SgiDatin1c {
    #[doc = "Input Data register."]
    #[must_use]
    #[inline(always)]
    pub const fn datin1c(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Data register."]
    #[inline(always)]
    pub const fn set_datin1c(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiDatin1c {
    #[inline(always)]
    fn default() -> SgiDatin1c {
        SgiDatin1c(0)
    }
}
impl core::fmt::Debug for SgiDatin1c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiDatin1c")
            .field("datin1c", &self.datin1c())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiDatin1c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiDatin1c {{ datin1c: {=u32:?} }}", self.datin1c())
    }
}
#[doc = "Input Data register 1 - Word-0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiDatin1d(pub u32);
impl SgiDatin1d {
    #[doc = "Input Data register."]
    #[must_use]
    #[inline(always)]
    pub const fn datin1d(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Data register."]
    #[inline(always)]
    pub const fn set_datin1d(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiDatin1d {
    #[inline(always)]
    fn default() -> SgiDatin1d {
        SgiDatin1d(0)
    }
}
impl core::fmt::Debug for SgiDatin1d {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiDatin1d")
            .field("datin1d", &self.datin1d())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiDatin1d {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiDatin1d {{ datin1d: {=u32:?} }}", self.datin1d())
    }
}
#[doc = "Input Data register 2 - Word-3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiDatin2a(pub u32);
impl SgiDatin2a {
    #[doc = "Input Data register."]
    #[must_use]
    #[inline(always)]
    pub const fn datin2a(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Data register."]
    #[inline(always)]
    pub const fn set_datin2a(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiDatin2a {
    #[inline(always)]
    fn default() -> SgiDatin2a {
        SgiDatin2a(0)
    }
}
impl core::fmt::Debug for SgiDatin2a {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiDatin2a")
            .field("datin2a", &self.datin2a())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiDatin2a {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiDatin2a {{ datin2a: {=u32:?} }}", self.datin2a())
    }
}
#[doc = "Input Data register 2 - Word-2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiDatin2b(pub u32);
impl SgiDatin2b {
    #[doc = "Input Data register."]
    #[must_use]
    #[inline(always)]
    pub const fn datin2b(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Data register."]
    #[inline(always)]
    pub const fn set_datin2b(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiDatin2b {
    #[inline(always)]
    fn default() -> SgiDatin2b {
        SgiDatin2b(0)
    }
}
impl core::fmt::Debug for SgiDatin2b {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiDatin2b")
            .field("datin2b", &self.datin2b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiDatin2b {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiDatin2b {{ datin2b: {=u32:?} }}", self.datin2b())
    }
}
#[doc = "Input Data register 2 - Word-1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiDatin2c(pub u32);
impl SgiDatin2c {
    #[doc = "Input Data register."]
    #[must_use]
    #[inline(always)]
    pub const fn datin2c(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Data register."]
    #[inline(always)]
    pub const fn set_datin2c(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiDatin2c {
    #[inline(always)]
    fn default() -> SgiDatin2c {
        SgiDatin2c(0)
    }
}
impl core::fmt::Debug for SgiDatin2c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiDatin2c")
            .field("datin2c", &self.datin2c())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiDatin2c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiDatin2c {{ datin2c: {=u32:?} }}", self.datin2c())
    }
}
#[doc = "Input Data register 2 - Word-0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiDatin2d(pub u32);
impl SgiDatin2d {
    #[doc = "Input Data register."]
    #[must_use]
    #[inline(always)]
    pub const fn datin2d(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Data register."]
    #[inline(always)]
    pub const fn set_datin2d(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiDatin2d {
    #[inline(always)]
    fn default() -> SgiDatin2d {
        SgiDatin2d(0)
    }
}
impl core::fmt::Debug for SgiDatin2d {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiDatin2d")
            .field("datin2d", &self.datin2d())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiDatin2d {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiDatin2d {{ datin2d: {=u32:?} }}", self.datin2d())
    }
}
#[doc = "Input Data register 3 - Word-3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiDatin3a(pub u32);
impl SgiDatin3a {
    #[doc = "Input Data register."]
    #[must_use]
    #[inline(always)]
    pub const fn datin3a(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Data register."]
    #[inline(always)]
    pub const fn set_datin3a(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiDatin3a {
    #[inline(always)]
    fn default() -> SgiDatin3a {
        SgiDatin3a(0)
    }
}
impl core::fmt::Debug for SgiDatin3a {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiDatin3a")
            .field("datin3a", &self.datin3a())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiDatin3a {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiDatin3a {{ datin3a: {=u32:?} }}", self.datin3a())
    }
}
#[doc = "Input Data register 3 - Word-2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiDatin3b(pub u32);
impl SgiDatin3b {
    #[doc = "Input Data register."]
    #[must_use]
    #[inline(always)]
    pub const fn datin3b(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Data register."]
    #[inline(always)]
    pub const fn set_datin3b(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiDatin3b {
    #[inline(always)]
    fn default() -> SgiDatin3b {
        SgiDatin3b(0)
    }
}
impl core::fmt::Debug for SgiDatin3b {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiDatin3b")
            .field("datin3b", &self.datin3b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiDatin3b {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiDatin3b {{ datin3b: {=u32:?} }}", self.datin3b())
    }
}
#[doc = "Input Data register 3 - Word-1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiDatin3c(pub u32);
impl SgiDatin3c {
    #[doc = "Input Data register."]
    #[must_use]
    #[inline(always)]
    pub const fn datin3c(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Data register."]
    #[inline(always)]
    pub const fn set_datin3c(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiDatin3c {
    #[inline(always)]
    fn default() -> SgiDatin3c {
        SgiDatin3c(0)
    }
}
impl core::fmt::Debug for SgiDatin3c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiDatin3c")
            .field("datin3c", &self.datin3c())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiDatin3c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiDatin3c {{ datin3c: {=u32:?} }}", self.datin3c())
    }
}
#[doc = "Input Data register 3 - Word-0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiDatin3d(pub u32);
impl SgiDatin3d {
    #[doc = "Input Data register."]
    #[must_use]
    #[inline(always)]
    pub const fn datin3d(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Data register."]
    #[inline(always)]
    pub const fn set_datin3d(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiDatin3d {
    #[inline(always)]
    fn default() -> SgiDatin3d {
        SgiDatin3d(0)
    }
}
impl core::fmt::Debug for SgiDatin3d {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiDatin3d")
            .field("datin3d", &self.datin3d())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiDatin3d {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiDatin3d {{ datin3d: {=u32:?} }}", self.datin3d())
    }
}
#[doc = "Output Data register - Word-3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiDatouta(pub u32);
impl SgiDatouta {
    #[doc = "Output Data register."]
    #[must_use]
    #[inline(always)]
    pub const fn datouta(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Output Data register."]
    #[inline(always)]
    pub const fn set_datouta(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiDatouta {
    #[inline(always)]
    fn default() -> SgiDatouta {
        SgiDatouta(0)
    }
}
impl core::fmt::Debug for SgiDatouta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiDatouta")
            .field("datouta", &self.datouta())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiDatouta {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiDatouta {{ datouta: {=u32:?} }}", self.datouta())
    }
}
#[doc = "Output Data register - Word-2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiDatoutb(pub u32);
impl SgiDatoutb {
    #[doc = "Output Data register."]
    #[must_use]
    #[inline(always)]
    pub const fn datoutb(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Output Data register."]
    #[inline(always)]
    pub const fn set_datoutb(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiDatoutb {
    #[inline(always)]
    fn default() -> SgiDatoutb {
        SgiDatoutb(0)
    }
}
impl core::fmt::Debug for SgiDatoutb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiDatoutb")
            .field("datoutb", &self.datoutb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiDatoutb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiDatoutb {{ datoutb: {=u32:?} }}", self.datoutb())
    }
}
#[doc = "Output Data register - Word-1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiDatoutc(pub u32);
impl SgiDatoutc {
    #[doc = "Output Data register."]
    #[must_use]
    #[inline(always)]
    pub const fn datoutc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Output Data register."]
    #[inline(always)]
    pub const fn set_datoutc(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiDatoutc {
    #[inline(always)]
    fn default() -> SgiDatoutc {
        SgiDatoutc(0)
    }
}
impl core::fmt::Debug for SgiDatoutc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiDatoutc")
            .field("datoutc", &self.datoutc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiDatoutc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiDatoutc {{ datoutc: {=u32:?} }}", self.datoutc())
    }
}
#[doc = "Output Data register - Word-0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiDatoutd(pub u32);
impl SgiDatoutd {
    #[doc = "Output Data register."]
    #[must_use]
    #[inline(always)]
    pub const fn datoutd(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Output Data register."]
    #[inline(always)]
    pub const fn set_datoutd(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiDatoutd {
    #[inline(always)]
    fn default() -> SgiDatoutd {
        SgiDatoutd(0)
    }
}
impl core::fmt::Debug for SgiDatoutd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiDatoutd")
            .field("datoutd", &self.datoutd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiDatoutd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiDatoutd {{ datoutd: {=u32:?} }}", self.datoutd())
    }
}
#[doc = "Configuration of dummy controls."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiDummyCtrl(pub u32);
impl SgiDummyCtrl {
    #[doc = "DES dummy controlPlease refer to the relevant kernel document for details on dummy ctrl."]
    #[must_use]
    #[inline(always)]
    pub const fn ddctrl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "DES dummy controlPlease refer to the relevant kernel document for details on dummy ctrl."]
    #[inline(always)]
    pub const fn set_ddctrl(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "AES dummy controlPlease refer to the relevant kernel document for details on dummy ctrl."]
    #[must_use]
    #[inline(always)]
    pub const fn adctrl(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "AES dummy controlPlease refer to the relevant kernel document for details on dummy ctrl."]
    #[inline(always)]
    pub const fn set_adctrl(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for SgiDummyCtrl {
    #[inline(always)]
    fn default() -> SgiDummyCtrl {
        SgiDummyCtrl(0)
    }
}
impl core::fmt::Debug for SgiDummyCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiDummyCtrl")
            .field("ddctrl", &self.ddctrl())
            .field("adctrl", &self.adctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiDummyCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SgiDummyCtrl {{ ddctrl: {=u16:?}, adctrl: {=u16:?} }}",
            self.ddctrl(),
            self.adctrl()
        )
    }
}
#[doc = "Interrupt enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiIntEnable(pub u32);
impl SgiIntEnable {
    #[doc = "Interrupt enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn int_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt enable bit."]
    #[inline(always)]
    pub const fn set_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for SgiIntEnable {
    #[inline(always)]
    fn default() -> SgiIntEnable {
        SgiIntEnable(0)
    }
}
impl core::fmt::Debug for SgiIntEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiIntEnable")
            .field("int_en", &self.int_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiIntEnable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiIntEnable {{ int_en: {=bool:?} }}", self.int_en())
    }
}
#[doc = "Interrupt status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiIntStatus(pub u32);
impl SgiIntStatus {
    #[doc = "Interrupt status flag: INT_PDONE is set independent from the interrupt enable SGI_INT_ENABLE."]
    #[must_use]
    #[inline(always)]
    pub const fn int_pdone(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status flag: INT_PDONE is set independent from the interrupt enable SGI_INT_ENABLE."]
    #[inline(always)]
    pub const fn set_int_pdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for SgiIntStatus {
    #[inline(always)]
    fn default() -> SgiIntStatus {
        SgiIntStatus(0)
    }
}
impl core::fmt::Debug for SgiIntStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiIntStatus")
            .field("int_pdone", &self.int_pdone())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiIntStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SgiIntStatus {{ int_pdone: {=bool:?} }}",
            self.int_pdone()
        )
    }
}
#[doc = "Interrupt status clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiIntStatusClr(pub u32);
impl SgiIntStatusClr {
    #[doc = "Write to clear interrupt status flag (SGI_INT_STATUS.INT_PDONE=0)."]
    #[must_use]
    #[inline(always)]
    pub const fn int_clr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write to clear interrupt status flag (SGI_INT_STATUS.INT_PDONE=0)."]
    #[inline(always)]
    pub const fn set_int_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for SgiIntStatusClr {
    #[inline(always)]
    fn default() -> SgiIntStatusClr {
        SgiIntStatusClr(0)
    }
}
impl core::fmt::Debug for SgiIntStatusClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiIntStatusClr")
            .field("int_clr", &self.int_clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiIntStatusClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SgiIntStatusClr {{ int_clr: {=bool:?} }}",
            self.int_clr()
        )
    }
}
#[doc = "Interrupt status set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiIntStatusSet(pub u32);
impl SgiIntStatusSet {
    #[doc = "Write to set interrupt status flag (SGI_INT_STATUS.INT_PDONE=1) to trigger a SGI interrupt via software, e.g. for debug purposes."]
    #[must_use]
    #[inline(always)]
    pub const fn int_set(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write to set interrupt status flag (SGI_INT_STATUS.INT_PDONE=1) to trigger a SGI interrupt via software, e.g. for debug purposes."]
    #[inline(always)]
    pub const fn set_int_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for SgiIntStatusSet {
    #[inline(always)]
    fn default() -> SgiIntStatusSet {
        SgiIntStatusSet(0)
    }
}
impl core::fmt::Debug for SgiIntStatusSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiIntStatusSet")
            .field("int_set", &self.int_set())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiIntStatusSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SgiIntStatusSet {{ int_set: {=bool:?} }}",
            self.int_set()
        )
    }
}
#[doc = "Input Key register 0 - Word-3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey0a(pub u32);
impl SgiKey0a {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key0a(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key0a(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey0a {
    #[inline(always)]
    fn default() -> SgiKey0a {
        SgiKey0a(0)
    }
}
impl core::fmt::Debug for SgiKey0a {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey0a")
            .field("key0a", &self.key0a())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey0a {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey0a {{ key0a: {=u32:?} }}", self.key0a())
    }
}
#[doc = "Input Key register 0 - Word-2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey0b(pub u32);
impl SgiKey0b {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key0b(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key0b(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey0b {
    #[inline(always)]
    fn default() -> SgiKey0b {
        SgiKey0b(0)
    }
}
impl core::fmt::Debug for SgiKey0b {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey0b")
            .field("key0b", &self.key0b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey0b {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey0b {{ key0b: {=u32:?} }}", self.key0b())
    }
}
#[doc = "Input Key register 0 - Word-1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey0c(pub u32);
impl SgiKey0c {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key0c(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key0c(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey0c {
    #[inline(always)]
    fn default() -> SgiKey0c {
        SgiKey0c(0)
    }
}
impl core::fmt::Debug for SgiKey0c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey0c")
            .field("key0c", &self.key0c())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey0c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey0c {{ key0c: {=u32:?} }}", self.key0c())
    }
}
#[doc = "Input Key register 0 - Word-0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey0d(pub u32);
impl SgiKey0d {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key0d(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key0d(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey0d {
    #[inline(always)]
    fn default() -> SgiKey0d {
        SgiKey0d(0)
    }
}
impl core::fmt::Debug for SgiKey0d {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey0d")
            .field("key0d", &self.key0d())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey0d {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey0d {{ key0d: {=u32:?} }}", self.key0d())
    }
}
#[doc = "Input Key register 1 - Word-3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey1a(pub u32);
impl SgiKey1a {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key1a(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key1a(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey1a {
    #[inline(always)]
    fn default() -> SgiKey1a {
        SgiKey1a(0)
    }
}
impl core::fmt::Debug for SgiKey1a {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey1a")
            .field("key1a", &self.key1a())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey1a {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey1a {{ key1a: {=u32:?} }}", self.key1a())
    }
}
#[doc = "Input Key register 1 - Word-2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey1b(pub u32);
impl SgiKey1b {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key1b(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key1b(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey1b {
    #[inline(always)]
    fn default() -> SgiKey1b {
        SgiKey1b(0)
    }
}
impl core::fmt::Debug for SgiKey1b {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey1b")
            .field("key1b", &self.key1b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey1b {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey1b {{ key1b: {=u32:?} }}", self.key1b())
    }
}
#[doc = "Input Key register 1 - Word-1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey1c(pub u32);
impl SgiKey1c {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key1c(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key1c(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey1c {
    #[inline(always)]
    fn default() -> SgiKey1c {
        SgiKey1c(0)
    }
}
impl core::fmt::Debug for SgiKey1c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey1c")
            .field("key1c", &self.key1c())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey1c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey1c {{ key1c: {=u32:?} }}", self.key1c())
    }
}
#[doc = "Input Key register 1 - Word-0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey1d(pub u32);
impl SgiKey1d {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key1d(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key1d(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey1d {
    #[inline(always)]
    fn default() -> SgiKey1d {
        SgiKey1d(0)
    }
}
impl core::fmt::Debug for SgiKey1d {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey1d")
            .field("key1d", &self.key1d())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey1d {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey1d {{ key1d: {=u32:?} }}", self.key1d())
    }
}
#[doc = "Input Key register 2 - Word-3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey2a(pub u32);
impl SgiKey2a {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key2a(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key2a(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey2a {
    #[inline(always)]
    fn default() -> SgiKey2a {
        SgiKey2a(0)
    }
}
impl core::fmt::Debug for SgiKey2a {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey2a")
            .field("key2a", &self.key2a())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey2a {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey2a {{ key2a: {=u32:?} }}", self.key2a())
    }
}
#[doc = "Input Key register 2 - Word-2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey2b(pub u32);
impl SgiKey2b {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key2b(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key2b(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey2b {
    #[inline(always)]
    fn default() -> SgiKey2b {
        SgiKey2b(0)
    }
}
impl core::fmt::Debug for SgiKey2b {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey2b")
            .field("key2b", &self.key2b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey2b {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey2b {{ key2b: {=u32:?} }}", self.key2b())
    }
}
#[doc = "Input Key register 2 - Word-1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey2c(pub u32);
impl SgiKey2c {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key2c(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key2c(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey2c {
    #[inline(always)]
    fn default() -> SgiKey2c {
        SgiKey2c(0)
    }
}
impl core::fmt::Debug for SgiKey2c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey2c")
            .field("key2c", &self.key2c())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey2c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey2c {{ key2c: {=u32:?} }}", self.key2c())
    }
}
#[doc = "Input Key register 2 - Word-0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey2d(pub u32);
impl SgiKey2d {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key2d(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key2d(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey2d {
    #[inline(always)]
    fn default() -> SgiKey2d {
        SgiKey2d(0)
    }
}
impl core::fmt::Debug for SgiKey2d {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey2d")
            .field("key2d", &self.key2d())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey2d {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey2d {{ key2d: {=u32:?} }}", self.key2d())
    }
}
#[doc = "Input Key register 3 - Word-3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey3a(pub u32);
impl SgiKey3a {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key3a(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key3a(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey3a {
    #[inline(always)]
    fn default() -> SgiKey3a {
        SgiKey3a(0)
    }
}
impl core::fmt::Debug for SgiKey3a {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey3a")
            .field("key3a", &self.key3a())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey3a {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey3a {{ key3a: {=u32:?} }}", self.key3a())
    }
}
#[doc = "Input Key register 3 - Word-2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey3b(pub u32);
impl SgiKey3b {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key3b(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key3b(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey3b {
    #[inline(always)]
    fn default() -> SgiKey3b {
        SgiKey3b(0)
    }
}
impl core::fmt::Debug for SgiKey3b {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey3b")
            .field("key3b", &self.key3b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey3b {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey3b {{ key3b: {=u32:?} }}", self.key3b())
    }
}
#[doc = "Input Key register 3 - Word-1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey3c(pub u32);
impl SgiKey3c {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key3c(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key3c(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey3c {
    #[inline(always)]
    fn default() -> SgiKey3c {
        SgiKey3c(0)
    }
}
impl core::fmt::Debug for SgiKey3c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey3c")
            .field("key3c", &self.key3c())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey3c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey3c {{ key3c: {=u32:?} }}", self.key3c())
    }
}
#[doc = "Input Key register 3 - Word-0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey3d(pub u32);
impl SgiKey3d {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key3d(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key3d(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey3d {
    #[inline(always)]
    fn default() -> SgiKey3d {
        SgiKey3d(0)
    }
}
impl core::fmt::Debug for SgiKey3d {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey3d")
            .field("key3d", &self.key3d())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey3d {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey3d {{ key3d: {=u32:?} }}", self.key3d())
    }
}
#[doc = "Input Key register 4 - Word-3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey4a(pub u32);
impl SgiKey4a {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key4a(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key4a(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey4a {
    #[inline(always)]
    fn default() -> SgiKey4a {
        SgiKey4a(0)
    }
}
impl core::fmt::Debug for SgiKey4a {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey4a")
            .field("key4a", &self.key4a())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey4a {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey4a {{ key4a: {=u32:?} }}", self.key4a())
    }
}
#[doc = "Input Key register 4 - Word-2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey4b(pub u32);
impl SgiKey4b {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key4b(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key4b(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey4b {
    #[inline(always)]
    fn default() -> SgiKey4b {
        SgiKey4b(0)
    }
}
impl core::fmt::Debug for SgiKey4b {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey4b")
            .field("key4b", &self.key4b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey4b {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey4b {{ key4b: {=u32:?} }}", self.key4b())
    }
}
#[doc = "Input Key register 4 - Word-1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey4c(pub u32);
impl SgiKey4c {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key4c(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key4c(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey4c {
    #[inline(always)]
    fn default() -> SgiKey4c {
        SgiKey4c(0)
    }
}
impl core::fmt::Debug for SgiKey4c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey4c")
            .field("key4c", &self.key4c())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey4c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey4c {{ key4c: {=u32:?} }}", self.key4c())
    }
}
#[doc = "Input Key register 4 - Word-0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey4d(pub u32);
impl SgiKey4d {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key4d(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key4d(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey4d {
    #[inline(always)]
    fn default() -> SgiKey4d {
        SgiKey4d(0)
    }
}
impl core::fmt::Debug for SgiKey4d {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey4d")
            .field("key4d", &self.key4d())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey4d {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey4d {{ key4d: {=u32:?} }}", self.key4d())
    }
}
#[doc = "Input Key register 5 - Word-3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey5a(pub u32);
impl SgiKey5a {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key5a(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key5a(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey5a {
    #[inline(always)]
    fn default() -> SgiKey5a {
        SgiKey5a(0)
    }
}
impl core::fmt::Debug for SgiKey5a {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey5a")
            .field("key5a", &self.key5a())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey5a {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey5a {{ key5a: {=u32:?} }}", self.key5a())
    }
}
#[doc = "Input Key register 5 - Word-2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey5b(pub u32);
impl SgiKey5b {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key5b(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key5b(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey5b {
    #[inline(always)]
    fn default() -> SgiKey5b {
        SgiKey5b(0)
    }
}
impl core::fmt::Debug for SgiKey5b {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey5b")
            .field("key5b", &self.key5b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey5b {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey5b {{ key5b: {=u32:?} }}", self.key5b())
    }
}
#[doc = "Input Key register 5 - Word-1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey5c(pub u32);
impl SgiKey5c {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key5c(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key5c(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey5c {
    #[inline(always)]
    fn default() -> SgiKey5c {
        SgiKey5c(0)
    }
}
impl core::fmt::Debug for SgiKey5c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey5c")
            .field("key5c", &self.key5c())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey5c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey5c {{ key5c: {=u32:?} }}", self.key5c())
    }
}
#[doc = "Input Key register 5 - Word-0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey5d(pub u32);
impl SgiKey5d {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key5d(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key5d(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey5d {
    #[inline(always)]
    fn default() -> SgiKey5d {
        SgiKey5d(0)
    }
}
impl core::fmt::Debug for SgiKey5d {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey5d")
            .field("key5d", &self.key5d())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey5d {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey5d {{ key5d: {=u32:?} }}", self.key5d())
    }
}
#[doc = "Input Key register 6 - Word-3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey6a(pub u32);
impl SgiKey6a {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key6a(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key6a(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey6a {
    #[inline(always)]
    fn default() -> SgiKey6a {
        SgiKey6a(0)
    }
}
impl core::fmt::Debug for SgiKey6a {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey6a")
            .field("key6a", &self.key6a())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey6a {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey6a {{ key6a: {=u32:?} }}", self.key6a())
    }
}
#[doc = "Input Key register 6 - Word-2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey6b(pub u32);
impl SgiKey6b {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key6b(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key6b(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey6b {
    #[inline(always)]
    fn default() -> SgiKey6b {
        SgiKey6b(0)
    }
}
impl core::fmt::Debug for SgiKey6b {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey6b")
            .field("key6b", &self.key6b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey6b {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey6b {{ key6b: {=u32:?} }}", self.key6b())
    }
}
#[doc = "Input Key register 6 - Word-1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey6c(pub u32);
impl SgiKey6c {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key6c(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key6c(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey6c {
    #[inline(always)]
    fn default() -> SgiKey6c {
        SgiKey6c(0)
    }
}
impl core::fmt::Debug for SgiKey6c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey6c")
            .field("key6c", &self.key6c())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey6c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey6c {{ key6c: {=u32:?} }}", self.key6c())
    }
}
#[doc = "Input Key register 6 - Word-0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey6d(pub u32);
impl SgiKey6d {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key6d(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key6d(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey6d {
    #[inline(always)]
    fn default() -> SgiKey6d {
        SgiKey6d(0)
    }
}
impl core::fmt::Debug for SgiKey6d {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey6d")
            .field("key6d", &self.key6d())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey6d {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey6d {{ key6d: {=u32:?} }}", self.key6d())
    }
}
#[doc = "Input Key register 7 - Word-3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey7a(pub u32);
impl SgiKey7a {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key7a(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key7a(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey7a {
    #[inline(always)]
    fn default() -> SgiKey7a {
        SgiKey7a(0)
    }
}
impl core::fmt::Debug for SgiKey7a {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey7a")
            .field("key7a", &self.key7a())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey7a {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey7a {{ key7a: {=u32:?} }}", self.key7a())
    }
}
#[doc = "Input Key register 7 - Word-2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey7b(pub u32);
impl SgiKey7b {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key7b(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key7b(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey7b {
    #[inline(always)]
    fn default() -> SgiKey7b {
        SgiKey7b(0)
    }
}
impl core::fmt::Debug for SgiKey7b {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey7b")
            .field("key7b", &self.key7b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey7b {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey7b {{ key7b: {=u32:?} }}", self.key7b())
    }
}
#[doc = "Input Key register 7 - Word-1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey7c(pub u32);
impl SgiKey7c {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key7c(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key7c(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey7c {
    #[inline(always)]
    fn default() -> SgiKey7c {
        SgiKey7c(0)
    }
}
impl core::fmt::Debug for SgiKey7c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey7c")
            .field("key7c", &self.key7c())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey7c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey7c {{ key7c: {=u32:?} }}", self.key7c())
    }
}
#[doc = "Input Key register 7 - Word-0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKey7d(pub u32);
impl SgiKey7d {
    #[doc = "Input Key register."]
    #[must_use]
    #[inline(always)]
    pub const fn key7d(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input Key register."]
    #[inline(always)]
    pub const fn set_key7d(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKey7d {
    #[inline(always)]
    fn default() -> SgiKey7d {
        SgiKey7d(0)
    }
}
impl core::fmt::Debug for SgiKey7d {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKey7d")
            .field("key7d", &self.key7d())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKey7d {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKey7d {{ key7d: {=u32:?} }}", self.key7d())
    }
}
#[doc = "SGI Key Control SFR."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKeyCtrl(pub u32);
impl SgiKeyCtrl {
    #[doc = "SGI Key control register(1-bit per KEY SFR) 1'b0 - Key SFR is readable 1'b1 - Key SFR is not-readable(write-only)."]
    #[must_use]
    #[inline(always)]
    pub const fn key_wo(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SGI Key control register(1-bit per KEY SFR) 1'b0 - Key SFR is readable 1'b1 - Key SFR is not-readable(write-only)."]
    #[inline(always)]
    pub const fn set_key_wo(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKeyCtrl {
    #[inline(always)]
    fn default() -> SgiKeyCtrl {
        SgiKeyCtrl(0)
    }
}
impl core::fmt::Debug for SgiKeyCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKeyCtrl")
            .field("key_wo", &self.key_wo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKeyCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKeyCtrl {{ key_wo: {=u32:?} }}", self.key_wo())
    }
}
#[doc = "Wrapped key read SFR."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKeyWrap(pub u32);
impl SgiKeyWrap {
    #[doc = "Field contains wrapped key, auto-updated by HW for each word."]
    #[must_use]
    #[inline(always)]
    pub const fn kw_data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Field contains wrapped key, auto-updated by HW for each word."]
    #[inline(always)]
    pub const fn set_kw_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKeyWrap {
    #[inline(always)]
    fn default() -> SgiKeyWrap {
        SgiKeyWrap(0)
    }
}
impl core::fmt::Debug for SgiKeyWrap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKeyWrap")
            .field("kw_data", &self.kw_data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKeyWrap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKeyWrap {{ kw_data: {=u32:?} }}", self.kw_data())
    }
}
#[doc = "Key checksum register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiKeychk(pub u32);
impl SgiKeychk {
    #[doc = "Key checksum (32-bit)."]
    #[must_use]
    #[inline(always)]
    pub const fn keychksum(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Key checksum (32-bit)."]
    #[inline(always)]
    pub const fn set_keychksum(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiKeychk {
    #[inline(always)]
    fn default() -> SgiKeychk {
        SgiKeychk(0)
    }
}
impl core::fmt::Debug for SgiKeychk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiKeychk")
            .field("keychksum", &self.keychksum())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiKeychk {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiKeychk {{ keychksum: {=u32:?} }}", self.keychksum())
    }
}
#[doc = "Module ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiModuleId(pub u32);
impl SgiModuleId {
    #[doc = "Module ID."]
    #[must_use]
    #[inline(always)]
    pub const fn placeholder(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Module ID."]
    #[inline(always)]
    pub const fn set_placeholder(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiModuleId {
    #[inline(always)]
    fn default() -> SgiModuleId {
        SgiModuleId(0)
    }
}
impl core::fmt::Debug for SgiModuleId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiModuleId")
            .field("placeholder", &self.placeholder())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiModuleId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SgiModuleId {{ placeholder: {=u32:?} }}",
            self.placeholder()
        )
    }
}
#[doc = "SGI internal PRNG SW seeding register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiPrngSwSeed(pub u32);
impl SgiPrngSwSeed {
    #[doc = "32-bits SEED field. A write to the SEED field will seed the internal PRNG."]
    #[must_use]
    #[inline(always)]
    pub const fn seed(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "32-bits SEED field. A write to the SEED field will seed the internal PRNG."]
    #[inline(always)]
    pub const fn set_seed(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiPrngSwSeed {
    #[inline(always)]
    fn default() -> SgiPrngSwSeed {
        SgiPrngSwSeed(0)
    }
}
impl core::fmt::Debug for SgiPrngSwSeed {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiPrngSwSeed")
            .field("seed", &self.seed())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiPrngSwSeed {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiPrngSwSeed {{ seed: {=u32:?} }}", self.seed())
    }
}
#[doc = "SFRSEED register for SFRMASK feature."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiSfrseed(pub u32);
impl SgiSfrseed {
    #[doc = "Seed/mask used for sw level masking."]
    #[must_use]
    #[inline(always)]
    pub const fn sfrseed(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Seed/mask used for sw level masking."]
    #[inline(always)]
    pub const fn set_sfrseed(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiSfrseed {
    #[inline(always)]
    fn default() -> SgiSfrseed {
        SgiSfrseed(0)
    }
}
impl core::fmt::Debug for SgiSfrseed {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiSfrseed")
            .field("sfrseed", &self.sfrseed())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiSfrseed {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiSfrseed {{ sfrseed: {=u32:?} }}", self.sfrseed())
    }
}
#[doc = "SHA Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiSha2Ctrl(pub u32);
impl SgiSha2Ctrl {
    #[doc = "SHA enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sha2_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SHA enable."]
    #[inline(always)]
    pub const fn set_sha2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SHA mode normal or automatic."]
    #[must_use]
    #[inline(always)]
    pub const fn sha2_mode(&self) -> Sha2Mode {
        let val = (self.0 >> 1usize) & 0x01;
        Sha2Mode::from_bits(val as u8)
    }
    #[doc = "SHA mode normal or automatic."]
    #[inline(always)]
    pub const fn set_sha2_mode(&mut self, val: Sha2Mode) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates SHA size."]
    #[must_use]
    #[inline(always)]
    pub const fn sha2_size(&self) -> Sha2Size {
        let val = (self.0 >> 2usize) & 0x03;
        Sha2Size::from_bits(val as u8)
    }
    #[doc = "Indicates SHA size."]
    #[inline(always)]
    pub const fn set_sha2_size(&mut self, val: Sha2Size) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "SHA FIFO low limit."]
    #[must_use]
    #[inline(always)]
    pub const fn sha2_low_lim(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "SHA FIFO low limit."]
    #[inline(always)]
    pub const fn set_sha2_low_lim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "SHA FIFO high limit."]
    #[must_use]
    #[inline(always)]
    pub const fn sha2_high_lim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "SHA FIFO high limit."]
    #[inline(always)]
    pub const fn set_sha2_high_lim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "SHA Calculation counter enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sha2_count_en(&self) -> Sha2CountEn {
        let val = (self.0 >> 12usize) & 0x01;
        Sha2CountEn::from_bits(val as u8)
    }
    #[doc = "SHA Calculation counter enable."]
    #[inline(always)]
    pub const fn set_sha2_count_en(&mut self, val: Sha2CountEn) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "SHA HASH reload."]
    #[must_use]
    #[inline(always)]
    pub const fn hash_reload(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SHA HASH reload."]
    #[inline(always)]
    pub const fn set_hash_reload(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "STOP SHA AUTO mode."]
    #[must_use]
    #[inline(always)]
    pub const fn sha2_stop(&self) -> Sha2Stop {
        let val = (self.0 >> 14usize) & 0x01;
        Sha2Stop::from_bits(val as u8)
    }
    #[doc = "STOP SHA AUTO mode."]
    #[inline(always)]
    pub const fn set_sha2_stop(&mut self, val: Sha2Stop) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "SHA no automatic HASH initialisation."]
    #[must_use]
    #[inline(always)]
    pub const fn no_auto_init(&self) -> NoAutoInit {
        let val = (self.0 >> 15usize) & 0x01;
        NoAutoInit::from_bits(val as u8)
    }
    #[doc = "SHA no automatic HASH initialisation."]
    #[inline(always)]
    pub const fn set_no_auto_init(&mut self, val: NoAutoInit) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for SgiSha2Ctrl {
    #[inline(always)]
    fn default() -> SgiSha2Ctrl {
        SgiSha2Ctrl(0)
    }
}
impl core::fmt::Debug for SgiSha2Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiSha2Ctrl")
            .field("sha2_en", &self.sha2_en())
            .field("sha2_mode", &self.sha2_mode())
            .field("sha2_size", &self.sha2_size())
            .field("sha2_low_lim", &self.sha2_low_lim())
            .field("sha2_high_lim", &self.sha2_high_lim())
            .field("sha2_count_en", &self.sha2_count_en())
            .field("hash_reload", &self.hash_reload())
            .field("sha2_stop", &self.sha2_stop())
            .field("no_auto_init", &self.no_auto_init())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiSha2Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SgiSha2Ctrl {{ sha2_en: {=bool:?}, sha2_mode: {:?}, sha2_size: {:?}, sha2_low_lim: {=u8:?}, sha2_high_lim: {=u8:?}, sha2_count_en: {:?}, hash_reload: {=bool:?}, sha2_stop: {:?}, no_auto_init: {:?} }}",
            self.sha2_en(),
            self.sha2_mode(),
            self.sha2_size(),
            self.sha2_low_lim(),
            self.sha2_high_lim(),
            self.sha2_count_en(),
            self.hash_reload(),
            self.sha2_stop(),
            self.no_auto_init()
        )
    }
}
#[doc = "SHA FIFO lower-bank low."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiShaFifo(pub u32);
impl SgiShaFifo {
    #[doc = "SHA FIFO register."]
    #[must_use]
    #[inline(always)]
    pub const fn fifo(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SHA FIFO register."]
    #[inline(always)]
    pub const fn set_fifo(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SgiShaFifo {
    #[inline(always)]
    fn default() -> SgiShaFifo {
        SgiShaFifo(0)
    }
}
impl core::fmt::Debug for SgiShaFifo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiShaFifo")
            .field("fifo", &self.fifo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiShaFifo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SgiShaFifo {{ fifo: {=u32:?} }}", self.fifo())
    }
}
#[doc = "Status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiStatus(pub u32);
impl SgiStatus {
    #[doc = "Combined busy flag that remains high until end of calculation."]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Combined busy flag that remains high until end of calculation."]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Overflow in INCR operation flag."]
    #[must_use]
    #[inline(always)]
    pub const fn oflow(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow in INCR operation flag."]
    #[inline(always)]
    pub const fn set_oflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "prng is ready after boot-up-phase."]
    #[must_use]
    #[inline(always)]
    pub const fn prng_rdy(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "prng is ready after boot-up-phase."]
    #[inline(always)]
    pub const fn set_prng_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Error detected."]
    #[must_use]
    #[inline(always)]
    pub const fn error(&self) -> Error {
        let val = (self.0 >> 3usize) & 0x07;
        Error::from_bits(val as u8)
    }
    #[doc = "Error detected."]
    #[inline(always)]
    pub const fn set_error(&mut self, val: Error) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "SHA2 is busy."]
    #[must_use]
    #[inline(always)]
    pub const fn sha2_busy(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "SHA2 is busy."]
    #[inline(always)]
    pub const fn set_sha2_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "interrupt detected."]
    #[must_use]
    #[inline(always)]
    pub const fn irq(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "interrupt detected."]
    #[inline(always)]
    pub const fn set_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "SHA FIFO is full(operates in SHA AUTO mode)."]
    #[must_use]
    #[inline(always)]
    pub const fn sha_fifo_full(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SHA FIFO is full(operates in SHA AUTO mode)."]
    #[inline(always)]
    pub const fn set_sha_fifo_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SHA FIFO level."]
    #[must_use]
    #[inline(always)]
    pub const fn sha_fifo_level(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x3f;
        val as u8
    }
    #[doc = "SHA FIFO level."]
    #[inline(always)]
    pub const fn set_sha_fifo_level(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u32) & 0x3f) << 9usize);
    }
    #[doc = "SHA ERROR."]
    #[must_use]
    #[inline(always)]
    pub const fn sha_error(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "SHA ERROR."]
    #[inline(always)]
    pub const fn set_sha_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "KEY SFR READ ERROR, sticky, cleared only with reset or flush."]
    #[must_use]
    #[inline(always)]
    pub const fn key_read_err(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "KEY SFR READ ERROR, sticky, cleared only with reset or flush."]
    #[inline(always)]
    pub const fn set_key_read_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "KEY UNWRAP ERROR , sticky, cleared only with reset or flush."]
    #[must_use]
    #[inline(always)]
    pub const fn key_unwrap_err(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "KEY UNWRAP ERROR , sticky, cleared only with reset or flush."]
    #[inline(always)]
    pub const fn set_key_unwrap_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for SgiStatus {
    #[inline(always)]
    fn default() -> SgiStatus {
        SgiStatus(0)
    }
}
impl core::fmt::Debug for SgiStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiStatus")
            .field("busy", &self.busy())
            .field("oflow", &self.oflow())
            .field("prng_rdy", &self.prng_rdy())
            .field("error", &self.error())
            .field("sha2_busy", &self.sha2_busy())
            .field("irq", &self.irq())
            .field("sha_fifo_full", &self.sha_fifo_full())
            .field("sha_fifo_level", &self.sha_fifo_level())
            .field("sha_error", &self.sha_error())
            .field("key_read_err", &self.key_read_err())
            .field("key_unwrap_err", &self.key_unwrap_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SgiStatus {{ busy: {=bool:?}, oflow: {=bool:?}, prng_rdy: {=bool:?}, error: {:?}, sha2_busy: {=bool:?}, irq: {=bool:?}, sha_fifo_full: {=bool:?}, sha_fifo_level: {=u8:?}, sha_error: {=bool:?}, key_read_err: {=bool:?}, key_unwrap_err: {=bool:?} }}",
            self.busy(),
            self.oflow(),
            self.prng_rdy(),
            self.error(),
            self.sha2_busy(),
            self.irq(),
            self.sha_fifo_full(),
            self.sha_fifo_level(),
            self.sha_error(),
            self.key_read_err(),
            self.key_unwrap_err()
        )
    }
}
#[doc = "SGI Version."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SgiVersion(pub u32);
impl SgiVersion {
    #[doc = "Extended revision number in X.Y1Y2.Z, e.g. 1.20.3."]
    #[must_use]
    #[inline(always)]
    pub const fn z(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Extended revision number in X.Y1Y2.Z, e.g. 1.20.3."]
    #[inline(always)]
    pub const fn set_z(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Minor revision number 2 in X.Y1Y2.Z, e.g. 1.20.3."]
    #[must_use]
    #[inline(always)]
    pub const fn y2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor revision number 2 in X.Y1Y2.Z, e.g. 1.20.3."]
    #[inline(always)]
    pub const fn set_y2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Minor revision number 1 in X.Y1Y2.Z, e.g. 1.20.3."]
    #[must_use]
    #[inline(always)]
    pub const fn y1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor revision number 1 in X.Y1Y2.Z, e.g. 1.20.3."]
    #[inline(always)]
    pub const fn set_y1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Major revision number in X.Y1Y2.Z, e.g. 1.20.3."]
    #[must_use]
    #[inline(always)]
    pub const fn x(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Major revision number in X.Y1Y2.Z, e.g. 1.20.3."]
    #[inline(always)]
    pub const fn set_x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Release milestone."]
    #[must_use]
    #[inline(always)]
    pub const fn milestone(&self) -> Milestone {
        let val = (self.0 >> 16usize) & 0x03;
        Milestone::from_bits(val as u8)
    }
    #[doc = "Release milestone."]
    #[inline(always)]
    pub const fn set_milestone(&mut self, val: Milestone) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
}
impl Default for SgiVersion {
    #[inline(always)]
    fn default() -> SgiVersion {
        SgiVersion(0)
    }
}
impl core::fmt::Debug for SgiVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SgiVersion")
            .field("z", &self.z())
            .field("y2", &self.y2())
            .field("y1", &self.y1())
            .field("x", &self.x())
            .field("milestone", &self.milestone())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SgiVersion {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SgiVersion {{ z: {=u8:?}, y2: {=u8:?}, y1: {=u8:?}, x: {=u8:?}, milestone: {:?} }}",
            self.z(),
            self.y2(),
            self.y1(),
            self.x(),
            self.milestone()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AesKeysize {
    #[doc = "128 0nly."]
    AES_128_ONLY = 0x0,
    #[doc = "192 only."]
    AES_192_ONLY = 0x01,
    #[doc = "256 only."]
    AES_256_ONLY = 0x02,
    #[doc = "All key sizes."]
    ALL_KEYSIZE = 0x03,
}
impl AesKeysize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AesKeysize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AesKeysize {
    #[inline(always)]
    fn from(val: u8) -> AesKeysize {
        AesKeysize::from_bits(val)
    }
}
impl From<AesKeysize> for u8 {
    #[inline(always)]
    fn from(val: AesKeysize) -> u8 {
        AesKeysize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AesNoKl {
    #[doc = "new AES key will be loaded."]
    NEW = 0x0,
    #[doc = "No AES key will be loaded, and previously loaded key will be used."]
    NO = 0x01,
}
impl AesNoKl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AesNoKl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AesNoKl {
    #[inline(always)]
    fn from(val: u8) -> AesNoKl {
        AesNoKl::from_bits(val)
    }
}
impl From<AesNoKl> for u8 {
    #[inline(always)]
    fn from(val: AesNoKl) -> u8 {
        AesNoKl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AesSel {
    #[doc = "First AES selected."]
    FIRST_AES = 0x0,
    #[doc = "Second AES selected (when enabled)."]
    SECOND_AES = 0x01,
}
impl AesSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AesSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AesSel {
    #[inline(always)]
    fn from(val: u8) -> AesSel {
        AesSel::from_bits(val)
    }
}
impl From<AesSel> for u8 {
    #[inline(always)]
    fn from(val: AesSel) -> u8 {
        AesSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AesUsed {
    #[doc = "Apollo."]
    APOLLO = 0x0,
    #[doc = "Aegis."]
    AEGIS = 0x01,
    #[doc = "Ayna."]
    AYNA = 0x02,
    #[doc = "Athenium."]
    ATHENIUM = 0x03,
    #[doc = "Ajax."]
    AJAX = 0x04,
    #[doc = "Aegis_hs."]
    AEGIS_HS = 0x05,
    #[doc = "Athenium_hs."]
    ATHENIUM_HS = 0x06,
    #[doc = "ATE."]
    ATE = 0x07,
    #[doc = "ATOM."]
    ATOM = 0x08,
    #[doc = "Asterix."]
    ASTERIX = 0x09,
    #[doc = "RFU."]
    RFU_10 = 0x0a,
    #[doc = "RFU."]
    RFU_11 = 0x0b,
    #[doc = "RFU."]
    RFU_12 = 0x0c,
    #[doc = "RFU."]
    RFU_13 = 0x0d,
    #[doc = "RFU."]
    RFU_14 = 0x0e,
    #[doc = "RFU."]
    RFU_15 = 0x0f,
}
impl AesUsed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AesUsed {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AesUsed {
    #[inline(always)]
    fn from(val: u8) -> AesUsed {
        AesUsed::from_bits(val)
    }
}
impl From<AesUsed> for u8 {
    #[inline(always)]
    fn from(val: AesUsed) -> u8 {
        AesUsed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aeskeysz {
    #[doc = "AES-128."]
    AES_128 = 0x0,
    #[doc = "AES-192."]
    AES_192 = 0x01,
    #[doc = "AES-256."]
    AES_256 = 0x02,
    #[doc = "RFU (defaults to AES-128)."]
    RFU = 0x03,
}
impl Aeskeysz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aeskeysz {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aeskeysz {
    #[inline(always)]
    fn from(val: u8) -> Aeskeysz {
        Aeskeysz::from_bits(val)
    }
}
impl From<Aeskeysz> for u8 {
    #[inline(always)]
    fn from(val: Aeskeysz) -> u8 {
        Aeskeysz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BytesOrder {
    #[doc = "Normal."]
    NORMAL_ORDER = 0x0,
    #[doc = "Swapped."]
    SWAPPED_ORDER = 0x01,
}
impl BytesOrder {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BytesOrder {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BytesOrder {
    #[inline(always)]
    fn from(val: u8) -> BytesOrder {
        BytesOrder::from_bits(val)
    }
}
impl From<BytesOrder> for u8 {
    #[inline(always)]
    fn from(val: BytesOrder) -> u8 {
        BytesOrder::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cmd(u8);
impl Cmd {
    #[doc = "ECB mode."]
    pub const ECB: Self = Self(0x0);
    #[doc = "CTR mode."]
    pub const CTR: Self = Self(0x01);
    #[doc = "CBC mode."]
    pub const CBC: Self = Self(0x02);
    #[doc = "CBCMAC mode."]
    pub const CBCMAC: Self = Self(0x03);
    #[doc = "Key Wrap/Unwrap (128 bit key data)."]
    pub const WRAP_128_BIT: Self = Self(0x10);
    #[doc = "Key Wrap/Unwrap (256 bit key data)."]
    pub const WRAP_256_BIT: Self = Self(0x11);
}
impl Cmd {
    pub const fn from_bits(val: u8) -> Cmd {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Cmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ECB"),
            0x01 => f.write_str("CTR"),
            0x02 => f.write_str("CBC"),
            0x03 => f.write_str("CBCMAC"),
            0x10 => f.write_str("WRAP_128_BIT"),
            0x11 => f.write_str("WRAP_256_BIT"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ECB"),
            0x01 => defmt::write!(f, "CTR"),
            0x02 => defmt::write!(f, "CBC"),
            0x03 => defmt::write!(f, "CBCMAC"),
            0x10 => defmt::write!(f, "WRAP_128_BIT"),
            0x11 => defmt::write!(f, "WRAP_256_BIT"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Cmd {
    #[inline(always)]
    fn from(val: u8) -> Cmd {
        Cmd::from_bits(val)
    }
}
impl From<Cmd> for u8 {
    #[inline(always)]
    fn from(val: Cmd) -> u8 {
        Cmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CryptoOp {
    #[doc = "AES."]
    AES = 0x0,
    #[doc = "DES (If Included)."]
    DES = 0x01,
    #[doc = "TDES (If Included)."]
    TDES = 0x02,
    #[doc = "GFMUL(If Included)."]
    GFMUL = 0x03,
    #[doc = "SHA2 (If Included)."]
    SHA = 0x04,
    #[doc = "CMAC (If Included)."]
    CMAC = 0x05,
    #[doc = "others - RFU (Defaults to 1st available OP)."]
    OTHERS_6 = 0x06,
    #[doc = "others - RFU (Defaults to 1st available OP)."]
    OTHERS_7 = 0x07,
}
impl CryptoOp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CryptoOp {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CryptoOp {
    #[inline(always)]
    fn from(val: u8) -> CryptoOp {
        CryptoOp::from_bits(val)
    }
}
impl From<CryptoOp> for u8 {
    #[inline(always)]
    fn from(val: CryptoOp) -> u8 {
        CryptoOp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DatinFlush {
    #[doc = "Clr has no effect."]
    NO_EFFECT = 0x0,
    #[doc = "Set to start flush."]
    FLUSH = 0x01,
}
impl DatinFlush {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DatinFlush {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DatinFlush {
    #[inline(always)]
    fn from(val: u8) -> DatinFlush {
        DatinFlush::from_bits(val)
    }
}
impl From<DatinFlush> for u8 {
    #[inline(always)]
    fn from(val: DatinFlush) -> u8 {
        DatinFlush::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DatoutRes {
    #[doc = "END_UP."]
    END_UP = 0x0,
    #[doc = "START_UP."]
    START_UP = 0x01,
    #[doc = "TRIGGER_UP."]
    TRIGGER_UP = 0x02,
    #[doc = "NO_UP."]
    NO_UP = 0x03,
}
impl DatoutRes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DatoutRes {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DatoutRes {
    #[inline(always)]
    fn from(val: u8) -> DatoutRes {
        DatoutRes::from_bits(val)
    }
}
impl From<DatoutRes> for u8 {
    #[inline(always)]
    fn from(val: DatoutRes) -> u8 {
        DatoutRes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Decrypt {
    #[doc = "Encryption."]
    ENC = 0x0,
    #[doc = "Decryption."]
    DEC = 0x01,
}
impl Decrypt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Decrypt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Decrypt {
    #[inline(always)]
    fn from(val: u8) -> Decrypt {
        Decrypt::from_bits(val)
    }
}
impl From<Decrypt> for u8 {
    #[inline(always)]
    fn from(val: Decrypt) -> u8 {
        Decrypt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DesUsed {
    #[doc = "Dakar."]
    DAKAR = 0x0,
    #[doc = "Danube."]
    DANUBE = 0x01,
    #[doc = "Depicta."]
    DEPICTA = 0x02,
    #[doc = "Digi."]
    DIGI = 0x03,
    #[doc = "Date."]
    DATE = 0x04,
    #[doc = "Desert."]
    DESERT = 0x05,
    #[doc = "RFU."]
    RFU_6 = 0x06,
    #[doc = "RFU."]
    RFU_7 = 0x07,
    #[doc = "RFU."]
    RFU_8 = 0x08,
    #[doc = "RFU."]
    RFU_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl DesUsed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DesUsed {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DesUsed {
    #[inline(always)]
    fn from(val: u8) -> DesUsed {
        DesUsed::from_bits(val)
    }
}
impl From<DesUsed> for u8 {
    #[inline(always)]
    fn from(val: DesUsed) -> u8 {
        DesUsed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    #[doc = "ERROR (all values other than 0x05 indicate ERROR)."]
    ERROR = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "NO_ERROR."]
    NO_ERROR = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Error {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Error {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Error {
    #[inline(always)]
    fn from(val: u8) -> Error {
        Error::from_bits(val)
    }
}
impl From<Error> for u8 {
    #[inline(always)]
    fn from(val: Error) -> u8 {
        Error::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flush {
    #[doc = "Clr has no effect."]
    NO_EFFECT = 0x0,
    #[doc = "Set to start flush."]
    FLUSH = 0x01,
}
impl Flush {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flush {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flush {
    #[inline(always)]
    fn from(val: u8) -> Flush {
        Flush::from_bits(val)
    }
}
impl From<Flush> for u8 {
    #[inline(always)]
    fn from(val: Flush) -> u8 {
        Flush::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IncrCin {
    #[doc = "Carry-In for INCR is 1."]
    INCR_ONE = 0x0,
    #[doc = "Carry-In for INCR is overflow from previous INCR operation."]
    INCR_PREVIOUS = 0x01,
}
impl IncrCin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IncrCin {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IncrCin {
    #[inline(always)]
    fn from(val: u8) -> IncrCin {
        IncrCin::from_bits(val)
    }
}
impl From<IncrCin> for u8 {
    #[inline(always)]
    fn from(val: IncrCin) -> u8 {
        IncrCin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IncrMode {
    #[doc = "2**32 increment mode."]
    INCR_MODE_32 = 0x0,
    #[doc = "2**64 increment mode."]
    INCR_MODE_64 = 0x01,
    #[doc = "2**96 increment mode."]
    INCR_MODE_96 = 0x02,
    #[doc = "2**128 increment mode."]
    INCR_MODE_128 = 0x03,
}
impl IncrMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IncrMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IncrMode {
    #[inline(always)]
    fn from(val: u8) -> IncrMode {
        IncrMode::from_bits(val)
    }
}
impl From<IncrMode> for u8 {
    #[inline(always)]
    fn from(val: IncrMode) -> u8 {
        IncrMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Insel {
    #[doc = "DATIN\\[0\\]."]
    DATIN0 = 0x0,
    #[doc = "DATIN\\[1\\]*."]
    DATIN1 = 0x01,
    #[doc = "DATIN\\[2\\]*."]
    DATIN2 = 0x02,
    #[doc = "DATIN\\[3\\]*."]
    DATIN3 = 0x03,
    #[doc = "DATIN\\[0\\] ^ DATOUT."]
    DATIN0_DATOUT = 0x04,
    #[doc = "DATIN\\[1\\] ^ DATOUT*."]
    DATIN1_DATOUT = 0x05,
    #[doc = "DATIN\\[2\\] ^ DATOUT*."]
    DATIN2_DATOUT = 0x06,
    #[doc = "DATIN\\[3\\] ^ DATOUT*."]
    DATIN3_DATOUT = 0x07,
    #[doc = "DATOUT."]
    DATOUT = 0x08,
    #[doc = "others - DATIN\\[0\\] * - only if DATIN\\[num\\] exists, else \\[0\\]."]
    OTHERS_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Insel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Insel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Insel {
    #[inline(always)]
    fn from(val: u8) -> Insel {
        Insel::from_bits(val)
    }
}
impl From<Insel> for u8 {
    #[inline(always)]
    fn from(val: Insel) -> u8 {
        Insel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeyFlush {
    #[doc = "Clr has no effect."]
    NO_EFFECT = 0x0,
    #[doc = "Set to start flush."]
    FLUSH = 0x01,
}
impl KeyFlush {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KeyFlush {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KeyFlush {
    #[inline(always)]
    fn from(val: u8) -> KeyFlush {
        KeyFlush::from_bits(val)
    }
}
impl From<KeyFlush> for u8 {
    #[inline(always)]
    fn from(val: KeyFlush) -> u8 {
        KeyFlush::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Milestone {
    #[doc = "PREL."]
    PREL = 0x0,
    #[doc = "BR."]
    BR = 0x01,
    #[doc = "SI."]
    SI = 0x02,
    #[doc = "GO."]
    GO = 0x03,
}
impl Milestone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Milestone {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Milestone {
    #[inline(always)]
    fn from(val: u8) -> Milestone {
        Milestone::from_bits(val)
    }
}
impl From<Milestone> for u8 {
    #[inline(always)]
    fn from(val: Milestone) -> u8 {
        Milestone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NoAutoInit {
    #[doc = "SHA automatic HASH initialisation."]
    SHA_INIT = 0x0,
    #[doc = "No SHA automatic HASH initialisation."]
    NO_SHA_INIT = 0x01,
}
impl NoAutoInit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NoAutoInit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NoAutoInit {
    #[inline(always)]
    fn from(val: u8) -> NoAutoInit {
        NoAutoInit::from_bits(val)
    }
}
impl From<NoAutoInit> for u8 {
    #[inline(always)]
    fn from(val: NoAutoInit) -> u8 {
        NoAutoInit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Outsel {
    #[doc = "DATOUT = 'Kernel Res'."]
    DATOUT_KER_RES = 0x0,
    #[doc = "DATOUT = 'Kernel Res' ^ DATIN\\[0\\]."]
    DATOUT_DATIN0 = 0x01,
    #[doc = "DATOUT = 'Kernel Res' ^ DATIN\\[1\\]*."]
    DATOUT_DATIN1 = 0x02,
    #[doc = "DATOUT = 'Kernel Res' ^ DATIN\\[2\\]*."]
    DATOUT_DATIN2 = 0x03,
    #[doc = "DATOUT = 'Kernel Res' ^DATIN\\[3\\]*."]
    DATOUT_DATIN3 = 0x04,
    #[doc = "others - DATOUT = 'Kernel Res' * - only if DATIN\\[num\\] exists, else \\[0\\]."]
    OTHERS_5 = 0x05,
    #[doc = "others - DATOUT = 'Kernel Res' * - only if DATIN\\[num\\] exists, else \\[0\\]."]
    OTHERS_6 = 0x06,
    #[doc = "others - DATOUT = 'Kernel Res' * - only if DATIN\\[num\\] exists, else \\[0\\]."]
    OTHERS_7 = 0x07,
}
impl Outsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Outsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Outsel {
    #[inline(always)]
    fn from(val: u8) -> Outsel {
        Outsel::from_bits(val)
    }
}
impl From<Outsel> for u8 {
    #[inline(always)]
    fn from(val: Outsel) -> u8 {
        Outsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rkey {
    #[doc = "DATOUT register bank."]
    DATOUT = 0x0,
    #[doc = "KEY register bank."]
    KEY = 0x01,
}
impl Rkey {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rkey {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rkey {
    #[inline(always)]
    fn from(val: u8) -> Rkey {
        Rkey::from_bits(val)
    }
}
impl From<Rkey> for u8 {
    #[inline(always)]
    fn from(val: Rkey) -> u8 {
        Rkey::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sha2CountEn {
    #[doc = "SHA operation DOES NOT increment COUNT."]
    COUNT = 0x0,
    #[doc = "SHA operation DOES increment count."]
    NO_COUNT = 0x01,
}
impl Sha2CountEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sha2CountEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sha2CountEn {
    #[inline(always)]
    fn from(val: u8) -> Sha2CountEn {
        Sha2CountEn::from_bits(val)
    }
}
impl From<Sha2CountEn> for u8 {
    #[inline(always)]
    fn from(val: Sha2CountEn) -> u8 {
        Sha2CountEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sha2Mode {
    #[doc = "SHA NORM Mode."]
    NORMAL = 0x0,
    #[doc = "SHA AUTO Mode."]
    AUTO = 0x01,
}
impl Sha2Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sha2Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sha2Mode {
    #[inline(always)]
    fn from(val: u8) -> Sha2Mode {
        Sha2Mode::from_bits(val)
    }
}
impl From<Sha2Mode> for u8 {
    #[inline(always)]
    fn from(val: Sha2Mode) -> u8 {
        Sha2Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sha2Size {
    #[doc = "SHA-224."]
    SHA_224 = 0x0,
    #[doc = "SHA-256."]
    SHA_256 = 0x01,
    #[doc = "SHA-384(or SHA-224 if SHA-256 only)."]
    SHA_384 = 0x02,
    #[doc = "SHA-512 (or SHA-256 if SHA-256 only)."]
    SHA_512 = 0x03,
}
impl Sha2Size {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sha2Size {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sha2Size {
    #[inline(always)]
    fn from(val: u8) -> Sha2Size {
        Sha2Size::from_bits(val)
    }
}
impl From<Sha2Size> for u8 {
    #[inline(always)]
    fn from(val: Sha2Size) -> u8 {
        Sha2Size::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sha2Stop {
    #[doc = "Keep running."]
    RUNNING = 0x0,
    #[doc = "Stop auto mode."]
    STOP = 0x01,
}
impl Sha2Stop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sha2Stop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sha2Stop {
    #[inline(always)]
    fn from(val: u8) -> Sha2Stop {
        Sha2Stop::from_bits(val)
    }
}
impl From<Sha2Stop> for u8 {
    #[inline(always)]
    fn from(val: Sha2Stop) -> u8 {
        Sha2Stop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smaskstep {
    #[doc = "SFRSEED increments every regbank access."]
    REGBANK = 0x0,
    #[doc = "SFRSEED increments every regbank access PLUS when SFRSEED in read."]
    REGBANK_AND_PLUS = 0x01,
}
impl Smaskstep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smaskstep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smaskstep {
    #[inline(always)]
    fn from(val: u8) -> Smaskstep {
        Smaskstep::from_bits(val)
    }
}
impl From<Smaskstep> for u8 {
    #[inline(always)]
    fn from(val: Smaskstep) -> u8 {
        Smaskstep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smasksw {
    #[doc = "SFR MASK output directly controlled by HW mask generator."]
    HW = 0x0,
    #[doc = "SFR MASK output directly controlled by SW."]
    SW = 0x01,
}
impl Smasksw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smasksw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smasksw {
    #[inline(always)]
    fn from(val: u8) -> Smasksw {
        Smasksw::from_bits(val)
    }
}
impl From<Smasksw> for u8 {
    #[inline(always)]
    fn from(val: Smasksw) -> u8 {
        Smasksw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Start {
    #[doc = "Clr has no effect."]
    NO_EFFECT = 0x0,
    #[doc = "Set to start operation."]
    START_OP = 0x01,
}
impl Start {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Start {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Start {
    #[inline(always)]
    fn from(val: u8) -> Start {
        Start::from_bits(val)
    }
}
impl From<Start> for u8 {
    #[inline(always)]
    fn from(val: Start) -> u8 {
        Start::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tdeskey {
    #[doc = "2-key TDES."]
    TWO_KEY = 0x0,
    #[doc = "3-key TDES."]
    THREE_KEY = 0x01,
}
impl Tdeskey {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tdeskey {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tdeskey {
    #[inline(always)]
    fn from(val: u8) -> Tdeskey {
        Tdeskey::from_bits(val)
    }
}
impl From<Tdeskey> for u8 {
    #[inline(always)]
    fn from(val: Tdeskey) -> u8 {
        Tdeskey::to_bits(val)
    }
}
