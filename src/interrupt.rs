#[doc = r"Enumeration of all the interrupts."]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "3 - Interrupt ddrc0_int"]
    ddrc0 = 3,
    #[doc = "4 - Interrupt ddrc1_int"]
    ddrc1 = 4,
    #[doc = "5 - Interrupt dmac0_perilp_irq_abort"]
    dmac0_perilp_abort = 5,
    #[doc = "6 - Interrupt dmac0_perilp_irq"]
    dmac0_perilp = 6,
    #[doc = "7 - Interrupt dmac1_perilp_irq_abort"]
    dmac1_perilp_abort = 7,
    #[doc = "8 - Interrupt dmac1_perilp_irq"]
    dmac1_perilp = 8,
    #[doc = "9 - Interrupt dp_irq"]
    dp = 9,
    #[doc = "11 - Interrupt emmccore_int"]
    emmccore = 11,
    #[doc = "12 - Interrupt gmac_int"]
    gmac = 12,
    #[doc = "13 - Interrupt gmac_pmt_int"]
    gmac_pmt = 13,
    #[doc = "14 - Interrupt gpio0_int"]
    gpio0 = 14,
    #[doc = "15 - Interrupt gpio1_int"]
    gpio1 = 15,
    #[doc = "16 - Interrupt gpio2_intr"]
    gpio2 = 16,
    #[doc = "17 - Interrupt gpio3_intr"]
    gpio3 = 17,
    #[doc = "18 - Interrupt gpio4_intr"]
    gpio4 = 18,
    #[doc = "23 - Interrupt hdmi_irq"]
    hdmi = 23,
    #[doc = "24 - Interrupt hdmi_wakeup_irq"]
    hdmi_wakeup = 24,
    #[doc = "39 - Interrupt i2s0_int"]
    i2s0 = 39,
    #[doc = "40 - Interrupt i2s1_int"]
    i2s1 = 40,
    #[doc = "41 - Interrupt i2s2_int"]
    i2s2 = 41,
    #[doc = "52 - Interrupt spi2_int"]
    spi2 = 52,
    #[doc = "53 - Interrupt spi1_int"]
    spi1 = 53,
    #[doc = "54 - Interrupt pmu_int"]
    pmu = 54,
    #[doc = "60 - Interrupt spi3_int"]
    spi3 = 60,
    #[doc = "61 - Interrupt pwm_int"]
    pwm = 61,
    #[doc = "62 - Interrupt saradc_int"]
    saradc = 62,
    #[doc = "65 - Interrupt sdmmc_int"]
    sdmmc = 65,
    #[doc = "66 - Interrupt spdif_int"]
    spdif = 66,
    #[doc = "67 - Interrupt spi4_int"]
    spi4 = 67,
    #[doc = "68 - Interrupt spi0_int"]
    spi0 = 68,
    #[doc = "97 - Interrupt tsadc_int"]
    tsadc = 97,
    #[doc = "98 - Interrupt uart1_int"]
    uart1 = 98,
    #[doc = "99 - Interrupt uart0_int"]
    uart0 = 99,
    #[doc = "100 - Interrupt uart2_int"]
    uart2 = 100,
    #[doc = "101 - Interrupt uart3_int"]
    uart3 = 101,
    #[doc = "102 - Interrupt uart4_int"]
    uart4 = 102,
    #[doc = "120 - Interrupt wdt0_intr"]
    wdt0 = 120,
    #[doc = "121 - Interrupt wdt1_intr"]
    wdt1 = 121,
    #[doc = "122 - Interrupt wdt2_int"]
    wdt2 = 122,
    #[doc = "132 - Interrupt spi5_int"]
    spi5 = 132,
}
#[doc = r" TryFromInterruptError"]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            3 => Ok(Interrupt::ddrc0),
            4 => Ok(Interrupt::ddrc1),
            5 => Ok(Interrupt::dmac0_perilp_abort),
            6 => Ok(Interrupt::dmac0_perilp),
            7 => Ok(Interrupt::dmac1_perilp_abort),
            8 => Ok(Interrupt::dmac1_perilp),
            9 => Ok(Interrupt::dp),
            11 => Ok(Interrupt::emmccore),
            12 => Ok(Interrupt::gmac),
            13 => Ok(Interrupt::gmac_pmt),
            14 => Ok(Interrupt::gpio0),
            15 => Ok(Interrupt::gpio1),
            16 => Ok(Interrupt::gpio2),
            17 => Ok(Interrupt::gpio3),
            18 => Ok(Interrupt::gpio4),
            23 => Ok(Interrupt::hdmi),
            24 => Ok(Interrupt::hdmi_wakeup),
            39 => Ok(Interrupt::i2s0),
            40 => Ok(Interrupt::i2s1),
            41 => Ok(Interrupt::i2s2),
            52 => Ok(Interrupt::spi2),
            53 => Ok(Interrupt::spi1),
            54 => Ok(Interrupt::pmu),
            60 => Ok(Interrupt::spi3),
            61 => Ok(Interrupt::pwm),
            62 => Ok(Interrupt::saradc),
            65 => Ok(Interrupt::sdmmc),
            66 => Ok(Interrupt::spdif),
            67 => Ok(Interrupt::spi4),
            68 => Ok(Interrupt::spi0),
            97 => Ok(Interrupt::tsadc),
            98 => Ok(Interrupt::uart1),
            99 => Ok(Interrupt::uart0),
            100 => Ok(Interrupt::uart2),
            101 => Ok(Interrupt::uart3),
            102 => Ok(Interrupt::uart4),
            120 => Ok(Interrupt::wdt0),
            121 => Ok(Interrupt::wdt1),
            122 => Ok(Interrupt::wdt2),
            132 => Ok(Interrupt::spi5),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
