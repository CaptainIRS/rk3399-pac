#[doc = "Register `FC_RDRB13` reader"]
pub type R = crate::R<FcRdrb13Spec>;
#[doc = "Register `FC_RDRB13` writer"]
pub type W = crate::W<FcRdrb13Spec>;
#[doc = "Field `DRMPACKETLINESPACING` reader - DRM packets line spacing"]
pub type DrmpacketlinespacingR = crate::FieldReader;
#[doc = "Field `DRMPACKETLINESPACING` writer - DRM packets line spacing"]
pub type DrmpacketlinespacingW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DRMPACKETSINFRAME` reader - DRM packets per frame"]
pub type DrmpacketsinframeR = crate::FieldReader;
#[doc = "Field `DRMPACKETSINFRAME` writer - DRM packets per frame"]
pub type DrmpacketsinframeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - DRM packets line spacing"]
    #[inline(always)]
    pub fn drmpacketlinespacing(&self) -> DrmpacketlinespacingR {
        DrmpacketlinespacingR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - DRM packets per frame"]
    #[inline(always)]
    pub fn drmpacketsinframe(&self) -> DrmpacketsinframeR {
        DrmpacketsinframeR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - DRM packets line spacing"]
    #[inline(always)]
    #[must_use]
    pub fn drmpacketlinespacing(&mut self) -> DrmpacketlinespacingW<FcRdrb13Spec> {
        DrmpacketlinespacingW::new(self, 0)
    }
    #[doc = "Bits 4:7 - DRM packets per frame"]
    #[inline(always)]
    #[must_use]
    pub fn drmpacketsinframe(&mut self) -> DrmpacketsinframeW<FcRdrb13Spec> {
        DrmpacketsinframeW::new(self, 4)
    }
}
#[doc = "Frame Composer Round Robin DRM Packet Insertion Register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_rdrb13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_rdrb13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcRdrb13Spec;
impl crate::RegisterSpec for FcRdrb13Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_rdrb13::R`](R) reader structure"]
impl crate::Readable for FcRdrb13Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_rdrb13::W`](W) writer structure"]
impl crate::Writable for FcRdrb13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_RDRB13 to value 0"]
impl crate::Resettable for FcRdrb13Spec {
    const RESET_VALUE: u8 = 0;
}
