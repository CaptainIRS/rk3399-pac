#[doc = "Register `DENALI_CTL_172` reader"]
pub type R = crate::R<DenaliCtl172Spec>;
#[doc = "Register `DENALI_CTL_172` writer"]
pub type W = crate::W<DenaliCtl172Spec>;
#[doc = "Field `ZQ_CS_HIGH_THRESHOLD_F1` reader - ZQ CS number of long counts until the high priority request is asserted for frequency copy 1."]
pub type ZqCsHighThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CS_HIGH_THRESHOLD_F1` writer - ZQ CS number of long counts until the high priority request is asserted for frequency copy 1."]
pub type ZqCsHighThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ZQ_CALSTART_TIMEOUT_F1` reader - ZQ START number of long counts until the timeout is asserted for frequency copy 1."]
pub type ZqCalstartTimeoutF1R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CALSTART_TIMEOUT_F1` writer - ZQ START number of long counts until the timeout is asserted for frequency copy 1."]
pub type ZqCalstartTimeoutF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ZQ CS number of long counts until the high priority request is asserted for frequency copy 1."]
    #[inline(always)]
    pub fn zq_cs_high_threshold_f1(&self) -> ZqCsHighThresholdF1R {
        ZqCsHighThresholdF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ZQ START number of long counts until the timeout is asserted for frequency copy 1."]
    #[inline(always)]
    pub fn zq_calstart_timeout_f1(&self) -> ZqCalstartTimeoutF1R {
        ZqCalstartTimeoutF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ZQ CS number of long counts until the high priority request is asserted for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn zq_cs_high_threshold_f1(&mut self) -> ZqCsHighThresholdF1W<DenaliCtl172Spec> {
        ZqCsHighThresholdF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ZQ START number of long counts until the timeout is asserted for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn zq_calstart_timeout_f1(&mut self) -> ZqCalstartTimeoutF1W<DenaliCtl172Spec> {
        ZqCalstartTimeoutF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_172::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_172::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl172Spec;
impl crate::RegisterSpec for DenaliCtl172Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_172::R`](R) reader structure"]
impl crate::Readable for DenaliCtl172Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_172::W`](W) writer structure"]
impl crate::Writable for DenaliCtl172Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_172 to value 0"]
impl crate::Resettable for DenaliCtl172Spec {
    const RESET_VALUE: u32 = 0;
}
