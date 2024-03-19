#[doc = "Register `DDR_DENALI_CTL_168` reader"]
pub type R = crate::R<DdrDenaliCtl168Spec>;
#[doc = "Register `DDR_DENALI_CTL_168` writer"]
pub type W = crate::W<DdrDenaliCtl168Spec>;
#[doc = "Field `ZQ_CALSTART_TIMEOUT_F0` reader - ZQ START number of long counts until the timeout is asserted for frequency copy 0."]
pub type ZqCalstartTimeoutF0R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CALSTART_TIMEOUT_F0` writer - ZQ START number of long counts until the timeout is asserted for frequency copy 0."]
pub type ZqCalstartTimeoutF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ZQ_CALLATCH_TIMEOUT_F0` reader - ZQ LATCH number of long counts until the timeout is asserted for frequency copy 0."]
pub type ZqCallatchTimeoutF0R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CALLATCH_TIMEOUT_F0` writer - ZQ LATCH number of long counts until the timeout is asserted for frequency copy 0."]
pub type ZqCallatchTimeoutF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ZQ START number of long counts until the timeout is asserted for frequency copy 0."]
    #[inline(always)]
    pub fn zq_calstart_timeout_f0(&self) -> ZqCalstartTimeoutF0R {
        ZqCalstartTimeoutF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ZQ LATCH number of long counts until the timeout is asserted for frequency copy 0."]
    #[inline(always)]
    pub fn zq_callatch_timeout_f0(&self) -> ZqCallatchTimeoutF0R {
        ZqCallatchTimeoutF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ZQ START number of long counts until the timeout is asserted for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn zq_calstart_timeout_f0(&mut self) -> ZqCalstartTimeoutF0W<DdrDenaliCtl168Spec> {
        ZqCalstartTimeoutF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ZQ LATCH number of long counts until the timeout is asserted for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn zq_callatch_timeout_f0(&mut self) -> ZqCallatchTimeoutF0W<DdrDenaliCtl168Spec> {
        ZqCallatchTimeoutF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_168::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_168::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl168Spec;
impl crate::RegisterSpec for DdrDenaliCtl168Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_168::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl168Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_168::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl168Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_168 to value 0"]
impl crate::Resettable for DdrDenaliCtl168Spec {
    const RESET_VALUE: u32 = 0;
}
