#[doc = "Register `FC_RDRB2` reader"]
pub type R = crate::R<FcRdrb2Spec>;
#[doc = "Register `FC_RDRB2` writer"]
pub type W = crate::W<FcRdrb2Spec>;
#[doc = "Field `AUDIFRAMEINTERPOLATION` reader - Audio frame interpolation"]
pub type AudiframeinterpolationR = crate::FieldReader;
#[doc = "Field `AUDIFRAMEINTERPOLATION` writer - Audio frame interpolation"]
pub type AudiframeinterpolationW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Audio frame interpolation"]
    #[inline(always)]
    pub fn audiframeinterpolation(&self) -> AudiframeinterpolationR {
        AudiframeinterpolationR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Audio frame interpolation"]
    #[inline(always)]
    #[must_use]
    pub fn audiframeinterpolation(&mut self) -> AudiframeinterpolationW<FcRdrb2Spec> {
        AudiframeinterpolationW::new(self, 0)
    }
}
#[doc = "Audio frame interpolation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcRdrb2Spec;
impl crate::RegisterSpec for FcRdrb2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_rdrb2::R`](R) reader structure"]
impl crate::Readable for FcRdrb2Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_rdrb2::W`](W) writer structure"]
impl crate::Writable for FcRdrb2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_RDRB2 to value 0"]
impl crate::Resettable for FcRdrb2Spec {
    const RESET_VALUE: u8 = 0;
}
