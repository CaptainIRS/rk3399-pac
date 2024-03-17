#[doc = "Register `DENALI_CTL_89` reader"]
pub type R = crate::R<DenaliCtl89Spec>;
#[doc = "Register `DENALI_CTL_89` writer"]
pub type W = crate::W<DenaliCtl89Spec>;
#[doc = "Field `MRR_TEMPCHK_NORM_THRESHOLD_F1` reader - MRR temp check number of long counts until the normal priority request is asserted for frequency copy 1."]
pub type MrrTempchkNormThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `MRR_TEMPCHK_NORM_THRESHOLD_F1` writer - MRR temp check number of long counts until the normal priority request is asserted for frequency copy 1."]
pub type MrrTempchkNormThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MRR_TEMPCHK_HIGH_THRESHOLD_F1` reader - MRR temp check number of long counts until the high priority request is asserted for frequency copy 1."]
pub type MrrTempchkHighThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `MRR_TEMPCHK_HIGH_THRESHOLD_F1` writer - MRR temp check number of long counts until the high priority request is asserted for frequency copy 1."]
pub type MrrTempchkHighThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MRR temp check number of long counts until the normal priority request is asserted for frequency copy 1."]
    #[inline(always)]
    pub fn mrr_tempchk_norm_threshold_f1(&self) -> MrrTempchkNormThresholdF1R {
        MrrTempchkNormThresholdF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - MRR temp check number of long counts until the high priority request is asserted for frequency copy 1."]
    #[inline(always)]
    pub fn mrr_tempchk_high_threshold_f1(&self) -> MrrTempchkHighThresholdF1R {
        MrrTempchkHighThresholdF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MRR temp check number of long counts until the normal priority request is asserted for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn mrr_tempchk_norm_threshold_f1(&mut self) -> MrrTempchkNormThresholdF1W<DenaliCtl89Spec> {
        MrrTempchkNormThresholdF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - MRR temp check number of long counts until the high priority request is asserted for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn mrr_tempchk_high_threshold_f1(&mut self) -> MrrTempchkHighThresholdF1W<DenaliCtl89Spec> {
        MrrTempchkHighThresholdF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_89::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_89::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl89Spec;
impl crate::RegisterSpec for DenaliCtl89Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_89::R`](R) reader structure"]
impl crate::Readable for DenaliCtl89Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_89::W`](W) writer structure"]
impl crate::Writable for DenaliCtl89Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_89 to value 0"]
impl crate::Resettable for DenaliCtl89Spec {
    const RESET_VALUE: u32 = 0;
}
