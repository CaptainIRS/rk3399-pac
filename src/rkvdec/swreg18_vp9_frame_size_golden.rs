#[doc = "Register `SWREG18_VP9_FRAME_SIZE_GOLDEN` reader"]
pub type R = crate::R<Swreg18Vp9FrameSizeGoldenSpec>;
#[doc = "Register `SWREG18_VP9_FRAME_SIZE_GOLDEN` writer"]
pub type W = crate::W<Swreg18Vp9FrameSizeGoldenSpec>;
#[doc = "Field `SW_FRAMEWIDTH_GOLDEN` reader - golden frame_size_width\n\ngolden frame_size_width"]
pub type SwFramewidthGoldenR = crate::FieldReader<u16>;
#[doc = "Field `SW_FRAMEWIDTH_GOLDEN` writer - golden frame_size_width\n\ngolden frame_size_width"]
pub type SwFramewidthGoldenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SW_FRAMEHEIGHT_GOLDEN` reader - golden frame_size_height\n\ngolden frame_size_height"]
pub type SwFrameheightGoldenR = crate::FieldReader<u16>;
#[doc = "Field `SW_FRAMEHEIGHT_GOLDEN` writer - golden frame_size_height\n\ngolden frame_size_height"]
pub type SwFrameheightGoldenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - golden frame_size_width\n\ngolden frame_size_width"]
    #[inline(always)]
    pub fn sw_framewidth_golden(&self) -> SwFramewidthGoldenR {
        SwFramewidthGoldenR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - golden frame_size_height\n\ngolden frame_size_height"]
    #[inline(always)]
    pub fn sw_frameheight_golden(&self) -> SwFrameheightGoldenR {
        SwFrameheightGoldenR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - golden frame_size_width\n\ngolden frame_size_width"]
    #[inline(always)]
    #[must_use]
    pub fn sw_framewidth_golden(&mut self) -> SwFramewidthGoldenW<Swreg18Vp9FrameSizeGoldenSpec> {
        SwFramewidthGoldenW::new(self, 0)
    }
    #[doc = "Bits 16:31 - golden frame_size_height\n\ngolden frame_size_height"]
    #[inline(always)]
    #[must_use]
    pub fn sw_frameheight_golden(&mut self) -> SwFrameheightGoldenW<Swreg18Vp9FrameSizeGoldenSpec> {
        SwFrameheightGoldenW::new(self, 16)
    }
}
#[doc = "vp9 golden frame size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg18_vp9_frame_size_golden::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg18_vp9_frame_size_golden::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg18Vp9FrameSizeGoldenSpec;
impl crate::RegisterSpec for Swreg18Vp9FrameSizeGoldenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg18_vp9_frame_size_golden::R`](R) reader structure"]
impl crate::Readable for Swreg18Vp9FrameSizeGoldenSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg18_vp9_frame_size_golden::W`](W) writer structure"]
impl crate::Writable for Swreg18Vp9FrameSizeGoldenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG18_VP9_FRAME_SIZE_GOLDEN to value 0"]
impl crate::Resettable for Swreg18Vp9FrameSizeGoldenSpec {
    const RESET_VALUE: u32 = 0;
}
