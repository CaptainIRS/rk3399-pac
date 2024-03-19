#[doc = r"Enumeration of all the interrupts."]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "3 - Interrupt ddrc0_int"]
    ddrc0_int = 3,
    #[doc = "4 - Interrupt ddrc1_int"]
    ddrc1_int = 4,
    #[doc = "5 - Interrupt dmac0_perilp_irq_abort"]
    dmac0_perilp_irq_abort = 5,
    #[doc = "6 - Interrupt dmac0_perilp_irq"]
    dmac0_perilp_irq = 6,
    #[doc = "7 - Interrupt dmac1_perilp_irq_abort"]
    dmac1_perilp_irq_abort = 7,
    #[doc = "8 - Interrupt dmac1_perilp_irq"]
    dmac1_perilp_irq = 8,
    #[doc = "9 - Interrupt dp_irq"]
    dp_irq = 9,
    #[doc = "11 - Interrupt emmccore_int"]
    emmccore_int = 11,
    #[doc = "12 - Interrupt gmac_int"]
    gmac_int = 12,
    #[doc = "13 - Interrupt gmac_pmt_int"]
    gmac_pmt_int = 13,
    #[doc = "14 - Interrupt gpio0_int"]
    gpio0_int = 14,
    #[doc = "15 - Interrupt gpio1_int"]
    gpio1_int = 15,
    #[doc = "16 - Interrupt gpio2_intr"]
    gpio2_intr = 16,
    #[doc = "17 - Interrupt gpio3_intr"]
    gpio3_intr = 17,
    #[doc = "18 - Interrupt gpio4_intr"]
    gpio4_intr = 18,
    #[doc = "23 - Interrupt hdmi_irq"]
    hdmi_irq = 23,
    #[doc = "24 - Interrupt hdmi_wakeup_irq"]
    hdmi_wakeup_irq = 24,
    #[doc = "39 - Interrupt i2s0_int"]
    i2s0_int = 39,
    #[doc = "40 - Interrupt i2s1_int"]
    i2s1_int = 40,
    #[doc = "41 - Interrupt i2s2_int"]
    i2s2_int = 41,
    #[doc = "52 - Interrupt spi2_int"]
    spi2_int = 52,
    #[doc = "53 - Interrupt spi1_int"]
    spi1_int = 53,
    #[doc = "54 - Interrupt pmu_int"]
    pmu_int = 54,
    #[doc = "60 - Interrupt spi3_int"]
    spi3_int = 60,
    #[doc = "61 - Interrupt pwm_int"]
    pwm_int = 61,
    #[doc = "62 - Interrupt saradc_int"]
    saradc_int = 62,
    #[doc = "65 - Interrupt sdmmc_int"]
    sdmmc_int = 65,
    #[doc = "66 - Interrupt spdif_int"]
    spdif_int = 66,
    #[doc = "67 - Interrupt spi4_int"]
    spi4_int = 67,
    #[doc = "68 - Interrupt spi0_int"]
    spi0_int = 68,
    #[doc = "97 - Interrupt tsadc_int"]
    tsadc_int = 97,
    #[doc = "98 - Interrupt uart1_int"]
    uart1_int = 98,
    #[doc = "99 - Interrupt uart0_int"]
    uart0_int = 99,
    #[doc = "100 - Interrupt uart2_int"]
    uart2_int = 100,
    #[doc = "101 - Interrupt uart3_int"]
    uart3_int = 101,
    #[doc = "102 - Interrupt uart4_int"]
    uart4_int = 102,
    #[doc = "120 - Interrupt wdt0_intr"]
    wdt0_intr = 120,
    #[doc = "121 - Interrupt wdt1_intr"]
    wdt1_intr = 121,
    #[doc = "122 - Interrupt wdt2_int"]
    wdt2_int = 122,
    #[doc = "132 - Interrupt spi5_int"]
    spi5_int = 132,
}
#[doc = r" TryFromInterruptError"]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            3 => Ok(Interrupt::ddrc0_int),
            4 => Ok(Interrupt::ddrc1_int),
            5 => Ok(Interrupt::dmac0_perilp_irq_abort),
            6 => Ok(Interrupt::dmac0_perilp_irq),
            7 => Ok(Interrupt::dmac1_perilp_irq_abort),
            8 => Ok(Interrupt::dmac1_perilp_irq),
            9 => Ok(Interrupt::dp_irq),
            11 => Ok(Interrupt::emmccore_int),
            12 => Ok(Interrupt::gmac_int),
            13 => Ok(Interrupt::gmac_pmt_int),
            14 => Ok(Interrupt::gpio0_int),
            15 => Ok(Interrupt::gpio1_int),
            16 => Ok(Interrupt::gpio2_intr),
            17 => Ok(Interrupt::gpio3_intr),
            18 => Ok(Interrupt::gpio4_intr),
            23 => Ok(Interrupt::hdmi_irq),
            24 => Ok(Interrupt::hdmi_wakeup_irq),
            39 => Ok(Interrupt::i2s0_int),
            40 => Ok(Interrupt::i2s1_int),
            41 => Ok(Interrupt::i2s2_int),
            52 => Ok(Interrupt::spi2_int),
            53 => Ok(Interrupt::spi1_int),
            54 => Ok(Interrupt::pmu_int),
            60 => Ok(Interrupt::spi3_int),
            61 => Ok(Interrupt::pwm_int),
            62 => Ok(Interrupt::saradc_int),
            65 => Ok(Interrupt::sdmmc_int),
            66 => Ok(Interrupt::spdif_int),
            67 => Ok(Interrupt::spi4_int),
            68 => Ok(Interrupt::spi0_int),
            97 => Ok(Interrupt::tsadc_int),
            98 => Ok(Interrupt::uart1_int),
            99 => Ok(Interrupt::uart0_int),
            100 => Ok(Interrupt::uart2_int),
            101 => Ok(Interrupt::uart3_int),
            102 => Ok(Interrupt::uart4_int),
            120 => Ok(Interrupt::wdt0_intr),
            121 => Ok(Interrupt::wdt1_intr),
            122 => Ok(Interrupt::wdt2_int),
            132 => Ok(Interrupt::spi5_int),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
