#[doc = "Register `FC_DRM_PB[%s]` reader"]
pub type R = crate::R<FcDrmPbSpec>;
#[doc = "Register `FC_DRM_PB[%s]` writer"]
pub type W = crate::W<FcDrmPbSpec>;
#[doc = "Field `FC_DRM_PB` reader - Frame Composer DRM Packet Body Register Array"]
pub type FcDrmPbR = crate::FieldReader;
#[doc = "Field `FC_DRM_PB` writer - Frame Composer DRM Packet Body Register Array"]
pub type FcDrmPbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer DRM Packet Body Register Array"]
    #[inline(always)]
    pub fn fc_drm_pb(&self) -> FcDrmPbR {
        FcDrmPbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer DRM Packet Body Register Array"]
    #[inline(always)]
    #[must_use]
    pub fn fc_drm_pb(&mut self) -> FcDrmPbW<FcDrmPbSpec> {
        FcDrmPbW::new(self, 0)
    }
}
#[doc = "Frame Composer DRM Packet Body Register Array\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_drm_pb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_drm_pb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcDrmPbSpec;
impl crate::RegisterSpec for FcDrmPbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_drm_pb::R`](R) reader structure"]
impl crate::Readable for FcDrmPbSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_drm_pb::W`](W) writer structure"]
impl crate::Writable for FcDrmPbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_DRM_PB[%s]
to value 0"]
impl crate::Resettable for FcDrmPbSpec {
    const RESET_VALUE: u8 = 0;
}
