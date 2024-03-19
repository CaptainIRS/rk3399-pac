#[doc = "Register `DDR_DENALI_CTL_94` reader"]
pub type R = crate::R<DdrDenaliCtl94Spec>;
#[doc = "Register `DDR_DENALI_CTL_94` writer"]
pub type W = crate::W<DdrDenaliCtl94Spec>;
#[doc = "Field `LPI_PD_WAKEUP_F0` reader - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in power-down for frequency copy 0."]
pub type LpiPdWakeupF0R = crate::FieldReader;
#[doc = "Field `LPI_PD_WAKEUP_F0` writer - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in power-down for frequency copy 0."]
pub type LpiPdWakeupF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_SR_WAKEUP_F0` reader - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh for frequency copy 0."]
pub type LpiSrWakeupF0R = crate::FieldReader;
#[doc = "Field `LPI_SR_WAKEUP_F0` writer - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh for frequency copy 0."]
pub type LpiSrWakeupF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_SR_MCCLK_GATE_WAKEUP_F0` reader - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh with memory and controller clock gating, for frequency copy 0."]
pub type LpiSrMcclkGateWakeupF0R = crate::FieldReader;
#[doc = "Field `LPI_SR_MCCLK_GATE_WAKEUP_F0` writer - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh with memory and controller clock gating, for frequency copy 0."]
pub type LpiSrMcclkGateWakeupF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_SRPD_LITE_WAKEUP_F0` reader - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down lite, for frequncy copy 0."]
pub type LpiSrpdLiteWakeupF0R = crate::FieldReader;
#[doc = "Field `LPI_SRPD_LITE_WAKEUP_F0` writer - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down lite, for frequncy copy 0."]
pub type LpiSrpdLiteWakeupF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in power-down for frequency copy 0."]
    #[inline(always)]
    pub fn lpi_pd_wakeup_f0(&self) -> LpiPdWakeupF0R {
        LpiPdWakeupF0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh for frequency copy 0."]
    #[inline(always)]
    pub fn lpi_sr_wakeup_f0(&self) -> LpiSrWakeupF0R {
        LpiSrWakeupF0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh with memory and controller clock gating, for frequency copy 0."]
    #[inline(always)]
    pub fn lpi_sr_mcclk_gate_wakeup_f0(&self) -> LpiSrMcclkGateWakeupF0R {
        LpiSrMcclkGateWakeupF0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down lite, for frequncy copy 0."]
    #[inline(always)]
    pub fn lpi_srpd_lite_wakeup_f0(&self) -> LpiSrpdLiteWakeupF0R {
        LpiSrpdLiteWakeupF0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in power-down for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_pd_wakeup_f0(&mut self) -> LpiPdWakeupF0W<DdrDenaliCtl94Spec> {
        LpiPdWakeupF0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_sr_wakeup_f0(&mut self) -> LpiSrWakeupF0W<DdrDenaliCtl94Spec> {
        LpiSrWakeupF0W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh with memory and controller clock gating, for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_sr_mcclk_gate_wakeup_f0(&mut self) -> LpiSrMcclkGateWakeupF0W<DdrDenaliCtl94Spec> {
        LpiSrMcclkGateWakeupF0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Defines the DFI tLP_WAKEUP timing parameter (in DFI clocks) to be driven when memory is in self- refresh power down lite, for frequncy copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_srpd_lite_wakeup_f0(&mut self) -> LpiSrpdLiteWakeupF0W<DdrDenaliCtl94Spec> {
        LpiSrpdLiteWakeupF0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_94::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_94::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl94Spec;
impl crate::RegisterSpec for DdrDenaliCtl94Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_94::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl94Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_94::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl94Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_94 to value 0"]
impl crate::Resettable for DdrDenaliCtl94Spec {
    const RESET_VALUE: u32 = 0;
}
