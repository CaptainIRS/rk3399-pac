#[doc = "Register `LSR` reader"]
pub type R = crate::R<LsrSpec>;
#[doc = "Field `DATA_READY` reader - Data Ready bit.\n\nThis is used to indicate that the receiver contains at least one\n\ncharacter in the RBR or the receiver FIFO.\n\n0 = no data ready\n\n1 = data ready"]
pub type DataReadyR = crate::BitReader;
#[doc = "Field `OVERRUN_ERROR` reader - Overrun error bit.\n\nThis is used to indicate the occurrence of an overrun error. This\n\noccurs if a new data character was received before the previous\n\ndata was read."]
pub type OverrunErrorR = crate::BitReader;
#[doc = "Field `PARITY_EROR` reader - Parity Error bit.\n\nThis is used to indicate the occurrence of a parity error in the\n\nreceiver if the Parity Enable (PEN) bit (LCR\\[3\\]) is set."]
pub type ParityErorR = crate::BitReader;
#[doc = "Field `FRAMING_ERROR` reader - Framing Error bit.\n\nThis is used to indicate the occurrence of a framing error in the\n\nreceiver. A framing error occurs when the receiver does not\n\ndetect a valid STOP bit in the received data."]
pub type FramingErrorR = crate::BitReader;
#[doc = "Field `BREAK_INT` reader - Break Interrupt bit.\n\nThis is used to indicate the detection of a break sequence on the\n\nserial input data."]
pub type BreakIntR = crate::BitReader;
#[doc = "Field `TRANS_HOLD_REG_EMPTY` reader - Transmit Holding Register Empty bit.\n\nIf THRE mode is disabled (IER\\[7\\]
set to zero) and regardless of\n\nFIFO's being implemented/enabled or not, this bit indicates that\n\nthe THR or TX FIFO is empty.\n\nThis bit is set whenever data is transferred from the THR or TX\n\nFIFO to the transmitter shift register and no new data has been\n\nwritten to the THR or TX FIFO. This also causes a THRE Interrupt\n\nto occur, if the THRE Interrupt is enabled. If IER\\[7\\]
set to one\n\nand FCR\\[0\\]
set to one respectively, the functionality is switched\n\nto indicate the transmitter FIFO is full, and no longer controls\n\nTHRE interrupts, which are then controlled by the FCR\\[5:4\\]\n\nthreshold setting."]
pub type TransHoldRegEmptyR = crate::BitReader;
#[doc = "Field `TRANS_EMPTY` reader - Transmitter Empty bit.\n\nTransmitter Empty bit. If FIFOs enabled (FCR\\[0\\]
set to one), this\n\nbit is set whenever the Transmitter Shift Register and the FIFO\n\nare both empty. If FIFOs are disabled, this bit is set whenever the\n\nTransmitter Holding Register and the Transmitter Shift Register\n\nare both empty."]
pub type TransEmptyR = crate::BitReader;
#[doc = "Field `RECEIVER_FIFO_ERROR` reader - Receiver FIFO Error bit.\n\nThis bit is relevant FIFOs are enabled (FCR\\[0\\]
set to one). This is\n\nused to indicate if there is at least one parity error, framing error,\n\nor break indication in the FIFO.\n\n0 = no error in RX FIFO\n\n1 = error in RX FIFO"]
pub type ReceiverFifoErrorR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data Ready bit.\n\nThis is used to indicate that the receiver contains at least one\n\ncharacter in the RBR or the receiver FIFO.\n\n0 = no data ready\n\n1 = data ready"]
    #[inline(always)]
    pub fn data_ready(&self) -> DataReadyR {
        DataReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun error bit.\n\nThis is used to indicate the occurrence of an overrun error. This\n\noccurs if a new data character was received before the previous\n\ndata was read."]
    #[inline(always)]
    pub fn overrun_error(&self) -> OverrunErrorR {
        OverrunErrorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error bit.\n\nThis is used to indicate the occurrence of a parity error in the\n\nreceiver if the Parity Enable (PEN) bit (LCR\\[3\\]) is set."]
    #[inline(always)]
    pub fn parity_eror(&self) -> ParityErorR {
        ParityErorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Framing Error bit.\n\nThis is used to indicate the occurrence of a framing error in the\n\nreceiver. A framing error occurs when the receiver does not\n\ndetect a valid STOP bit in the received data."]
    #[inline(always)]
    pub fn framing_error(&self) -> FramingErrorR {
        FramingErrorR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Break Interrupt bit.\n\nThis is used to indicate the detection of a break sequence on the\n\nserial input data."]
    #[inline(always)]
    pub fn break_int(&self) -> BreakIntR {
        BreakIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Holding Register Empty bit.\n\nIf THRE mode is disabled (IER\\[7\\]
set to zero) and regardless of\n\nFIFO's being implemented/enabled or not, this bit indicates that\n\nthe THR or TX FIFO is empty.\n\nThis bit is set whenever data is transferred from the THR or TX\n\nFIFO to the transmitter shift register and no new data has been\n\nwritten to the THR or TX FIFO. This also causes a THRE Interrupt\n\nto occur, if the THRE Interrupt is enabled. If IER\\[7\\]
set to one\n\nand FCR\\[0\\]
set to one respectively, the functionality is switched\n\nto indicate the transmitter FIFO is full, and no longer controls\n\nTHRE interrupts, which are then controlled by the FCR\\[5:4\\]\n\nthreshold setting."]
    #[inline(always)]
    pub fn trans_hold_reg_empty(&self) -> TransHoldRegEmptyR {
        TransHoldRegEmptyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter Empty bit.\n\nTransmitter Empty bit. If FIFOs enabled (FCR\\[0\\]
set to one), this\n\nbit is set whenever the Transmitter Shift Register and the FIFO\n\nare both empty. If FIFOs are disabled, this bit is set whenever the\n\nTransmitter Holding Register and the Transmitter Shift Register\n\nare both empty."]
    #[inline(always)]
    pub fn trans_empty(&self) -> TransEmptyR {
        TransEmptyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receiver FIFO Error bit.\n\nThis bit is relevant FIFOs are enabled (FCR\\[0\\]
set to one). This is\n\nused to indicate if there is at least one parity error, framing error,\n\nor break indication in the FIFO.\n\n0 = no error in RX FIFO\n\n1 = error in RX FIFO"]
    #[inline(always)]
    pub fn receiver_fifo_error(&self) -> ReceiverFifoErrorR {
        ReceiverFifoErrorR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Line Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LsrSpec;
impl crate::RegisterSpec for LsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsr::R`](R) reader structure"]
impl crate::Readable for LsrSpec {}
#[doc = "`reset()` method sets LSR to value 0"]
impl crate::Resettable for LsrSpec {
    const RESET_VALUE: u32 = 0;
}
