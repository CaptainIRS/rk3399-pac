#[doc = "Register `DDR_DENALI_CTL_98` reader"]
pub type R = crate::R<DdrDenaliCtl98Spec>;
#[doc = "Register `DDR_DENALI_CTL_98` writer"]
pub type W = crate::W<DdrDenaliCtl98Spec>;
#[doc = "Field `LPI_SR_MCCLK_GATE_WAKEUP_F2` reader - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh with memory and controller clock gating, for frequency copy 2."]
pub type LpiSrMcclkGateWakeupF2R = crate::FieldReader;
#[doc = "Field `LPI_SR_MCCLK_GATE_WAKEUP_F2` writer - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh with memory and controller clock gating, for frequency copy 2."]
pub type LpiSrMcclkGateWakeupF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_SRPD_LITE_WAKEUP_F2` reader - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down lite, for frequncy copy 2."]
pub type LpiSrpdLiteWakeupF2R = crate::FieldReader;
#[doc = "Field `LPI_SRPD_LITE_WAKEUP_F2` writer - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down lite, for frequncy copy 2."]
pub type LpiSrpdLiteWakeupF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_SRPD_DEEP_WAKEUP_F2` reader - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down deep, for frequency copy 2."]
pub type LpiSrpdDeepWakeupF2R = crate::FieldReader;
#[doc = "Field `LPI_SRPD_DEEP_WAKEUP_F2` writer - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down deep, for frequency copy 2."]
pub type LpiSrpdDeepWakeupF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_SRPD_DEEP_MCCLK_GATE_WAKEUP_F2` reader - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down deep with memory and controller clock gating, for frequency copy 2."]
pub type LpiSrpdDeepMcclkGateWakeupF2R = crate::FieldReader;
#[doc = "Field `LPI_SRPD_DEEP_MCCLK_GATE_WAKEUP_F2` writer - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down deep with memory and controller clock gating, for frequency copy 2."]
pub type LpiSrpdDeepMcclkGateWakeupF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh with memory and controller clock gating, for frequency copy 2."]
    #[inline(always)]
    pub fn lpi_sr_mcclk_gate_wakeup_f2(&self) -> LpiSrMcclkGateWakeupF2R {
        LpiSrMcclkGateWakeupF2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down lite, for frequncy copy 2."]
    #[inline(always)]
    pub fn lpi_srpd_lite_wakeup_f2(&self) -> LpiSrpdLiteWakeupF2R {
        LpiSrpdLiteWakeupF2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down deep, for frequency copy 2."]
    #[inline(always)]
    pub fn lpi_srpd_deep_wakeup_f2(&self) -> LpiSrpdDeepWakeupF2R {
        LpiSrpdDeepWakeupF2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down deep with memory and controller clock gating, for frequency copy 2."]
    #[inline(always)]
    pub fn lpi_srpd_deep_mcclk_gate_wakeup_f2(&self) -> LpiSrpdDeepMcclkGateWakeupF2R {
        LpiSrpdDeepMcclkGateWakeupF2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh with memory and controller clock gating, for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_sr_mcclk_gate_wakeup_f2(&mut self) -> LpiSrMcclkGateWakeupF2W<DdrDenaliCtl98Spec> {
        LpiSrMcclkGateWakeupF2W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down lite, for frequncy copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_srpd_lite_wakeup_f2(&mut self) -> LpiSrpdLiteWakeupF2W<DdrDenaliCtl98Spec> {
        LpiSrpdLiteWakeupF2W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down deep, for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_srpd_deep_wakeup_f2(&mut self) -> LpiSrpdDeepWakeupF2W<DdrDenaliCtl98Spec> {
        LpiSrpdDeepWakeupF2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down deep with memory and controller clock gating, for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_srpd_deep_mcclk_gate_wakeup_f2(
        &mut self,
    ) -> LpiSrpdDeepMcclkGateWakeupF2W<DdrDenaliCtl98Spec> {
        LpiSrpdDeepMcclkGateWakeupF2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_98::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_98::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl98Spec;
impl crate::RegisterSpec for DdrDenaliCtl98Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_98::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl98Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_98::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl98Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_98 to value 0"]
impl crate::Resettable for DdrDenaliCtl98Spec {
    const RESET_VALUE: u32 = 0;
}
