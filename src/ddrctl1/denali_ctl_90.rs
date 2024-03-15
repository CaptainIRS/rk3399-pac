#[doc = "Register `DENALI_CTL_90` reader"]
pub type R = crate::R<DenaliCtl90Spec>;
#[doc = "Register `DENALI_CTL_90` writer"]
pub type W = crate::W<DenaliCtl90Spec>;
#[doc = "Field `MRR_TEMPCHK_TIMEOUT_F1` reader - MRR temp check number of long counts until the timeout is asserted for frequency copy 1."]
pub type MrrTempchkTimeoutF1R = crate::FieldReader<u16>;
#[doc = "Field `MRR_TEMPCHK_TIMEOUT_F1` writer - MRR temp check number of long counts until the timeout is asserted for frequency copy 1."]
pub type MrrTempchkTimeoutF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MRR_TEMPCHK_NORM_THRESHOLD_F2` reader - MRR temp check number of long counts until the normal priority request is asserted for frequency copy 2."]
pub type MrrTempchkNormThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `MRR_TEMPCHK_NORM_THRESHOLD_F2` writer - MRR temp check number of long counts until the normal priority request is asserted for frequency copy 2."]
pub type MrrTempchkNormThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MRR temp check number of long counts until the timeout is asserted for frequency copy 1."]
    #[inline(always)]
    pub fn mrr_tempchk_timeout_f1(&self) -> MrrTempchkTimeoutF1R {
        MrrTempchkTimeoutF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - MRR temp check number of long counts until the normal priority request is asserted for frequency copy 2."]
    #[inline(always)]
    pub fn mrr_tempchk_norm_threshold_f2(&self) -> MrrTempchkNormThresholdF2R {
        MrrTempchkNormThresholdF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MRR temp check number of long counts until the timeout is asserted for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn mrr_tempchk_timeout_f1(&mut self) -> MrrTempchkTimeoutF1W<DenaliCtl90Spec> {
        MrrTempchkTimeoutF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - MRR temp check number of long counts until the normal priority request is asserted for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn mrr_tempchk_norm_threshold_f2(&mut self) -> MrrTempchkNormThresholdF2W<DenaliCtl90Spec> {
        MrrTempchkNormThresholdF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_90::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_90::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl90Spec;
impl crate::RegisterSpec for DenaliCtl90Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_90::R`](R) reader structure"]
impl crate::Readable for DenaliCtl90Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_90::W`](W) writer structure"]
impl crate::Writable for DenaliCtl90Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_90 to value 0"]
impl crate::Resettable for DenaliCtl90Spec {
    const RESET_VALUE: u32 = 0;
}
