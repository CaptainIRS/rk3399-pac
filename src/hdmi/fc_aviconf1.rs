#[doc = "Register `FC_AVICONF1` reader"]
pub type R = crate::R<FcAviconf1Spec>;
#[doc = "Register `FC_AVICONF1` writer"]
pub type W = crate::W<FcAviconf1Spec>;
#[doc = "Field `ACTIVE_ASPECT_RATIO` reader - Active aspect ratio"]
pub type ActiveAspectRatioR = crate::FieldReader;
#[doc = "Field `ACTIVE_ASPECT_RATIO` writer - Active aspect ratio"]
pub type ActiveAspectRatioW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PICTURE_ASPECT_RATIO` reader - Picture aspect ratio"]
pub type PictureAspectRatioR = crate::FieldReader;
#[doc = "Field `PICTURE_ASPECT_RATIO` writer - Picture aspect ratio"]
pub type PictureAspectRatioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COLORIMETRY` reader - Colorimetry"]
pub type ColorimetryR = crate::FieldReader;
#[doc = "Field `COLORIMETRY` writer - Colorimetry"]
pub type ColorimetryW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Active aspect ratio"]
    #[inline(always)]
    pub fn active_aspect_ratio(&self) -> ActiveAspectRatioR {
        ActiveAspectRatioR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - Picture aspect ratio"]
    #[inline(always)]
    pub fn picture_aspect_ratio(&self) -> PictureAspectRatioR {
        PictureAspectRatioR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Colorimetry"]
    #[inline(always)]
    pub fn colorimetry(&self) -> ColorimetryR {
        ColorimetryR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:3 - Active aspect ratio"]
    #[inline(always)]
    #[must_use]
    pub fn active_aspect_ratio(&mut self) -> ActiveAspectRatioW<FcAviconf1Spec> {
        ActiveAspectRatioW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Picture aspect ratio"]
    #[inline(always)]
    #[must_use]
    pub fn picture_aspect_ratio(&mut self) -> PictureAspectRatioW<FcAviconf1Spec> {
        PictureAspectRatioW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Colorimetry"]
    #[inline(always)]
    #[must_use]
    pub fn colorimetry(&mut self) -> ColorimetryW<FcAviconf1Spec> {
        ColorimetryW::new(self, 6)
    }
}
#[doc = "Frame Composer AVI Packet Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_aviconf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_aviconf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAviconf1Spec;
impl crate::RegisterSpec for FcAviconf1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_aviconf1::R`](R) reader structure"]
impl crate::Readable for FcAviconf1Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_aviconf1::W`](W) writer structure"]
impl crate::Writable for FcAviconf1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AVICONF1 to value 0"]
impl crate::Resettable for FcAviconf1Spec {
    const RESET_VALUE: u8 = 0;
}
