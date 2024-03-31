#[doc = "Register `TXDR` writer"]
pub type W = crate::W<TxdrSpec>;
#[doc = "Field `TXDR` writer - Transmit FIFO Data Register\n\nWhen it is written to, data are moved into the transmit FIFO."]
pub type TxdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Transmit FIFO Data Register\n\nWhen it is written to, data are moved into the transmit FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn txdr(&mut self) -> TxdrW<TxdrSpec> {
        TxdrW::new(self, 0)
    }
}
#[doc = "Transmit FIFO Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdrSpec;
impl crate::RegisterSpec for TxdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`txdr::W`](W) writer structure"]
impl crate::Writable for TxdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDR to value 0"]
impl crate::Resettable for TxdrSpec {
    const RESET_VALUE: u32 = 0;
}
