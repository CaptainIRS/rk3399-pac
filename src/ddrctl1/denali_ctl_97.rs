#[doc = "Register `DENALI_CTL_97` reader"]
pub type R = crate::R<DenaliCtl97Spec>;
#[doc = "Register `DENALI_CTL_97` writer"]
pub type W = crate::W<DenaliCtl97Spec>;
#[doc = "Field `LPI_SRPD_DEEP_MCCLK_GATE_WAKEUP_F1` reader - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down deep with memory and controller clock gating, for frequency copy 1."]
pub type LpiSrpdDeepMcclkGateWakeupF1R = crate::FieldReader;
#[doc = "Field `LPI_SRPD_DEEP_MCCLK_GATE_WAKEUP_F1` writer - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down deep with memory and controller clock gating, for frequency copy 1."]
pub type LpiSrpdDeepMcclkGateWakeupF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_TIMER_WAKEUP_F1` reader - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when the LPI timer expires for frequency copy 1."]
pub type LpiTimerWakeupF1R = crate::FieldReader;
#[doc = "Field `LPI_TIMER_WAKEUP_F1` writer - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when the LPI timer expires for frequency copy 1."]
pub type LpiTimerWakeupF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_PD_WAKEUP_F2` reader - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in power-down for frequency copy 2."]
pub type LpiPdWakeupF2R = crate::FieldReader;
#[doc = "Field `LPI_PD_WAKEUP_F2` writer - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in power-down for frequency copy 2."]
pub type LpiPdWakeupF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_SR_WAKEUP_F2` reader - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh for frequency copy 2."]
pub type LpiSrWakeupF2R = crate::FieldReader;
#[doc = "Field `LPI_SR_WAKEUP_F2` writer - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh for frequency copy 2."]
pub type LpiSrWakeupF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down deep with memory and controller clock gating, for frequency copy 1."]
    #[inline(always)]
    pub fn lpi_srpd_deep_mcclk_gate_wakeup_f1(&self) -> LpiSrpdDeepMcclkGateWakeupF1R {
        LpiSrpdDeepMcclkGateWakeupF1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when the LPI timer expires for frequency copy 1."]
    #[inline(always)]
    pub fn lpi_timer_wakeup_f1(&self) -> LpiTimerWakeupF1R {
        LpiTimerWakeupF1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in power-down for frequency copy 2."]
    #[inline(always)]
    pub fn lpi_pd_wakeup_f2(&self) -> LpiPdWakeupF2R {
        LpiPdWakeupF2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh for frequency copy 2."]
    #[inline(always)]
    pub fn lpi_sr_wakeup_f2(&self) -> LpiSrWakeupF2R {
        LpiSrWakeupF2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down deep with memory and controller clock gating, for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_srpd_deep_mcclk_gate_wakeup_f1(
        &mut self,
    ) -> LpiSrpdDeepMcclkGateWakeupF1W<DenaliCtl97Spec> {
        LpiSrpdDeepMcclkGateWakeupF1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when the LPI timer expires for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_timer_wakeup_f1(&mut self) -> LpiTimerWakeupF1W<DenaliCtl97Spec> {
        LpiTimerWakeupF1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in power-down for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_pd_wakeup_f2(&mut self) -> LpiPdWakeupF2W<DenaliCtl97Spec> {
        LpiPdWakeupF2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_sr_wakeup_f2(&mut self) -> LpiSrWakeupF2W<DenaliCtl97Spec> {
        LpiSrWakeupF2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_97::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_97::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl97Spec;
impl crate::RegisterSpec for DenaliCtl97Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_97::R`](R) reader structure"]
impl crate::Readable for DenaliCtl97Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_97::W`](W) writer structure"]
impl crate::Writable for DenaliCtl97Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_97 to value 0"]
impl crate::Resettable for DenaliCtl97Spec {
    const RESET_VALUE: u32 = 0;
}
