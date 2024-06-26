#[doc = r"Enumeration of all the interrupts."]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - Interrupt crypto0_int"]
    crypto0 = 0,
    #[doc = "1 - Interrupt dcf_done_int"]
    dcf_done = 1,
    #[doc = "2 - Interrupt dcf_error_int"]
    dcf_error = 2,
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
    #[doc = "34 - Interrupt i2c3_int"]
    i2c3 = 34,
    #[doc = "35 - Interrupt i2c2_int"]
    i2c2 = 35,
    #[doc = "36 - Interrupt i2c7_int"]
    i2c7 = 36,
    #[doc = "37 - Interrupt i2c6_int"]
    i2c6 = 37,
    #[doc = "38 - Interrupt i2c5_int"]
    i2c5 = 38,
    #[doc = "39 - Interrupt i2s0_int"]
    i2s0 = 39,
    #[doc = "40 - Interrupt i2s1_int"]
    i2s1 = 40,
    #[doc = "41 - Interrupt i2s2_int"]
    i2s2 = 41,
    #[doc = "42 - Interrupt iep_int"]
    iep = 42,
    #[doc = "45 - Interrupt mipidsi_host0_irq"]
    mipidsi_host0 = 45,
    #[doc = "46 - Interrupt mipidsi_host1_irq"]
    mipidsi_host1 = 46,
    #[doc = "47 - Interrupt errirq_cci"]
    errirq_cci = 47,
    #[doc = "49 - Interrupt pciesys_int"]
    pciesys = 49,
    #[doc = "50 - Interrupt pcielegacy_int"]
    pcielegacy = 50,
    #[doc = "51 - Interrupt pcieclient_int"]
    pcieclient = 51,
    #[doc = "52 - Interrupt spi2_int"]
    spi2 = 52,
    #[doc = "53 - Interrupt spi1_int"]
    spi1 = 53,
    #[doc = "54 - Interrupt pmu_int"]
    pmu = 54,
    #[doc = "55 - Interrupt rga_intr"]
    rga = 55,
    #[doc = "56 - Interrupt i2c4_int"]
    i2c4 = 56,
    #[doc = "57 - Interrupt i2c0_int"]
    i2c0 = 57,
    #[doc = "58 - Interrupt i2c8_int"]
    i2c8 = 58,
    #[doc = "59 - Interrupt i2c1_int"]
    i2c1 = 59,
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
    #[doc = "69 - Interrupt stimer_intr0"]
    stimer0 = 69,
    #[doc = "70 - Interrupt stimer_intr1"]
    stimer1 = 70,
    #[doc = "71 - Interrupt stimer_intr2"]
    stimer2 = 71,
    #[doc = "72 - Interrupt stimer_intr3"]
    stimer3 = 72,
    #[doc = "73 - Interrupt stimer_intr4"]
    stimer4 = 73,
    #[doc = "74 - Interrupt stimer_intr5"]
    stimer5 = 74,
    #[doc = "75 - Interrupt stimer_intr6"]
    stimer6 = 75,
    #[doc = "76 - Interrupt stimer_intr7"]
    stimer7 = 76,
    #[doc = "77 - Interrupt stimer_intr8"]
    stimer8 = 77,
    #[doc = "78 - Interrupt stimer_intr9"]
    stimer9 = 78,
    #[doc = "79 - Interrupt stimer_intr10"]
    stimer10 = 79,
    #[doc = "80 - Interrupt stimer_intr11"]
    stimer11 = 80,
    #[doc = "81 - Interrupt timer_intr0"]
    timer0 = 81,
    #[doc = "82 - Interrupt timer_intr1"]
    timer1 = 82,
    #[doc = "83 - Interrupt timer_intr2"]
    timer2 = 83,
    #[doc = "84 - Interrupt timer_intr3"]
    timer3 = 84,
    #[doc = "85 - Interrupt timer_intr4"]
    timer4 = 85,
    #[doc = "86 - Interrupt timer_intr5"]
    timer5 = 86,
    #[doc = "87 - Interrupt timer_intr6"]
    timer6 = 87,
    #[doc = "88 - Interrupt timer_intr7"]
    timer7 = 88,
    #[doc = "89 - Interrupt timer_intr8"]
    timer8 = 89,
    #[doc = "90 - Interrupt timer_intr9"]
    timer9 = 90,
    #[doc = "91 - Interrupt timer_intr10"]
    timer10 = 91,
    #[doc = "92 - Interrupt timer_intr11"]
    timer11 = 92,
    #[doc = "95 - Interrupt pmutimer_intr0"]
    pmutimer0 = 95,
    #[doc = "96 - Interrupt pmutimer_intr1"]
    pmutimer1 = 96,
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
    #[doc = "103 - Interrupt usb3otg0_bvalid_irq"]
    usb3otg0_bvalid = 103,
    #[doc = "104 - Interrupt usb3otg0_id_irq"]
    usb3otg0_id = 104,
    #[doc = "105 - Interrupt usb3otg0_int"]
    usb3otg0 = 105,
    #[doc = "106 - Interrupt usb3otg0_linestate_irq"]
    usb3otg0_linestate = 106,
    #[doc = "107 - Interrupt usb3otg0_rxdet_irq"]
    usb3otg0_rxdet = 107,
    #[doc = "108 - Interrupt usb3otg1_bvalid_irq"]
    usb3otg1_bvalid = 108,
    #[doc = "109 - Interrupt usb3otg1_id_irq"]
    usb3otg1_id = 109,
    #[doc = "110 - Interrupt usb3otg1_int"]
    usb3otg1 = 110,
    #[doc = "111 - Interrupt usb3otg1_linestate_irq"]
    usb3otg1_linestate = 111,
    #[doc = "112 - Interrupt usb3otg1_rxdet_irq"]
    usb3otg1_rxdet = 112,
    #[doc = "113 - Interrupt vcodec_dec_int"]
    vcodec_dec = 113,
    #[doc = "114 - Interrupt vcodec_enc_int"]
    vcodec_enc = 114,
    #[doc = "115 - Interrupt vcodec_mmu_int"]
    vcodec_mmu = 115,
    #[doc = "116 - Interrupt vdu_dec_irq"]
    vdu_dec = 116,
    #[doc = "117 - Interrupt vdu_mmu_irq"]
    vdu_mmu = 117,
    #[doc = "118 - Interrupt vopbig_irq"]
    vopbig = 118,
    #[doc = "119 - Interrupt voplit_irq"]
    voplit = 119,
    #[doc = "120 - Interrupt wdt0_intr"]
    wdt0 = 120,
    #[doc = "121 - Interrupt wdt1_intr"]
    wdt1 = 121,
    #[doc = "122 - Interrupt wdt2_int"]
    wdt2 = 122,
    #[doc = "123 - Interrupt usb3otg0_pme_generation"]
    usb3otg0_pme_generation = 123,
    #[doc = "124 - Interrupt usb3otg0_host_legacy_smi_interrupt"]
    usb3otg0_host_legacy_smierrupt = 124,
    #[doc = "125 - Interrupt usb3otg0_host_sys_err"]
    usb3otg0_host_sys_err = 125,
    #[doc = "126 - Interrupt usb3otg1_pme_generation"]
    usb3otg1_pme_generation = 126,
    #[doc = "127 - Interrupt usb3otg1_host_legacy_smi_interrupt"]
    usb3otg1_host_legacy_smierrupt = 127,
    #[doc = "128 - Interrupt usb3otg1_host_sys_err"]
    usb3otg1_host_sys_err = 128,
    #[doc = "131 - Interrupt ddrmon_intr"]
    ddrmon = 131,
    #[doc = "132 - Interrupt spi5_int"]
    spi5 = 132,
    #[doc = "135 - Interrupt crypto1_int"]
    crypto1 = 135,
    #[doc = "137 - Interrupt pcierc_mode_elec_idle_irq"]
    pcierc_mode_elec_idle = 137,
}
#[doc = r" TryFromInterruptError"]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            0 => Ok(Interrupt::crypto0),
            1 => Ok(Interrupt::dcf_done),
            2 => Ok(Interrupt::dcf_error),
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
            34 => Ok(Interrupt::i2c3),
            35 => Ok(Interrupt::i2c2),
            36 => Ok(Interrupt::i2c7),
            37 => Ok(Interrupt::i2c6),
            38 => Ok(Interrupt::i2c5),
            39 => Ok(Interrupt::i2s0),
            40 => Ok(Interrupt::i2s1),
            41 => Ok(Interrupt::i2s2),
            42 => Ok(Interrupt::iep),
            45 => Ok(Interrupt::mipidsi_host0),
            46 => Ok(Interrupt::mipidsi_host1),
            47 => Ok(Interrupt::errirq_cci),
            49 => Ok(Interrupt::pciesys),
            50 => Ok(Interrupt::pcielegacy),
            51 => Ok(Interrupt::pcieclient),
            52 => Ok(Interrupt::spi2),
            53 => Ok(Interrupt::spi1),
            54 => Ok(Interrupt::pmu),
            55 => Ok(Interrupt::rga),
            56 => Ok(Interrupt::i2c4),
            57 => Ok(Interrupt::i2c0),
            58 => Ok(Interrupt::i2c8),
            59 => Ok(Interrupt::i2c1),
            60 => Ok(Interrupt::spi3),
            61 => Ok(Interrupt::pwm),
            62 => Ok(Interrupt::saradc),
            65 => Ok(Interrupt::sdmmc),
            66 => Ok(Interrupt::spdif),
            67 => Ok(Interrupt::spi4),
            68 => Ok(Interrupt::spi0),
            69 => Ok(Interrupt::stimer0),
            70 => Ok(Interrupt::stimer1),
            71 => Ok(Interrupt::stimer2),
            72 => Ok(Interrupt::stimer3),
            73 => Ok(Interrupt::stimer4),
            74 => Ok(Interrupt::stimer5),
            75 => Ok(Interrupt::stimer6),
            76 => Ok(Interrupt::stimer7),
            77 => Ok(Interrupt::stimer8),
            78 => Ok(Interrupt::stimer9),
            79 => Ok(Interrupt::stimer10),
            80 => Ok(Interrupt::stimer11),
            81 => Ok(Interrupt::timer0),
            82 => Ok(Interrupt::timer1),
            83 => Ok(Interrupt::timer2),
            84 => Ok(Interrupt::timer3),
            85 => Ok(Interrupt::timer4),
            86 => Ok(Interrupt::timer5),
            87 => Ok(Interrupt::timer6),
            88 => Ok(Interrupt::timer7),
            89 => Ok(Interrupt::timer8),
            90 => Ok(Interrupt::timer9),
            91 => Ok(Interrupt::timer10),
            92 => Ok(Interrupt::timer11),
            95 => Ok(Interrupt::pmutimer0),
            96 => Ok(Interrupt::pmutimer1),
            97 => Ok(Interrupt::tsadc),
            98 => Ok(Interrupt::uart1),
            99 => Ok(Interrupt::uart0),
            100 => Ok(Interrupt::uart2),
            101 => Ok(Interrupt::uart3),
            102 => Ok(Interrupt::uart4),
            103 => Ok(Interrupt::usb3otg0_bvalid),
            104 => Ok(Interrupt::usb3otg0_id),
            105 => Ok(Interrupt::usb3otg0),
            106 => Ok(Interrupt::usb3otg0_linestate),
            107 => Ok(Interrupt::usb3otg0_rxdet),
            108 => Ok(Interrupt::usb3otg1_bvalid),
            109 => Ok(Interrupt::usb3otg1_id),
            110 => Ok(Interrupt::usb3otg1),
            111 => Ok(Interrupt::usb3otg1_linestate),
            112 => Ok(Interrupt::usb3otg1_rxdet),
            113 => Ok(Interrupt::vcodec_dec),
            114 => Ok(Interrupt::vcodec_enc),
            115 => Ok(Interrupt::vcodec_mmu),
            116 => Ok(Interrupt::vdu_dec),
            117 => Ok(Interrupt::vdu_mmu),
            118 => Ok(Interrupt::vopbig),
            119 => Ok(Interrupt::voplit),
            120 => Ok(Interrupt::wdt0),
            121 => Ok(Interrupt::wdt1),
            122 => Ok(Interrupt::wdt2),
            123 => Ok(Interrupt::usb3otg0_pme_generation),
            124 => Ok(Interrupt::usb3otg0_host_legacy_smierrupt),
            125 => Ok(Interrupt::usb3otg0_host_sys_err),
            126 => Ok(Interrupt::usb3otg1_pme_generation),
            127 => Ok(Interrupt::usb3otg1_host_legacy_smierrupt),
            128 => Ok(Interrupt::usb3otg1_host_sys_err),
            131 => Ok(Interrupt::ddrmon),
            132 => Ok(Interrupt::spi5),
            135 => Ok(Interrupt::crypto1),
            137 => Ok(Interrupt::pcierc_mode_elec_idle),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
