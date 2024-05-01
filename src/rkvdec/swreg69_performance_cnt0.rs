#[doc = "Register `SWREG69_PERFORMANCE_CNT0` reader"]
pub type R = crate::R<Swreg69PerformanceCnt0Spec>;
#[doc = "Register `SWREG69_PERFORMANCE_CNT0` writer"]
pub type W = crate::W<Swreg69PerformanceCnt0Spec>;
#[doc = "Field `PERF_CNT0` reader - Field0000 Abstract\n\nField0000 Description"]
pub type PerfCnt0R = crate::FieldReader<u32>;
#[doc = "Field `PERF_CNT0` writer - Field0000 Abstract\n\nField0000 Description"]
pub type PerfCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Field0000 Abstract\n\nField0000 Description"]
    #[inline(always)]
    pub fn perf_cnt0(&self) -> PerfCnt0R {
        PerfCnt0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Field0000 Abstract\n\nField0000 Description"]
    #[inline(always)]
    #[must_use]
    pub fn perf_cnt0(&mut self) -> PerfCnt0W<Swreg69PerformanceCnt0Spec> {
        PerfCnt0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg69_performance_cnt0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg69_performance_cnt0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg69PerformanceCnt0Spec;
impl crate::RegisterSpec for Swreg69PerformanceCnt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg69_performance_cnt0::R`](R) reader structure"]
impl crate::Readable for Swreg69PerformanceCnt0Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg69_performance_cnt0::W`](W) writer structure"]
impl crate::Writable for Swreg69PerformanceCnt0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG69_PERFORMANCE_CNT0 to value 0"]
impl crate::Resettable for Swreg69PerformanceCnt0Spec {
    const RESET_VALUE: u32 = 0;
}
