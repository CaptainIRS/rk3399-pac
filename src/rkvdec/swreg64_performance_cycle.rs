#[doc = "Register `SWREG64_PERFORMANCE_CYCLE` reader"]
pub type R = crate::R<Swreg64PerformanceCycleSpec>;
#[doc = "Register `SWREG64_PERFORMANCE_CYCLE` writer"]
pub type W = crate::W<Swreg64PerformanceCycleSpec>;
#[doc = "Field `SW_PERFORMANCE_CYCLE` reader - hevc running cycle\n\nhevc running cycle\n\nif just want to analys a frame performance cycle, should set the\n\nregister 0 before start a frame"]
pub type SwPerformanceCycleR = crate::FieldReader<u32>;
#[doc = "Field `SW_PERFORMANCE_CYCLE` writer - hevc running cycle\n\nhevc running cycle\n\nif just want to analys a frame performance cycle, should set the\n\nregister 0 before start a frame"]
pub type SwPerformanceCycleW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - hevc running cycle\n\nhevc running cycle\n\nif just want to analys a frame performance cycle, should set the\n\nregister 0 before start a frame"]
    #[inline(always)]
    pub fn sw_performance_cycle(&self) -> SwPerformanceCycleR {
        SwPerformanceCycleR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - hevc running cycle\n\nhevc running cycle\n\nif just want to analys a frame performance cycle, should set the\n\nregister 0 before start a frame"]
    #[inline(always)]
    #[must_use]
    pub fn sw_performance_cycle(&mut self) -> SwPerformanceCycleW<Swreg64PerformanceCycleSpec> {
        SwPerformanceCycleW::new(self, 0)
    }
}
#[doc = "hevc performance cycle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg64_performance_cycle::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg64_performance_cycle::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg64PerformanceCycleSpec;
impl crate::RegisterSpec for Swreg64PerformanceCycleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg64_performance_cycle::R`](R) reader structure"]
impl crate::Readable for Swreg64PerformanceCycleSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg64_performance_cycle::W`](W) writer structure"]
impl crate::Writable for Swreg64PerformanceCycleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG64_PERFORMANCE_CYCLE to value 0"]
impl crate::Resettable for Swreg64PerformanceCycleSpec {
    const RESET_VALUE: u32 = 0;
}
