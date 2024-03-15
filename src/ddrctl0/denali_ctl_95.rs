#[doc = "Register `DENALI_CTL_95` reader"]
pub type R = crate::R<DenaliCtl95Spec>;
#[doc = "Register `DENALI_CTL_95` writer"]
pub type W = crate::W<DenaliCtl95Spec>;
#[doc = "Field `LPI_SRPD_DEEP_WAKEUP_F0` reader - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down deep, for frequency copy 0."]
pub type LpiSrpdDeepWakeupF0R = crate::FieldReader;
#[doc = "Field `LPI_SRPD_DEEP_WAKEUP_F0` writer - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down deep, for frequency copy 0."]
pub type LpiSrpdDeepWakeupF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_SRPD_DEEP_MCCLK_GATE_WAKEUP_F0` reader - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down deep with memory and controller clock gating, for frequency copy 0."]
pub type LpiSrpdDeepMcclkGateWakeupF0R = crate::FieldReader;
#[doc = "Field `LPI_SRPD_DEEP_MCCLK_GATE_WAKEUP_F0` writer - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down deep with memory and controller clock gating, for frequency copy 0."]
pub type LpiSrpdDeepMcclkGateWakeupF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_TIMER_WAKEUP_F0` reader - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when the LPI timer expires for frequency copy 0."]
pub type LpiTimerWakeupF0R = crate::FieldReader;
#[doc = "Field `LPI_TIMER_WAKEUP_F0` writer - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when the LPI timer expires for frequency copy 0."]
pub type LpiTimerWakeupF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_PD_WAKEUP_F1` reader - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in power-down for frequency copy 1."]
pub type LpiPdWakeupF1R = crate::FieldReader;
#[doc = "Field `LPI_PD_WAKEUP_F1` writer - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in power-down for frequency copy 1."]
pub type LpiPdWakeupF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down deep, for frequency copy 0."]
    #[inline(always)]
    pub fn lpi_srpd_deep_wakeup_f0(&self) -> LpiSrpdDeepWakeupF0R {
        LpiSrpdDeepWakeupF0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down deep with memory and controller clock gating, for frequency copy 0."]
    #[inline(always)]
    pub fn lpi_srpd_deep_mcclk_gate_wakeup_f0(&self) -> LpiSrpdDeepMcclkGateWakeupF0R {
        LpiSrpdDeepMcclkGateWakeupF0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when the LPI timer expires for frequency copy 0."]
    #[inline(always)]
    pub fn lpi_timer_wakeup_f0(&self) -> LpiTimerWakeupF0R {
        LpiTimerWakeupF0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in power-down for frequency copy 1."]
    #[inline(always)]
    pub fn lpi_pd_wakeup_f1(&self) -> LpiPdWakeupF1R {
        LpiPdWakeupF1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down deep, for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_srpd_deep_wakeup_f0(&mut self) -> LpiSrpdDeepWakeupF0W<DenaliCtl95Spec> {
        LpiSrpdDeepWakeupF0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down deep with memory and controller clock gating, for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_srpd_deep_mcclk_gate_wakeup_f0(
        &mut self,
    ) -> LpiSrpdDeepMcclkGateWakeupF0W<DenaliCtl95Spec> {
        LpiSrpdDeepMcclkGateWakeupF0W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when the LPI timer expires for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_timer_wakeup_f0(&mut self) -> LpiTimerWakeupF0W<DenaliCtl95Spec> {
        LpiTimerWakeupF0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in power-down for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_pd_wakeup_f1(&mut self) -> LpiPdWakeupF1W<DenaliCtl95Spec> {
        LpiPdWakeupF1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_95::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_95::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl95Spec;
impl crate::RegisterSpec for DenaliCtl95Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_95::R`](R) reader structure"]
impl crate::Readable for DenaliCtl95Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_95::W`](W) writer structure"]
impl crate::Writable for DenaliCtl95Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_95 to value 0"]
impl crate::Resettable for DenaliCtl95Spec {
    const RESET_VALUE: u32 = 0;
}
