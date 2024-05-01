#[doc = "Register `SWREG171_PERF_WORKING_CNT` reader"]
pub type R = crate::R<Swreg171PerfWorkingCntSpec>;
#[doc = "Register `SWREG171_PERF_WORKING_CNT` writer"]
pub type W = crate::W<Swreg171PerfWorkingCntSpec>;
#[doc = "Field `PERF_WORKING_CNT` reader - perf_working_cnt\n\nperf_working_cnt"]
pub type PerfWorkingCntR = crate::FieldReader<u32>;
#[doc = "Field `PERF_WORKING_CNT` writer - perf_working_cnt\n\nperf_working_cnt"]
pub type PerfWorkingCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - perf_working_cnt\n\nperf_working_cnt"]
    #[inline(always)]
    pub fn perf_working_cnt(&self) -> PerfWorkingCntR {
        PerfWorkingCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - perf_working_cnt\n\nperf_working_cnt"]
    #[inline(always)]
    #[must_use]
    pub fn perf_working_cnt(&mut self) -> PerfWorkingCntW<Swreg171PerfWorkingCntSpec> {
        PerfWorkingCntW::new(self, 0)
    }
}
#[doc = "perf_working_cnt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg171_perf_working_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg171_perf_working_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg171PerfWorkingCntSpec;
impl crate::RegisterSpec for Swreg171PerfWorkingCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg171_perf_working_cnt::R`](R) reader structure"]
impl crate::Readable for Swreg171PerfWorkingCntSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg171_perf_working_cnt::W`](W) writer structure"]
impl crate::Writable for Swreg171PerfWorkingCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG171_PERF_WORKING_CNT to value 0"]
impl crate::Resettable for Swreg171PerfWorkingCntSpec {
    const RESET_VALUE: u32 = 0;
}
