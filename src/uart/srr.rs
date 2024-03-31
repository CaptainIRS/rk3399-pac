#[doc = "Register `SRR` writer"]
pub type W = crate::W<SrrSpec>;
#[doc = "Field `UART_RESET` writer - UART Reset.\n\nThis asynchronously resets the UART and synchronously removes\n\nthe reset assertion. For a two clock implementation both pclk and\n\nsclk domains are reset."]
pub type UartResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCVR_FIFO_RESET` writer - RCVR FIFO Reset.\n\nThis is a shadow register for the RCVR FIFO Reset bit (FCR\\[1\\])."]
pub type RcvrFifoResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XMIT_FIFO_RESET` writer - XMIT FIFO Reset.\n\nThis is a shadow register for the XMIT FIFO Reset bit (FCR\\[2\\])."]
pub type XmitFifoResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - UART Reset.\n\nThis asynchronously resets the UART and synchronously removes\n\nthe reset assertion. For a two clock implementation both pclk and\n\nsclk domains are reset."]
    #[inline(always)]
    #[must_use]
    pub fn uart_reset(&mut self) -> UartResetW<SrrSpec> {
        UartResetW::new(self, 0)
    }
    #[doc = "Bit 1 - RCVR FIFO Reset.\n\nThis is a shadow register for the RCVR FIFO Reset bit (FCR\\[1\\])."]
    #[inline(always)]
    #[must_use]
    pub fn rcvr_fifo_reset(&mut self) -> RcvrFifoResetW<SrrSpec> {
        RcvrFifoResetW::new(self, 1)
    }
    #[doc = "Bit 2 - XMIT FIFO Reset.\n\nThis is a shadow register for the XMIT FIFO Reset bit (FCR\\[2\\])."]
    #[inline(always)]
    #[must_use]
    pub fn xmit_fifo_reset(&mut self) -> XmitFifoResetW<SrrSpec> {
        XmitFifoResetW::new(self, 2)
    }
}
#[doc = "Software Reset Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrrSpec;
impl crate::RegisterSpec for SrrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`srr::W`](W) writer structure"]
impl crate::Writable for SrrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRR to value 0"]
impl crate::Resettable for SrrSpec {
    const RESET_VALUE: u32 = 0;
}
