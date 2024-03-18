#[doc = "Register `UART_FCR` writer"]
pub type W = crate::W<UartFcrSpec>;
#[doc = "Field `FIFO_EN` writer - FIFO Enable. FIFO Enable. This enables/disables the transmit (XMIT) and receive (RCVR) FIFOs. Whenever the value of this bit is changed both the XMIT and RCVR controller portion of FIFOs is reset."]
pub type FifoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCVR_FIFO_RESET` writer - RCVR FIFO Reset. This resets the control portion of the receive FIFO and treats the FIFO as empty. This also de-asserts the DMA RX request and single signals when additional DMA handshaking signals are selected. Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
pub type RcvrFifoResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XMIT_FIFO_RESET` writer - XMIT FIFO Reset. This resets the control portion of the transmit FIFO and treats the FIFO as empty. This also de-asserts the DMA TX request and single signals when additional DMA handshaking signals are selected . Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
pub type XmitFifoResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_MODE` writer - DMA Mode This determines the DMA signalling mode used for the dma_tx_req_n and dma_rx_req_n output signals when additional DMA handshaking signals are not selected . 0 = mode 0 1 = mode 11100 = character timeout."]
pub type DmaModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_EMPTY_TRIGGER` writer - TX Empty Trigger. This is used to select the empty threshold level at which the THRE Interrupts are generated when the mode is active. It also determines when the dma_tx_req_n signal is asserted when in certain modes of operation. The following trigger levels are supported: 00 = FIFO empty 01 = 2 characters in the FIFO 10 = FIFO 1/4 full 11 = FIFO 1/2 full"]
pub type TxEmptyTriggerW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RCVR_TRIGGER` writer - RCVR Trigger. This is used to select the trigger level in the receiver FIFO at which the Received Data Available Interrupt is generated. In auto flow control mode it is used to determine when the rts_n signal is de-asserted. It also determines when the dma_rx_req_n signal is asserted in certain modes of operation. The following trigger levels are supported: 00 = 1 character in the FIFO 01 = FIFO 1/4 full 10 = FIFO 1/2 full 11 = FIFO 2 less than ful"]
pub type RcvrTriggerW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl W {
    #[doc = "Bit 0 - FIFO Enable. FIFO Enable. This enables/disables the transmit (XMIT) and receive (RCVR) FIFOs. Whenever the value of this bit is changed both the XMIT and RCVR controller portion of FIFOs is reset."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_en(&mut self) -> FifoEnW<UartFcrSpec> {
        FifoEnW::new(self, 0)
    }
    #[doc = "Bit 1 - RCVR FIFO Reset. This resets the control portion of the receive FIFO and treats the FIFO as empty. This also de-asserts the DMA RX request and single signals when additional DMA handshaking signals are selected. Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn rcvr_fifo_reset(&mut self) -> RcvrFifoResetW<UartFcrSpec> {
        RcvrFifoResetW::new(self, 1)
    }
    #[doc = "Bit 2 - XMIT FIFO Reset. This resets the control portion of the transmit FIFO and treats the FIFO as empty. This also de-asserts the DMA TX request and single signals when additional DMA handshaking signals are selected . Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn xmit_fifo_reset(&mut self) -> XmitFifoResetW<UartFcrSpec> {
        XmitFifoResetW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Mode This determines the DMA signalling mode used for the dma_tx_req_n and dma_rx_req_n output signals when additional DMA handshaking signals are not selected . 0 = mode 0 1 = mode 11100 = character timeout."]
    #[inline(always)]
    #[must_use]
    pub fn dma_mode(&mut self) -> DmaModeW<UartFcrSpec> {
        DmaModeW::new(self, 3)
    }
    #[doc = "Bits 4:5 - TX Empty Trigger. This is used to select the empty threshold level at which the THRE Interrupts are generated when the mode is active. It also determines when the dma_tx_req_n signal is asserted when in certain modes of operation. The following trigger levels are supported: 00 = FIFO empty 01 = 2 characters in the FIFO 10 = FIFO 1/4 full 11 = FIFO 1/2 full"]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty_trigger(&mut self) -> TxEmptyTriggerW<UartFcrSpec> {
        TxEmptyTriggerW::new(self, 4)
    }
    #[doc = "Bits 6:7 - RCVR Trigger. This is used to select the trigger level in the receiver FIFO at which the Received Data Available Interrupt is generated. In auto flow control mode it is used to determine when the rts_n signal is de-asserted. It also determines when the dma_rx_req_n signal is asserted in certain modes of operation. The following trigger levels are supported: 00 = 1 character in the FIFO 01 = FIFO 1/4 full 10 = FIFO 1/2 full 11 = FIFO 2 less than ful"]
    #[inline(always)]
    #[must_use]
    pub fn rcvr_trigger(&mut self) -> RcvrTriggerW<UartFcrSpec> {
        RcvrTriggerW::new(self, 6)
    }
}
#[doc = "FIFO Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartFcrSpec;
impl crate::RegisterSpec for UartFcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uart_fcr::W`](W) writer structure"]
impl crate::Writable for UartFcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_FCR to value 0"]
impl crate::Resettable for UartFcrSpec {
    const RESET_VALUE: u32 = 0;
}
