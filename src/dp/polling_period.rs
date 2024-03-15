#[doc = "Register `POLLING_PERIOD` reader"]
pub type R = crate::R<PollingPeriodSpec>;
#[doc = "Register `POLLING_PERIOD` writer"]
pub type W = crate::W<PollingPeriodSpec>;
#[doc = "Field `POLLING_PERIOD` reader - This register controls the interval between each time of polling operation. Interval time = POLLING_PERIOD * 2^16 * Period of 24M clock."]
pub type PollingPeriodR = crate::FieldReader;
#[doc = "Field `POLLING_PERIOD` writer - This register controls the interval between each time of polling operation. Interval time = POLLING_PERIOD * 2^16 * Period of 24M clock."]
pub type PollingPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This register controls the interval between each time of polling operation. Interval time = POLLING_PERIOD * 2^16 * Period of 24M clock."]
    #[inline(always)]
    pub fn polling_period(&self) -> PollingPeriodR {
        PollingPeriodR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register controls the interval between each time of polling operation. Interval time = POLLING_PERIOD * 2^16 * Period of 24M clock."]
    #[inline(always)]
    #[must_use]
    pub fn polling_period(&mut self) -> PollingPeriodW<PollingPeriodSpec> {
        PollingPeriodW::new(self, 0)
    }
}
#[doc = "DP polling period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`polling_period::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`polling_period::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PollingPeriodSpec;
impl crate::RegisterSpec for PollingPeriodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`polling_period::R`](R) reader structure"]
impl crate::Readable for PollingPeriodSpec {}
#[doc = "`write(|w| ..)` method takes [`polling_period::W`](W) writer structure"]
impl crate::Writable for PollingPeriodSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POLLING_PERIOD to value 0x0e"]
impl crate::Resettable for PollingPeriodSpec {
    const RESET_VALUE: u32 = 0x0e;
}
