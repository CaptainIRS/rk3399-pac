#[doc = "Register `GEVNTADRLO0` reader"]
pub type R = crate::R<Gevntadrlo0Spec>;
#[doc = "Register `GEVNTADRLO0` writer"]
pub type W = crate::W<Gevntadrlo0Spec>;
#[doc = "Field `EVNTADRLO` reader - Event Buffer Address\n\nHolds the lower 32 bits of start address of the external memory\n\nfor the Event Buffer. During operation, hardware does not update\n\nthis address."]
pub type EvntadrloR = crate::FieldReader<u32>;
#[doc = "Field `EVNTADRLO` writer - Event Buffer Address\n\nHolds the lower 32 bits of start address of the external memory\n\nfor the Event Buffer. During operation, hardware does not update\n\nthis address."]
pub type EvntadrloW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Event Buffer Address\n\nHolds the lower 32 bits of start address of the external memory\n\nfor the Event Buffer. During operation, hardware does not update\n\nthis address."]
    #[inline(always)]
    pub fn evntadrlo(&self) -> EvntadrloR {
        EvntadrloR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Event Buffer Address\n\nHolds the lower 32 bits of start address of the external memory\n\nfor the Event Buffer. During operation, hardware does not update\n\nthis address."]
    #[inline(always)]
    #[must_use]
    pub fn evntadrlo(&mut self) -> EvntadrloW<Gevntadrlo0Spec> {
        EvntadrloW::new(self, 0)
    }
}
#[doc = "Global Event Buffer Address (Low) Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gevntadrlo0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gevntadrlo0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gevntadrlo0Spec;
impl crate::RegisterSpec for Gevntadrlo0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gevntadrlo0::R`](R) reader structure"]
impl crate::Readable for Gevntadrlo0Spec {}
#[doc = "`write(|w| ..)` method takes [`gevntadrlo0::W`](W) writer structure"]
impl crate::Writable for Gevntadrlo0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GEVNTADRLO0 to value 0"]
impl crate::Resettable for Gevntadrlo0Spec {
    const RESET_VALUE: u32 = 0;
}
