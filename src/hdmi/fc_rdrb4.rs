#[doc = "Register `FC_RDRB4` reader"]
pub type R = crate::R<FcRdrb4Spec>;
#[doc = "Register `FC_RDRB4` writer"]
pub type W = crate::W<FcRdrb4Spec>;
#[doc = "Field `GCPFRAMEINTERPOLATION` reader - Frames interpolated between GCP packets"]
pub type GcpframeinterpolationR = crate::FieldReader;
#[doc = "Field `GCPFRAMEINTERPOLATION` writer - Frames interpolated between GCP packets"]
pub type GcpframeinterpolationW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Frames interpolated between GCP packets"]
    #[inline(always)]
    pub fn gcpframeinterpolation(&self) -> GcpframeinterpolationR {
        GcpframeinterpolationR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Frames interpolated between GCP packets"]
    #[inline(always)]
    #[must_use]
    pub fn gcpframeinterpolation(&mut self) -> GcpframeinterpolationW<FcRdrb4Spec> {
        GcpframeinterpolationW::new(self, 0)
    }
}
#[doc = "Frames interpolated between GCP packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcRdrb4Spec;
impl crate::RegisterSpec for FcRdrb4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_rdrb4::R`](R) reader structure"]
impl crate::Readable for FcRdrb4Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_rdrb4::W`](W) writer structure"]
impl crate::Writable for FcRdrb4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_RDRB4 to value 0"]
impl crate::Resettable for FcRdrb4Spec {
    const RESET_VALUE: u8 = 0;
}
