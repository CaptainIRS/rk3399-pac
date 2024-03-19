#[doc = "Register `DDR_DENALI_CTL_96` reader"]
pub type R = crate::R<DdrDenaliCtl96Spec>;
#[doc = "Register `DDR_DENALI_CTL_96` writer"]
pub type W = crate::W<DdrDenaliCtl96Spec>;
#[doc = "Field `LPI_SR_WAKEUP_F1` reader - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh for frequency copy 1."]
pub type LpiSrWakeupF1R = crate::FieldReader;
#[doc = "Field `LPI_SR_WAKEUP_F1` writer - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh for frequency copy 1."]
pub type LpiSrWakeupF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_SR_MCCLK_GATE_WAKEUP_F1` reader - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh with memory and controller clock gating, for frequency copy 1."]
pub type LpiSrMcclkGateWakeupF1R = crate::FieldReader;
#[doc = "Field `LPI_SR_MCCLK_GATE_WAKEUP_F1` writer - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh with memory and controller clock gating, for frequency copy 1."]
pub type LpiSrMcclkGateWakeupF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_SRPD_LITE_WAKEUP_F1` reader - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down lite, for frequncy copy 1."]
pub type LpiSrpdLiteWakeupF1R = crate::FieldReader;
#[doc = "Field `LPI_SRPD_LITE_WAKEUP_F1` writer - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down lite, for frequncy copy 1."]
pub type LpiSrpdLiteWakeupF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_SRPD_DEEP_WAKEUP_F1` reader - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down deep, for frequency copy 1."]
pub type LpiSrpdDeepWakeupF1R = crate::FieldReader;
#[doc = "Field `LPI_SRPD_DEEP_WAKEUP_F1` writer - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down deep, for frequency copy 1."]
pub type LpiSrpdDeepWakeupF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh for frequency copy 1."]
    #[inline(always)]
    pub fn lpi_sr_wakeup_f1(&self) -> LpiSrWakeupF1R {
        LpiSrWakeupF1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh with memory and controller clock gating, for frequency copy 1."]
    #[inline(always)]
    pub fn lpi_sr_mcclk_gate_wakeup_f1(&self) -> LpiSrMcclkGateWakeupF1R {
        LpiSrMcclkGateWakeupF1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down lite, for frequncy copy 1."]
    #[inline(always)]
    pub fn lpi_srpd_lite_wakeup_f1(&self) -> LpiSrpdLiteWakeupF1R {
        LpiSrpdLiteWakeupF1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down deep, for frequency copy 1."]
    #[inline(always)]
    pub fn lpi_srpd_deep_wakeup_f1(&self) -> LpiSrpdDeepWakeupF1R {
        LpiSrpdDeepWakeupF1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_sr_wakeup_f1(&mut self) -> LpiSrWakeupF1W<DdrDenaliCtl96Spec> {
        LpiSrWakeupF1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh with memory and controller clock gating, for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_sr_mcclk_gate_wakeup_f1(&mut self) -> LpiSrMcclkGateWakeupF1W<DdrDenaliCtl96Spec> {
        LpiSrMcclkGateWakeupF1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down lite, for frequncy copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_srpd_lite_wakeup_f1(&mut self) -> LpiSrpdLiteWakeupF1W<DdrDenaliCtl96Spec> {
        LpiSrpdLiteWakeupF1W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down deep, for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_srpd_deep_wakeup_f1(&mut self) -> LpiSrpdDeepWakeupF1W<DdrDenaliCtl96Spec> {
        LpiSrpdDeepWakeupF1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_96::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_96::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl96Spec;
impl crate::RegisterSpec for DdrDenaliCtl96Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_96::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl96Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_96::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl96Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_96 to value 0"]
impl crate::Resettable for DdrDenaliCtl96Spec {
    const RESET_VALUE: u32 = 0;
}
