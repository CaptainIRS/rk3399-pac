#[doc = "Register `GEVNTSIZ0` reader"]
pub type R = crate::R<Gevntsiz0Spec>;
#[doc = "Register `GEVNTSIZ0` writer"]
pub type W = crate::W<Gevntsiz0Spec>;
#[doc = "Field `EVENTSIZ` reader - Event Buffer Size in bytes\n\nHolds the size of the Event Buffer in bytes; must be a multiple of\n\nfour. This is programmed by software once during initialization.\n\nThe minimum size of the event buffer is 32 bytes."]
pub type EventsizR = crate::FieldReader<u16>;
#[doc = "Field `EVENTSIZ` writer - Event Buffer Size in bytes\n\nHolds the size of the Event Buffer in bytes; must be a multiple of\n\nfour. This is programmed by software once during initialization.\n\nThe minimum size of the event buffer is 32 bytes."]
pub type EventsizW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EVNTINTRPTMASK` reader - Event Interrupt Mask\n\nWhen set to '1', this prevents the interrupt from being generated.\n\nHowever, even when the mask is set, the events are queued."]
pub type EvntintrptmaskR = crate::BitReader;
#[doc = "Field `EVNTINTRPTMASK` writer - Event Interrupt Mask\n\nWhen set to '1', this prevents the interrupt from being generated.\n\nHowever, even when the mask is set, the events are queued."]
pub type EvntintrptmaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Event Buffer Size in bytes\n\nHolds the size of the Event Buffer in bytes; must be a multiple of\n\nfour. This is programmed by software once during initialization.\n\nThe minimum size of the event buffer is 32 bytes."]
    #[inline(always)]
    pub fn eventsiz(&self) -> EventsizR {
        EventsizR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Event Interrupt Mask\n\nWhen set to '1', this prevents the interrupt from being generated.\n\nHowever, even when the mask is set, the events are queued."]
    #[inline(always)]
    pub fn evntintrptmask(&self) -> EvntintrptmaskR {
        EvntintrptmaskR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Event Buffer Size in bytes\n\nHolds the size of the Event Buffer in bytes; must be a multiple of\n\nfour. This is programmed by software once during initialization.\n\nThe minimum size of the event buffer is 32 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn eventsiz(&mut self) -> EventsizW<Gevntsiz0Spec> {
        EventsizW::new(self, 0)
    }
    #[doc = "Bit 31 - Event Interrupt Mask\n\nWhen set to '1', this prevents the interrupt from being generated.\n\nHowever, even when the mask is set, the events are queued."]
    #[inline(always)]
    #[must_use]
    pub fn evntintrptmask(&mut self) -> EvntintrptmaskW<Gevntsiz0Spec> {
        EvntintrptmaskW::new(self, 31)
    }
}
#[doc = "Global Event Buffer Size Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gevntsiz0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gevntsiz0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gevntsiz0Spec;
impl crate::RegisterSpec for Gevntsiz0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gevntsiz0::R`](R) reader structure"]
impl crate::Readable for Gevntsiz0Spec {}
#[doc = "`write(|w| ..)` method takes [`gevntsiz0::W`](W) writer structure"]
impl crate::Writable for Gevntsiz0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GEVNTSIZ0 to value 0"]
impl crate::Resettable for Gevntsiz0Spec {
    const RESET_VALUE: u32 = 0;
}
