#[doc = "Register `DDR_DENALI_CTL_86` reader"]
pub type R = crate::R<DdrDenaliCtl86Spec>;
#[doc = "Register `DDR_DENALI_CTL_86` writer"]
pub type W = crate::W<DdrDenaliCtl86Spec>;
#[doc = "Field `TDFI_PHYMSTR_RESP_F2` reader - Defines the DFI tPHYMSTR_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion, for frequency copy 2. If programmed to a non-zero, a timing violation will cause an interrupt and bit (1) set in the PHYMSTR_ERROR_STATUS parameter."]
pub type TdfiPhymstrRespF2R = crate::FieldReader<u16>;
#[doc = "Field `TDFI_PHYMSTR_RESP_F2` writer - Defines the DFI tPHYMSTR_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion, for frequency copy 2. If programmed to a non-zero, a timing violation will cause an interrupt and bit (1) set in the PHYMSTR_ERROR_STATUS parameter."]
pub type TdfiPhymstrRespF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PHYMSTR_DFI_PROMOTE_THRESHOLD_F2` reader - DFI PHY master request promotion number of long counts until the high priority request is asserted for frequency copy 2."]
pub type PhymstrDfiPromoteThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `PHYMSTR_DFI_PROMOTE_THRESHOLD_F2` writer - DFI PHY master request promotion number of long counts until the high priority request is asserted for frequency copy 2."]
pub type PhymstrDfiPromoteThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Defines the DFI tPHYMSTR_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion, for frequency copy 2. If programmed to a non-zero, a timing violation will cause an interrupt and bit (1) set in the PHYMSTR_ERROR_STATUS parameter."]
    #[inline(always)]
    pub fn tdfi_phymstr_resp_f2(&self) -> TdfiPhymstrRespF2R {
        TdfiPhymstrRespF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - DFI PHY master request promotion number of long counts until the high priority request is asserted for frequency copy 2."]
    #[inline(always)]
    pub fn phymstr_dfi_promote_threshold_f2(&self) -> PhymstrDfiPromoteThresholdF2R {
        PhymstrDfiPromoteThresholdF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the DFI tPHYMSTR_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion, for frequency copy 2. If programmed to a non-zero, a timing violation will cause an interrupt and bit (1) set in the PHYMSTR_ERROR_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phymstr_resp_f2(&mut self) -> TdfiPhymstrRespF2W<DdrDenaliCtl86Spec> {
        TdfiPhymstrRespF2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - DFI PHY master request promotion number of long counts until the high priority request is asserted for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn phymstr_dfi_promote_threshold_f2(
        &mut self,
    ) -> PhymstrDfiPromoteThresholdF2W<DdrDenaliCtl86Spec> {
        PhymstrDfiPromoteThresholdF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_86::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_86::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl86Spec;
impl crate::RegisterSpec for DdrDenaliCtl86Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_86::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl86Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_86::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl86Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_86 to value 0"]
impl crate::Resettable for DdrDenaliCtl86Spec {
    const RESET_VALUE: u32 = 0;
}
