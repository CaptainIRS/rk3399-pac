#[doc = "Register `USB3_GEVNTADRHI0` reader"]
pub type R = crate::R<Usb3Gevntadrhi0Spec>;
#[doc = "Register `USB3_GEVNTADRHI0` writer"]
pub type W = crate::W<Usb3Gevntadrhi0Spec>;
#[doc = "Field `EVNTADRHI` reader - Event Buffer Address\n\nHolds the higher 32 bits of start address of the external memory\n\nfor the Event Buffer. During operation, hardware does not update\n\nthis address."]
pub type EvntadrhiR = crate::FieldReader<u32>;
#[doc = "Field `EVNTADRHI` writer - Event Buffer Address\n\nHolds the higher 32 bits of start address of the external memory\n\nfor the Event Buffer. During operation, hardware does not update\n\nthis address."]
pub type EvntadrhiW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Event Buffer Address\n\nHolds the higher 32 bits of start address of the external memory\n\nfor the Event Buffer. During operation, hardware does not update\n\nthis address."]
    #[inline(always)]
    pub fn evntadrhi(&self) -> EvntadrhiR {
        EvntadrhiR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Event Buffer Address\n\nHolds the higher 32 bits of start address of the external memory\n\nfor the Event Buffer. During operation, hardware does not update\n\nthis address."]
    #[inline(always)]
    #[must_use]
    pub fn evntadrhi(&mut self) -> EvntadrhiW<Usb3Gevntadrhi0Spec> {
        EvntadrhiW::new(self, 0)
    }
}
#[doc = "Global Event Buffer Address (High) Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gevntadrhi0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gevntadrhi0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3Gevntadrhi0Spec;
impl crate::RegisterSpec for Usb3Gevntadrhi0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_gevntadrhi0::R`](R) reader structure"]
impl crate::Readable for Usb3Gevntadrhi0Spec {}
#[doc = "`write(|w| ..)` method takes [`usb3_gevntadrhi0::W`](W) writer structure"]
impl crate::Writable for Usb3Gevntadrhi0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_GEVNTADRHI0 to value 0"]
impl crate::Resettable for Usb3Gevntadrhi0Spec {
    const RESET_VALUE: u32 = 0;
}
