#[doc = "Register `FC_RDRB6` reader"]
pub type R = crate::R<FcRdrb6Spec>;
#[doc = "Register `FC_RDRB6` writer"]
pub type W = crate::W<FcRdrb6Spec>;
#[doc = "Field `AVIFRAMEINTERPOLATION` reader - Frames interpolated between AVI packets"]
pub type AviframeinterpolationR = crate::FieldReader;
#[doc = "Field `AVIFRAMEINTERPOLATION` writer - Frames interpolated between AVI packets"]
pub type AviframeinterpolationW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Frames interpolated between AVI packets"]
    #[inline(always)]
    pub fn aviframeinterpolation(&self) -> AviframeinterpolationR {
        AviframeinterpolationR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Frames interpolated between AVI packets"]
    #[inline(always)]
    #[must_use]
    pub fn aviframeinterpolation(&mut self) -> AviframeinterpolationW<FcRdrb6Spec> {
        AviframeinterpolationW::new(self, 0)
    }
}
#[doc = "Frames interpolated between AVI packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcRdrb6Spec;
impl crate::RegisterSpec for FcRdrb6Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_rdrb6::R`](R) reader structure"]
impl crate::Readable for FcRdrb6Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_rdrb6::W`](W) writer structure"]
impl crate::Writable for FcRdrb6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_RDRB6 to value 0"]
impl crate::Resettable for FcRdrb6Spec {
    const RESET_VALUE: u8 = 0;
}
