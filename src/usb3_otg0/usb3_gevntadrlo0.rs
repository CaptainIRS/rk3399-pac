#[doc = "Register `USB3_GEVNTADRLO0` reader"]
pub type R = crate::R<Usb3Gevntadrlo0Spec>;
#[doc = "Register `USB3_GEVNTADRLO0` writer"]
pub type W = crate::W<Usb3Gevntadrlo0Spec>;
#[doc = "Field `EVNTADRLO` reader - Event Buffer Address Holds the lower 32 bits of start address of the external memory for the Event Buffer. During operation, hardware does not update this address."]
pub type EvntadrloR = crate::FieldReader<u32>;
#[doc = "Field `EVNTADRLO` writer - Event Buffer Address Holds the lower 32 bits of start address of the external memory for the Event Buffer. During operation, hardware does not update this address."]
pub type EvntadrloW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Event Buffer Address Holds the lower 32 bits of start address of the external memory for the Event Buffer. During operation, hardware does not update this address."]
    #[inline(always)]
    pub fn evntadrlo(&self) -> EvntadrloR {
        EvntadrloR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Event Buffer Address Holds the lower 32 bits of start address of the external memory for the Event Buffer. During operation, hardware does not update this address."]
    #[inline(always)]
    #[must_use]
    pub fn evntadrlo(&mut self) -> EvntadrloW<Usb3Gevntadrlo0Spec> {
        EvntadrloW::new(self, 0)
    }
}
#[doc = "Global Event Buffer Address (Low) Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gevntadrlo0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gevntadrlo0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3Gevntadrlo0Spec;
impl crate::RegisterSpec for Usb3Gevntadrlo0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_gevntadrlo0::R`](R) reader structure"]
impl crate::Readable for Usb3Gevntadrlo0Spec {}
#[doc = "`write(|w| ..)` method takes [`usb3_gevntadrlo0::W`](W) writer structure"]
impl crate::Writable for Usb3Gevntadrlo0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_GEVNTADRLO0 to value 0"]
impl crate::Resettable for Usb3Gevntadrlo0Spec {
    const RESET_VALUE: u32 = 0;
}
