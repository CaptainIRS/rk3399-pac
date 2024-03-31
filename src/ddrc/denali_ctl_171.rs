#[doc = "Register `DENALI_CTL_171` reader"]
pub type R = crate::R<DenaliCtl171Spec>;
#[doc = "Register `DENALI_CTL_171` writer"]
pub type W = crate::W<DenaliCtl171Spec>;
#[doc = "Field `ZQ_CALLATCH_HIGH_THRESHOLD_F1` reader - ZQ LATCH number of long counts until the high priority request is asserted for frequency copy 1."]
pub type ZqCallatchHighThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CALLATCH_HIGH_THRESHOLD_F1` writer - ZQ LATCH number of long counts until the high priority request is asserted for frequency copy 1."]
pub type ZqCallatchHighThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ZQ_CS_NORM_THRESHOLD_F1` reader - ZQ CS number of long counts until the normal priority request is asserted for frequency copy 1."]
pub type ZqCsNormThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CS_NORM_THRESHOLD_F1` writer - ZQ CS number of long counts until the normal priority request is asserted for frequency copy 1."]
pub type ZqCsNormThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ZQ LATCH number of long counts until the high priority request is asserted for frequency copy 1."]
    #[inline(always)]
    pub fn zq_callatch_high_threshold_f1(&self) -> ZqCallatchHighThresholdF1R {
        ZqCallatchHighThresholdF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ZQ CS number of long counts until the normal priority request is asserted for frequency copy 1."]
    #[inline(always)]
    pub fn zq_cs_norm_threshold_f1(&self) -> ZqCsNormThresholdF1R {
        ZqCsNormThresholdF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ZQ LATCH number of long counts until the high priority request is asserted for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn zq_callatch_high_threshold_f1(
        &mut self,
    ) -> ZqCallatchHighThresholdF1W<DenaliCtl171Spec> {
        ZqCallatchHighThresholdF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ZQ CS number of long counts until the normal priority request is asserted for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn zq_cs_norm_threshold_f1(&mut self) -> ZqCsNormThresholdF1W<DenaliCtl171Spec> {
        ZqCsNormThresholdF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_171::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_171::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl171Spec;
impl crate::RegisterSpec for DenaliCtl171Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_171::R`](R) reader structure"]
impl crate::Readable for DenaliCtl171Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_171::W`](W) writer structure"]
impl crate::Writable for DenaliCtl171Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_171 to value 0"]
impl crate::Resettable for DenaliCtl171Spec {
    const RESET_VALUE: u32 = 0;
}
