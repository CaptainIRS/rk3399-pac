#[doc = "Register `SWREG17_VP9_FRAME_SIZE_LAST` reader"]
pub type R = crate::R<Swreg17Vp9FrameSizeLastSpec>;
#[doc = "Register `SWREG17_VP9_FRAME_SIZE_LAST` writer"]
pub type W = crate::W<Swreg17Vp9FrameSizeLastSpec>;
#[doc = "Field `SW_FRAMEWIDTH_LAST` reader - last frame frame_size_width\n\nlast frame frame_size_width"]
pub type SwFramewidthLastR = crate::FieldReader<u16>;
#[doc = "Field `SW_FRAMEWIDTH_LAST` writer - last frame frame_size_width\n\nlast frame frame_size_width"]
pub type SwFramewidthLastW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SW_FRAMEHEIGHT_LAST` reader - last frame frame_size_height\n\nlast frame frame_size_height"]
pub type SwFrameheightLastR = crate::FieldReader<u16>;
#[doc = "Field `SW_FRAMEHEIGHT_LAST` writer - last frame frame_size_height\n\nlast frame frame_size_height"]
pub type SwFrameheightLastW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - last frame frame_size_width\n\nlast frame frame_size_width"]
    #[inline(always)]
    pub fn sw_framewidth_last(&self) -> SwFramewidthLastR {
        SwFramewidthLastR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - last frame frame_size_height\n\nlast frame frame_size_height"]
    #[inline(always)]
    pub fn sw_frameheight_last(&self) -> SwFrameheightLastR {
        SwFrameheightLastR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - last frame frame_size_width\n\nlast frame frame_size_width"]
    #[inline(always)]
    #[must_use]
    pub fn sw_framewidth_last(&mut self) -> SwFramewidthLastW<Swreg17Vp9FrameSizeLastSpec> {
        SwFramewidthLastW::new(self, 0)
    }
    #[doc = "Bits 16:31 - last frame frame_size_height\n\nlast frame frame_size_height"]
    #[inline(always)]
    #[must_use]
    pub fn sw_frameheight_last(&mut self) -> SwFrameheightLastW<Swreg17Vp9FrameSizeLastSpec> {
        SwFrameheightLastW::new(self, 16)
    }
}
#[doc = "vp9 last frame size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg17_vp9_frame_size_last::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg17_vp9_frame_size_last::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg17Vp9FrameSizeLastSpec;
impl crate::RegisterSpec for Swreg17Vp9FrameSizeLastSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg17_vp9_frame_size_last::R`](R) reader structure"]
impl crate::Readable for Swreg17Vp9FrameSizeLastSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg17_vp9_frame_size_last::W`](W) writer structure"]
impl crate::Writable for Swreg17Vp9FrameSizeLastSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG17_VP9_FRAME_SIZE_LAST to value 0"]
impl crate::Resettable for Swreg17Vp9FrameSizeLastSpec {
    const RESET_VALUE: u32 = 0;
}
