#[doc = "Register `EVENT_RIS` reader"]
pub type R = crate::R<EventRisSpec>;
#[doc = "Field `EVENT_RIS_BITS_0` reader - Returns the status of the event-interrupt resources:\n\nBit \\[N\\]
= 0 Event N is inactive or irq\\[N\\]
is LOW.\n\nBit \\[N\\]
= 1 Event N is active or irq\\[N\\]
is HIGH."]
pub type EventRisBits0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Returns the status of the event-interrupt resources:\n\nBit \\[N\\]
= 0 Event N is inactive or irq\\[N\\]
is LOW.\n\nBit \\[N\\]
= 1 Event N is active or irq\\[N\\]
is HIGH."]
    #[inline(always)]
    pub fn event_ris_bits_0(&self) -> EventRisBits0R {
        EventRisBits0R::new(self.bits)
    }
}
#[doc = "Event-Interrupt Raw Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`event_ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventRisSpec;
impl crate::RegisterSpec for EventRisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`event_ris::R`](R) reader structure"]
impl crate::Readable for EventRisSpec {}
#[doc = "`reset()` method sets EVENT_RIS to value 0"]
impl crate::Resettable for EventRisSpec {
    const RESET_VALUE: u32 = 0;
}
