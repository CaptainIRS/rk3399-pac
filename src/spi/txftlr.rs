#[doc = "Register `TXFTLR` reader"]
pub type R = crate::R<TxftlrSpec>;
#[doc = "Register `TXFTLR` writer"]
pub type W = crate::W<TxftlrSpec>;
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
    pub fn txftlr(&mut self) -> TxftlrW<TxftlrSpec> {
        TxftlrW::new(self, 0)
    }
}
#[doc = "Transmit FIFO Threshold Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txftlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txftlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxftlrSpec;
impl crate::RegisterSpec for TxftlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txftlr::R`](R) reader structure"]
impl crate::Readable for TxftlrSpec {}
#[doc = "`write(|w| ..)` method takes [`txftlr::W`](W) writer structure"]
impl crate::Writable for TxftlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXFTLR to value 0"]
impl crate::Resettable for TxftlrSpec {
    const RESET_VALUE: u32 = 0;
}
