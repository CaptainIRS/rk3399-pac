#[doc = "Register `FC_RDRB9` reader"]
pub type R = crate::R<FcRdrb9Spec>;
#[doc = "Register `FC_RDRB9` writer"]
pub type W = crate::W<FcRdrb9Spec>;
#[doc = "Field `AMPPACKETLINESPACING` reader - AMP packets line spacing"]
pub type AmppacketlinespacingR = crate::FieldReader;
#[doc = "Field `AMPPACKETLINESPACING` writer - AMP packets line spacing"]
pub type AmppacketlinespacingW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AMPPACKETSINFRAME` reader - AMP packets per frame"]
pub type AmppacketsinframeR = crate::FieldReader;
#[doc = "Field `AMPPACKETSINFRAME` writer - AMP packets per frame"]
pub type AmppacketsinframeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - AMP packets line spacing"]
    #[inline(always)]
    pub fn amppacketlinespacing(&self) -> AmppacketlinespacingR {
        AmppacketlinespacingR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - AMP packets per frame"]
    #[inline(always)]
    pub fn amppacketsinframe(&self) -> AmppacketsinframeR {
        AmppacketsinframeR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - AMP packets line spacing"]
    #[inline(always)]
    #[must_use]
    pub fn amppacketlinespacing(&mut self) -> AmppacketlinespacingW<FcRdrb9Spec> {
        AmppacketlinespacingW::new(self, 0)
    }
    #[doc = "Bits 4:7 - AMP packets per frame"]
    #[inline(always)]
    #[must_use]
    pub fn amppacketsinframe(&mut self) -> AmppacketsinframeW<FcRdrb9Spec> {
        AmppacketsinframeW::new(self, 4)
    }
}
#[doc = "AMP packets line spacing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcRdrb9Spec;
impl crate::RegisterSpec for FcRdrb9Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_rdrb9::R`](R) reader structure"]
impl crate::Readable for FcRdrb9Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_rdrb9::W`](W) writer structure"]
impl crate::Writable for FcRdrb9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_RDRB9 to value 0"]
impl crate::Resettable for FcRdrb9Spec {
    const RESET_VALUE: u8 = 0;
}
