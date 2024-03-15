#[doc = "Register `SPI_TXDR` writer"]
pub type W = crate::W<SpiTxdrSpec>;
#[doc = "Field `TXDR` writer - Transimt FIFO Data Register. When it is written to, data are moved into the transmit FIFO."]
pub type TxdrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Transimt FIFO Data Register. When it is written to, data are moved into the transmit FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn txdr(&mut self) -> TxdrW<SpiTxdrSpec> {
        TxdrW::new(self, 0)
    }
}
#[doc = "Transmit FIFO Data\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_txdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiTxdrSpec;
impl crate::RegisterSpec for SpiTxdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`spi_txdr::W`](W) writer structure"]
impl crate::Writable for SpiTxdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_TXDR to value 0"]
impl crate::Resettable for SpiTxdrSpec {
    const RESET_VALUE: u32 = 0;
}
