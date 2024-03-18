#[doc = "Register `UART_TFR` reader"]
pub type R = crate::R<UartTfrSpec>;
#[doc = "Field `TRANS_FIFO_READ` reader - Transmit FIFO Read. These bits are only valid when FIFO access mode is enabled (FAR\\[0\\]
is set to one).When FIFOs are implemented and enabled, reading this register gives the data at the top of the transmit FIFO. Each consecutive read pops the transmit FIFO and gives the next data value that is currently at the top of the FIFO."]
pub type TransFifoReadR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Transmit FIFO Read. These bits are only valid when FIFO access mode is enabled (FAR\\[0\\]
is set to one).When FIFOs are implemented and enabled, reading this register gives the data at the top of the transmit FIFO. Each consecutive read pops the transmit FIFO and gives the next data value that is currently at the top of the FIFO."]
    #[inline(always)]
    pub fn trans_fifo_read(&self) -> TransFifoReadR {
        TransFifoReadR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Transmit FIFO Read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_tfr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartTfrSpec;
impl crate::RegisterSpec for UartTfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_tfr::R`](R) reader structure"]
impl crate::Readable for UartTfrSpec {}
#[doc = "`reset()` method sets UART_TFR to value 0"]
impl crate::Resettable for UartTfrSpec {
    const RESET_VALUE: u32 = 0;
}
