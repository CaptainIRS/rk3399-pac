#[doc = "Register `FC_RDRB0` reader"]
pub type R = crate::R<FcRdrb0Spec>;
#[doc = "Register `FC_RDRB0` writer"]
pub type W = crate::W<FcRdrb0Spec>;
#[doc = "Field `ACRFRAMEINTERPOLATION` reader - ACR Frame interpolation"]
pub type AcrframeinterpolationR = crate::FieldReader;
#[doc = "Field `ACRFRAMEINTERPOLATION` writer - ACR Frame interpolation"]
pub type AcrframeinterpolationW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ACR Frame interpolation"]
    #[inline(always)]
    pub fn acrframeinterpolation(&self) -> AcrframeinterpolationR {
        AcrframeinterpolationR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - ACR Frame interpolation"]
    #[inline(always)]
    #[must_use]
    pub fn acrframeinterpolation(&mut self) -> AcrframeinterpolationW<FcRdrb0Spec> {
        AcrframeinterpolationW::new(self, 0)
    }
}
#[doc = "Frame Composer Round Robin ACR Packet Insertion Register 0\n\nConfigures the Frame Composer (FC) RDRB frame interpolation for ACR packet insertion on\n\ndata island when FC is on RDRB mode for this packet.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcRdrb0Spec;
impl crate::RegisterSpec for FcRdrb0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_rdrb0::R`](R) reader structure"]
impl crate::Readable for FcRdrb0Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_rdrb0::W`](W) writer structure"]
impl crate::Writable for FcRdrb0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_RDRB0 to value 0"]
impl crate::Resettable for FcRdrb0Spec {
    const RESET_VALUE: u8 = 0;
}
