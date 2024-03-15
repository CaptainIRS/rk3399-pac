#[doc = "Register `DENALI_CTL_166` reader"]
pub type R = crate::R<DenaliCtl166Spec>;
#[doc = "Register `DENALI_CTL_166` writer"]
pub type W = crate::W<DenaliCtl166Spec>;
#[doc = "Field `ZQ_CALSTART_HIGH_THRESHOLD_F0` reader - ZQ START number of long counts until the high priority request is asserted for frequency copy 0."]
pub type ZqCalstartHighThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CALSTART_HIGH_THRESHOLD_F0` writer - ZQ START number of long counts until the high priority request is asserted for frequency copy 0."]
pub type ZqCalstartHighThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ZQ_CALLATCH_HIGH_THRESHOLD_F0` reader - ZQ LATCH number of long counts until the high priority request is asserted for frequency copy 0."]
pub type ZqCallatchHighThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CALLATCH_HIGH_THRESHOLD_F0` writer - ZQ LATCH number of long counts until the high priority request is asserted for frequency copy 0."]
pub type ZqCallatchHighThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ZQ START number of long counts until the high priority request is asserted for frequency copy 0."]
    #[inline(always)]
    pub fn zq_calstart_high_threshold_f0(&self) -> ZqCalstartHighThresholdF0R {
        ZqCalstartHighThresholdF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ZQ LATCH number of long counts until the high priority request is asserted for frequency copy 0."]
    #[inline(always)]
    pub fn zq_callatch_high_threshold_f0(&self) -> ZqCallatchHighThresholdF0R {
        ZqCallatchHighThresholdF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ZQ START number of long counts until the high priority request is asserted for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn zq_calstart_high_threshold_f0(
        &mut self,
    ) -> ZqCalstartHighThresholdF0W<DenaliCtl166Spec> {
        ZqCalstartHighThresholdF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ZQ LATCH number of long counts until the high priority request is asserted for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn zq_callatch_high_threshold_f0(
        &mut self,
    ) -> ZqCallatchHighThresholdF0W<DenaliCtl166Spec> {
        ZqCallatchHighThresholdF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_166::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_166::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl166Spec;
impl crate::RegisterSpec for DenaliCtl166Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_166::R`](R) reader structure"]
impl crate::Readable for DenaliCtl166Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_166::W`](W) writer structure"]
impl crate::Writable for DenaliCtl166Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_166 to value 0"]
impl crate::Resettable for DenaliCtl166Spec {
    const RESET_VALUE: u32 = 0;
}
