#[doc = "Register `DENALI_CTL_173` reader"]
pub type R = crate::R<DenaliCtl173Spec>;
#[doc = "Register `DENALI_CTL_173` writer"]
pub type W = crate::W<DenaliCtl173Spec>;
#[doc = "Field `ZQ_CALLATCH_TIMEOUT_F1` reader - ZQ LATCH number of long counts until the timeout is asserted for frequency copy 1."]
pub type ZqCallatchTimeoutF1R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CALLATCH_TIMEOUT_F1` writer - ZQ LATCH number of long counts until the timeout is asserted for frequency copy 1."]
pub type ZqCallatchTimeoutF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ZQ_CS_TIMEOUT_F1` reader - ZQ CS number of long counts until the timeout is asserted for frequency copy 1."]
pub type ZqCsTimeoutF1R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CS_TIMEOUT_F1` writer - ZQ CS number of long counts until the timeout is asserted for frequency copy 1."]
pub type ZqCsTimeoutF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ZQ LATCH number of long counts until the timeout is asserted for frequency copy 1."]
    #[inline(always)]
    pub fn zq_callatch_timeout_f1(&self) -> ZqCallatchTimeoutF1R {
        ZqCallatchTimeoutF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ZQ CS number of long counts until the timeout is asserted for frequency copy 1."]
    #[inline(always)]
    pub fn zq_cs_timeout_f1(&self) -> ZqCsTimeoutF1R {
        ZqCsTimeoutF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ZQ LATCH number of long counts until the timeout is asserted for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn zq_callatch_timeout_f1(&mut self) -> ZqCallatchTimeoutF1W<DenaliCtl173Spec> {
        ZqCallatchTimeoutF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ZQ CS number of long counts until the timeout is asserted for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn zq_cs_timeout_f1(&mut self) -> ZqCsTimeoutF1W<DenaliCtl173Spec> {
        ZqCsTimeoutF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_173::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_173::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl173Spec;
impl crate::RegisterSpec for DenaliCtl173Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_173::R`](R) reader structure"]
impl crate::Readable for DenaliCtl173Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_173::W`](W) writer structure"]
impl crate::Writable for DenaliCtl173Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_173 to value 0"]
impl crate::Resettable for DenaliCtl173Spec {
    const RESET_VALUE: u32 = 0;
}
