#[doc = "Register `SPI_RXFLR` reader"]
pub type R = crate::R<SpiRxflrSpec>;
#[doc = "Field `RXFLR` reader - Receive FIFO Level\n\nContains the number of valid data entries in the receive FIFO."]
pub type RxflrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Receive FIFO Level\n\nContains the number of valid data entries in the receive FIFO."]
    #[inline(always)]
    pub fn rxflr(&self) -> RxflrR {
        RxflrR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Receive FIFO Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_rxflr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiRxflrSpec;
impl crate::RegisterSpec for SpiRxflrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_rxflr::R`](R) reader structure"]
impl crate::Readable for SpiRxflrSpec {}
#[doc = "`reset()` method sets SPI_RXFLR to value 0"]
impl crate::Resettable for SpiRxflrSpec {
    const RESET_VALUE: u32 = 0;
}
