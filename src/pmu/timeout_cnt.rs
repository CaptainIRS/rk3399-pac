#[doc = "Register `TIMEOUT_CNT` reader"]
pub type R = crate::R<TimeoutCntSpec>;
#[doc = "Register `TIMEOUT_CNT` writer"]
pub type W = crate::W<TimeoutCntSpec>;
#[doc = "Field `TIMEOUT_COUNT` reader - timeout wakeup counter value"]
pub type TimeoutCountR = crate::FieldReader<u32>;
#[doc = "Field `TIMEOUT_COUNT` writer - timeout wakeup counter value"]
pub type TimeoutCountW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - timeout wakeup counter value"]
    #[inline(always)]
    pub fn timeout_count(&self) -> TimeoutCountR {
        TimeoutCountR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - timeout wakeup counter value"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_count(&mut self) -> TimeoutCountW<TimeoutCntSpec> {
        TimeoutCountW::new(self, 0)
    }
}
#[doc = "pmu timeout count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeout_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimeoutCntSpec;
impl crate::RegisterSpec for TimeoutCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeout_cnt::R`](R) reader structure"]
impl crate::Readable for TimeoutCntSpec {}
#[doc = "`write(|w| ..)` method takes [`timeout_cnt::W`](W) writer structure"]
impl crate::Writable for TimeoutCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMEOUT_CNT to value 0"]
impl crate::Resettable for TimeoutCntSpec {
    const RESET_VALUE: u32 = 0;
}
