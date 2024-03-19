#[doc = "Register `SPI_TXFTLR` reader"]
pub type R = crate::R<SpiTxftlrSpec>;
#[doc = "Register `SPI_TXFTLR` writer"]
pub type W = crate::W<SpiTxftlrSpec>;
#[doc = "Field `TXFTLR` reader - Transmit FIFO Threshold Level\n\nWhen the number of transmit FIFO entries is less than or equal to\n\nthis value, the transmit FIFO empty interrupt is triggered."]
pub type TxftlrR = crate::FieldReader;
#[doc = "Field `TXFTLR` writer - Transmit FIFO Threshold Level\n\nWhen the number of transmit FIFO entries is less than or equal to\n\nthis value, the transmit FIFO empty interrupt is triggered."]
pub type TxftlrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Transmit FIFO Threshold Level\n\nWhen the number of transmit FIFO entries is less than or equal to\n\nthis value, the transmit FIFO empty interrupt is triggered."]
    #[inline(always)]
    pub fn txftlr(&self) -> TxftlrR {
        TxftlrR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Transmit FIFO Threshold Level\n\nWhen the number of transmit FIFO entries is less than or equal to\n\nthis value, the transmit FIFO empty interrupt is triggered."]
    #[inline(always)]
    #[must_use]
    pub fn txftlr(&mut self) -> TxftlrW<SpiTxftlrSpec> {
        TxftlrW::new(self, 0)
    }
}
#[doc = "Transmit FIFO Threshold Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_txftlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_txftlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiTxftlrSpec;
impl crate::RegisterSpec for SpiTxftlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_txftlr::R`](R) reader structure"]
impl crate::Readable for SpiTxftlrSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_txftlr::W`](W) writer structure"]
impl crate::Writable for SpiTxftlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_TXFTLR to value 0"]
impl crate::Resettable for SpiTxftlrSpec {
    const RESET_VALUE: u32 = 0;
}
