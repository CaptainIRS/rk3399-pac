#[doc = "Register `SWREG71_PERFORMANCE_CNT2` reader"]
pub type R = crate::R<Swreg71PerformanceCnt2Spec>;
#[doc = "Register `SWREG71_PERFORMANCE_CNT2` writer"]
pub type W = crate::W<Swreg71PerformanceCnt2Spec>;
#[doc = "Field `PERF_CNT2` reader - Field0000 Abstract\n\nField0000 Description"]
pub type PerfCnt2R = crate::FieldReader<u32>;
#[doc = "Field `PERF_CNT2` writer - Field0000 Abstract\n\nField0000 Description"]
pub type PerfCnt2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Field0000 Abstract\n\nField0000 Description"]
    #[inline(always)]
    pub fn perf_cnt2(&self) -> PerfCnt2R {
        PerfCnt2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Field0000 Abstract\n\nField0000 Description"]
    #[inline(always)]
    #[must_use]
    pub fn perf_cnt2(&mut self) -> PerfCnt2W<Swreg71PerformanceCnt2Spec> {
        PerfCnt2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg71_performance_cnt2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg71_performance_cnt2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg71PerformanceCnt2Spec;
impl crate::RegisterSpec for Swreg71PerformanceCnt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg71_performance_cnt2::R`](R) reader structure"]
impl crate::Readable for Swreg71PerformanceCnt2Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg71_performance_cnt2::W`](W) writer structure"]
impl crate::Writable for Swreg71PerformanceCnt2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG71_PERFORMANCE_CNT2 to value 0"]
impl crate::Resettable for Swreg71PerformanceCnt2Spec {
    const RESET_VALUE: u32 = 0;
}
