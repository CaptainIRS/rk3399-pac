#[doc = "Register `A53_PERF_WORKING_CNT` reader"]
pub type R = crate::R<A53PerfWorkingCntSpec>;
#[doc = "Register `A53_PERF_WORKING_CNT` writer"]
pub type W = crate::W<A53PerfWorkingCntSpec>;
#[doc = "Field `WORKING_CNT_R` reader - working counter"]
pub type WorkingCntRR = crate::FieldReader<u32>;
#[doc = "Field `WORKING_CNT_R` writer - working counter"]
pub type WorkingCntRW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - working counter"]
    #[inline(always)]
    pub fn working_cnt_r(&self) -> WorkingCntRR {
        WorkingCntRR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - working counter"]
    #[inline(always)]
    #[must_use]
    pub fn working_cnt_r(&mut self) -> WorkingCntRW<A53PerfWorkingCntSpec> {
        WorkingCntRW::new(self, 0)
    }
}
#[doc = "a53 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a53_perf_working_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a53_perf_working_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A53PerfWorkingCntSpec;
impl crate::RegisterSpec for A53PerfWorkingCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a53_perf_working_cnt::R`](R) reader structure"]
impl crate::Readable for A53PerfWorkingCntSpec {}
#[doc = "`write(|w| ..)` method takes [`a53_perf_working_cnt::W`](W) writer structure"]
impl crate::Writable for A53PerfWorkingCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets A53_PERF_WORKING_CNT to value 0"]
impl crate::Resettable for A53PerfWorkingCntSpec {
    const RESET_VALUE: u32 = 0;
}
