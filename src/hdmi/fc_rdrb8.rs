#[doc = "Register `FC_RDRB8` reader"]
pub type R = crate::R<FcRdrb8Spec>;
#[doc = "Register `FC_RDRB8` writer"]
pub type W = crate::W<FcRdrb8Spec>;
#[doc = "Field `AMPFRAMEINTERPOLATION` reader - AMP frame interpolation"]
pub type AmpframeinterpolationR = crate::FieldReader;
#[doc = "Field `AMPFRAMEINTERPOLATION` writer - AMP frame interpolation"]
pub type AmpframeinterpolationW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - AMP frame interpolation"]
    #[inline(always)]
    pub fn ampframeinterpolation(&self) -> AmpframeinterpolationR {
        AmpframeinterpolationR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - AMP frame interpolation"]
    #[inline(always)]
    #[must_use]
    pub fn ampframeinterpolation(&mut self) -> AmpframeinterpolationW<FcRdrb8Spec> {
        AmpframeinterpolationW::new(self, 0)
    }
}
#[doc = "Frame Composer Round Robin AMP Packet Insertion Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcRdrb8Spec;
impl crate::RegisterSpec for FcRdrb8Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_rdrb8::R`](R) reader structure"]
impl crate::Readable for FcRdrb8Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_rdrb8::W`](W) writer structure"]
impl crate::Writable for FcRdrb8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_RDRB8 to value 0"]
impl crate::Resettable for FcRdrb8Spec {
    const RESET_VALUE: u8 = 0;
}
