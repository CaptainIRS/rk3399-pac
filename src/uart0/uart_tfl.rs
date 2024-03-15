#[doc = "Register `UART_TFL` reader"]
pub type R = crate::R<UartTflSpec>;
#[doc = "Register `UART_TFL` writer"]
pub type W = crate::W<UartTflSpec>;
#[doc = "Field `TRANS_FIFO_LEVEL` reader - Transmit FIFO Level. This is indicates the number of data entries in the transmit FIFO."]
pub type TransFifoLevelR = crate::FieldReader;
#[doc = "Field `TRANS_FIFO_LEVEL` writer - Transmit FIFO Level. This is indicates the number of data entries in the transmit FIFO."]
pub type TransFifoLevelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Transmit FIFO Level. This is indicates the number of data entries in the transmit FIFO."]
    #[inline(always)]
    pub fn trans_fifo_level(&self) -> TransFifoLevelR {
        TransFifoLevelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Transmit FIFO Level. This is indicates the number of data entries in the transmit FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn trans_fifo_level(&mut self) -> TransFifoLevelW<UartTflSpec> {
        TransFifoLevelW::new(self, 0)
    }
}
#[doc = "Transmit FIFO Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_tfl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_tfl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartTflSpec;
impl crate::RegisterSpec for UartTflSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_tfl::R`](R) reader structure"]
impl crate::Readable for UartTflSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_tfl::W`](W) writer structure"]
impl crate::Writable for UartTflSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_TFL to value 0"]
impl crate::Resettable for UartTflSpec {
    const RESET_VALUE: u32 = 0;
}
