#[doc = "Register `FC_RDRB7` reader"]
pub type R = crate::R<FcRdrb7Spec>;
#[doc = "Register `FC_RDRB7` writer"]
pub type W = crate::W<FcRdrb7Spec>;
#[doc = "Field `AVIPACKETLINESPACING` reader - AVI packets line spacing"]
pub type AvipacketlinespacingR = crate::FieldReader;
#[doc = "Field `AVIPACKETLINESPACING` writer - AVI packets line spacing"]
pub type AvipacketlinespacingW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AVIPACKETSINFRAME` reader - AVI packets per frame"]
pub type AvipacketsinframeR = crate::FieldReader;
#[doc = "Field `AVIPACKETSINFRAME` writer - AVI packets per frame"]
pub type AvipacketsinframeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - AVI packets line spacing"]
    #[inline(always)]
    pub fn avipacketlinespacing(&self) -> AvipacketlinespacingR {
        AvipacketlinespacingR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - AVI packets per frame"]
    #[inline(always)]
    pub fn avipacketsinframe(&self) -> AvipacketsinframeR {
        AvipacketsinframeR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - AVI packets line spacing"]
    #[inline(always)]
    #[must_use]
    pub fn avipacketlinespacing(&mut self) -> AvipacketlinespacingW<FcRdrb7Spec> {
        AvipacketlinespacingW::new(self, 0)
    }
    #[doc = "Bits 4:7 - AVI packets per frame"]
    #[inline(always)]
    #[must_use]
    pub fn avipacketsinframe(&mut self) -> AvipacketsinframeW<FcRdrb7Spec> {
        AvipacketsinframeW::new(self, 4)
    }
}
#[doc = "Frame Composer Round Robin AVI Packet Insertion Register 7\n\nConfigures the Frame Composer (FC) RDRB line interpolation and number of packets in\n\nframe for the AVI packet insertion on data island when FC is on RDRB mode this packet.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcRdrb7Spec;
impl crate::RegisterSpec for FcRdrb7Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_rdrb7::R`](R) reader structure"]
impl crate::Readable for FcRdrb7Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_rdrb7::W`](W) writer structure"]
impl crate::Writable for FcRdrb7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_RDRB7 to value 0"]
impl crate::Resettable for FcRdrb7Spec {
    const RESET_VALUE: u8 = 0;
}
