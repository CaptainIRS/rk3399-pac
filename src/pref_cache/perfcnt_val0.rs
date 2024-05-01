#[doc = "Register `PERFCNT_VAL0` reader"]
pub type R = crate::R<PerfcntVal0Spec>;
#[doc = "Register `PERFCNT_VAL0` writer"]
pub type W = crate::W<PerfcntVal0Spec>;
#[doc = "Field `PERFCNT_VAL0` reader - Performance counter 0 value"]
pub type PerfcntVal0R = crate::FieldReader<u32>;
#[doc = "Field `PERFCNT_VAL0` writer - Performance counter 0 value"]
pub type PerfcntVal0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Performance counter 0 value"]
    #[inline(always)]
    pub fn perfcnt_val0(&self) -> PerfcntVal0R {
        PerfcntVal0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Performance counter 0 value"]
    #[inline(always)]
    #[must_use]
    pub fn perfcnt_val0(&mut self) -> PerfcntVal0W<PerfcntVal0Spec> {
        PerfcntVal0W::new(self, 0)
    }
}
#[doc = "performance counter 0 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perfcnt_val0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perfcnt_val0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerfcntVal0Spec;
impl crate::RegisterSpec for PerfcntVal0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perfcnt_val0::R`](R) reader structure"]
impl crate::Readable for PerfcntVal0Spec {}
#[doc = "`write(|w| ..)` method takes [`perfcnt_val0::W`](W) writer structure"]
impl crate::Writable for PerfcntVal0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERFCNT_VAL0 to value 0"]
impl crate::Resettable for PerfcntVal0Spec {
    const RESET_VALUE: u32 = 0;
}
