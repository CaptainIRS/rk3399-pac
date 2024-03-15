#[doc = "Register `DENALI_CTL_91` reader"]
pub type R = crate::R<DenaliCtl91Spec>;
#[doc = "Register `DENALI_CTL_91` writer"]
pub type W = crate::W<DenaliCtl91Spec>;
#[doc = "Field `MRR_TEMPCHK_HIGH_THRESHOLD_F2` reader - MRR temp check number of long counts until the high priority request is asserted for frequency copy 2."]
pub type MrrTempchkHighThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `MRR_TEMPCHK_HIGH_THRESHOLD_F2` writer - MRR temp check number of long counts until the high priority request is asserted for frequency copy 2."]
pub type MrrTempchkHighThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MRR_TEMPCHK_TIMEOUT_F2` reader - MRR temp check number of long counts until the timeout is asserted for frequency copy 2."]
pub type MrrTempchkTimeoutF2R = crate::FieldReader<u16>;
#[doc = "Field `MRR_TEMPCHK_TIMEOUT_F2` writer - MRR temp check number of long counts until the timeout is asserted for frequency copy 2."]
pub type MrrTempchkTimeoutF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MRR temp check number of long counts until the high priority request is asserted for frequency copy 2."]
    #[inline(always)]
    pub fn mrr_tempchk_high_threshold_f2(&self) -> MrrTempchkHighThresholdF2R {
        MrrTempchkHighThresholdF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - MRR temp check number of long counts until the timeout is asserted for frequency copy 2."]
    #[inline(always)]
    pub fn mrr_tempchk_timeout_f2(&self) -> MrrTempchkTimeoutF2R {
        MrrTempchkTimeoutF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MRR temp check number of long counts until the high priority request is asserted for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn mrr_tempchk_high_threshold_f2(&mut self) -> MrrTempchkHighThresholdF2W<DenaliCtl91Spec> {
        MrrTempchkHighThresholdF2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - MRR temp check number of long counts until the timeout is asserted for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn mrr_tempchk_timeout_f2(&mut self) -> MrrTempchkTimeoutF2W<DenaliCtl91Spec> {
        MrrTempchkTimeoutF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_91::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_91::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl91Spec;
impl crate::RegisterSpec for DenaliCtl91Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_91::R`](R) reader structure"]
impl crate::Readable for DenaliCtl91Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_91::W`](W) writer structure"]
impl crate::Writable for DenaliCtl91Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_91 to value 0"]
impl crate::Resettable for DenaliCtl91Spec {
    const RESET_VALUE: u32 = 0;
}
