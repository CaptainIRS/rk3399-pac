#[doc = "Register `UART_HTX` reader"]
pub type R = crate::R<UartHtxSpec>;
#[doc = "Register `UART_HTX` writer"]
pub type W = crate::W<UartHtxSpec>;
#[doc = "Field `HALT_TX_EN` reader - This register is use to halt transmissions for testing, so that the transmit FIFO can be filled by the master when FIFOs are implemented and enabled. 0 = Halt TX disabled 1 = Halt TX enabled"]
pub type HaltTxEnR = crate::BitReader;
#[doc = "Field `HALT_TX_EN` writer - This register is use to halt transmissions for testing, so that the transmit FIFO can be filled by the master when FIFOs are implemented and enabled. 0 = Halt TX disabled 1 = Halt TX enabled"]
pub type HaltTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This register is use to halt transmissions for testing, so that the transmit FIFO can be filled by the master when FIFOs are implemented and enabled. 0 = Halt TX disabled 1 = Halt TX enabled"]
    #[inline(always)]
    pub fn halt_tx_en(&self) -> HaltTxEnR {
        HaltTxEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This register is use to halt transmissions for testing, so that the transmit FIFO can be filled by the master when FIFOs are implemented and enabled. 0 = Halt TX disabled 1 = Halt TX enabled"]
    #[inline(always)]
    #[must_use]
    pub fn halt_tx_en(&mut self) -> HaltTxEnW<UartHtxSpec> {
        HaltTxEnW::new(self, 0)
    }
}
#[doc = "Halt TX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_htx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_htx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartHtxSpec;
impl crate::RegisterSpec for UartHtxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_htx::R`](R) reader structure"]
impl crate::Readable for UartHtxSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_htx::W`](W) writer structure"]
impl crate::Writable for UartHtxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_HTX to value 0"]
impl crate::Resettable for UartHtxSpec {
    const RESET_VALUE: u32 = 0;
}
