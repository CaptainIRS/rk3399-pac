#[doc = "Register `SWREG36_VP9_LASTTILE_SIZE` reader"]
pub type R = crate::R<Swreg36Vp9LasttileSizeSpec>;
#[doc = "Register `SWREG36_VP9_LASTTILE_SIZE` writer"]
pub type W = crate::W<Swreg36Vp9LasttileSizeSpec>;
#[doc = "Field `SW_VP9_LASTTILE_SIZE` reader - vp9 last tile size\n\nvp9 last tile size ofr cur frame\n\nits unit is byte\n\nThe real meaning the current frame size"]
pub type SwVp9LasttileSizeR = crate::FieldReader<u32>;
#[doc = "Field `SW_VP9_LASTTILE_SIZE` writer - vp9 last tile size\n\nvp9 last tile size ofr cur frame\n\nits unit is byte\n\nThe real meaning the current frame size"]
pub type SwVp9LasttileSizeW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - vp9 last tile size\n\nvp9 last tile size ofr cur frame\n\nits unit is byte\n\nThe real meaning the current frame size"]
    #[inline(always)]
    pub fn sw_vp9_lasttile_size(&self) -> SwVp9LasttileSizeR {
        SwVp9LasttileSizeR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - vp9 last tile size\n\nvp9 last tile size ofr cur frame\n\nits unit is byte\n\nThe real meaning the current frame size"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_lasttile_size(&mut self) -> SwVp9LasttileSizeW<Swreg36Vp9LasttileSizeSpec> {
        SwVp9LasttileSizeW::new(self, 0)
    }
}
#[doc = "vp9 lasttile size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg36_vp9_lasttile_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg36_vp9_lasttile_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg36Vp9LasttileSizeSpec;
impl crate::RegisterSpec for Swreg36Vp9LasttileSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg36_vp9_lasttile_size::R`](R) reader structure"]
impl crate::Readable for Swreg36Vp9LasttileSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg36_vp9_lasttile_size::W`](W) writer structure"]
impl crate::Writable for Swreg36Vp9LasttileSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG36_VP9_LASTTILE_SIZE to value 0"]
impl crate::Resettable for Swreg36Vp9LasttileSizeSpec {
    const RESET_VALUE: u32 = 0;
}
