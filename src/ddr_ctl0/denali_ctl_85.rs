#[doc = "Register `DENALI_CTL_85` reader"]
pub type R = crate::R<DenaliCtl85Spec>;
#[doc = "Register `DENALI_CTL_85` writer"]
pub type W = crate::W<DenaliCtl85Spec>;
#[doc = "Field `PHYMSTR_DFI_PROMOTE_THRESHOLD_F1` reader - DFI PHY master request promotion number of long counts until the high priority request is asserted for frequency copy 1."]
pub type PhymstrDfiPromoteThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `PHYMSTR_DFI_PROMOTE_THRESHOLD_F1` writer - DFI PHY master request promotion number of long counts until the high priority request is asserted for frequency copy 1."]
pub type PhymstrDfiPromoteThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TDFI_PHYMSTR_MAX_F2` reader - Defines the DFI tPHYMSTR_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_phymstr_req following the assertion of dfi_phymstr_ack can be asserted, for frequency copy 2. If programmed to a non-zero, a timing violation will cause an interrupt and bit (0) set in the PHYMSTR_ERROR_STATUS parameter."]
pub type TdfiPhymstrMaxF2R = crate::FieldReader<u16>;
#[doc = "Field `TDFI_PHYMSTR_MAX_F2` writer - Defines the DFI tPHYMSTR_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_phymstr_req following the assertion of dfi_phymstr_ack can be asserted, for frequency copy 2. If programmed to a non-zero, a timing violation will cause an interrupt and bit (0) set in the PHYMSTR_ERROR_STATUS parameter."]
pub type TdfiPhymstrMaxF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DFI PHY master request promotion number of long counts until the high priority request is asserted for frequency copy 1."]
    #[inline(always)]
    pub fn phymstr_dfi_promote_threshold_f1(&self) -> PhymstrDfiPromoteThresholdF1R {
        PhymstrDfiPromoteThresholdF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Defines the DFI tPHYMSTR_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_phymstr_req following the assertion of dfi_phymstr_ack can be asserted, for frequency copy 2. If programmed to a non-zero, a timing violation will cause an interrupt and bit (0) set in the PHYMSTR_ERROR_STATUS parameter."]
    #[inline(always)]
    pub fn tdfi_phymstr_max_f2(&self) -> TdfiPhymstrMaxF2R {
        TdfiPhymstrMaxF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DFI PHY master request promotion number of long counts until the high priority request is asserted for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn phymstr_dfi_promote_threshold_f1(
        &mut self,
    ) -> PhymstrDfiPromoteThresholdF1W<DenaliCtl85Spec> {
        PhymstrDfiPromoteThresholdF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Defines the DFI tPHYMSTR_MAX timing parameter (in DFI clocks), the maximum cycles that dfi_phymstr_req following the assertion of dfi_phymstr_ack can be asserted, for frequency copy 2. If programmed to a non-zero, a timing violation will cause an interrupt and bit (0) set in the PHYMSTR_ERROR_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phymstr_max_f2(&mut self) -> TdfiPhymstrMaxF2W<DenaliCtl85Spec> {
        TdfiPhymstrMaxF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_85::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_85::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl85Spec;
impl crate::RegisterSpec for DenaliCtl85Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_85::R`](R) reader structure"]
impl crate::Readable for DenaliCtl85Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_85::W`](W) writer structure"]
impl crate::Writable for DenaliCtl85Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_85 to value 0"]
impl crate::Resettable for DenaliCtl85Spec {
    const RESET_VALUE: u32 = 0;
}
