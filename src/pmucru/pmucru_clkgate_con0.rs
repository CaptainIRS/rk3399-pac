#[doc = "Register `PMUCRU_CLKGATE_CON0` reader"]
pub type R = crate::R<PmucruClkgateCon0Spec>;
#[doc = "Register `PMUCRU_CLKGATE_CON0` writer"]
pub type W = crate::W<PmucruClkgateCon0Spec>;
#[doc = "Field `FCLK_CM0S_PMU_PPLL_SRC_EN` reader - fclk_cm0s_pmu_ppll_src clock disable bit When HIGH, disable clock"]
pub type FclkCm0sPmuPpllSrcEnR = crate::BitReader;
#[doc = "Field `FCLK_CM0S_PMU_PPLL_SRC_EN` writer - fclk_cm0s_pmu_ppll_src clock disable bit When HIGH, disable clock"]
pub type FclkCm0sPmuPpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SPI3_SRC_EN` reader - clk_spi3_src clock disable bit When HIGH, disable clock"]
pub type ClkSpi3SrcEnR = crate::BitReader;
#[doc = "Field `CLK_SPI3_SRC_EN` writer - clk_spi3_src clock disable bit When HIGH, disable clock"]
pub type ClkSpi3SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_TIMER0_EN` reader - clk_timer0 clock disable bit When HIGH, disable clock"]
pub type ClkTimer0EnR = crate::BitReader;
#[doc = "Field `CLK_TIMER0_EN` writer - clk_timer0 clock disable bit When HIGH, disable clock"]
pub type ClkTimer0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_TIMER1_EN` reader - clk_timer1 clock disable bit When HIGH, disable clock"]
pub type ClkTimer1EnR = crate::BitReader;
#[doc = "Field `CLK_TIMER1_EN` writer - clk_timer1 clock disable bit When HIGH, disable clock"]
pub type ClkTimer1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_UART4_SRC_EN` reader - clk_uart4_src clock disable bit When HIGH, disable clock"]
pub type ClkUart4SrcEnR = crate::BitReader;
#[doc = "Field `CLK_UART4_SRC_EN` writer - clk_uart4_src clock disable bit When HIGH, disable clock"]
pub type ClkUart4SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_UART4_FRAC_SRC_EN` reader - clk_uart4_frac_src clock disable bit When HIGH, disable clock"]
pub type ClkUart4FracSrcEnR = crate::BitReader;
#[doc = "Field `CLK_UART4_FRAC_SRC_EN` writer - clk_uart4_frac_src clock disable bit When HIGH, disable clock"]
pub type ClkUart4FracSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_PVTM_PMU_EN` reader - clk_pvtm_pmu clock disable bit When HIGH, disable clock"]
pub type ClkPvtmPmuEnR = crate::BitReader;
#[doc = "Field `CLK_PVTM_PMU_EN` writer - clk_pvtm_pmu clock disable bit When HIGH, disable clock"]
pub type ClkPvtmPmuEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_WIFI_EN` reader - clk_wifi clock disable bit When HIGH, disable clock"]
pub type ClkWifiEnR = crate::BitReader;
#[doc = "Field `CLK_WIFI_EN` writer - clk_wifi clock disable bit When HIGH, disable clock"]
pub type ClkWifiEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_I2C0_SRC_EN` reader - clk_i2c0_src clock disable bit When HIGH, disable clock"]
pub type ClkI2c0SrcEnR = crate::BitReader;
#[doc = "Field `CLK_I2C0_SRC_EN` writer - clk_i2c0_src clock disable bit When HIGH, disable clock"]
pub type ClkI2c0SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_I2C4_SRC_EN` reader - clk_i2c4_src clock disable bit When HIGH, disable clock"]
pub type ClkI2c4SrcEnR = crate::BitReader;
#[doc = "Field `CLK_I2C4_SRC_EN` writer - clk_i2c4_src clock disable bit When HIGH, disable clock"]
pub type ClkI2c4SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_I2C8_SRC_EN` reader - clk_i2c8_src clock disable bit When HIGH, disable clock"]
pub type ClkI2c8SrcEnR = crate::BitReader;
#[doc = "Field `CLK_I2C8_SRC_EN` writer - clk_i2c8_src clock disable bit When HIGH, disable clock"]
pub type ClkI2c8SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 1 - fclk_cm0s_pmu_ppll_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn fclk_cm0s_pmu_ppll_src_en(&self) -> FclkCm0sPmuPpllSrcEnR {
        FclkCm0sPmuPpllSrcEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - clk_spi3_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_spi3_src_en(&self) -> ClkSpi3SrcEnR {
        ClkSpi3SrcEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - clk_timer0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_timer0_en(&self) -> ClkTimer0EnR {
        ClkTimer0EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - clk_timer1 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_timer1_en(&self) -> ClkTimer1EnR {
        ClkTimer1EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - clk_uart4_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_uart4_src_en(&self) -> ClkUart4SrcEnR {
        ClkUart4SrcEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - clk_uart4_frac_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_uart4_frac_src_en(&self) -> ClkUart4FracSrcEnR {
        ClkUart4FracSrcEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - clk_pvtm_pmu clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_pvtm_pmu_en(&self) -> ClkPvtmPmuEnR {
        ClkPvtmPmuEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - clk_wifi clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_wifi_en(&self) -> ClkWifiEnR {
        ClkWifiEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - clk_i2c0_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_i2c0_src_en(&self) -> ClkI2c0SrcEnR {
        ClkI2c0SrcEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - clk_i2c4_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_i2c4_src_en(&self) -> ClkI2c4SrcEnR {
        ClkI2c4SrcEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - clk_i2c8_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_i2c8_src_en(&self) -> ClkI2c8SrcEnR {
        ClkI2c8SrcEnR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - fclk_cm0s_pmu_ppll_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn fclk_cm0s_pmu_ppll_src_en(&mut self) -> FclkCm0sPmuPpllSrcEnW<PmucruClkgateCon0Spec> {
        FclkCm0sPmuPpllSrcEnW::new(self, 1)
    }
    #[doc = "Bit 2 - clk_spi3_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spi3_src_en(&mut self) -> ClkSpi3SrcEnW<PmucruClkgateCon0Spec> {
        ClkSpi3SrcEnW::new(self, 2)
    }
    #[doc = "Bit 3 - clk_timer0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_timer0_en(&mut self) -> ClkTimer0EnW<PmucruClkgateCon0Spec> {
        ClkTimer0EnW::new(self, 3)
    }
    #[doc = "Bit 4 - clk_timer1 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_timer1_en(&mut self) -> ClkTimer1EnW<PmucruClkgateCon0Spec> {
        ClkTimer1EnW::new(self, 4)
    }
    #[doc = "Bit 5 - clk_uart4_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uart4_src_en(&mut self) -> ClkUart4SrcEnW<PmucruClkgateCon0Spec> {
        ClkUart4SrcEnW::new(self, 5)
    }
    #[doc = "Bit 6 - clk_uart4_frac_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uart4_frac_src_en(&mut self) -> ClkUart4FracSrcEnW<PmucruClkgateCon0Spec> {
        ClkUart4FracSrcEnW::new(self, 6)
    }
    #[doc = "Bit 7 - clk_pvtm_pmu clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_pvtm_pmu_en(&mut self) -> ClkPvtmPmuEnW<PmucruClkgateCon0Spec> {
        ClkPvtmPmuEnW::new(self, 7)
    }
    #[doc = "Bit 8 - clk_wifi clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_wifi_en(&mut self) -> ClkWifiEnW<PmucruClkgateCon0Spec> {
        ClkWifiEnW::new(self, 8)
    }
    #[doc = "Bit 9 - clk_i2c0_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c0_src_en(&mut self) -> ClkI2c0SrcEnW<PmucruClkgateCon0Spec> {
        ClkI2c0SrcEnW::new(self, 9)
    }
    #[doc = "Bit 10 - clk_i2c4_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c4_src_en(&mut self) -> ClkI2c4SrcEnW<PmucruClkgateCon0Spec> {
        ClkI2c4SrcEnW::new(self, 10)
    }
    #[doc = "Bit 11 - clk_i2c8_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c8_src_en(&mut self) -> ClkI2c8SrcEnW<PmucruClkgateCon0Spec> {
        ClkI2c8SrcEnW::new(self, 11)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<PmucruClkgateCon0Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_clkgate_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_clkgate_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmucruClkgateCon0Spec;
impl crate::RegisterSpec for PmucruClkgateCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmucru_clkgate_con0::R`](R) reader structure"]
impl crate::Readable for PmucruClkgateCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`pmucru_clkgate_con0::W`](W) writer structure"]
impl crate::Writable for PmucruClkgateCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUCRU_CLKGATE_CON0 to value 0"]
impl crate::Resettable for PmucruClkgateCon0Spec {
    const RESET_VALUE: u32 = 0;
}
