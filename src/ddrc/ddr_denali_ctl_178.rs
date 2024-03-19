#[doc = "Register `DDR_DENALI_CTL_178` reader"]
pub type R = crate::R<DdrDenaliCtl178Spec>;
#[doc = "Register `DDR_DENALI_CTL_178` writer"]
pub type W = crate::W<DdrDenaliCtl178Spec>;
#[doc = "Field `ZQ_CS_TIMEOUT_F2` reader - ZQ CS number of long counts until the timeout is asserted for frequency copy 2."]
pub type ZqCsTimeoutF2R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CS_TIMEOUT_F2` writer - ZQ CS number of long counts until the timeout is asserted for frequency copy 2."]
pub type ZqCsTimeoutF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ZQ_PROMOTE_THRESHOLD_F2` reader - ZQ SW promotion number of long counts until the high priority request is asserted for frequency copy 2."]
pub type ZqPromoteThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_PROMOTE_THRESHOLD_F2` writer - ZQ SW promotion number of long counts until the high priority request is asserted for frequency copy 2."]
pub type ZqPromoteThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ZQ CS number of long counts until the timeout is asserted for frequency copy 2."]
    #[inline(always)]
    pub fn zq_cs_timeout_f2(&self) -> ZqCsTimeoutF2R {
        ZqCsTimeoutF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ZQ SW promotion number of long counts until the high priority request is asserted for frequency copy 2."]
    #[inline(always)]
    pub fn zq_promote_threshold_f2(&self) -> ZqPromoteThresholdF2R {
        ZqPromoteThresholdF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ZQ CS number of long counts until the timeout is asserted for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn zq_cs_timeout_f2(&mut self) -> ZqCsTimeoutF2W<DdrDenaliCtl178Spec> {
        ZqCsTimeoutF2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ZQ SW promotion number of long counts until the high priority request is asserted for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn zq_promote_threshold_f2(&mut self) -> ZqPromoteThresholdF2W<DdrDenaliCtl178Spec> {
        ZqPromoteThresholdF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_178::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_178::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl178Spec;
impl crate::RegisterSpec for DdrDenaliCtl178Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_178::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl178Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_178::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl178Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_178 to value 0"]
impl crate::Resettable for DdrDenaliCtl178Spec {
    const RESET_VALUE: u32 = 0;
}
