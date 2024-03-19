#[doc = "Register `PMUCRU_RSTNHOLD_CON0` reader"]
pub type R = crate::R<PmucruRstnholdCon0Spec>;
#[doc = "Register `PMUCRU_RSTNHOLD_CON0` writer"]
pub type W = crate::W<PmucruRstnholdCon0Spec>;
#[doc = "Field `PRESETN_NOC_PMU_HOLD` reader - presetn_noc_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnNocPmuHoldR = crate::BitReader;
#[doc = "Field `PRESETN_NOC_PMU_HOLD` writer - presetn_noc_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnNocPmuHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_INTMEM_PMU_HOLD` reader - presetn_intmem_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnIntmemPmuHoldR = crate::BitReader;
#[doc = "Field `PRESETN_INTMEM_PMU_HOLD` writer - presetn_intmem_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnIntmemPmuHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_CM0S_PMU_HOLD` reader - hresetn_cm0s_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type HresetnCm0sPmuHoldR = crate::BitReader;
#[doc = "Field `HRESETN_CM0S_PMU_HOLD` writer - hresetn_cm0s_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type HresetnCm0sPmuHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_CM0S_NOC_PMU_HOLD` reader - hresetn_cm0s_noc_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type HresetnCm0sNocPmuHoldR = crate::BitReader;
#[doc = "Field `HRESETN_CM0S_NOC_PMU_HOLD` writer - hresetn_cm0s_noc_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type HresetnCm0sNocPmuHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGRESETN_CM0S_PMU_HOLD` reader - dbgresetn_cm0s_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type DbgresetnCm0sPmuHoldR = crate::BitReader;
#[doc = "Field `DBGRESETN_CM0S_PMU_HOLD` writer - dbgresetn_cm0s_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type DbgresetnCm0sPmuHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORESETN_CM0S_PMU_HOLD` reader - poresetn_cm0s_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PoresetnCm0sPmuHoldR = crate::BitReader;
#[doc = "Field `PORESETN_CM0S_PMU_HOLD` writer - poresetn_cm0s_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PoresetnCm0sPmuHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_SPI3_HOLD` reader - presetn_spi3_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnSpi3HoldR = crate::BitReader;
#[doc = "Field `PRESETN_SPI3_HOLD` writer - presetn_spi3_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnSpi3HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_SPI3_HOLD` reader - resetn_spi3_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type ResetnSpi3HoldR = crate::BitReader;
#[doc = "Field `RESETN_SPI3_HOLD` writer - resetn_spi3_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type ResetnSpi3HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_TIMER_PMU_0_1_HOLD` reader - presetn_timer_pmu_0_1_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnTimerPmu0_1HoldR = crate::BitReader;
#[doc = "Field `PRESETN_TIMER_PMU_0_1_HOLD` writer - presetn_timer_pmu_0_1_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnTimerPmu0_1HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_TIMER_PMU_0_HOLD` reader - resetn_timer_pmu_0_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type ResetnTimerPmu0HoldR = crate::BitReader;
#[doc = "Field `RESETN_TIMER_PMU_0_HOLD` writer - resetn_timer_pmu_0_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type ResetnTimerPmu0HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_TIMER_PMU_1_HOLD` reader - resetn_timer_pmu_1_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type ResetnTimerPmu1HoldR = crate::BitReader;
#[doc = "Field `RESETN_TIMER_PMU_1_HOLD` writer - resetn_timer_pmu_1_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type ResetnTimerPmu1HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_UART_M0_PMU_HOLD` reader - presetn_uart_m0_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnUartM0PmuHoldR = crate::BitReader;
#[doc = "Field `PRESETN_UART_M0_PMU_HOLD` writer - presetn_uart_m0_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnUartM0PmuHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_UART_M0_PMU_HOLD` reader - resetn_uart_m0_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type ResetnUartM0PmuHoldR = crate::BitReader;
#[doc = "Field `RESETN_UART_M0_PMU_HOLD` writer - resetn_uart_m0_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type ResetnUartM0PmuHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_WDT_PMU_HOLD` reader - presetn_wdt_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnWdtPmuHoldR = crate::BitReader;
#[doc = "Field `PRESETN_WDT_PMU_HOLD` writer - presetn_wdt_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
pub type PresetnWdtPmuHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 1 - presetn_noc_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn presetn_noc_pmu_hold(&self) -> PresetnNocPmuHoldR {
        PresetnNocPmuHoldR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - presetn_intmem_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn presetn_intmem_pmu_hold(&self) -> PresetnIntmemPmuHoldR {
        PresetnIntmemPmuHoldR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - hresetn_cm0s_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn hresetn_cm0s_pmu_hold(&self) -> HresetnCm0sPmuHoldR {
        HresetnCm0sPmuHoldR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - hresetn_cm0s_noc_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn hresetn_cm0s_noc_pmu_hold(&self) -> HresetnCm0sNocPmuHoldR {
        HresetnCm0sNocPmuHoldR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - dbgresetn_cm0s_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn dbgresetn_cm0s_pmu_hold(&self) -> DbgresetnCm0sPmuHoldR {
        DbgresetnCm0sPmuHoldR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - poresetn_cm0s_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn poresetn_cm0s_pmu_hold(&self) -> PoresetnCm0sPmuHoldR {
        PoresetnCm0sPmuHoldR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - presetn_spi3_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn presetn_spi3_hold(&self) -> PresetnSpi3HoldR {
        PresetnSpi3HoldR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - resetn_spi3_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn resetn_spi3_hold(&self) -> ResetnSpi3HoldR {
        ResetnSpi3HoldR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - presetn_timer_pmu_0_1_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn presetn_timer_pmu_0_1_hold(&self) -> PresetnTimerPmu0_1HoldR {
        PresetnTimerPmu0_1HoldR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - resetn_timer_pmu_0_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn resetn_timer_pmu_0_hold(&self) -> ResetnTimerPmu0HoldR {
        ResetnTimerPmu0HoldR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - resetn_timer_pmu_1_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn resetn_timer_pmu_1_hold(&self) -> ResetnTimerPmu1HoldR {
        ResetnTimerPmu1HoldR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - presetn_uart_m0_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn presetn_uart_m0_pmu_hold(&self) -> PresetnUartM0PmuHoldR {
        PresetnUartM0PmuHoldR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - resetn_uart_m0_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn resetn_uart_m0_pmu_hold(&self) -> ResetnUartM0PmuHoldR {
        ResetnUartM0PmuHoldR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - presetn_wdt_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    pub fn presetn_wdt_pmu_hold(&self) -> PresetnWdtPmuHoldR {
        PresetnWdtPmuHoldR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - presetn_noc_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_noc_pmu_hold(&mut self) -> PresetnNocPmuHoldW<PmucruRstnholdCon0Spec> {
        PresetnNocPmuHoldW::new(self, 1)
    }
    #[doc = "Bit 2 - presetn_intmem_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_intmem_pmu_hold(&mut self) -> PresetnIntmemPmuHoldW<PmucruRstnholdCon0Spec> {
        PresetnIntmemPmuHoldW::new(self, 2)
    }
    #[doc = "Bit 3 - hresetn_cm0s_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_cm0s_pmu_hold(&mut self) -> HresetnCm0sPmuHoldW<PmucruRstnholdCon0Spec> {
        HresetnCm0sPmuHoldW::new(self, 3)
    }
    #[doc = "Bit 4 - hresetn_cm0s_noc_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_cm0s_noc_pmu_hold(&mut self) -> HresetnCm0sNocPmuHoldW<PmucruRstnholdCon0Spec> {
        HresetnCm0sNocPmuHoldW::new(self, 4)
    }
    #[doc = "Bit 5 - dbgresetn_cm0s_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn dbgresetn_cm0s_pmu_hold(&mut self) -> DbgresetnCm0sPmuHoldW<PmucruRstnholdCon0Spec> {
        DbgresetnCm0sPmuHoldW::new(self, 5)
    }
    #[doc = "Bit 6 - poresetn_cm0s_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn poresetn_cm0s_pmu_hold(&mut self) -> PoresetnCm0sPmuHoldW<PmucruRstnholdCon0Spec> {
        PoresetnCm0sPmuHoldW::new(self, 6)
    }
    #[doc = "Bit 7 - presetn_spi3_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_spi3_hold(&mut self) -> PresetnSpi3HoldW<PmucruRstnholdCon0Spec> {
        PresetnSpi3HoldW::new(self, 7)
    }
    #[doc = "Bit 8 - resetn_spi3_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_spi3_hold(&mut self) -> ResetnSpi3HoldW<PmucruRstnholdCon0Spec> {
        ResetnSpi3HoldW::new(self, 8)
    }
    #[doc = "Bit 9 - presetn_timer_pmu_0_1_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_timer_pmu_0_1_hold(
        &mut self,
    ) -> PresetnTimerPmu0_1HoldW<PmucruRstnholdCon0Spec> {
        PresetnTimerPmu0_1HoldW::new(self, 9)
    }
    #[doc = "Bit 10 - resetn_timer_pmu_0_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_timer_pmu_0_hold(&mut self) -> ResetnTimerPmu0HoldW<PmucruRstnholdCon0Spec> {
        ResetnTimerPmu0HoldW::new(self, 10)
    }
    #[doc = "Bit 11 - resetn_timer_pmu_1_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_timer_pmu_1_hold(&mut self) -> ResetnTimerPmu1HoldW<PmucruRstnholdCon0Spec> {
        ResetnTimerPmu1HoldW::new(self, 11)
    }
    #[doc = "Bit 12 - presetn_uart_m0_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_uart_m0_pmu_hold(&mut self) -> PresetnUartM0PmuHoldW<PmucruRstnholdCon0Spec> {
        PresetnUartM0PmuHoldW::new(self, 12)
    }
    #[doc = "Bit 13 - resetn_uart_m0_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_uart_m0_pmu_hold(&mut self) -> ResetnUartM0PmuHoldW<PmucruRstnholdCon0Spec> {
        ResetnUartM0PmuHoldW::new(self, 13)
    }
    #[doc = "Bit 14 - presetn_wdt_pmu_hold control bit\n\nWhen HIGH, reset hold, can't be reset by any reset source"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_wdt_pmu_hold(&mut self) -> PresetnWdtPmuHoldW<PmucruRstnholdCon0Spec> {
        PresetnWdtPmuHoldW::new(self, 14)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<PmucruRstnholdCon0Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal reset hold control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_rstnhold_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_rstnhold_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmucruRstnholdCon0Spec;
impl crate::RegisterSpec for PmucruRstnholdCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmucru_rstnhold_con0::R`](R) reader structure"]
impl crate::Readable for PmucruRstnholdCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`pmucru_rstnhold_con0::W`](W) writer structure"]
impl crate::Writable for PmucruRstnholdCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUCRU_RSTNHOLD_CON0 to value 0"]
impl crate::Resettable for PmucruRstnholdCon0Spec {
    const RESET_VALUE: u32 = 0;
}
