#[doc = "Register `AWB_FRAMES` reader"]
pub type R = crate::R<AwbFramesSpec>;
#[doc = "Register `AWB_FRAMES` writer"]
pub type W = crate::W<AwbFramesSpec>;
#[doc = "Field `AWB_FRAMES` reader - number of frames-1 used for mean value calculation\n\n(value of 0 means 1 frame, value of 7 means 8 frames)\n\n"]
pub type AwbFramesR = crate::FieldReader;
#[doc = "Field `AWB_FRAMES` writer - number of frames-1 used for mean value calculation\n\n(value of 0 means 1 frame, value of 7 means 8 frames)\n\n"]
pub type AwbFramesW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - number of frames-1 used for mean value calculation\n\n(value of 0 means 1 frame, value of 7 means 8 frames)\n\n"]
    #[inline(always)]
    pub fn awb_frames(&self) -> AwbFramesR {
        AwbFramesR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - number of frames-1 used for mean value calculation\n\n(value of 0 means 1 frame, value of 7 means 8 frames)\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn awb_frames(&mut self) -> AwbFramesW<AwbFramesSpec> {
        AwbFramesW::new(self, 0)
    }
}
#[doc = "Auto white balance mean value over multiple frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_frames::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_frames::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwbFramesSpec;
impl crate::RegisterSpec for AwbFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb_frames::R`](R) reader structure"]
impl crate::Readable for AwbFramesSpec {}
#[doc = "`write(|w| ..)` method takes [`awb_frames::W`](W) writer structure"]
impl crate::Writable for AwbFramesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWB_FRAMES to value 0"]
impl crate::Resettable for AwbFramesSpec {
    const RESET_VALUE: u32 = 0;
}
