#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - RESERVED16"]
    RESERVED16 = 0,
    #[doc = "1 - CMC"]
    CMC = 1,
    #[doc = "2 - DMA_CH0"]
    DMA_CH0 = 2,
    #[doc = "3 - DMA_CH1"]
    DMA_CH1 = 3,
    #[doc = "4 - DMA_CH2"]
    DMA_CH2 = 4,
    #[doc = "5 - DMA_CH3"]
    DMA_CH3 = 5,
    #[doc = "6 - DMA_CH4"]
    DMA_CH4 = 6,
    #[doc = "7 - DMA_CH5"]
    DMA_CH5 = 7,
    #[doc = "8 - DMA_CH6"]
    DMA_CH6 = 8,
    #[doc = "9 - DMA_CH7"]
    DMA_CH7 = 9,
    #[doc = "10 - ERM0_SINGLE_BIT"]
    ERM0_SINGLE_BIT = 10,
    #[doc = "11 - ERM0_MULTI_BIT"]
    ERM0_MULTI_BIT = 11,
    #[doc = "12 - FMU0"]
    FMU0 = 12,
    #[doc = "13 - GLIKEY0"]
    GLIKEY0 = 13,
    #[doc = "14 - MBC0"]
    MBC0 = 14,
    #[doc = "15 - SCG0"]
    SCG0 = 15,
    #[doc = "16 - SPC0"]
    SPC0 = 16,
    #[doc = "17 - TDET"]
    TDET = 17,
    #[doc = "18 - WUU0"]
    WUU0 = 18,
    #[doc = "19 - CAN0"]
    CAN0 = 19,
    #[doc = "20 - RESERVED36"]
    RESERVED36 = 20,
    #[doc = "23 - FLEXIO"]
    FLEXIO = 23,
    #[doc = "24 - I3C0"]
    I3C0 = 24,
    #[doc = "26 - LPI2C0"]
    LPI2C0 = 26,
    #[doc = "27 - LPI2C1"]
    LPI2C1 = 27,
    #[doc = "28 - LPSPI0"]
    LPSPI0 = 28,
    #[doc = "29 - LPSPI1"]
    LPSPI1 = 29,
    #[doc = "31 - LPUART0"]
    LPUART0 = 31,
    #[doc = "32 - LPUART1"]
    LPUART1 = 32,
    #[doc = "33 - LPUART2"]
    LPUART2 = 33,
    #[doc = "34 - LPUART3"]
    LPUART3 = 34,
    #[doc = "35 - LPUART4"]
    LPUART4 = 35,
    #[doc = "36 - USB0"]
    USB0 = 36,
    #[doc = "38 - CDOG0"]
    CDOG0 = 38,
    #[doc = "39 - CTIMER0"]
    CTIMER0 = 39,
    #[doc = "40 - CTIMER1"]
    CTIMER1 = 40,
    #[doc = "41 - CTIMER2"]
    CTIMER2 = 41,
    #[doc = "42 - CTIMER3"]
    CTIMER3 = 42,
    #[doc = "43 - CTIMER4"]
    CTIMER4 = 43,
    #[doc = "44 - FLEXPWM0_RELOAD_ERROR"]
    FLEXPWM0_RELOAD_ERROR = 44,
    #[doc = "45 - FLEXPWM0_FAULT"]
    FLEXPWM0_FAULT = 45,
    #[doc = "46 - FLEXPWM0_SUBMODULE0"]
    FLEXPWM0_SUBMODULE0 = 46,
    #[doc = "47 - FLEXPWM0_SUBMODULE1"]
    FLEXPWM0_SUBMODULE1 = 47,
    #[doc = "48 - FLEXPWM0_SUBMODULE2"]
    FLEXPWM0_SUBMODULE2 = 48,
    #[doc = "49 - FLEXPWM0_SUBMODULE3"]
    FLEXPWM0_SUBMODULE3 = 49,
    #[doc = "50 - EQDC0_COMPARE"]
    EQDC0_COMPARE = 50,
    #[doc = "51 - EQDC0_HOME"]
    EQDC0_HOME = 51,
    #[doc = "52 - EQDC0_WATCHDOG"]
    EQDC0_WATCHDOG = 52,
    #[doc = "53 - EQDC0_INDEX"]
    EQDC0_INDEX = 53,
    #[doc = "54 - FREQME0"]
    FREQME0 = 54,
    #[doc = "55 - LPTMR0"]
    LPTMR0 = 55,
    #[doc = "57 - OS_EVENT"]
    OS_EVENT = 57,
    #[doc = "58 - WAKETIMER0"]
    WAKETIMER0 = 58,
    #[doc = "59 - UTICK0"]
    UTICK0 = 59,
    #[doc = "60 - WWDT0"]
    WWDT0 = 60,
    #[doc = "62 - ADC0"]
    ADC0 = 62,
    #[doc = "63 - ADC1"]
    ADC1 = 63,
    #[doc = "64 - CMP0"]
    CMP0 = 64,
    #[doc = "65 - CMP1"]
    CMP1 = 65,
    #[doc = "66 - RESERVED82"]
    RESERVED82 = 66,
    #[doc = "67 - DAC0"]
    DAC0 = 67,
    #[doc = "71 - GPIO0"]
    GPIO0 = 71,
    #[doc = "72 - GPIO1"]
    GPIO1 = 72,
    #[doc = "73 - GPIO2"]
    GPIO2 = 73,
    #[doc = "74 - GPIO3"]
    GPIO3 = 74,
    #[doc = "75 - GPIO4"]
    GPIO4 = 75,
    #[doc = "77 - LPI2C2"]
    LPI2C2 = 77,
    #[doc = "78 - LPI2C3"]
    LPI2C3 = 78,
    #[doc = "79 - FLEXPWM1_RELOAD_ERROR"]
    FLEXPWM1_RELOAD_ERROR = 79,
    #[doc = "80 - FLEXPWM1_FAULT"]
    FLEXPWM1_FAULT = 80,
    #[doc = "81 - FLEXPWM1_SUBMODULE0"]
    FLEXPWM1_SUBMODULE0 = 81,
    #[doc = "82 - FLEXPWM1_SUBMODULE1"]
    FLEXPWM1_SUBMODULE1 = 82,
    #[doc = "83 - FLEXPWM1_SUBMODULE2"]
    FLEXPWM1_SUBMODULE2 = 83,
    #[doc = "84 - FLEXPWM1_SUBMODULE3"]
    FLEXPWM1_SUBMODULE3 = 84,
    #[doc = "85 - EQDC1_COMPARE"]
    EQDC1_COMPARE = 85,
    #[doc = "86 - EQDC1_HOME"]
    EQDC1_HOME = 86,
    #[doc = "87 - EQDC1_WATCHDOG"]
    EQDC1_WATCHDOG = 87,
    #[doc = "88 - EQDC1_INDEX"]
    EQDC1_INDEX = 88,
    #[doc = "95 - RESERVED111"]
    RESERVED111 = 95,
    #[doc = "107 - MAU"]
    MAU = 107,
    #[doc = "108 - SMARTDMA"]
    SMARTDMA = 108,
    #[doc = "109 - CDOG1"]
    CDOG1 = 109,
    #[doc = "110 - PKC"]
    PKC = 110,
    #[doc = "111 - SGI"]
    SGI = 111,
    #[doc = "113 - TRNG0"]
    TRNG0 = 113,
    #[doc = "114 - SECURE_ERR"]
    SECURE_ERR = 114,
    #[doc = "116 - RESERVED132"]
    RESERVED132 = 116,
    #[doc = "117 - RESERVED133"]
    RESERVED133 = 117,
    #[doc = "119 - RTC"]
    RTC = 119,
    #[doc = "120 - RTC_1HZ"]
    RTC_1HZ = 120,
    #[doc = "121 - RESERVED137"]
    RESERVED137 = 121,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors;
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
pub const ADC0: adc::Adc = unsafe { adc::Adc::from_ptr(0x400AF000 as _) };
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x400B0000 as _) };
pub const AOI0: aoi::Aoi = unsafe { aoi::Aoi::from_ptr(0x40089000 as _) };
pub const AOI1: aoi::Aoi = unsafe { aoi::Aoi::from_ptr(0x40097000 as _) };
pub const CDOG0: cdog::Cdog = unsafe { cdog::Cdog::from_ptr(0x40100000 as _) };
pub const CDOG1: cdog::Cdog = unsafe { cdog::Cdog::from_ptr(0x40107000 as _) };
pub const CMC: cmc::Cmc = unsafe { cmc::Cmc::from_ptr(0x4008B000 as _) };
pub const CMP0: cmp::Cmp = unsafe { cmp::Cmp::from_ptr(0x400B1000 as _) };
pub const CMP1: cmp::Cmp = unsafe { cmp::Cmp::from_ptr(0x400B2000 as _) };
pub const CRC0: crc0::Crc0 = unsafe { crc0::Crc0::from_ptr(0x4008A000 as _) };
pub const CTIMER0: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x40004000 as _) };
pub const CTIMER1: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x40005000 as _) };
pub const CTIMER2: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x40006000 as _) };
pub const CTIMER3: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x40007000 as _) };
pub const CTIMER4: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x40008000 as _) };
pub const DAC0: dac0::Dac0 = unsafe { dac0::Dac0::from_ptr(0x400B4000 as _) };
pub const DBGMAILBOX: dbgmailbox::Dbgmailbox =
    unsafe { dbgmailbox::Dbgmailbox::from_ptr(0x40101000 as _) };
pub const DMA0: dma0::Dma0 = unsafe { dma0::Dma0::from_ptr(0x40080000 as _) };
pub const EIM0: eim0::Eim0 = unsafe { eim0::Eim0::from_ptr(0x4008C000 as _) };
pub const ERM0: erm0::Erm0 = unsafe { erm0::Erm0::from_ptr(0x4008D000 as _) };
pub const FLEXIO0: flexio0::Flexio0 = unsafe { flexio0::Flexio0::from_ptr(0x40099000 as _) };
pub const FMC0: fmc0::Fmc0 = unsafe { fmc0::Fmc0::from_ptr(0x40094000 as _) };
pub const FMU0: fmu0::Fmu0 = unsafe { fmu0::Fmu0::from_ptr(0x40095000 as _) };
pub const FREQME0: freqme0::Freqme0 = unsafe { freqme0::Freqme0::from_ptr(0x40009000 as _) };
pub const FLEX_PWM0: flexpwm::Flexpwm = unsafe { flexpwm::Flexpwm::from_ptr(0x400A9000 as _) };
pub const FLEX_PWM1: flexpwm::Flexpwm = unsafe { flexpwm::Flexpwm::from_ptr(0x400AA000 as _) };
pub const GPIO0: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x40102000 as _) };
pub const GPIO1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x40103000 as _) };
pub const GPIO2: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x40104000 as _) };
pub const GPIO3: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x40105000 as _) };
pub const GPIO4: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x40106000 as _) };
pub const I3C0: i3c0::I3c0 = unsafe { i3c0::I3c0::from_ptr(0x40002000 as _) };
pub const INPUTMUX0: inputmux0::Inputmux0 =
    unsafe { inputmux0::Inputmux0::from_ptr(0x40001000 as _) };
pub const LPI2C0: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x4009A000 as _) };
pub const LPI2C1: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x4009B000 as _) };
pub const LPI2C2: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400D4000 as _) };
pub const LPI2C3: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400D5000 as _) };
pub const LPSPI0: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4009C000 as _) };
pub const LPSPI1: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4009D000 as _) };
pub const LPTMR0: lptmr0::Lptmr0 = unsafe { lptmr0::Lptmr0::from_ptr(0x400AB000 as _) };
pub const LPUART0: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4009F000 as _) };
pub const LPUART1: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400A0000 as _) };
pub const LPUART2: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400A1000 as _) };
pub const LPUART3: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400A2000 as _) };
pub const LPUART4: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400A3000 as _) };
pub const OPAMP0: opamp0::Opamp0 = unsafe { opamp0::Opamp0::from_ptr(0x400B7000 as _) };
pub const OSTIMER0: ostimer0::Ostimer0 = unsafe { ostimer0::Ostimer0::from_ptr(0x400AD000 as _) };
pub const PKC0: pkc0::Pkc0 = unsafe { pkc0::Pkc0::from_ptr(0x400EA000 as _) };
pub const PORT0: port0::Port0 = unsafe { port0::Port0::from_ptr(0x400BC000 as _) };
pub const PORT1: port1::Port1 = unsafe { port1::Port1::from_ptr(0x400BD000 as _) };
pub const PORT2: port2::Port2 = unsafe { port2::Port2::from_ptr(0x400BE000 as _) };
pub const PORT3: port3::Port3 = unsafe { port3::Port3::from_ptr(0x400BF000 as _) };
pub const PORT4: port4::Port4 = unsafe { port4::Port4::from_ptr(0x400C0000 as _) };
pub const EQDC0: eqdc::Eqdc = unsafe { eqdc::Eqdc::from_ptr(0x400A7000 as _) };
pub const EQDC1: eqdc::Eqdc = unsafe { eqdc::Eqdc::from_ptr(0x400A8000 as _) };
pub const RTC0: rtc0::Rtc0 = unsafe { rtc0::Rtc0::from_ptr(0x400EE000 as _) };
pub const SCG0: scg0::Scg0 = unsafe { scg0::Scg0::from_ptr(0x4008F000 as _) };
pub const SGI0: sgi0::Sgi0 = unsafe { sgi0::Sgi0::from_ptr(0x400EB000 as _) };
pub const SPC0: spc0::Spc0 = unsafe { spc0::Spc0::from_ptr(0x40090000 as _) };
pub const SYSCON: syscon::Syscon = unsafe { syscon::Syscon::from_ptr(0x40091000 as _) };
pub const SMART_DMA0: smartdma0::Smartdma0 =
    unsafe { smartdma0::Smartdma0::from_ptr(0x4000E000 as _) };
pub const TDET0: tdet0::Tdet0 = unsafe { tdet0::Tdet0::from_ptr(0x400E9000 as _) };
pub const TRNG0: trng0::Trng0 = unsafe { trng0::Trng0::from_ptr(0x400EC000 as _) };
pub const UDF0: udf0::Udf0 = unsafe { udf0::Udf0::from_ptr(0x400ED000 as _) };
pub const USB0: usb0::Usb0 = unsafe { usb0::Usb0::from_ptr(0x400A4000 as _) };
pub const UTICK0: utick0::Utick0 = unsafe { utick0::Utick0::from_ptr(0x4000B000 as _) };
pub const VBAT0: vbat0::Vbat0 = unsafe { vbat0::Vbat0::from_ptr(0x40093000 as _) };
pub const WAKETIMER0: waketimer0::Waketimer0 =
    unsafe { waketimer0::Waketimer0::from_ptr(0x400AE000 as _) };
pub const WUU0: wuu0::Wuu0 = unsafe { wuu0::Wuu0::from_ptr(0x40092000 as _) };
pub const WWDT0: wwdt0::Wwdt0 = unsafe { wwdt0::Wwdt0::from_ptr(0x4000C000 as _) };
pub const EDMA_0_TCD0: tcd::Tcd = unsafe { tcd::Tcd::from_ptr(0x40081000 as _) };
#[path = "../../meta_peripherals/mcxa/ADC.rs"]
pub mod adc;
#[path = "../../meta_peripherals/mcxa/AOI.rs"]
pub mod aoi;
#[path = "../../meta_peripherals/mcxa/CDOG.rs"]
pub mod cdog;
#[path = "../../meta_peripherals/mcxa/CMC.rs"]
pub mod cmc;
#[path = "../../meta_peripherals/mcxa/CMP.rs"]
pub mod cmp;
pub mod common;
#[path = "../../meta_peripherals/mcxa/CRC0.rs"]
pub mod crc0;
#[path = "../../meta_peripherals/mcxa/CTIMER.rs"]
pub mod ctimer;
#[path = "../../meta_peripherals/mcxa/DAC0.rs"]
pub mod dac0;
#[path = "../../meta_peripherals/mcxa/DBGMAILBOX.rs"]
pub mod dbgmailbox;
#[path = "../../meta_peripherals/mcxa/DMA0.rs"]
pub mod dma0;
#[path = "../../meta_peripherals/mcxa/EIM0.rs"]
pub mod eim0;
#[path = "../../meta_peripherals/mcxa/EQDC.rs"]
pub mod eqdc;
#[path = "../../meta_peripherals/mcxa/ERM0.rs"]
pub mod erm0;
#[path = "../../meta_peripherals/mcxa/FLEXIO0.rs"]
pub mod flexio0;
#[path = "../../meta_peripherals/mcxa/FLEXPWM.rs"]
pub mod flexpwm;
#[path = "../../meta_peripherals/mcxa/FMC0.rs"]
pub mod fmc0;
#[path = "../../meta_peripherals/mcxa/FMU0.rs"]
pub mod fmu0;
#[path = "../../meta_peripherals/mcxa/FREQME0.rs"]
pub mod freqme0;
#[path = "../../meta_peripherals/mcxa/GPIO.rs"]
pub mod gpio;
#[path = "../../meta_peripherals/mcxa/I3C0.rs"]
pub mod i3c0;
#[path = "../../meta_peripherals/mcxa/INPUTMUX0.rs"]
pub mod inputmux0;
#[path = "../../meta_peripherals/mcxa/LPI2C.rs"]
pub mod lpi2c;
#[path = "../../meta_peripherals/mcxa/LPSPI.rs"]
pub mod lpspi;
#[path = "../../meta_peripherals/mcxa/LPTMR0.rs"]
pub mod lptmr0;
#[path = "../../meta_peripherals/mcxa/LPUART.rs"]
pub mod lpuart;
#[path = "../../meta_peripherals/mcxa/OPAMP0.rs"]
pub mod opamp0;
#[path = "../../meta_peripherals/mcxa/OSTIMER0.rs"]
pub mod ostimer0;
#[path = "../../meta_peripherals/mcxa/PKC0.rs"]
pub mod pkc0;
#[path = "../../meta_peripherals/mcxa/PORT0.rs"]
pub mod port0;
#[path = "../../meta_peripherals/mcxa/PORT1.rs"]
pub mod port1;
#[path = "../../meta_peripherals/mcxa/PORT2.rs"]
pub mod port2;
#[path = "../../meta_peripherals/mcxa/PORT3.rs"]
pub mod port3;
#[path = "../../meta_peripherals/mcxa/PORT4.rs"]
pub mod port4;
#[path = "../../meta_peripherals/mcxa/RTC0.rs"]
pub mod rtc0;
#[path = "../../meta_peripherals/mcxa/SCG0.rs"]
pub mod scg0;
#[path = "../../meta_peripherals/mcxa/SGI0.rs"]
pub mod sgi0;
#[path = "../../meta_peripherals/mcxa/SMARTDMA0.rs"]
pub mod smartdma0;
#[path = "../../meta_peripherals/mcxa/SPC0.rs"]
pub mod spc0;
#[path = "../../meta_peripherals/mcxa/SYSCON.rs"]
pub mod syscon;
#[path = "../../meta_peripherals/mcxa/TCD.rs"]
pub mod tcd;
#[path = "../../meta_peripherals/mcxa/TDET0.rs"]
pub mod tdet0;
#[path = "../../meta_peripherals/mcxa/TRNG0.rs"]
pub mod trng0;
#[path = "../../meta_peripherals/mcxa/UDF0.rs"]
pub mod udf0;
#[path = "../../meta_peripherals/mcxa/USB0.rs"]
pub mod usb0;
#[path = "../../meta_peripherals/mcxa/UTICK0.rs"]
pub mod utick0;
#[path = "../../meta_peripherals/mcxa/VBAT0.rs"]
pub mod vbat0;
#[path = "../../meta_peripherals/mcxa/WAKETIMER0.rs"]
pub mod waketimer0;
#[path = "../../meta_peripherals/mcxa/WUU0.rs"]
pub mod wuu0;
#[path = "../../meta_peripherals/mcxa/WWDT0.rs"]
pub mod wwdt0;
