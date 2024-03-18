#[doc = "Register `I2S_TXDR` writer"]
pub type W = crate::W<I2sTxdrSpec>;
#[doc = "Field `TXDR` writer - Transmit FIFO Data Register When it is written to, data are moved into the transmit FIFO."]
pub type TxdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Transmit FIFO Data Register When it is written to, data are moved into the transmit FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn txdr(&mut self) -> TxdrW<I2sTxdrSpec> {
        TxdrW::new(self, 0)
    }
}
#[doc = "Transmit FIFO Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_txdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sTxdrSpec;
impl crate::RegisterSpec for I2sTxdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`i2s_txdr::W`](W) writer structure"]
impl crate::Writable for I2sTxdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2S_TXDR to value 0"]
impl crate::Resettable for I2sTxdrSpec {
    const RESET_VALUE: u32 = 0;
}
