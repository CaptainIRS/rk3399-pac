#[doc = "Register `PERFCNT_VAL1` reader"]
pub type R = crate::R<PerfcntVal1Spec>;
#[doc = "Register `PERFCNT_VAL1` writer"]
pub type W = crate::W<PerfcntVal1Spec>;
#[doc = "Field `PERFCNT_VAL1` reader - Performance counter 1 value"]
pub type PerfcntVal1R = crate::FieldReader<u32>;
#[doc = "Field `PERFCNT_VAL1` writer - Performance counter 1 value"]
pub type PerfcntVal1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Performance counter 1 value"]
    #[inline(always)]
    pub fn perfcnt_val1(&self) -> PerfcntVal1R {
        PerfcntVal1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Performance counter 1 value"]
    #[inline(always)]
    #[must_use]
    pub fn perfcnt_val1(&mut self) -> PerfcntVal1W<PerfcntVal1Spec> {
        PerfcntVal1W::new(self, 0)
    }
}
#[doc = "performance counter 1 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perfcnt_val1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perfcnt_val1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerfcntVal1Spec;
impl crate::RegisterSpec for PerfcntVal1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perfcnt_val1::R`](R) reader structure"]
impl crate::Readable for PerfcntVal1Spec {}
#[doc = "`write(|w| ..)` method takes [`perfcnt_val1::W`](W) writer structure"]
impl crate::Writable for PerfcntVal1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERFCNT_VAL1 to value 0"]
impl crate::Resettable for PerfcntVal1Spec {
    const RESET_VALUE: u32 = 0;
}
