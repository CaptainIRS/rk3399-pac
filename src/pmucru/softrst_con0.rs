#[doc = "Register `SOFTRST_CON0` reader"]
pub type R = crate::R<SoftrstCon0Spec>;
#[doc = "Register `SOFTRST_CON0` writer"]
pub type W = crate::W<SoftrstCon0Spec>;
#[doc = "Field `PRESETN_NOC_PMU_REQ` reader - presetn_noc_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnNocPmuReqR = crate::BitReader;
#[doc = "Field `PRESETN_NOC_PMU_REQ` writer - presetn_noc_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnNocPmuReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_INTMEM_PMU_REQ` reader - presetn_intmem_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnIntmemPmuReqR = crate::BitReader;
#[doc = "Field `PRESETN_INTMEM_PMU_REQ` writer - presetn_intmem_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnIntmemPmuReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_CM0S_PMU_REQ` reader - hresetn_cm0s_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnCm0sPmuReqR = crate::BitReader;
#[doc = "Field `HRESETN_CM0S_PMU_REQ` writer - hresetn_cm0s_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnCm0sPmuReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_CM0S_NOC_PMU_REQ` reader - hresetn_cm0s_noc_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnCm0sNocPmuReqR = crate::BitReader;
#[doc = "Field `HRESETN_CM0S_NOC_PMU_REQ` writer - hresetn_cm0s_noc_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnCm0sNocPmuReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGRESETN_CM0S_PMU_REQ` reader - dbgresetn_cm0s_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type DbgresetnCm0sPmuReqR = crate::BitReader;
#[doc = "Field `DBGRESETN_CM0S_PMU_REQ` writer - dbgresetn_cm0s_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type DbgresetnCm0sPmuReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORESETN_CM0S_PMU_REQ` reader - poresetn_cm0s_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type PoresetnCm0sPmuReqR = crate::BitReader;
#[doc = "Field `PORESETN_CM0S_PMU_REQ` writer - poresetn_cm0s_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type PoresetnCm0sPmuReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_SPI3_REQ` reader - presetn_spi3 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnSpi3ReqR = crate::BitReader;
#[doc = "Field `PRESETN_SPI3_REQ` writer - presetn_spi3 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnSpi3ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_SPI3_REQ` reader - resetn_spi3 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnSpi3ReqR = crate::BitReader;
#[doc = "Field `RESETN_SPI3_REQ` writer - resetn_spi3 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnSpi3ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_TIMER_PMU_0_1_REQ` reader - presetn_timer_pmu_0_1 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnTimerPmu0_1ReqR = crate::BitReader;
#[doc = "Field `PRESETN_TIMER_PMU_0_1_REQ` writer - presetn_timer_pmu_0_1 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnTimerPmu0_1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_TIMER_PMU_0_REQ` reader - resetn_timer_pmu_0 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnTimerPmu0ReqR = crate::BitReader;
#[doc = "Field `RESETN_TIMER_PMU_0_REQ` writer - resetn_timer_pmu_0 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnTimerPmu0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_TIMER_PMU_1_REQ` reader - resetn_timer_pmu_1 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnTimerPmu1ReqR = crate::BitReader;
#[doc = "Field `RESETN_TIMER_PMU_1_REQ` writer - resetn_timer_pmu_1 request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnTimerPmu1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_UART_M0_PMU_REQ` reader - presetn_uart_m0_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnUartM0PmuReqR = crate::BitReader;
#[doc = "Field `PRESETN_UART_M0_PMU_REQ` writer - presetn_uart_m0_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnUartM0PmuReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_UART_M0_PMU_REQ` reader - resetn_uart_m0_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUartM0PmuReqR = crate::BitReader;
#[doc = "Field `RESETN_UART_M0_PMU_REQ` writer - resetn_uart_m0_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnUartM0PmuReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_WDT_PMU_REQ` reader - presetn_wdt_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnWdtPmuReqR = crate::BitReader;
#[doc = "Field `PRESETN_WDT_PMU_REQ` writer - presetn_wdt_pmu request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnWdtPmuReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - presetn_noc_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_noc_pmu_req(&self) -> PresetnNocPmuReqR {
        PresetnNocPmuReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - presetn_intmem_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_intmem_pmu_req(&self) -> PresetnIntmemPmuReqR {
        PresetnIntmemPmuReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - hresetn_cm0s_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_cm0s_pmu_req(&self) -> HresetnCm0sPmuReqR {
        HresetnCm0sPmuReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - hresetn_cm0s_noc_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_cm0s_noc_pmu_req(&self) -> HresetnCm0sNocPmuReqR {
        HresetnCm0sNocPmuReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - dbgresetn_cm0s_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn dbgresetn_cm0s_pmu_req(&self) -> DbgresetnCm0sPmuReqR {
        DbgresetnCm0sPmuReqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - poresetn_cm0s_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn poresetn_cm0s_pmu_req(&self) -> PoresetnCm0sPmuReqR {
        PoresetnCm0sPmuReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - presetn_spi3 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_spi3_req(&self) -> PresetnSpi3ReqR {
        PresetnSpi3ReqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - resetn_spi3 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_spi3_req(&self) -> ResetnSpi3ReqR {
        ResetnSpi3ReqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - presetn_timer_pmu_0_1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_timer_pmu_0_1_req(&self) -> PresetnTimerPmu0_1ReqR {
        PresetnTimerPmu0_1ReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - resetn_timer_pmu_0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_timer_pmu_0_req(&self) -> ResetnTimerPmu0ReqR {
        ResetnTimerPmu0ReqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - resetn_timer_pmu_1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_timer_pmu_1_req(&self) -> ResetnTimerPmu1ReqR {
        ResetnTimerPmu1ReqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - presetn_uart_m0_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_uart_m0_pmu_req(&self) -> PresetnUartM0PmuReqR {
        PresetnUartM0PmuReqR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - resetn_uart_m0_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_uart_m0_pmu_req(&self) -> ResetnUartM0PmuReqR {
        ResetnUartM0PmuReqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - presetn_wdt_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_wdt_pmu_req(&self) -> PresetnWdtPmuReqR {
        PresetnWdtPmuReqR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - presetn_noc_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_noc_pmu_req(&mut self) -> PresetnNocPmuReqW<SoftrstCon0Spec> {
        PresetnNocPmuReqW::new(self, 0)
    }
    #[doc = "Bit 1 - presetn_intmem_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_intmem_pmu_req(&mut self) -> PresetnIntmemPmuReqW<SoftrstCon0Spec> {
        PresetnIntmemPmuReqW::new(self, 1)
    }
    #[doc = "Bit 2 - hresetn_cm0s_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_cm0s_pmu_req(&mut self) -> HresetnCm0sPmuReqW<SoftrstCon0Spec> {
        HresetnCm0sPmuReqW::new(self, 2)
    }
    #[doc = "Bit 3 - hresetn_cm0s_noc_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_cm0s_noc_pmu_req(&mut self) -> HresetnCm0sNocPmuReqW<SoftrstCon0Spec> {
        HresetnCm0sNocPmuReqW::new(self, 3)
    }
    #[doc = "Bit 4 - dbgresetn_cm0s_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn dbgresetn_cm0s_pmu_req(&mut self) -> DbgresetnCm0sPmuReqW<SoftrstCon0Spec> {
        DbgresetnCm0sPmuReqW::new(self, 4)
    }
    #[doc = "Bit 5 - poresetn_cm0s_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn poresetn_cm0s_pmu_req(&mut self) -> PoresetnCm0sPmuReqW<SoftrstCon0Spec> {
        PoresetnCm0sPmuReqW::new(self, 5)
    }
    #[doc = "Bit 6 - presetn_spi3 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_spi3_req(&mut self) -> PresetnSpi3ReqW<SoftrstCon0Spec> {
        PresetnSpi3ReqW::new(self, 6)
    }
    #[doc = "Bit 7 - resetn_spi3 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_spi3_req(&mut self) -> ResetnSpi3ReqW<SoftrstCon0Spec> {
        ResetnSpi3ReqW::new(self, 7)
    }
    #[doc = "Bit 8 - presetn_timer_pmu_0_1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_timer_pmu_0_1_req(&mut self) -> PresetnTimerPmu0_1ReqW<SoftrstCon0Spec> {
        PresetnTimerPmu0_1ReqW::new(self, 8)
    }
    #[doc = "Bit 9 - resetn_timer_pmu_0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_timer_pmu_0_req(&mut self) -> ResetnTimerPmu0ReqW<SoftrstCon0Spec> {
        ResetnTimerPmu0ReqW::new(self, 9)
    }
    #[doc = "Bit 10 - resetn_timer_pmu_1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_timer_pmu_1_req(&mut self) -> ResetnTimerPmu1ReqW<SoftrstCon0Spec> {
        ResetnTimerPmu1ReqW::new(self, 10)
    }
    #[doc = "Bit 11 - presetn_uart_m0_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_uart_m0_pmu_req(&mut self) -> PresetnUartM0PmuReqW<SoftrstCon0Spec> {
        PresetnUartM0PmuReqW::new(self, 11)
    }
    #[doc = "Bit 12 - resetn_uart_m0_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_uart_m0_pmu_req(&mut self) -> ResetnUartM0PmuReqW<SoftrstCon0Spec> {
        ResetnUartM0PmuReqW::new(self, 12)
    }
    #[doc = "Bit 13 - presetn_wdt_pmu request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_wdt_pmu_req(&mut self) -> PresetnWdtPmuReqW<SoftrstCon0Spec> {
        PresetnWdtPmuReqW::new(self, 13)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<SoftrstCon0Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal software reset control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SoftrstCon0Spec;
impl crate::RegisterSpec for SoftrstCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`softrst_con0::R`](R) reader structure"]
impl crate::Readable for SoftrstCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`softrst_con0::W`](W) writer structure"]
impl crate::Writable for SoftrstCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOFTRST_CON0 to value 0x24"]
impl crate::Resettable for SoftrstCon0Spec {
    const RESET_VALUE: u32 = 0x24;
}
