#[doc = "Register `DDR_DENALI_CTL_83` reader"]
pub type R = crate::R<DdrDenaliCtl83Spec>;
#[doc = "Register `DDR_DENALI_CTL_83` writer"]
pub type W = crate::W<DdrDenaliCtl83Spec>;
#[doc = "Field `TDFI_PHYMSTR_RESP_F0` reader - Defines the DFI tPHYMSTR_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion, for frequency copy 0. If programmed to a non-zero, a timing violation will cause an interrupt and bit (1) set in the PHYMSTR_ERROR_STATUS parameter."]
pub type TdfiPhymstrRespF0R = crate::FieldReader<u16>;
#[doc = "Field `TDFI_PHYMSTR_RESP_F0` writer - Defines the DFI tPHYMSTR_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion, for frequency copy 0. If programmed to a non-zero, a timing violation will cause an interrupt and bit (1) set in the PHYMSTR_ERROR_STATUS parameter."]
pub type TdfiPhymstrRespF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PHYMSTR_DFI_PROMOTE_THRESHOLD_F0` reader - DFI PHY master request promotion number of long counts until the high priority request is asserted for frequency copy 0."]
pub type PhymstrDfiPromoteThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `PHYMSTR_DFI_PROMOTE_THRESHOLD_F0` writer - DFI PHY master request promotion number of long counts until the high priority request is asserted for frequency copy 0."]
pub type PhymstrDfiPromoteThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Defines the DFI tPHYMSTR_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion, for frequency copy 0. If programmed to a non-zero, a timing violation will cause an interrupt and bit (1) set in the PHYMSTR_ERROR_STATUS parameter."]
    #[inline(always)]
    pub fn tdfi_phymstr_resp_f0(&self) -> TdfiPhymstrRespF0R {
        TdfiPhymstrRespF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - DFI PHY master request promotion number of long counts until the high priority request is asserted for frequency copy 0."]
    #[inline(always)]
    pub fn phymstr_dfi_promote_threshold_f0(&self) -> PhymstrDfiPromoteThresholdF0R {
        PhymstrDfiPromoteThresholdF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the DFI tPHYMSTR_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion, for frequency copy 0. If programmed to a non-zero, a timing violation will cause an interrupt and bit (1) set in the PHYMSTR_ERROR_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phymstr_resp_f0(&mut self) -> TdfiPhymstrRespF0W<DdrDenaliCtl83Spec> {
        TdfiPhymstrRespF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - DFI PHY master request promotion number of long counts until the high priority request is asserted for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn phymstr_dfi_promote_threshold_f0(
        &mut self,
    ) -> PhymstrDfiPromoteThresholdF0W<DdrDenaliCtl83Spec> {
        PhymstrDfiPromoteThresholdF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_83::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_83::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl83Spec;
impl crate::RegisterSpec for DdrDenaliCtl83Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_83::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl83Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_83::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl83Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_83 to value 0"]
impl crate::Resettable for DdrDenaliCtl83Spec {
    const RESET_VALUE: u32 = 0;
}
