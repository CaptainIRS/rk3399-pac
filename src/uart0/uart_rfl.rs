#[doc = "Register `UART_RFL` reader"]
pub type R = crate::R<UartRflSpec>;
#[doc = "Field `RECEIVE_FIFO_LEVEL` reader - Receive FIFO Level. This is indicates the number of data entries in the receive FIFO."]
pub type ReceiveFifoLevelR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Receive FIFO Level. This is indicates the number of data entries in the receive FIFO."]
    #[inline(always)]
    pub fn receive_fifo_level(&self) -> ReceiveFifoLevelR {
        ReceiveFifoLevelR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Receive FIFO Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_rfl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartRflSpec;
impl crate::RegisterSpec for UartRflSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_rfl::R`](R) reader structure"]
impl crate::Readable for UartRflSpec {}
#[doc = "`reset()` method sets UART_RFL to value 0"]
impl crate::Resettable for UartRflSpec {
    const RESET_VALUE: u32 = 0;
}
