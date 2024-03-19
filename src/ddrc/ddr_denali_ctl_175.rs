#[doc = "Register `DDR_DENALI_CTL_175` reader"]
pub type R = crate::R<DdrDenaliCtl175Spec>;
#[doc = "Register `DDR_DENALI_CTL_175` writer"]
pub type W = crate::W<DdrDenaliCtl175Spec>;
#[doc = "Field `ZQ_CALSTART_HIGH_THRESHOLD_F2` reader - ZQ START number of long counts until the high priority request is asserted for frequency copy 2."]
pub type ZqCalstartHighThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CALSTART_HIGH_THRESHOLD_F2` writer - ZQ START number of long counts until the high priority request is asserted for frequency copy 2."]
pub type ZqCalstartHighThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ZQ_CALLATCH_HIGH_THRESHOLD_F2` reader - ZQ LATCH number of long counts until the high priority request is asserted for frequency copy 2."]
pub type ZqCallatchHighThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CALLATCH_HIGH_THRESHOLD_F2` writer - ZQ LATCH number of long counts until the high priority request is asserted for frequency copy 2."]
pub type ZqCallatchHighThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ZQ START number of long counts until the high priority request is asserted for frequency copy 2."]
    #[inline(always)]
    pub fn zq_calstart_high_threshold_f2(&self) -> ZqCalstartHighThresholdF2R {
        ZqCalstartHighThresholdF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ZQ LATCH number of long counts until the high priority request is asserted for frequency copy 2."]
    #[inline(always)]
    pub fn zq_callatch_high_threshold_f2(&self) -> ZqCallatchHighThresholdF2R {
        ZqCallatchHighThresholdF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ZQ START number of long counts until the high priority request is asserted for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn zq_calstart_high_threshold_f2(
        &mut self,
    ) -> ZqCalstartHighThresholdF2W<DdrDenaliCtl175Spec> {
        ZqCalstartHighThresholdF2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ZQ LATCH number of long counts until the high priority request is asserted for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn zq_callatch_high_threshold_f2(
        &mut self,
    ) -> ZqCallatchHighThresholdF2W<DdrDenaliCtl175Spec> {
        ZqCallatchHighThresholdF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_175::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_175::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl175Spec;
impl crate::RegisterSpec for DdrDenaliCtl175Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_175::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl175Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_175::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl175Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_175 to value 0"]
impl crate::Resettable for DdrDenaliCtl175Spec {
    const RESET_VALUE: u32 = 0;
}
