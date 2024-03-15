#[doc = "Register `FC_DRM_HB[%s]` reader"]
pub type R = crate::R<FcDrmHbSpec>;
#[doc = "Register `FC_DRM_HB[%s]` writer"]
pub type W = crate::W<FcDrmHbSpec>;
#[doc = "Field `FC_DRM_HB` reader - Frame Composer DRM Packet Header Register Array"]
pub type FcDrmHbR = crate::FieldReader;
#[doc = "Field `FC_DRM_HB` writer - Frame Composer DRM Packet Header Register Array"]
pub type FcDrmHbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer DRM Packet Header Register Array"]
    #[inline(always)]
    pub fn fc_drm_hb(&self) -> FcDrmHbR {
        FcDrmHbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer DRM Packet Header Register Array"]
    #[inline(always)]
    #[must_use]
    pub fn fc_drm_hb(&mut self) -> FcDrmHbW<FcDrmHbSpec> {
        FcDrmHbW::new(self, 0)
    }
}
#[doc = "Frame Composer DRM Packet Header Register Array\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_drm_hb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_drm_hb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcDrmHbSpec;
impl crate::RegisterSpec for FcDrmHbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_drm_hb::R`](R) reader structure"]
impl crate::Readable for FcDrmHbSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_drm_hb::W`](W) writer structure"]
impl crate::Writable for FcDrmHbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_DRM_HB[%s]
to value 0"]
impl crate::Resettable for FcDrmHbSpec {
    const RESET_VALUE: u8 = 0;
}
