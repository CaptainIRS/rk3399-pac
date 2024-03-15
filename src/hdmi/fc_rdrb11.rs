#[doc = "Register `FC_RDRB11` reader"]
pub type R = crate::R<FcRdrb11Spec>;
#[doc = "Register `FC_RDRB11` writer"]
pub type W = crate::W<FcRdrb11Spec>;
#[doc = "Field `NVBIPACKETLINESPACING` reader - NTSC VBI packets line spacing"]
pub type NvbipacketlinespacingR = crate::FieldReader;
#[doc = "Field `NVBIPACKETLINESPACING` writer - NTSC VBI packets line spacing"]
pub type NvbipacketlinespacingW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NVBIPACKETSINFRAME` reader - NTSC VBI packets per frame"]
pub type NvbipacketsinframeR = crate::FieldReader;
#[doc = "Field `NVBIPACKETSINFRAME` writer - NTSC VBI packets per frame"]
pub type NvbipacketsinframeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - NTSC VBI packets line spacing"]
    #[inline(always)]
    pub fn nvbipacketlinespacing(&self) -> NvbipacketlinespacingR {
        NvbipacketlinespacingR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - NTSC VBI packets per frame"]
    #[inline(always)]
    pub fn nvbipacketsinframe(&self) -> NvbipacketsinframeR {
        NvbipacketsinframeR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - NTSC VBI packets line spacing"]
    #[inline(always)]
    #[must_use]
    pub fn nvbipacketlinespacing(&mut self) -> NvbipacketlinespacingW<FcRdrb11Spec> {
        NvbipacketlinespacingW::new(self, 0)
    }
    #[doc = "Bits 4:7 - NTSC VBI packets per frame"]
    #[inline(always)]
    #[must_use]
    pub fn nvbipacketsinframe(&mut self) -> NvbipacketsinframeW<FcRdrb11Spec> {
        NvbipacketsinframeW::new(self, 4)
    }
}
#[doc = "NTSC VBI packets line spacing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcRdrb11Spec;
impl crate::RegisterSpec for FcRdrb11Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_rdrb11::R`](R) reader structure"]
impl crate::Readable for FcRdrb11Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_rdrb11::W`](W) writer structure"]
impl crate::Writable for FcRdrb11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_RDRB11 to value 0"]
impl crate::Resettable for FcRdrb11Spec {
    const RESET_VALUE: u8 = 0;
}
