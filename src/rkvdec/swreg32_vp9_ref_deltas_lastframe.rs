#[doc = "Register `SWREG32_VP9_REF_DELTAS_LASTFRAME` reader"]
pub type R = crate::R<Swreg32Vp9RefDeltasLastframeSpec>;
#[doc = "Register `SWREG32_VP9_REF_DELTAS_LASTFRAME` writer"]
pub type W = crate::W<Swreg32Vp9RefDeltasLastframeSpec>;
#[doc = "Field `SW_VP9_REF_DELTAS_LASTFRAME` reader - vp9 ref deltas\n\nvp9 ref deltas of lastframe, for cal loopfilter filter type use"]
pub type SwVp9RefDeltasLastframeR = crate::FieldReader<u32>;
#[doc = "Field `SW_VP9_REF_DELTAS_LASTFRAME` writer - vp9 ref deltas\n\nvp9 ref deltas of lastframe, for cal loopfilter filter type use"]
pub type SwVp9RefDeltasLastframeW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - vp9 ref deltas\n\nvp9 ref deltas of lastframe, for cal loopfilter filter type use"]
    #[inline(always)]
    pub fn sw_vp9_ref_deltas_lastframe(&self) -> SwVp9RefDeltasLastframeR {
        SwVp9RefDeltasLastframeR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - vp9 ref deltas\n\nvp9 ref deltas of lastframe, for cal loopfilter filter type use"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_ref_deltas_lastframe(
        &mut self,
    ) -> SwVp9RefDeltasLastframeW<Swreg32Vp9RefDeltasLastframeSpec> {
        SwVp9RefDeltasLastframeW::new(self, 0)
    }
}
#[doc = "vp9 ref deltas\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg32_vp9_ref_deltas_lastframe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg32_vp9_ref_deltas_lastframe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg32Vp9RefDeltasLastframeSpec;
impl crate::RegisterSpec for Swreg32Vp9RefDeltasLastframeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg32_vp9_ref_deltas_lastframe::R`](R) reader structure"]
impl crate::Readable for Swreg32Vp9RefDeltasLastframeSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg32_vp9_ref_deltas_lastframe::W`](W) writer structure"]
impl crate::Writable for Swreg32Vp9RefDeltasLastframeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG32_VP9_REF_DELTAS_LASTFRAME to value 0"]
impl crate::Resettable for Swreg32Vp9RefDeltasLastframeSpec {
    const RESET_VALUE: u32 = 0;
}
