#[doc = "Register `DMAC_EVENT_RIS` reader"]
pub type R = crate::R<DmacEventRisSpec>;
#[doc = "Field `DMAC_EVENT_RIS_BITS_0` reader - Returns the status of the event-interrupt resources: Bit \\[N\\]
= 0 Event N is inactive or irq\\[N\\]
is LOW. Bit \\[N\\]
= 1 Event N is active or irq\\[N\\]
is HIGH."]
pub type DmacEventRisBits0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Returns the status of the event-interrupt resources: Bit \\[N\\]
= 0 Event N is inactive or irq\\[N\\]
is LOW. Bit \\[N\\]
= 1 Event N is active or irq\\[N\\]
is HIGH."]
    #[inline(always)]
    pub fn dmac_event_ris_bits_0(&self) -> DmacEventRisBits0R {
        DmacEventRisBits0R::new(self.bits)
    }
}
#[doc = "Event-Interrupt Raw Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_event_ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacEventRisSpec;
impl crate::RegisterSpec for DmacEventRisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_event_ris::R`](R) reader structure"]
impl crate::Readable for DmacEventRisSpec {}
#[doc = "`reset()` method sets DMAC_EVENT_RIS to value 0"]
impl crate::Resettable for DmacEventRisSpec {
    const RESET_VALUE: u32 = 0;
}
