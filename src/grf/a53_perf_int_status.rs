#[doc = "Register `A53_PERF_INT_STATUS` reader"]
pub type R = crate::R<A53PerfIntStatusSpec>;
#[doc = "Register `A53_PERF_INT_STATUS` writer"]
pub type W = crate::W<A53PerfIntStatusSpec>;
#[doc = "Field `INT_STATUS` reader - interrupt status bit"]
pub type IntStatusR = crate::BitReader;
#[doc = "Field `INT_STATUS` writer - interrupt status bit"]
pub type IntStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - interrupt status bit"]
    #[inline(always)]
    pub fn int_status(&self) -> IntStatusR {
        IntStatusR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - interrupt status bit"]
    #[inline(always)]
    #[must_use]
    pub fn int_status(&mut self) -> IntStatusW<A53PerfIntStatusSpec> {
        IntStatusW::new(self, 0)
    }
}
#[doc = "a53 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a53_perf_int_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a53_perf_int_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A53PerfIntStatusSpec;
impl crate::RegisterSpec for A53PerfIntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a53_perf_int_status::R`](R) reader structure"]
impl crate::Readable for A53PerfIntStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`a53_perf_int_status::W`](W) writer structure"]
impl crate::Writable for A53PerfIntStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets A53_PERF_INT_STATUS to value 0"]
impl crate::Resettable for A53PerfIntStatusSpec {
    const RESET_VALUE: u32 = 0;
}
