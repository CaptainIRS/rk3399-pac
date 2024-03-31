#[doc = "Register `TFR` reader"]
pub type R = crate::R<TfrSpec>;
#[doc = "Field `TRANS_FIFO_READ` reader - Transmit FIFO Read.\n\nThese bits are only valid when FIFO access mode is enabled\n\n(FAR\\[0\\]
is set to one).When FIFOs are implemented and enabled,\n\nreading this register gives the data at the top of the transmit\n\nFIFO. Each consecutive read pops the transmit FIFO and gives\n\nthe next data value that is currently at the top of the FIFO."]
pub type TransFifoReadR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Transmit FIFO Read.\n\nThese bits are only valid when FIFO access mode is enabled\n\n(FAR\\[0\\]
is set to one).When FIFOs are implemented and enabled,\n\nreading this register gives the data at the top of the transmit\n\nFIFO. Each consecutive read pops the transmit FIFO and gives\n\nthe next data value that is currently at the top of the FIFO."]
    #[inline(always)]
    pub fn trans_fifo_read(&self) -> TransFifoReadR {
        TransFifoReadR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Transmit FIFO Read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TfrSpec;
impl crate::RegisterSpec for TfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfr::R`](R) reader structure"]
impl crate::Readable for TfrSpec {}
#[doc = "`reset()` method sets TFR to value 0"]
impl crate::Resettable for TfrSpec {
    const RESET_VALUE: u32 = 0;
}
