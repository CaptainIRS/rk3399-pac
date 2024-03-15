#[doc = "Register `FC_RDRB5` reader"]
pub type R = crate::R<FcRdrb5Spec>;
#[doc = "Register `FC_RDRB5` writer"]
pub type W = crate::W<FcRdrb5Spec>;
#[doc = "Field `GCPPACKETLINESPACING` reader - GCP packets line spacing"]
pub type GcppacketlinespacingR = crate::FieldReader;
#[doc = "Field `GCPPACKETLINESPACING` writer - GCP packets line spacing"]
pub type GcppacketlinespacingW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GCPPACKETSINFRAME` reader - GCP packets per frame"]
pub type GcppacketsinframeR = crate::FieldReader;
#[doc = "Field `GCPPACKETSINFRAME` writer - GCP packets per frame"]
pub type GcppacketsinframeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - GCP packets line spacing"]
    #[inline(always)]
    pub fn gcppacketlinespacing(&self) -> GcppacketlinespacingR {
        GcppacketlinespacingR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - GCP packets per frame"]
    #[inline(always)]
    pub fn gcppacketsinframe(&self) -> GcppacketsinframeR {
        GcppacketsinframeR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - GCP packets line spacing"]
    #[inline(always)]
    #[must_use]
    pub fn gcppacketlinespacing(&mut self) -> GcppacketlinespacingW<FcRdrb5Spec> {
        GcppacketlinespacingW::new(self, 0)
    }
    #[doc = "Bits 4:7 - GCP packets per frame"]
    #[inline(always)]
    #[must_use]
    pub fn gcppacketsinframe(&mut self) -> GcppacketsinframeW<FcRdrb5Spec> {
        GcppacketsinframeW::new(self, 4)
    }
}
#[doc = "GCP packets line spacing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcRdrb5Spec;
impl crate::RegisterSpec for FcRdrb5Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_rdrb5::R`](R) reader structure"]
impl crate::Readable for FcRdrb5Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_rdrb5::W`](W) writer structure"]
impl crate::Writable for FcRdrb5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_RDRB5 to value 0"]
impl crate::Resettable for FcRdrb5Spec {
    const RESET_VALUE: u8 = 0;
}
