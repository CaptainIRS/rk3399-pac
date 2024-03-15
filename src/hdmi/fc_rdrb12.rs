#[doc = "Register `FC_RDRB12` reader"]
pub type R = crate::R<FcRdrb12Spec>;
#[doc = "Register `FC_RDRB12` writer"]
pub type W = crate::W<FcRdrb12Spec>;
#[doc = "Field `DRMFRAMEINTERPOLATION` reader - Description: DRM frame interpolation"]
pub type DrmframeinterpolationR = crate::FieldReader;
#[doc = "Field `DRMFRAMEINTERPOLATION` writer - Description: DRM frame interpolation"]
pub type DrmframeinterpolationW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Description: DRM frame interpolation"]
    #[inline(always)]
    pub fn drmframeinterpolation(&self) -> DrmframeinterpolationR {
        DrmframeinterpolationR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Description: DRM frame interpolation"]
    #[inline(always)]
    #[must_use]
    pub fn drmframeinterpolation(&mut self) -> DrmframeinterpolationW<FcRdrb12Spec> {
        DrmframeinterpolationW::new(self, 0)
    }
}
#[doc = "Description: DRM frame interpolation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcRdrb12Spec;
impl crate::RegisterSpec for FcRdrb12Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_rdrb12::R`](R) reader structure"]
impl crate::Readable for FcRdrb12Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_rdrb12::W`](W) writer structure"]
impl crate::Writable for FcRdrb12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_RDRB12 to value 0"]
impl crate::Resettable for FcRdrb12Spec {
    const RESET_VALUE: u8 = 0;
}
