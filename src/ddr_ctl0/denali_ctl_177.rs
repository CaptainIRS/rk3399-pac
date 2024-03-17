#[doc = "Register `DENALI_CTL_177` reader"]
pub type R = crate::R<DenaliCtl177Spec>;
#[doc = "Register `DENALI_CTL_177` writer"]
pub type W = crate::W<DenaliCtl177Spec>;
#[doc = "Field `ZQ_CALSTART_TIMEOUT_F2` reader - ZQ START number of long counts until the timeout is asserted for frequency copy 2."]
pub type ZqCalstartTimeoutF2R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CALSTART_TIMEOUT_F2` writer - ZQ START number of long counts until the timeout is asserted for frequency copy 2."]
pub type ZqCalstartTimeoutF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ZQ_CALLATCH_TIMEOUT_F2` reader - ZQ LATCH number of long counts until the timeout is asserted for frequency copy 2."]
pub type ZqCallatchTimeoutF2R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CALLATCH_TIMEOUT_F2` writer - ZQ LATCH number of long counts until the timeout is asserted for frequency copy 2."]
pub type ZqCallatchTimeoutF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ZQ START number of long counts until the timeout is asserted for frequency copy 2."]
    #[inline(always)]
    pub fn zq_calstart_timeout_f2(&self) -> ZqCalstartTimeoutF2R {
        ZqCalstartTimeoutF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ZQ LATCH number of long counts until the timeout is asserted for frequency copy 2."]
    #[inline(always)]
    pub fn zq_callatch_timeout_f2(&self) -> ZqCallatchTimeoutF2R {
        ZqCallatchTimeoutF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ZQ START number of long counts until the timeout is asserted for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn zq_calstart_timeout_f2(&mut self) -> ZqCalstartTimeoutF2W<DenaliCtl177Spec> {
        ZqCalstartTimeoutF2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ZQ LATCH number of long counts until the timeout is asserted for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn zq_callatch_timeout_f2(&mut self) -> ZqCallatchTimeoutF2W<DenaliCtl177Spec> {
        ZqCallatchTimeoutF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_177::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_177::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl177Spec;
impl crate::RegisterSpec for DenaliCtl177Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_177::R`](R) reader structure"]
impl crate::Readable for DenaliCtl177Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_177::W`](W) writer structure"]
impl crate::Writable for DenaliCtl177Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_177 to value 0"]
impl crate::Resettable for DenaliCtl177Spec {
    const RESET_VALUE: u32 = 0;
}
