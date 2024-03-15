#[doc = "Register `PMUCRU_CLKGATE_CON1` reader"]
pub type R = crate::R<PmucruClkgateCon1Spec>;
#[doc = "Register `PMUCRU_CLKGATE_CON1` writer"]
pub type W = crate::W<PmucruClkgateCon1Spec>;
#[doc = "Field `PCLK_PMU_EN` reader - pclk_pmu clock disable bit When HIGH, disable clock"]
pub type PclkPmuEnR = crate::BitReader;
#[doc = "Field `PCLK_PMU_EN` writer - pclk_pmu clock disable bit When HIGH, disable clock"]
pub type PclkPmuEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_PMUGRF_EN` reader - pclk_pmugrf clock disable bit When HIGH, disable clock Suggest always on"]
pub type PclkPmugrfEnR = crate::BitReader;
#[doc = "Field `PCLK_PMUGRF_EN` writer - pclk_pmugrf clock disable bit When HIGH, disable clock Suggest always on"]
pub type PclkPmugrfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_INTMEM1_EN` reader - pclk_intmem1 clock disable bit When HIGH, disable clock"]
pub type PclkIntmem1EnR = crate::BitReader;
#[doc = "Field `PCLK_INTMEM1_EN` writer - pclk_intmem1 clock disable bit When HIGH, disable clock"]
pub type PclkIntmem1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_GPIO0_EN` reader - pclk_gpio0 clock disable bit When HIGH, disable clock"]
pub type PclkGpio0EnR = crate::BitReader;
#[doc = "Field `PCLK_GPIO0_EN` writer - pclk_gpio0 clock disable bit When HIGH, disable clock"]
pub type PclkGpio0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_GPIO1_EN` reader - pclk_gpio1 clock disable bit When HIGH, disable clock"]
pub type PclkGpio1EnR = crate::BitReader;
#[doc = "Field `PCLK_GPIO1_EN` writer - pclk_gpio1 clock disable bit When HIGH, disable clock"]
pub type PclkGpio1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_SGRF_EN` reader - pclk_sgrf clock disable bit When HIGH, disable clock Suggest always on"]
pub type PclkSgrfEnR = crate::BitReader;
#[doc = "Field `PCLK_SGRF_EN` writer - pclk_sgrf clock disable bit When HIGH, disable clock Suggest always on"]
pub type PclkSgrfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_NOC_PMU_EN` reader - pclk_noc_pmu clock disable bit When HIGH, disable clock Suggest always on"]
pub type PclkNocPmuEnR = crate::BitReader;
#[doc = "Field `PCLK_NOC_PMU_EN` writer - pclk_noc_pmu clock disable bit When HIGH, disable clock Suggest always on"]
pub type PclkNocPmuEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_I2C0_EN` reader - pclk_i2c0 clock disable bit When HIGH, disable clock"]
pub type PclkI2c0EnR = crate::BitReader;
#[doc = "Field `PCLK_I2C0_EN` writer - pclk_i2c0 clock disable bit When HIGH, disable clock"]
pub type PclkI2c0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_I2C4_EN` reader - pclk_i2c4 clock disable bit When HIGH, disable clock"]
pub type PclkI2c4EnR = crate::BitReader;
#[doc = "Field `PCLK_I2C4_EN` writer - pclk_i2c4 clock disable bit When HIGH, disable clock"]
pub type PclkI2c4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_I2C8_EN` reader - pclk_i2c8 clock disable bit When HIGH, disable clock"]
pub type PclkI2c8EnR = crate::BitReader;
#[doc = "Field `PCLK_I2C8_EN` writer - pclk_i2c8 clock disable bit When HIGH, disable clock"]
pub type PclkI2c8EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_RKPWM_PMU_EN` reader - pclk_rkpwm_pmu clock disable bit When HIGH, disable clock"]
pub type PclkRkpwmPmuEnR = crate::BitReader;
#[doc = "Field `PCLK_RKPWM_PMU_EN` writer - pclk_rkpwm_pmu clock disable bit When HIGH, disable clock"]
pub type PclkRkpwmPmuEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_SPI3_EN` reader - pclk_spi3 clock disable bit When HIGH, disable clock"]
pub type PclkSpi3EnR = crate::BitReader;
#[doc = "Field `PCLK_SPI3_EN` writer - pclk_spi3 clock disable bit When HIGH, disable clock"]
pub type PclkSpi3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_TIMER_PMU_EN` reader - pclk_timer_pmu clock disable bit When HIGH, disable clock"]
pub type PclkTimerPmuEnR = crate::BitReader;
#[doc = "Field `PCLK_TIMER_PMU_EN` writer - pclk_timer_pmu clock disable bit When HIGH, disable clock"]
pub type PclkTimerPmuEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_MAILBOX_PMU_EN` reader - pclk_mailbox_pmu clock disable bit When HIGH, disable clock"]
pub type PclkMailboxPmuEnR = crate::BitReader;
#[doc = "Field `PCLK_MAILBOX_PMU_EN` writer - pclk_mailbox_pmu clock disable bit When HIGH, disable clock"]
pub type PclkMailboxPmuEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_UARTM0_EN` reader - pclk_uartm0 clock disable bit When HIGH, disable clock"]
pub type PclkUartm0EnR = crate::BitReader;
#[doc = "Field `PCLK_UARTM0_EN` writer - pclk_uartm0 clock disable bit When HIGH, disable clock"]
pub type PclkUartm0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_WDT_M0_PMU_EN` reader - pclk_wdt_m0_pmu clock disable bit When HIGH, disable clock"]
pub type PclkWdtM0PmuEnR = crate::BitReader;
#[doc = "Field `PCLK_WDT_M0_PMU_EN` writer - pclk_wdt_m0_pmu clock disable bit When HIGH, disable clock"]
pub type PclkWdtM0PmuEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - pclk_pmu clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_pmu_en(&self) -> PclkPmuEnR {
        PclkPmuEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - pclk_pmugrf clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn pclk_pmugrf_en(&self) -> PclkPmugrfEnR {
        PclkPmugrfEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - pclk_intmem1 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_intmem1_en(&self) -> PclkIntmem1EnR {
        PclkIntmem1EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - pclk_gpio0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_gpio0_en(&self) -> PclkGpio0EnR {
        PclkGpio0EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - pclk_gpio1 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_gpio1_en(&self) -> PclkGpio1EnR {
        PclkGpio1EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - pclk_sgrf clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn pclk_sgrf_en(&self) -> PclkSgrfEnR {
        PclkSgrfEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - pclk_noc_pmu clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn pclk_noc_pmu_en(&self) -> PclkNocPmuEnR {
        PclkNocPmuEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - pclk_i2c0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_i2c0_en(&self) -> PclkI2c0EnR {
        PclkI2c0EnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - pclk_i2c4 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_i2c4_en(&self) -> PclkI2c4EnR {
        PclkI2c4EnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - pclk_i2c8 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_i2c8_en(&self) -> PclkI2c8EnR {
        PclkI2c8EnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - pclk_rkpwm_pmu clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_rkpwm_pmu_en(&self) -> PclkRkpwmPmuEnR {
        PclkRkpwmPmuEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - pclk_spi3 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_spi3_en(&self) -> PclkSpi3EnR {
        PclkSpi3EnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - pclk_timer_pmu clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_timer_pmu_en(&self) -> PclkTimerPmuEnR {
        PclkTimerPmuEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - pclk_mailbox_pmu clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_mailbox_pmu_en(&self) -> PclkMailboxPmuEnR {
        PclkMailboxPmuEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - pclk_uartm0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_uartm0_en(&self) -> PclkUartm0EnR {
        PclkUartm0EnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - pclk_wdt_m0_pmu clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_wdt_m0_pmu_en(&self) -> PclkWdtM0PmuEnR {
        PclkWdtM0PmuEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - pclk_pmu clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_pmu_en(&mut self) -> PclkPmuEnW<PmucruClkgateCon1Spec> {
        PclkPmuEnW::new(self, 0)
    }
    #[doc = "Bit 1 - pclk_pmugrf clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_pmugrf_en(&mut self) -> PclkPmugrfEnW<PmucruClkgateCon1Spec> {
        PclkPmugrfEnW::new(self, 1)
    }
    #[doc = "Bit 2 - pclk_intmem1 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_intmem1_en(&mut self) -> PclkIntmem1EnW<PmucruClkgateCon1Spec> {
        PclkIntmem1EnW::new(self, 2)
    }
    #[doc = "Bit 3 - pclk_gpio0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_gpio0_en(&mut self) -> PclkGpio0EnW<PmucruClkgateCon1Spec> {
        PclkGpio0EnW::new(self, 3)
    }
    #[doc = "Bit 4 - pclk_gpio1 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_gpio1_en(&mut self) -> PclkGpio1EnW<PmucruClkgateCon1Spec> {
        PclkGpio1EnW::new(self, 4)
    }
    #[doc = "Bit 5 - pclk_sgrf clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_sgrf_en(&mut self) -> PclkSgrfEnW<PmucruClkgateCon1Spec> {
        PclkSgrfEnW::new(self, 5)
    }
    #[doc = "Bit 6 - pclk_noc_pmu clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_noc_pmu_en(&mut self) -> PclkNocPmuEnW<PmucruClkgateCon1Spec> {
        PclkNocPmuEnW::new(self, 6)
    }
    #[doc = "Bit 7 - pclk_i2c0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_i2c0_en(&mut self) -> PclkI2c0EnW<PmucruClkgateCon1Spec> {
        PclkI2c0EnW::new(self, 7)
    }
    #[doc = "Bit 8 - pclk_i2c4 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_i2c4_en(&mut self) -> PclkI2c4EnW<PmucruClkgateCon1Spec> {
        PclkI2c4EnW::new(self, 8)
    }
    #[doc = "Bit 9 - pclk_i2c8 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_i2c8_en(&mut self) -> PclkI2c8EnW<PmucruClkgateCon1Spec> {
        PclkI2c8EnW::new(self, 9)
    }
    #[doc = "Bit 10 - pclk_rkpwm_pmu clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_rkpwm_pmu_en(&mut self) -> PclkRkpwmPmuEnW<PmucruClkgateCon1Spec> {
        PclkRkpwmPmuEnW::new(self, 10)
    }
    #[doc = "Bit 11 - pclk_spi3 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_spi3_en(&mut self) -> PclkSpi3EnW<PmucruClkgateCon1Spec> {
        PclkSpi3EnW::new(self, 11)
    }
    #[doc = "Bit 12 - pclk_timer_pmu clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_timer_pmu_en(&mut self) -> PclkTimerPmuEnW<PmucruClkgateCon1Spec> {
        PclkTimerPmuEnW::new(self, 12)
    }
    #[doc = "Bit 13 - pclk_mailbox_pmu clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_mailbox_pmu_en(&mut self) -> PclkMailboxPmuEnW<PmucruClkgateCon1Spec> {
        PclkMailboxPmuEnW::new(self, 13)
    }
    #[doc = "Bit 14 - pclk_uartm0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_uartm0_en(&mut self) -> PclkUartm0EnW<PmucruClkgateCon1Spec> {
        PclkUartm0EnW::new(self, 14)
    }
    #[doc = "Bit 15 - pclk_wdt_m0_pmu clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_wdt_m0_pmu_en(&mut self) -> PclkWdtM0PmuEnW<PmucruClkgateCon1Spec> {
        PclkWdtM0PmuEnW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<PmucruClkgateCon1Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_clkgate_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_clkgate_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmucruClkgateCon1Spec;
impl crate::RegisterSpec for PmucruClkgateCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmucru_clkgate_con1::R`](R) reader structure"]
impl crate::Readable for PmucruClkgateCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`pmucru_clkgate_con1::W`](W) writer structure"]
impl crate::Writable for PmucruClkgateCon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUCRU_CLKGATE_CON1 to value 0"]
impl crate::Resettable for PmucruClkgateCon1Spec {
    const RESET_VALUE: u32 = 0;
}
