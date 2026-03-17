#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BusErr {
    #[doc = "Read and write accesses are normal."]
    NORMAL = 0x0,
    #[doc = "Read or write accesses occurred when STATUS\\[INVAL_BIT\\] was asserted."]
    ASSERTED = 0x01,
}
impl BusErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BusErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BusErr {
    #[inline(always)]
    fn from(val: u8) -> BusErr {
        BusErr::from_bits(val)
    }
}
impl From<BusErr> for u8 {
    #[inline(always)]
    fn from(val: BusErr) -> u8 {
        BusErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swr {
    #[doc = "Software Reset cleared."]
    CLEARED = 0x0,
    #[doc = "Software Reset asserted."]
    ASSERTED = 0x01,
}
impl Swr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swr {
    #[inline(always)]
    fn from(val: u8) -> Swr {
        Swr::from_bits(val)
    }
}
impl From<Swr> for u8 {
    #[inline(always)]
    fn from(val: Swr) -> u8 {
        Swr::to_bits(val)
    }
}
