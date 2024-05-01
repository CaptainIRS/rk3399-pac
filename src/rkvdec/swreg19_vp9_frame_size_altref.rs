#[doc = "Register `SWREG19_VP9_FRAME_SIZE_ALTREF` reader"]
pub type R = crate::R<Swreg19Vp9FrameSizeAltrefSpec>;
#[doc = "Register `SWREG19_VP9_FRAME_SIZE_ALTREF` writer"]
pub type W = crate::W<Swreg19Vp9FrameSizeAltrefSpec>;
#[doc = "Field `SW_FRAMEWIDTH_ALFTER` reader - alfter frame_size_width\n\nalfter frame_size_width"]
pub type SwFramewidthAlfterR = crate::FieldReader<u16>;
#[doc = "Field `SW_FRAMEWIDTH_ALFTER` writer - alfter frame_size_width\n\nalfter frame_size_width"]
pub type SwFramewidthAlfterW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SW_FRAMEHEIGHT_ALFTER` reader - alfter frame_size_height\n\nalfter frame_size_height"]
pub type SwFrameheightAlfterR = crate::FieldReader<u16>;
#[doc = "Field `SW_FRAMEHEIGHT_ALFTER` writer - alfter frame_size_height\n\nalfter frame_size_height"]
pub type SwFrameheightAlfterW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - alfter frame_size_width\n\nalfter frame_size_width"]
    #[inline(always)]
    pub fn sw_framewidth_alfter(&self) -> SwFramewidthAlfterR {
        SwFramewidthAlfterR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - alfter frame_size_height\n\nalfter frame_size_height"]
    #[inline(always)]
    pub fn sw_frameheight_alfter(&self) -> SwFrameheightAlfterR {
        SwFrameheightAlfterR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - alfter frame_size_width\n\nalfter frame_size_width"]
    #[inline(always)]
    #[must_use]
    pub fn sw_framewidth_alfter(&mut self) -> SwFramewidthAlfterW<Swreg19Vp9FrameSizeAltrefSpec> {
        SwFramewidthAlfterW::new(self, 0)
    }
    #[doc = "Bits 16:31 - alfter frame_size_height\n\nalfter frame_size_height"]
    #[inline(always)]
    #[must_use]
    pub fn sw_frameheight_alfter(&mut self) -> SwFrameheightAlfterW<Swreg19Vp9FrameSizeAltrefSpec> {
        SwFrameheightAlfterW::new(self, 16)
    }
}
#[doc = "vp9 alfter frame size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg19_vp9_frame_size_altref::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg19_vp9_frame_size_altref::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg19Vp9FrameSizeAltrefSpec;
impl crate::RegisterSpec for Swreg19Vp9FrameSizeAltrefSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg19_vp9_frame_size_altref::R`](R) reader structure"]
impl crate::Readable for Swreg19Vp9FrameSizeAltrefSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg19_vp9_frame_size_altref::W`](W) writer structure"]
impl crate::Writable for Swreg19Vp9FrameSizeAltrefSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG19_VP9_FRAME_SIZE_ALTREF to value 0"]
impl crate::Resettable for Swreg19Vp9FrameSizeAltrefSpec {
    const RESET_VALUE: u32 = 0;
}
