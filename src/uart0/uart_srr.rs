#[doc = "Register `UART_SRR` writer"]
pub type W = crate::W<UartSrrSpec>;
#[doc = "Field `UART_RESET` writer - UART Reset. This asynchronously resets the UART and synchronously removes the reset assertion. For a two clock implementation both pclk and sclk domains are reset."]
pub type UartResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCVR_FIFO_RESET` writer - RCVR FIFO Reset. This is a shadow register for the RCVR FIFO Reset bit (FCR\\[1\\])."]
pub type RcvrFifoResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XMIT_FIFO_RESET` writer - XMIT FIFO Reset. This is a shadow register for the XMIT FIFO Reset bit (FCR\\[2\\])."]
pub type XmitFifoResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - UART Reset. This asynchronously resets the UART and synchronously removes the reset assertion. For a two clock implementation both pclk and sclk domains are reset."]
    #[inline(always)]
    #[must_use]
    pub fn uart_reset(&mut self) -> UartResetW<UartSrrSpec> {
        UartResetW::new(self, 0)
    }
    #[doc = "Bit 1 - RCVR FIFO Reset. This is a shadow register for the RCVR FIFO Reset bit (FCR\\[1\\])."]
    #[inline(always)]
    #[must_use]
    pub fn rcvr_fifo_reset(&mut self) -> RcvrFifoResetW<UartSrrSpec> {
        RcvrFifoResetW::new(self, 1)
    }
    #[doc = "Bit 2 - XMIT FIFO Reset. This is a shadow register for the XMIT FIFO Reset bit (FCR\\[2\\])."]
    #[inline(always)]
    #[must_use]
    pub fn xmit_fifo_reset(&mut self) -> XmitFifoResetW<UartSrrSpec> {
        XmitFifoResetW::new(self, 2)
    }
}
#[doc = "Software Reset Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_srr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartSrrSpec;
impl crate::RegisterSpec for UartSrrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uart_srr::W`](W) writer structure"]
impl crate::Writable for UartSrrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_SRR to value 0"]
impl crate::Resettable for UartSrrSpec {
    const RESET_VALUE: u32 = 0;
}
