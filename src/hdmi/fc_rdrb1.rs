#[doc = "Register `FC_RDRB1` reader"]
pub type R = crate::R<FcRdrb1Spec>;
#[doc = "Register `FC_RDRB1` writer"]
pub type W = crate::W<FcRdrb1Spec>;
#[doc = "Field `ACRPACKETLINESPACING` reader - ACR packet line spacing"]
pub type AcrpacketlinespacingR = crate::FieldReader;
#[doc = "Field `ACRPACKETLINESPACING` writer - ACR packet line spacing"]
pub type AcrpacketlinespacingW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ACRPACKETSINFRAME` reader - ACR packets in frame"]
pub type AcrpacketsinframeR = crate::FieldReader;
#[doc = "Field `ACRPACKETSINFRAME` writer - ACR packets in frame"]
pub type AcrpacketsinframeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ACR packet line spacing"]
    #[inline(always)]
    pub fn acrpacketlinespacing(&self) -> AcrpacketlinespacingR {
        AcrpacketlinespacingR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - ACR packets in frame"]
    #[inline(always)]
    pub fn acrpacketsinframe(&self) -> AcrpacketsinframeR {
        AcrpacketsinframeR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - ACR packet line spacing"]
    #[inline(always)]
    #[must_use]
    pub fn acrpacketlinespacing(&mut self) -> AcrpacketlinespacingW<FcRdrb1Spec> {
        AcrpacketlinespacingW::new(self, 0)
    }
    #[doc = "Bits 4:7 - ACR packets in frame"]
    #[inline(always)]
    #[must_use]
    pub fn acrpacketsinframe(&mut self) -> AcrpacketsinframeW<FcRdrb1Spec> {
        AcrpacketsinframeW::new(self, 4)
    }
}
#[doc = "Frame Composer Round Robin ACR Packet Insertion Register 1\n\nConfigures the Frame Composer (FC) RDRB line interpolation and number of packets in\n\nframe for the ACR packet insertion on data island when FC is on RDRB mode this packet.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcRdrb1Spec;
impl crate::RegisterSpec for FcRdrb1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_rdrb1::R`](R) reader structure"]
impl crate::Readable for FcRdrb1Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_rdrb1::W`](W) writer structure"]
impl crate::Writable for FcRdrb1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_RDRB1 to value 0"]
impl crate::Resettable for FcRdrb1Spec {
    const RESET_VALUE: u8 = 0;
}
