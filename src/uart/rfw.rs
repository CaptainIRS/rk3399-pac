#[doc = "Register `RFW` writer"]
pub type W = crate::W<RfwSpec>;
#[doc = "Field `RECEIVE_FIFO_WRITE` writer - Receive FIFO Write Data.\n\nThese bits are only valid when FIFO access mode is enabled\n\n(FAR\\[0\\]
is set to one).\n\nWhen FIFOs are enabled, the data that is written to the RFWD is\n\npushed into the receive FIFO. Each consecutive write pushes the\n\nnew data to the next write location in the receive FIFO.\n\nWhen FIFOs not enabled, the data that is written to the RFWD is\n\npushed into the RBR."]
pub type ReceiveFifoWriteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RECEIVE_FIFO_PARITY_ERROR` writer - Receive FIFO Parity Error.\n\nThese bits are only valid when FIFO access mode is enabled\n\n(FAR\\[0\\]
is set to one)."]
pub type ReceiveFifoParityErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECEIVE_FIFO_FRAMING_ERROR` writer - Receive FIFO Framing Error.\n\nThese bits are only valid when FIFO access mode is enabled\n\n(FAR\\[0\\]
is set to one)."]
pub type ReceiveFifoFramingErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:7 - Receive FIFO Write Data.\n\nThese bits are only valid when FIFO access mode is enabled\n\n(FAR\\[0\\]
is set to one).\n\nWhen FIFOs are enabled, the data that is written to the RFWD is\n\npushed into the receive FIFO. Each consecutive write pushes the\n\nnew data to the next write location in the receive FIFO.\n\nWhen FIFOs not enabled, the data that is written to the RFWD is\n\npushed into the RBR."]
    #[inline(always)]
    #[must_use]
    pub fn receive_fifo_write(&mut self) -> ReceiveFifoWriteW<RfwSpec> {
        ReceiveFifoWriteW::new(self, 0)
    }
    #[doc = "Bit 8 - Receive FIFO Parity Error.\n\nThese bits are only valid when FIFO access mode is enabled\n\n(FAR\\[0\\]
is set to one)."]
    #[inline(always)]
    #[must_use]
    pub fn receive_fifo_parity_error(&mut self) -> ReceiveFifoParityErrorW<RfwSpec> {
        ReceiveFifoParityErrorW::new(self, 8)
    }
    #[doc = "Bit 9 - Receive FIFO Framing Error.\n\nThese bits are only valid when FIFO access mode is enabled\n\n(FAR\\[0\\]
is set to one)."]
    #[inline(always)]
    #[must_use]
    pub fn receive_fifo_framing_error(&mut self) -> ReceiveFifoFramingErrorW<RfwSpec> {
        ReceiveFifoFramingErrorW::new(self, 9)
    }
}
#[doc = "Receive FIFO Write\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfw::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfwSpec;
impl crate::RegisterSpec for RfwSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rfw::W`](W) writer structure"]
impl crate::Writable for RfwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFW to value 0"]
impl crate::Resettable for RfwSpec {
    const RESET_VALUE: u32 = 0;
}
