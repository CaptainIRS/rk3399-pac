#[doc = "Register `GEVNTCOUNT0` reader"]
pub type R = crate::R<Gevntcount0Spec>;
#[doc = "Register `GEVNTCOUNT0` writer"]
pub type W = crate::W<Gevntcount0Spec>;
#[doc = "Field `EVNTCOUNT` reader - Event Count\n\nWhen read, returns the number of valid events in the Event\n\nBuffer (in bytes).\n\nWhen written, hardware decrements the count by the value\n\nwritten. The interrupt line remains high when count is not 0."]
pub type EvntcountR = crate::FieldReader<u16>;
#[doc = "Field `EVNTCOUNT` writer - Event Count\n\nWhen read, returns the number of valid events in the Event\n\nBuffer (in bytes).\n\nWhen written, hardware decrements the count by the value\n\nwritten. The interrupt line remains high when count is not 0."]
pub type EvntcountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Event Count\n\nWhen read, returns the number of valid events in the Event\n\nBuffer (in bytes).\n\nWhen written, hardware decrements the count by the value\n\nwritten. The interrupt line remains high when count is not 0."]
    #[inline(always)]
    pub fn evntcount(&self) -> EvntcountR {
        EvntcountR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Event Count\n\nWhen read, returns the number of valid events in the Event\n\nBuffer (in bytes).\n\nWhen written, hardware decrements the count by the value\n\nwritten. The interrupt line remains high when count is not 0."]
    #[inline(always)]
    #[must_use]
    pub fn evntcount(&mut self) -> EvntcountW<Gevntcount0Spec> {
        EvntcountW::new(self, 0)
    }
}
#[doc = "Global Event Buffer Count Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gevntcount0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gevntcount0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gevntcount0Spec;
impl crate::RegisterSpec for Gevntcount0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gevntcount0::R`](R) reader structure"]
impl crate::Readable for Gevntcount0Spec {}
#[doc = "`write(|w| ..)` method takes [`gevntcount0::W`](W) writer structure"]
impl crate::Writable for Gevntcount0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GEVNTCOUNT0 to value 0"]
impl crate::Resettable for Gevntcount0Spec {
    const RESET_VALUE: u32 = 0;
}
