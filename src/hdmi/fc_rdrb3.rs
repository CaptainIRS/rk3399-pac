#[doc = "Register `FC_RDRB3` reader"]
pub type R = crate::R<FcRdrb3Spec>;
#[doc = "Register `FC_RDRB3` writer"]
pub type W = crate::W<FcRdrb3Spec>;
#[doc = "Field `AUDIPACKETLINESPACING` reader - Audio packets line spacing"]
pub type AudipacketlinespacingR = crate::FieldReader;
#[doc = "Field `AUDIPACKETLINESPACING` writer - Audio packets line spacing"]
pub type AudipacketlinespacingW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AUDIPACKETSINFRAME` reader - Audio packets per frame"]
pub type AudipacketsinframeR = crate::FieldReader;
#[doc = "Field `AUDIPACKETSINFRAME` writer - Audio packets per frame"]
pub type AudipacketsinframeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Audio packets line spacing"]
    #[inline(always)]
    pub fn audipacketlinespacing(&self) -> AudipacketlinespacingR {
        AudipacketlinespacingR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Audio packets per frame"]
    #[inline(always)]
    pub fn audipacketsinframe(&self) -> AudipacketsinframeR {
        AudipacketsinframeR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Audio packets line spacing"]
    #[inline(always)]
    #[must_use]
    pub fn audipacketlinespacing(&mut self) -> AudipacketlinespacingW<FcRdrb3Spec> {
        AudipacketlinespacingW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Audio packets per frame"]
    #[inline(always)]
    #[must_use]
    pub fn audipacketsinframe(&mut self) -> AudipacketsinframeW<FcRdrb3Spec> {
        AudipacketsinframeW::new(self, 4)
    }
}
#[doc = "Audio packets line spacing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcRdrb3Spec;
impl crate::RegisterSpec for FcRdrb3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_rdrb3::R`](R) reader structure"]
impl crate::Readable for FcRdrb3Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_rdrb3::W`](W) writer structure"]
impl crate::Writable for FcRdrb3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_RDRB3 to value 0"]
impl crate::Resettable for FcRdrb3Spec {
    const RESET_VALUE: u8 = 0;
}
