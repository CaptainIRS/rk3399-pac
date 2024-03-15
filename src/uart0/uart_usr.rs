#[doc = "Register `UART_USR` reader"]
pub type R = crate::R<UartUsrSpec>;
#[doc = "Field `UART_BUSY` reader - UART Busy. UART Busy. This is indicates that a serial transfer is in progress, when cleared indicates that the UART is idle or inactive. 0 = UART is idle or inactive 1 = UART is busy (actively transferring data)"]
pub type UartBusyR = crate::BitReader;
#[doc = "Field `TRANS_FIFO_NOT_FULL` reader - Transmit FIFO Not Full. This is used to indicate that the transmit FIFO in not full. 0 = Transmit FIFO is full 1 = Transmit FIFO is not full This bit is cleared when the TX FIFO is full."]
pub type TransFifoNotFullR = crate::BitReader;
#[doc = "Field `TRASN_FIFO_EMPTY` reader - Transmit FIFO Empty. This is used to indicate that the transmit FIFO is completely empty. 0 = Transmit FIFO is not empty 1 = Transmit FIFO is empty This bit is cleared when the TX FIFO is no longer empty"]
pub type TrasnFifoEmptyR = crate::BitReader;
#[doc = "Field `RECEIVE_FIFO_NOT_EMPTY` reader - Receive FIFO Not Empty. This is used to indicate that the receive FIFO contains one or more entries. 0 = Receive FIFO is empty 1 = Receive FIFO is not empty This bit is cleared when the RX FIFO is empty."]
pub type ReceiveFifoNotEmptyR = crate::BitReader;
#[doc = "Field `RECEIVE_FIFO_FULL` reader - Receive FIFO Full. This is used to indicate that the receive FIFO is completely full. 0 = Receive FIFO not full 1 = Receive FIFO Full This bit is cleared when the RX FIFO is no longer full."]
pub type ReceiveFifoFullR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - UART Busy. UART Busy. This is indicates that a serial transfer is in progress, when cleared indicates that the UART is idle or inactive. 0 = UART is idle or inactive 1 = UART is busy (actively transferring data)"]
    #[inline(always)]
    pub fn uart_busy(&self) -> UartBusyR {
        UartBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Not Full. This is used to indicate that the transmit FIFO in not full. 0 = Transmit FIFO is full 1 = Transmit FIFO is not full This bit is cleared when the TX FIFO is full."]
    #[inline(always)]
    pub fn trans_fifo_not_full(&self) -> TransFifoNotFullR {
        TransFifoNotFullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO Empty. This is used to indicate that the transmit FIFO is completely empty. 0 = Transmit FIFO is not empty 1 = Transmit FIFO is empty This bit is cleared when the TX FIFO is no longer empty"]
    #[inline(always)]
    pub fn trasn_fifo_empty(&self) -> TrasnFifoEmptyR {
        TrasnFifoEmptyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Not Empty. This is used to indicate that the receive FIFO contains one or more entries. 0 = Receive FIFO is empty 1 = Receive FIFO is not empty This bit is cleared when the RX FIFO is empty."]
    #[inline(always)]
    pub fn receive_fifo_not_empty(&self) -> ReceiveFifoNotEmptyR {
        ReceiveFifoNotEmptyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Full. This is used to indicate that the receive FIFO is completely full. 0 = Receive FIFO not full 1 = Receive FIFO Full This bit is cleared when the RX FIFO is no longer full."]
    #[inline(always)]
    pub fn receive_fifo_full(&self) -> ReceiveFifoFullR {
        ReceiveFifoFullR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "UART Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_usr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartUsrSpec;
impl crate::RegisterSpec for UartUsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_usr::R`](R) reader structure"]
impl crate::Readable for UartUsrSpec {}
#[doc = "`reset()` method sets UART_USR to value 0"]
impl crate::Resettable for UartUsrSpec {
    const RESET_VALUE: u32 = 0;
}
