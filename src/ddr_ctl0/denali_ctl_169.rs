#[doc = "Register `DENALI_CTL_169` reader"]
pub type R = crate::R<DenaliCtl169Spec>;
#[doc = "Register `DENALI_CTL_169` writer"]
pub type W = crate::W<DenaliCtl169Spec>;
#[doc = "Field `ZQ_CS_TIMEOUT_F0` reader - ZQ CS number of long counts until the timeout is asserted for frequency copy 0."]
pub type ZqCsTimeoutF0R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CS_TIMEOUT_F0` writer - ZQ CS number of long counts until the timeout is asserted for frequency copy 0."]
pub type ZqCsTimeoutF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ZQ_PROMOTE_THRESHOLD_F0` reader - ZQ SW promotion number of long counts until the high priority request is asserted for frequency copy 0."]
pub type ZqPromoteThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_PROMOTE_THRESHOLD_F0` writer - ZQ SW promotion number of long counts until the high priority request is asserted for frequency copy 0."]
pub type ZqPromoteThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ZQ CS number of long counts until the timeout is asserted for frequency copy 0."]
    #[inline(always)]
    pub fn zq_cs_timeout_f0(&self) -> ZqCsTimeoutF0R {
        ZqCsTimeoutF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ZQ SW promotion number of long counts until the high priority request is asserted for frequency copy 0."]
    #[inline(always)]
    pub fn zq_promote_threshold_f0(&self) -> ZqPromoteThresholdF0R {
        ZqPromoteThresholdF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ZQ CS number of long counts until the timeout is asserted for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn zq_cs_timeout_f0(&mut self) -> ZqCsTimeoutF0W<DenaliCtl169Spec> {
        ZqCsTimeoutF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ZQ SW promotion number of long counts until the high priority request is asserted for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn zq_promote_threshold_f0(&mut self) -> ZqPromoteThresholdF0W<DenaliCtl169Spec> {
        ZqPromoteThresholdF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_169::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_169::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl169Spec;
impl crate::RegisterSpec for DenaliCtl169Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_169::R`](R) reader structure"]
impl crate::Readable for DenaliCtl169Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_169::W`](W) writer structure"]
impl crate::Writable for DenaliCtl169Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_169 to value 0"]
impl crate::Resettable for DenaliCtl169Spec {
    const RESET_VALUE: u32 = 0;
}
