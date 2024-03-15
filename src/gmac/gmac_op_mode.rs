#[doc = "Register `GMAC_OP_MODE` reader"]
pub type R = crate::R<GmacOpModeSpec>;
#[doc = "Register `GMAC_OP_MODE` writer"]
pub type W = crate::W<GmacOpModeSpec>;
#[doc = "Field `SR` reader - Start/Stop Receive When this bit is set, the Receive process is placed in the Running state. The DMA attempts to acquire the descriptor from the Receive list and processes incoming frames. Descriptor acquisition is attempted from the current position in the list, which is the address set by register GMAC_RX_DESC_LIST_ADDR or the position retained when the Receive process was previously stopped. If no descriptor is owned by the DMA, reception is suspended and Receive Buffer Unavailable (Register GMAC_STATUS\\[7\\]) is set. The Start Receive command is effective only when reception has stopped. If the command was issued before setting register GMAC_RX_DESC_LIST_ADDR, DMA behavior is unpredictable. When this bit is cleared, RxDMA operation is stopped after the transfer of the current frame. The next descriptor position in the Receive list is saved and becomes the current position after the Receive process is restarted. The Stop Receive command is effective only when the Receive process is in either the Running (waiting for receive packet) or in the Suspended state."]
pub type SrR = crate::BitReader;
#[doc = "Field `SR` writer - Start/Stop Receive When this bit is set, the Receive process is placed in the Running state. The DMA attempts to acquire the descriptor from the Receive list and processes incoming frames. Descriptor acquisition is attempted from the current position in the list, which is the address set by register GMAC_RX_DESC_LIST_ADDR or the position retained when the Receive process was previously stopped. If no descriptor is owned by the DMA, reception is suspended and Receive Buffer Unavailable (Register GMAC_STATUS\\[7\\]) is set. The Start Receive command is effective only when reception has stopped. If the command was issued before setting register GMAC_RX_DESC_LIST_ADDR, DMA behavior is unpredictable. When this bit is cleared, RxDMA operation is stopped after the transfer of the current frame. The next descriptor position in the Receive list is saved and becomes the current position after the Receive process is restarted. The Stop Receive command is effective only when the Receive process is in either the Running (waiting for receive packet) or in the Suspended state."]
pub type SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSF` reader - Operate on Second Frame When this bit is set, this bit instructs the DMA to process a second frame of Transmit data even before status for first frame is obtained."]
pub type OsfR = crate::BitReader;
#[doc = "Field `OSF` writer - Operate on Second Frame When this bit is set, this bit instructs the DMA to process a second frame of Transmit data even before status for first frame is obtained."]
pub type OsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Receive Threshold Control These two bits control the threshold level of the MTL Receive FIFO. Transfer (request) to DMA starts when the frame size within the MTL Receive FIFO is larger than the threshold. In addition, full frames with a length less than the threshold are transferred automatically. Note that value of 11 is not applicable if the configured Receive FIFO size is 128 bytes. These bits are valid only when the RSF bit is zero, and are ignored when the RSF bit is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtc {
    #[doc = "0: 128"]
    B00 = 0,
    #[doc = "1: 128"]
    B01 = 1,
    #[doc = "2: 128"]
    B10 = 2,
    #[doc = "3: 128"]
    B11 = 3,
}
impl From<Rtc> for u8 {
    #[inline(always)]
    fn from(variant: Rtc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtc {
    type Ux = u8;
}
#[doc = "Field `RTC` reader - Receive Threshold Control These two bits control the threshold level of the MTL Receive FIFO. Transfer (request) to DMA starts when the frame size within the MTL Receive FIFO is larger than the threshold. In addition, full frames with a length less than the threshold are transferred automatically. Note that value of 11 is not applicable if the configured Receive FIFO size is 128 bytes. These bits are valid only when the RSF bit is zero, and are ignored when the RSF bit is set to 1."]
pub type RtcR = crate::FieldReader<Rtc>;
impl RtcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtc {
        match self.bits {
            0 => Rtc::B00,
            1 => Rtc::B01,
            2 => Rtc::B10,
            3 => Rtc::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Rtc::B00
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Rtc::B01
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Rtc::B10
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Rtc::B11
    }
}
#[doc = "Field `RTC` writer - Receive Threshold Control These two bits control the threshold level of the MTL Receive FIFO. Transfer (request) to DMA starts when the frame size within the MTL Receive FIFO is larger than the threshold. In addition, full frames with a length less than the threshold are transferred automatically. Note that value of 11 is not applicable if the configured Receive FIFO size is 128 bytes. These bits are valid only when the RSF bit is zero, and are ignored when the RSF bit is set to 1."]
pub type RtcW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Rtc>;
impl<'a, REG> RtcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::B00)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::B01)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::B10)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::B11)
    }
}
#[doc = "Field `FUF` reader - Forward Undersized Good Frames When set, the Rx FIFO will forward Undersized frames (frames with no Error and length less than 64 bytes) including pad-bytes and CRC). When reset, the Rx FIFO will drop all frames of less than 64 bytes, unless it is already transferred due to lower value of Receive Threshold (e.g., RTC = 01)."]
pub type FufR = crate::BitReader;
#[doc = "Field `FUF` writer - Forward Undersized Good Frames When set, the Rx FIFO will forward Undersized frames (frames with no Error and length less than 64 bytes) including pad-bytes and CRC). When reset, the Rx FIFO will drop all frames of less than 64 bytes, unless it is already transferred due to lower value of Receive Threshold (e.g., RTC = 01)."]
pub type FufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEF` reader - Forward Error Frames When this bit is reset, the Rx FIFO drops frames with error status (CRC error, collision error, GMII_ER, giant frame, watchdog timeout, overflow). However, if the frame's start byte (write) pointer is already transferred to the read controller side (in Threshold mode), then the frames are not dropped. When FEF is set, all frames except runt error frames are forwarded to the DMA. But when RxFIFO overflows when a partial frame is written, then such frames are dropped even when FEF is set."]
pub type FefR = crate::BitReader;
#[doc = "Field `FEF` writer - Forward Error Frames When this bit is reset, the Rx FIFO drops frames with error status (CRC error, collision error, GMII_ER, giant frame, watchdog timeout, overflow). However, if the frame's start byte (write) pointer is already transferred to the read controller side (in Threshold mode), then the frames are not dropped. When FEF is set, all frames except runt error frames are forwarded to the DMA. But when RxFIFO overflows when a partial frame is written, then such frames are dropped even when FEF is set."]
pub type FefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFC` reader - Enable HW flow control When this bit is set, the flow control signal operation based on fill-level of Rx FIFO is enabled. When reset, the flow control operation is disabled."]
pub type EfcR = crate::BitReader;
#[doc = "Field `EFC` writer - Enable HW flow control When this bit is set, the flow control signal operation based on fill-level of Rx FIFO is enabled. When reset, the flow control operation is disabled."]
pub type EfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Threshold for activating flow control (in both HD and FD) These bits control the threshold (Fill level of Rx FIFO) at which flow control is activated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rfa {
    #[doc = "0: Full minus 4 KB Note that the above only applies to Rx FIFOs of 4 KB or more when the EFC bit is set high."]
    B00 = 0,
    #[doc = "1: Full minus 4 KB Note that the above only applies to Rx FIFOs of 4 KB or more when the EFC bit is set high."]
    B01 = 1,
    #[doc = "2: Full minus 4 KB Note that the above only applies to Rx FIFOs of 4 KB or more when the EFC bit is set high."]
    B10 = 2,
    #[doc = "3: Full minus 4 KB Note that the above only applies to Rx FIFOs of 4 KB or more when the EFC bit is set high."]
    B11 = 3,
}
impl From<Rfa> for u8 {
    #[inline(always)]
    fn from(variant: Rfa) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rfa {
    type Ux = u8;
}
#[doc = "Field `RFA` reader - Threshold for activating flow control (in both HD and FD) These bits control the threshold (Fill level of Rx FIFO) at which flow control is activated."]
pub type RfaR = crate::FieldReader<Rfa>;
impl RfaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfa {
        match self.bits {
            0 => Rfa::B00,
            1 => Rfa::B01,
            2 => Rfa::B10,
            3 => Rfa::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Full minus 4 KB Note that the above only applies to Rx FIFOs of 4 KB or more when the EFC bit is set high."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Rfa::B00
    }
    #[doc = "Full minus 4 KB Note that the above only applies to Rx FIFOs of 4 KB or more when the EFC bit is set high."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Rfa::B01
    }
    #[doc = "Full minus 4 KB Note that the above only applies to Rx FIFOs of 4 KB or more when the EFC bit is set high."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Rfa::B10
    }
    #[doc = "Full minus 4 KB Note that the above only applies to Rx FIFOs of 4 KB or more when the EFC bit is set high."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Rfa::B11
    }
}
#[doc = "Field `RFA` writer - Threshold for activating flow control (in both HD and FD) These bits control the threshold (Fill level of Rx FIFO) at which flow control is activated."]
pub type RfaW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Rfa>;
impl<'a, REG> RfaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Full minus 4 KB Note that the above only applies to Rx FIFOs of 4 KB or more when the EFC bit is set high."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Rfa::B00)
    }
    #[doc = "Full minus 4 KB Note that the above only applies to Rx FIFOs of 4 KB or more when the EFC bit is set high."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Rfa::B01)
    }
    #[doc = "Full minus 4 KB Note that the above only applies to Rx FIFOs of 4 KB or more when the EFC bit is set high."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Rfa::B10)
    }
    #[doc = "Full minus 4 KB Note that the above only applies to Rx FIFOs of 4 KB or more when the EFC bit is set high."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Rfa::B11)
    }
}
#[doc = "Threshold for deactivating flow control (in both HD and FD) These bits control the threshold (Fill-level of Rx FIFO) at which the flow-control is de-asserted after activation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rfd {
    #[doc = "0: Full minus 4 KB Note that the de-assertion is effective only after flow control is asserted."]
    B00 = 0,
    #[doc = "1: Full minus 4 KB Note that the de-assertion is effective only after flow control is asserted."]
    B01 = 1,
    #[doc = "2: Full minus 4 KB Note that the de-assertion is effective only after flow control is asserted."]
    B10 = 2,
    #[doc = "3: Full minus 4 KB Note that the de-assertion is effective only after flow control is asserted."]
    B11 = 3,
}
impl From<Rfd> for u8 {
    #[inline(always)]
    fn from(variant: Rfd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rfd {
    type Ux = u8;
}
#[doc = "Field `RFD` reader - Threshold for deactivating flow control (in both HD and FD) These bits control the threshold (Fill-level of Rx FIFO) at which the flow-control is de-asserted after activation."]
pub type RfdR = crate::FieldReader<Rfd>;
impl RfdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfd {
        match self.bits {
            0 => Rfd::B00,
            1 => Rfd::B01,
            2 => Rfd::B10,
            3 => Rfd::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Full minus 4 KB Note that the de-assertion is effective only after flow control is asserted."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Rfd::B00
    }
    #[doc = "Full minus 4 KB Note that the de-assertion is effective only after flow control is asserted."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Rfd::B01
    }
    #[doc = "Full minus 4 KB Note that the de-assertion is effective only after flow control is asserted."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Rfd::B10
    }
    #[doc = "Full minus 4 KB Note that the de-assertion is effective only after flow control is asserted."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Rfd::B11
    }
}
#[doc = "Field `RFD` writer - Threshold for deactivating flow control (in both HD and FD) These bits control the threshold (Fill-level of Rx FIFO) at which the flow-control is de-asserted after activation."]
pub type RfdW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Rfd>;
impl<'a, REG> RfdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Full minus 4 KB Note that the de-assertion is effective only after flow control is asserted."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Rfd::B00)
    }
    #[doc = "Full minus 4 KB Note that the de-assertion is effective only after flow control is asserted."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Rfd::B01)
    }
    #[doc = "Full minus 4 KB Note that the de-assertion is effective only after flow control is asserted."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Rfd::B10)
    }
    #[doc = "Full minus 4 KB Note that the de-assertion is effective only after flow control is asserted."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Rfd::B11)
    }
}
#[doc = "Field `ST` reader - Start/Stop Transmission Command When this bit is set, transmission is placed in the Running state, and the DMA checks the Transmit List at the current position for a frame to be transmitted. Descriptor acquisition is attempted either from the current position in the list, which is the Transmit List Base Address set by Register GMAC_TX_DESC_LIST_ADDR, or from the position retained when transmission was stopped previously. If the current descriptor is not owned by the DMA, transmission enters the Suspended state and Transmit Buffer Unavailable (Register GMAC_STATUS\\[2\\]) is set. The Start Transmission command is effective only when transmission is stopped. If the command is issued before setting DMA Register TX_DESC_LIST_ADDR, then the DMA behavior is unpredictable. When this bit is reset, the transmission process is placed in the Stopped state after completing the transmission of the current frame. The Next Descriptor position in the Transmit List is saved, and becomes the current position when transmission is restarted. The stop transmission command is effective only the transmission of the current frame is complete or when the transmission is in the Suspended state."]
pub type StR = crate::BitReader;
#[doc = "Field `ST` writer - Start/Stop Transmission Command When this bit is set, transmission is placed in the Running state, and the DMA checks the Transmit List at the current position for a frame to be transmitted. Descriptor acquisition is attempted either from the current position in the list, which is the Transmit List Base Address set by Register GMAC_TX_DESC_LIST_ADDR, or from the position retained when transmission was stopped previously. If the current descriptor is not owned by the DMA, transmission enters the Suspended state and Transmit Buffer Unavailable (Register GMAC_STATUS\\[2\\]) is set. The Start Transmission command is effective only when transmission is stopped. If the command is issued before setting DMA Register TX_DESC_LIST_ADDR, then the DMA behavior is unpredictable. When this bit is reset, the transmission process is placed in the Stopped state after completing the transmission of the current frame. The Next Descriptor position in the Transmit List is saved, and becomes the current position when transmission is restarted. The stop transmission command is effective only the transmission of the current frame is complete or when the transmission is in the Suspended state."]
pub type StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Transmit Threshold Control These three bits control the threshold level of the MTL Transmit FIFO. Transmission starts when the frame size within the MTL Transmit FIFO is larger than the threshold. In addition, full frames with a length less than the threshold are also transmitted. These bits are used only when the TSF bit (Bit 21) is reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ttc {
    #[doc = "0: 16"]
    B000 = 0,
    #[doc = "1: 16"]
    B001 = 1,
    #[doc = "2: 16"]
    B010 = 2,
    #[doc = "3: 16"]
    B011 = 3,
    #[doc = "4: 16"]
    B100 = 4,
    #[doc = "5: 16"]
    B101 = 5,
    #[doc = "6: 16"]
    B110 = 6,
    #[doc = "7: 16"]
    B111 = 7,
}
impl From<Ttc> for u8 {
    #[inline(always)]
    fn from(variant: Ttc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ttc {
    type Ux = u8;
}
#[doc = "Field `TTC` reader - Transmit Threshold Control These three bits control the threshold level of the MTL Transmit FIFO. Transmission starts when the frame size within the MTL Transmit FIFO is larger than the threshold. In addition, full frames with a length less than the threshold are also transmitted. These bits are used only when the TSF bit (Bit 21) is reset."]
pub type TtcR = crate::FieldReader<Ttc>;
impl TtcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ttc {
        match self.bits {
            0 => Ttc::B000,
            1 => Ttc::B001,
            2 => Ttc::B010,
            3 => Ttc::B011,
            4 => Ttc::B100,
            5 => Ttc::B101,
            6 => Ttc::B110,
            7 => Ttc::B111,
            _ => unreachable!(),
        }
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Ttc::B000
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Ttc::B001
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Ttc::B010
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Ttc::B011
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Ttc::B100
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == Ttc::B101
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == Ttc::B110
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == Ttc::B111
    }
}
#[doc = "Field `TTC` writer - Transmit Threshold Control These three bits control the threshold level of the MTL Transmit FIFO. Transmission starts when the frame size within the MTL Transmit FIFO is larger than the threshold. In addition, full frames with a length less than the threshold are also transmitted. These bits are used only when the TSF bit (Bit 21) is reset."]
pub type TtcW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Ttc>;
impl<'a, REG> TtcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Ttc::B000)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(Ttc::B001)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(Ttc::B010)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(Ttc::B011)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Ttc::B100)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(Ttc::B101)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(Ttc::B110)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(Ttc::B111)
    }
}
#[doc = "Field `FTF` reader - Flush Transmit FIFO When this bit is set, the transmit FIFO controller logic is reset to its default values and thus all data in the Tx FIFO is lost/flushed. This bit is cleared internally when the flushing operation is completed fully. The Operation Mode register should not be written to until this bit is cleared. The data which is already accepted by the MAC transmitter will not be flushed. It will be scheduled for transmission and will result in underflow and runt frame transmission. Note: The flush operation completes only after emptying the TxFIFO of its contents and all the pending Transmit Status of the transmitted frames are accepted by the host. In order to complete this flush operation, the PHY transmit clock (clk_tx_i) is required to be active."]
pub type FtfR = crate::BitReader;
#[doc = "Field `FTF` writer - Flush Transmit FIFO When this bit is set, the transmit FIFO controller logic is reset to its default values and thus all data in the Tx FIFO is lost/flushed. This bit is cleared internally when the flushing operation is completed fully. The Operation Mode register should not be written to until this bit is cleared. The data which is already accepted by the MAC transmitter will not be flushed. It will be scheduled for transmission and will result in underflow and runt frame transmission. Note: The flush operation completes only after emptying the TxFIFO of its contents and all the pending Transmit Status of the transmitted frames are accepted by the host. In order to complete this flush operation, the PHY transmit clock (clk_tx_i) is required to be active."]
pub type FtfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TSF` reader - Transmit Store and Forward When this bit is set, transmission starts when a full frame resides in the MTL Transmit FIFO. When this bit is set, the TTC values specified in Register GMAC_OP_MODE\\[16:14\\]
are ignored. This bit should be changed only when transmission is stopped."]
pub type TsfR = crate::BitReader;
#[doc = "Field `TSF` writer - Transmit Store and Forward When this bit is set, transmission starts when a full frame resides in the MTL Transmit FIFO. When this bit is set, the TTC values specified in Register GMAC_OP_MODE\\[16:14\\]
are ignored. This bit should be changed only when transmission is stopped."]
pub type TsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFF` reader - Disable Flushing of Received Frames When this bit is set, the RxDMA does not flush any frames due to the unavailability of receive descriptors/buffers as it does normally when this bit is reset."]
pub type DffR = crate::BitReader;
#[doc = "Field `DFF` writer - Disable Flushing of Received Frames When this bit is set, the RxDMA does not flush any frames due to the unavailability of receive descriptors/buffers as it does normally when this bit is reset."]
pub type DffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSF` reader - Receive Store and Forward When this bit is set, the MTL only reads a frame from the Rx FIFO after the complete frame has been written to it, ignoring RTC bits. When this bit is reset, the Rx FIFO operates in Cut-Through mode, subject to the threshold specified by the RTC bits."]
pub type RsfR = crate::BitReader;
#[doc = "Field `RSF` writer - Receive Store and Forward When this bit is set, the MTL only reads a frame from the Rx FIFO after the complete frame has been written to it, ignoring RTC bits. When this bit is reset, the Rx FIFO operates in Cut-Through mode, subject to the threshold specified by the RTC bits."]
pub type RsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DT` reader - Disable Dropping of TCP/IP Checksum Error Frames When this bit is set, the core does not drop frames that only have errors detected by the Receive Checksum Offload engine. Such frames do not have any errors (including FCS error) in the Ethernet frame received by the MAC but have errors in the encapsulated payload only. When this bit is reset, all error frames are dropped if the FEF bit is reset."]
pub type DtR = crate::BitReader;
#[doc = "Field `DT` writer - Disable Dropping of TCP/IP Checksum Error Frames When this bit is set, the core does not drop frames that only have errors detected by the Receive Checksum Offload engine. Such frames do not have any errors (including FCS error) in the Ethernet frame received by the MAC but have errors in the encapsulated payload only. When this bit is reset, all error frames are dropped if the FEF bit is reset."]
pub type DtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Start/Stop Receive When this bit is set, the Receive process is placed in the Running state. The DMA attempts to acquire the descriptor from the Receive list and processes incoming frames. Descriptor acquisition is attempted from the current position in the list, which is the address set by register GMAC_RX_DESC_LIST_ADDR or the position retained when the Receive process was previously stopped. If no descriptor is owned by the DMA, reception is suspended and Receive Buffer Unavailable (Register GMAC_STATUS\\[7\\]) is set. The Start Receive command is effective only when reception has stopped. If the command was issued before setting register GMAC_RX_DESC_LIST_ADDR, DMA behavior is unpredictable. When this bit is cleared, RxDMA operation is stopped after the transfer of the current frame. The next descriptor position in the Receive list is saved and becomes the current position after the Receive process is restarted. The Stop Receive command is effective only when the Receive process is in either the Running (waiting for receive packet) or in the Suspended state."]
    #[inline(always)]
    pub fn sr(&self) -> SrR {
        SrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Operate on Second Frame When this bit is set, this bit instructs the DMA to process a second frame of Transmit data even before status for first frame is obtained."]
    #[inline(always)]
    pub fn osf(&self) -> OsfR {
        OsfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Receive Threshold Control These two bits control the threshold level of the MTL Receive FIFO. Transfer (request) to DMA starts when the frame size within the MTL Receive FIFO is larger than the threshold. In addition, full frames with a length less than the threshold are transferred automatically. Note that value of 11 is not applicable if the configured Receive FIFO size is 128 bytes. These bits are valid only when the RSF bit is zero, and are ignored when the RSF bit is set to 1."]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 6 - Forward Undersized Good Frames When set, the Rx FIFO will forward Undersized frames (frames with no Error and length less than 64 bytes) including pad-bytes and CRC). When reset, the Rx FIFO will drop all frames of less than 64 bytes, unless it is already transferred due to lower value of Receive Threshold (e.g., RTC = 01)."]
    #[inline(always)]
    pub fn fuf(&self) -> FufR {
        FufR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Forward Error Frames When this bit is reset, the Rx FIFO drops frames with error status (CRC error, collision error, GMII_ER, giant frame, watchdog timeout, overflow). However, if the frame's start byte (write) pointer is already transferred to the read controller side (in Threshold mode), then the frames are not dropped. When FEF is set, all frames except runt error frames are forwarded to the DMA. But when RxFIFO overflows when a partial frame is written, then such frames are dropped even when FEF is set."]
    #[inline(always)]
    pub fn fef(&self) -> FefR {
        FefR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable HW flow control When this bit is set, the flow control signal operation based on fill-level of Rx FIFO is enabled. When reset, the flow control operation is disabled."]
    #[inline(always)]
    pub fn efc(&self) -> EfcR {
        EfcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Threshold for activating flow control (in both HD and FD) These bits control the threshold (Fill level of Rx FIFO) at which flow control is activated."]
    #[inline(always)]
    pub fn rfa(&self) -> RfaR {
        RfaR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - Threshold for deactivating flow control (in both HD and FD) These bits control the threshold (Fill-level of Rx FIFO) at which the flow-control is de-asserted after activation."]
    #[inline(always)]
    pub fn rfd(&self) -> RfdR {
        RfdR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Start/Stop Transmission Command When this bit is set, transmission is placed in the Running state, and the DMA checks the Transmit List at the current position for a frame to be transmitted. Descriptor acquisition is attempted either from the current position in the list, which is the Transmit List Base Address set by Register GMAC_TX_DESC_LIST_ADDR, or from the position retained when transmission was stopped previously. If the current descriptor is not owned by the DMA, transmission enters the Suspended state and Transmit Buffer Unavailable (Register GMAC_STATUS\\[2\\]) is set. The Start Transmission command is effective only when transmission is stopped. If the command is issued before setting DMA Register TX_DESC_LIST_ADDR, then the DMA behavior is unpredictable. When this bit is reset, the transmission process is placed in the Stopped state after completing the transmission of the current frame. The Next Descriptor position in the Transmit List is saved, and becomes the current position when transmission is restarted. The stop transmission command is effective only the transmission of the current frame is complete or when the transmission is in the Suspended state."]
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16 - Transmit Threshold Control These three bits control the threshold level of the MTL Transmit FIFO. Transmission starts when the frame size within the MTL Transmit FIFO is larger than the threshold. In addition, full frames with a length less than the threshold are also transmitted. These bits are used only when the TSF bit (Bit 21) is reset."]
    #[inline(always)]
    pub fn ttc(&self) -> TtcR {
        TtcR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 20 - Flush Transmit FIFO When this bit is set, the transmit FIFO controller logic is reset to its default values and thus all data in the Tx FIFO is lost/flushed. This bit is cleared internally when the flushing operation is completed fully. The Operation Mode register should not be written to until this bit is cleared. The data which is already accepted by the MAC transmitter will not be flushed. It will be scheduled for transmission and will result in underflow and runt frame transmission. Note: The flush operation completes only after emptying the TxFIFO of its contents and all the pending Transmit Status of the transmitted frames are accepted by the host. In order to complete this flush operation, the PHY transmit clock (clk_tx_i) is required to be active."]
    #[inline(always)]
    pub fn ftf(&self) -> FtfR {
        FtfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit Store and Forward When this bit is set, transmission starts when a full frame resides in the MTL Transmit FIFO. When this bit is set, the TTC values specified in Register GMAC_OP_MODE\\[16:14\\]
are ignored. This bit should be changed only when transmission is stopped."]
    #[inline(always)]
    pub fn tsf(&self) -> TsfR {
        TsfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Disable Flushing of Received Frames When this bit is set, the RxDMA does not flush any frames due to the unavailability of receive descriptors/buffers as it does normally when this bit is reset."]
    #[inline(always)]
    pub fn dff(&self) -> DffR {
        DffR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Receive Store and Forward When this bit is set, the MTL only reads a frame from the Rx FIFO after the complete frame has been written to it, ignoring RTC bits. When this bit is reset, the Rx FIFO operates in Cut-Through mode, subject to the threshold specified by the RTC bits."]
    #[inline(always)]
    pub fn rsf(&self) -> RsfR {
        RsfR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Disable Dropping of TCP/IP Checksum Error Frames When this bit is set, the core does not drop frames that only have errors detected by the Receive Checksum Offload engine. Such frames do not have any errors (including FCS error) in the Ethernet frame received by the MAC but have errors in the encapsulated payload only. When this bit is reset, all error frames are dropped if the FEF bit is reset."]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Start/Stop Receive When this bit is set, the Receive process is placed in the Running state. The DMA attempts to acquire the descriptor from the Receive list and processes incoming frames. Descriptor acquisition is attempted from the current position in the list, which is the address set by register GMAC_RX_DESC_LIST_ADDR or the position retained when the Receive process was previously stopped. If no descriptor is owned by the DMA, reception is suspended and Receive Buffer Unavailable (Register GMAC_STATUS\\[7\\]) is set. The Start Receive command is effective only when reception has stopped. If the command was issued before setting register GMAC_RX_DESC_LIST_ADDR, DMA behavior is unpredictable. When this bit is cleared, RxDMA operation is stopped after the transfer of the current frame. The next descriptor position in the Receive list is saved and becomes the current position after the Receive process is restarted. The Stop Receive command is effective only when the Receive process is in either the Running (waiting for receive packet) or in the Suspended state."]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SrW<GmacOpModeSpec> {
        SrW::new(self, 1)
    }
    #[doc = "Bit 2 - Operate on Second Frame When this bit is set, this bit instructs the DMA to process a second frame of Transmit data even before status for first frame is obtained."]
    #[inline(always)]
    #[must_use]
    pub fn osf(&mut self) -> OsfW<GmacOpModeSpec> {
        OsfW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Receive Threshold Control These two bits control the threshold level of the MTL Receive FIFO. Transfer (request) to DMA starts when the frame size within the MTL Receive FIFO is larger than the threshold. In addition, full frames with a length less than the threshold are transferred automatically. Note that value of 11 is not applicable if the configured Receive FIFO size is 128 bytes. These bits are valid only when the RSF bit is zero, and are ignored when the RSF bit is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RtcW<GmacOpModeSpec> {
        RtcW::new(self, 3)
    }
    #[doc = "Bit 6 - Forward Undersized Good Frames When set, the Rx FIFO will forward Undersized frames (frames with no Error and length less than 64 bytes) including pad-bytes and CRC). When reset, the Rx FIFO will drop all frames of less than 64 bytes, unless it is already transferred due to lower value of Receive Threshold (e.g., RTC = 01)."]
    #[inline(always)]
    #[must_use]
    pub fn fuf(&mut self) -> FufW<GmacOpModeSpec> {
        FufW::new(self, 6)
    }
    #[doc = "Bit 7 - Forward Error Frames When this bit is reset, the Rx FIFO drops frames with error status (CRC error, collision error, GMII_ER, giant frame, watchdog timeout, overflow). However, if the frame's start byte (write) pointer is already transferred to the read controller side (in Threshold mode), then the frames are not dropped. When FEF is set, all frames except runt error frames are forwarded to the DMA. But when RxFIFO overflows when a partial frame is written, then such frames are dropped even when FEF is set."]
    #[inline(always)]
    #[must_use]
    pub fn fef(&mut self) -> FefW<GmacOpModeSpec> {
        FefW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable HW flow control When this bit is set, the flow control signal operation based on fill-level of Rx FIFO is enabled. When reset, the flow control operation is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn efc(&mut self) -> EfcW<GmacOpModeSpec> {
        EfcW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Threshold for activating flow control (in both HD and FD) These bits control the threshold (Fill level of Rx FIFO) at which flow control is activated."]
    #[inline(always)]
    #[must_use]
    pub fn rfa(&mut self) -> RfaW<GmacOpModeSpec> {
        RfaW::new(self, 9)
    }
    #[doc = "Bits 11:12 - Threshold for deactivating flow control (in both HD and FD) These bits control the threshold (Fill-level of Rx FIFO) at which the flow-control is de-asserted after activation."]
    #[inline(always)]
    #[must_use]
    pub fn rfd(&mut self) -> RfdW<GmacOpModeSpec> {
        RfdW::new(self, 11)
    }
    #[doc = "Bit 13 - Start/Stop Transmission Command When this bit is set, transmission is placed in the Running state, and the DMA checks the Transmit List at the current position for a frame to be transmitted. Descriptor acquisition is attempted either from the current position in the list, which is the Transmit List Base Address set by Register GMAC_TX_DESC_LIST_ADDR, or from the position retained when transmission was stopped previously. If the current descriptor is not owned by the DMA, transmission enters the Suspended state and Transmit Buffer Unavailable (Register GMAC_STATUS\\[2\\]) is set. The Start Transmission command is effective only when transmission is stopped. If the command is issued before setting DMA Register TX_DESC_LIST_ADDR, then the DMA behavior is unpredictable. When this bit is reset, the transmission process is placed in the Stopped state after completing the transmission of the current frame. The Next Descriptor position in the Transmit List is saved, and becomes the current position when transmission is restarted. The stop transmission command is effective only the transmission of the current frame is complete or when the transmission is in the Suspended state."]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> StW<GmacOpModeSpec> {
        StW::new(self, 13)
    }
    #[doc = "Bits 14:16 - Transmit Threshold Control These three bits control the threshold level of the MTL Transmit FIFO. Transmission starts when the frame size within the MTL Transmit FIFO is larger than the threshold. In addition, full frames with a length less than the threshold are also transmitted. These bits are used only when the TSF bit (Bit 21) is reset."]
    #[inline(always)]
    #[must_use]
    pub fn ttc(&mut self) -> TtcW<GmacOpModeSpec> {
        TtcW::new(self, 14)
    }
    #[doc = "Bit 20 - Flush Transmit FIFO When this bit is set, the transmit FIFO controller logic is reset to its default values and thus all data in the Tx FIFO is lost/flushed. This bit is cleared internally when the flushing operation is completed fully. The Operation Mode register should not be written to until this bit is cleared. The data which is already accepted by the MAC transmitter will not be flushed. It will be scheduled for transmission and will result in underflow and runt frame transmission. Note: The flush operation completes only after emptying the TxFIFO of its contents and all the pending Transmit Status of the transmitted frames are accepted by the host. In order to complete this flush operation, the PHY transmit clock (clk_tx_i) is required to be active."]
    #[inline(always)]
    #[must_use]
    pub fn ftf(&mut self) -> FtfW<GmacOpModeSpec> {
        FtfW::new(self, 20)
    }
    #[doc = "Bit 21 - Transmit Store and Forward When this bit is set, transmission starts when a full frame resides in the MTL Transmit FIFO. When this bit is set, the TTC values specified in Register GMAC_OP_MODE\\[16:14\\]
are ignored. This bit should be changed only when transmission is stopped."]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TsfW<GmacOpModeSpec> {
        TsfW::new(self, 21)
    }
    #[doc = "Bit 24 - Disable Flushing of Received Frames When this bit is set, the RxDMA does not flush any frames due to the unavailability of receive descriptors/buffers as it does normally when this bit is reset."]
    #[inline(always)]
    #[must_use]
    pub fn dff(&mut self) -> DffW<GmacOpModeSpec> {
        DffW::new(self, 24)
    }
    #[doc = "Bit 25 - Receive Store and Forward When this bit is set, the MTL only reads a frame from the Rx FIFO after the complete frame has been written to it, ignoring RTC bits. When this bit is reset, the Rx FIFO operates in Cut-Through mode, subject to the threshold specified by the RTC bits."]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RsfW<GmacOpModeSpec> {
        RsfW::new(self, 25)
    }
    #[doc = "Bit 26 - Disable Dropping of TCP/IP Checksum Error Frames When this bit is set, the core does not drop frames that only have errors detected by the Receive Checksum Offload engine. Such frames do not have any errors (including FCS error) in the Ethernet frame received by the MAC but have errors in the encapsulated payload only. When this bit is reset, all error frames are dropped if the FEF bit is reset."]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DtW<GmacOpModeSpec> {
        DtW::new(self, 26)
    }
}
#[doc = "Operation Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_op_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_op_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacOpModeSpec;
impl crate::RegisterSpec for GmacOpModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_op_mode::R`](R) reader structure"]
impl crate::Readable for GmacOpModeSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_op_mode::W`](W) writer structure"]
impl crate::Writable for GmacOpModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0010_0000;
}
#[doc = "`reset()` method sets GMAC_OP_MODE to value 0"]
impl crate::Resettable for GmacOpModeSpec {
    const RESET_VALUE: u32 = 0;
}
