#[doc = "Register `SWREG70_PERFORMANCE_CNT1` reader"]
pub type R = crate::R<Swreg70PerformanceCnt1Spec>;
#[doc = "Register `SWREG70_PERFORMANCE_CNT1` writer"]
pub type W = crate::W<Swreg70PerformanceCnt1Spec>;
#[doc = "Field `PERF_CNT1` reader - perf_cnt1"]
pub type PerfCnt1R = crate::FieldReader<u32>;
#[doc = "Field `PERF_CNT1` writer - perf_cnt1"]
pub type PerfCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - perf_cnt1"]
    #[inline(always)]
    pub fn perf_cnt1(&self) -> PerfCnt1R {
        PerfCnt1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - perf_cnt1"]
    #[inline(always)]
    #[must_use]
    pub fn perf_cnt1(&mut self) -> PerfCnt1W<Swreg70PerformanceCnt1Spec> {
        PerfCnt1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg70_performance_cnt1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg70_performance_cnt1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg70PerformanceCnt1Spec;
impl crate::RegisterSpec for Swreg70PerformanceCnt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg70_performance_cnt1::R`](R) reader structure"]
impl crate::Readable for Swreg70PerformanceCnt1Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg70_performance_cnt1::W`](W) writer structure"]
impl crate::Writable for Swreg70PerformanceCnt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG70_PERFORMANCE_CNT1 to value 0"]
impl crate::Resettable for Swreg70PerformanceCnt1Spec {
    const RESET_VALUE: u32 = 0;
}
