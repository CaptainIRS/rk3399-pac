#[doc = "Register `DDR_DENALI_CTL_84` reader"]
pub type R = crate::R<DdrDenaliCtl84Spec>;
#[doc = "Register `DDR_DENALI_CTL_84` writer"]
pub type W = crate::W<DdrDenaliCtl84Spec>;
#[doc = "Field `TDFI_PHYMSTR_MAX_F1` reader - Defines the DFI tPHYMSTR_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_phymstr_req following the assertion of dfi_phymstr_ack can be asserted, for frequency copy 1. If programmed to a non-zero, a timing violation will cause an interrupt and bit (0) set in the PHYMSTR_ERROR_STATUS parameter."]
pub type TdfiPhymstrMaxF1R = crate::FieldReader<u16>;
#[doc = "Field `TDFI_PHYMSTR_MAX_F1` writer - Defines the DFI tPHYMSTR_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_phymstr_req following the assertion of dfi_phymstr_ack can be asserted, for frequency copy 1. If programmed to a non-zero, a timing violation will cause an interrupt and bit (0) set in the PHYMSTR_ERROR_STATUS parameter."]
pub type TdfiPhymstrMaxF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TDFI_PHYMSTR_RESP_F1` reader - Defines the DFI tPHYMSTR_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion, for frequency copy 1. If programmed to a non-zero, a timing violation will cause an interrupt and bit (1) set in the PHYMSTR_ERROR_STATUS parameter."]
pub type TdfiPhymstrRespF1R = crate::FieldReader<u16>;
#[doc = "Field `TDFI_PHYMSTR_RESP_F1` writer - Defines the DFI tPHYMSTR_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion, for frequency copy 1. If programmed to a non-zero, a timing violation will cause an interrupt and bit (1) set in the PHYMSTR_ERROR_STATUS parameter."]
pub type TdfiPhymstrRespF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Defines the DFI tPHYMSTR_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_phymstr_req following the assertion of dfi_phymstr_ack can be asserted, for frequency copy 1. If programmed to a non-zero, a timing violation will cause an interrupt and bit (0) set in the PHYMSTR_ERROR_STATUS parameter."]
    #[inline(always)]
    pub fn tdfi_phymstr_max_f1(&self) -> TdfiPhymstrMaxF1R {
        TdfiPhymstrMaxF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Defines the DFI tPHYMSTR_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion, for frequency copy 1. If programmed to a non-zero, a timing violation will cause an interrupt and bit (1) set in the PHYMSTR_ERROR_STATUS parameter."]
    #[inline(always)]
    pub fn tdfi_phymstr_resp_f1(&self) -> TdfiPhymstrRespF1R {
        TdfiPhymstrRespF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the DFI tPHYMSTR_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_phymstr_req following the assertion of dfi_phymstr_ack can be asserted, for frequency copy 1. If programmed to a non-zero, a timing violation will cause an interrupt and bit (0) set in the PHYMSTR_ERROR_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phymstr_max_f1(&mut self) -> TdfiPhymstrMaxF1W<DdrDenaliCtl84Spec> {
        TdfiPhymstrMaxF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Defines the DFI tPHYMSTR_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion, for frequency copy 1. If programmed to a non-zero, a timing violation will cause an interrupt and bit (1) set in the PHYMSTR_ERROR_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phymstr_resp_f1(&mut self) -> TdfiPhymstrRespF1W<DdrDenaliCtl84Spec> {
        TdfiPhymstrRespF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_84::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_84::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl84Spec;
impl crate::RegisterSpec for DdrDenaliCtl84Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_84::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl84Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_84::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl84Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_84 to value 0"]
impl crate::Resettable for DdrDenaliCtl84Spec {
    const RESET_VALUE: u32 = 0;
}
