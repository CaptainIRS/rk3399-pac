#[doc = "Register `GMAC_OP_MODE` reader"]
pub type R = crate::R<GmacOpModeSpec>;
#[doc = "Register `GMAC_OP_MODE` writer"]
pub type W = crate::W<GmacOpModeSpec>;
#[doc = "Field `SR` reader - Start/Stop Receive\n\nWhen this bit is set, the Receive process is placed in the Running\n\nstate. The DMA attempts to acquire the descriptor from the\n\nReceive list and processes incoming frames. Descriptor\n\nacquisition is attempted from the current position in the list,\n\nwhich is the address set by register GMAC_RX_DESC_LIST_ADDR\n\nor the position retained when the Receive process was previously\n\nstopped. If no descriptor is owned by the DMA, reception is\n\nsuspended and Receive Buffer Unavailable (Register\n\nGMAC_STATUS\\[7\\]) is set. The Start Receive command is effective\n\nonly when reception has stopped. If the command was issued\n\nbefore setting register GMAC_RX_DESC_LIST_ADDR, DMA\n\nbehavior is unpredictable.\n\nWhen this bit is cleared, RxDMA operation is stopped after the\n\ntransfer of the current frame. The next descriptor position in the\n\nReceive list is saved and becomes the current position after the\n\nReceive process is restarted. The Stop Receive command is\n\neffective only when the Receive process is in either the Running\n\n(waiting for receive packet) or in the Suspended state."]
pub type SrR = crate::BitReader;
#[doc = "Field `SR` writer - Start/Stop Receive\n\nWhen this bit is set, the Receive process is placed in the Running\n\nstate. The DMA attempts to acquire the descriptor from the\n\nReceive list and processes incoming frames. Descriptor\n\nacquisition is attempted from the current position in the list,\n\nwhich is the address set by register GMAC_RX_DESC_LIST_ADDR\n\nor the position retained when the Receive process was previously\n\nstopped. If no descriptor is owned by the DMA, reception is\n\nsuspended and Receive Buffer Unavailable (Register\n\nGMAC_STATUS\\[7\\]) is set. The Start Receive command is effective\n\nonly when reception has stopped. If the command was issued\n\nbefore setting register GMAC_RX_DESC_LIST_ADDR, DMA\n\nbehavior is unpredictable.\n\nWhen this bit is cleared, RxDMA operation is stopped after the\n\ntransfer of the current frame. The next descriptor position in the\n\nReceive list is saved and becomes the current position after the\n\nReceive process is restarted. The Stop Receive command is\n\neffective only when the Receive process is in either the Running\n\n(waiting for receive packet) or in the Suspended state."]
pub type SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSF` reader - Operate on Second Frame\n\nWhen this bit is set, this bit instructs the DMA to process a\n\nsecond frame of Transmit data even before status for first frame\n\nis obtained."]
pub type OsfR = crate::BitReader;
#[doc = "Field `OSF` writer - Operate on Second Frame\n\nWhen this bit is set, this bit instructs the DMA to process a\n\nsecond frame of Transmit data even before status for first frame\n\nis obtained."]
pub type OsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Receive Threshold Control\n\nThese two bits control the threshold level of the MTL Receive\n\nFIFO. Transfer (request) to DMA starts when the frame size\n\nwithin the MTL Receive FIFO is larger than the threshold. In\n\naddition, full frames with a length less than the threshold are\n\ntransferred automatically. Note that value of 11 is not applicable\n\nif the configured Receive FIFO size is 128 bytes. These bits are\n\nvalid only when the RSF bit is zero, and are ignored when the\n\nRSF bit is set to 1.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtc {
    #[doc = "0: 64"]
    B00 = 0,
    #[doc = "1: 32"]
    B01 = 1,
    #[doc = "2: 96"]
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
#[doc = "Field `RTC` reader - Receive Threshold Control\n\nThese two bits control the threshold level of the MTL Receive\n\nFIFO. Transfer (request) to DMA starts when the frame size\n\nwithin the MTL Receive FIFO is larger than the threshold. In\n\naddition, full frames with a length less than the threshold are\n\ntransferred automatically. Note that value of 11 is not applicable\n\nif the configured Receive FIFO size is 128 bytes. These bits are\n\nvalid only when the RSF bit is zero, and are ignored when the\n\nRSF bit is set to 1."]
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
    #[doc = "64"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Rtc::B00
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Rtc::B01
    }
    #[doc = "96"]
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
#[doc = "Field `RTC` writer - Receive Threshold Control\n\nThese two bits control the threshold level of the MTL Receive\n\nFIFO. Transfer (request) to DMA starts when the frame size\n\nwithin the MTL Receive FIFO is larger than the threshold. In\n\naddition, full frames with a length less than the threshold are\n\ntransferred automatically. Note that value of 11 is not applicable\n\nif the configured Receive FIFO size is 128 bytes. These bits are\n\nvalid only when the RSF bit is zero, and are ignored when the\n\nRSF bit is set to 1."]
pub type RtcW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Rtc>;
impl<'a, REG> RtcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "64"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::B00)
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::B01)
    }
    #[doc = "96"]
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
#[doc = "Field `FUF` reader - Forward Undersized Good Frames\n\nWhen set, the Rx FIFO will forward Undersized frames (frames\n\nwith no Error and length less than 64 bytes) including pad-bytes\n\nand CRC).\n\nWhen reset, the Rx FIFO will drop all frames of less than 64\n\nbytes, unless it is already transferred due to lower value of\n\nReceive Threshold (e.g., RTC = 01)."]
pub type FufR = crate::BitReader;
#[doc = "Field `FUF` writer - Forward Undersized Good Frames\n\nWhen set, the Rx FIFO will forward Undersized frames (frames\n\nwith no Error and length less than 64 bytes) including pad-bytes\n\nand CRC).\n\nWhen reset, the Rx FIFO will drop all frames of less than 64\n\nbytes, unless it is already transferred due to lower value of\n\nReceive Threshold (e.g., RTC = 01)."]
pub type FufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEF` reader - Forward Error Frames\n\nWhen this bit is reset, the Rx FIFO drops frames with error status\n\n(CRC error, collision error, GMII_ER, giant frame, watchdog\n\ntimeout, overflow). However, if the frame's start byte (write)\n\npointer is already transferred to the read controller side (in\n\nThreshold mode), then the frames are not dropped.\n\nWhen FEF is set, all frames except runt error frames are\n\nforwarded to the DMA. But when RxFIFO overflows when a partial\n\nframe is written, then such frames are dropped even when FEF is\n\nset."]
pub type FefR = crate::BitReader;
#[doc = "Field `FEF` writer - Forward Error Frames\n\nWhen this bit is reset, the Rx FIFO drops frames with error status\n\n(CRC error, collision error, GMII_ER, giant frame, watchdog\n\ntimeout, overflow). However, if the frame's start byte (write)\n\npointer is already transferred to the read controller side (in\n\nThreshold mode), then the frames are not dropped.\n\nWhen FEF is set, all frames except runt error frames are\n\nforwarded to the DMA. But when RxFIFO overflows when a partial\n\nframe is written, then such frames are dropped even when FEF is\n\nset."]
pub type FefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFC` reader - Enable HW flow control\n\nWhen this bit is set, the flow control signal operation based on\n\nfill-level of Rx FIFO is enabled. When reset, the flow control\n\noperation is disabled."]
pub type EfcR = crate::BitReader;
#[doc = "Field `EFC` writer - Enable HW flow control\n\nWhen this bit is set, the flow control signal operation based on\n\nfill-level of Rx FIFO is enabled. When reset, the flow control\n\noperation is disabled."]
pub type EfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Threshold for activating flow control (in both HD and FD)\n\nThese bits control the threshold (Fill level of Rx FIFO) at which\n\nflow control is activated.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rfa {
    #[doc = "0: Full minus 1 KB"]
    B00 = 0,
    #[doc = "1: Full minus 2 KB"]
    B01 = 1,
    #[doc = "2: Full minus 3 KB"]
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
#[doc = "Field `RFA` reader - Threshold for activating flow control (in both HD and FD)\n\nThese bits control the threshold (Fill level of Rx FIFO) at which\n\nflow control is activated."]
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
    #[doc = "Full minus 1 KB"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Rfa::B00
    }
    #[doc = "Full minus 2 KB"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Rfa::B01
    }
    #[doc = "Full minus 3 KB"]
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
#[doc = "Field `RFA` writer - Threshold for activating flow control (in both HD and FD)\n\nThese bits control the threshold (Fill level of Rx FIFO) at which\n\nflow control is activated."]
pub type RfaW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Rfa>;
impl<'a, REG> RfaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Full minus 1 KB"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Rfa::B00)
    }
    #[doc = "Full minus 2 KB"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Rfa::B01)
    }
    #[doc = "Full minus 3 KB"]
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
#[doc = "Threshold for deactivating flow control (in both HD and FD)\n\nThese bits control the threshold (Fill-level of Rx FIFO) at which\n\nthe flow-control is de-asserted after activation.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rfd {
    #[doc = "0: Full minus 1 KB"]
    B00 = 0,
    #[doc = "1: Full minus 2 KB"]
    B01 = 1,
    #[doc = "2: Full minus 3 KB"]
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
#[doc = "Field `RFD` reader - Threshold for deactivating flow control (in both HD and FD)\n\nThese bits control the threshold (Fill-level of Rx FIFO) at which\n\nthe flow-control is de-asserted after activation."]
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
    #[doc = "Full minus 1 KB"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Rfd::B00
    }
    #[doc = "Full minus 2 KB"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Rfd::B01
    }
    #[doc = "Full minus 3 KB"]
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
#[doc = "Field `RFD` writer - Threshold for deactivating flow control (in both HD and FD)\n\nThese bits control the threshold (Fill-level of Rx FIFO) at which\n\nthe flow-control is de-asserted after activation."]
pub type RfdW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Rfd>;
impl<'a, REG> RfdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Full minus 1 KB"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Rfd::B00)
    }
    #[doc = "Full minus 2 KB"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Rfd::B01)
    }
    #[doc = "Full minus 3 KB"]
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
#[doc = "Field `ST` reader - Start/Stop Transmission Command\n\nWhen this bit is set, transmission is placed in the Running state,\n\nand the DMA checks the Transmit List at the current position for a\n\nframe to be transmitted. Descriptor acquisition is attempted\n\neither from the current position in the list, which is the Transmit\n\nList Base Address set by Register GMAC_TX_DESC_LIST_ADDR,\n\nor from the position retained when transmission was stopped\n\npreviously. If the current descriptor is not owned by the DMA,\n\ntransmission enters the Suspended state and Transmit Buffer\n\nUnavailable (Register GMAC_STATUS\\[2\\]) is set. The Start\n\nTransmission command is effective only when transmission is\n\nstopped. If the command is issued before setting DMA Register\n\nTX_DESC_LIST_ADDR, then the DMA behavior is unpredictable.\n\nWhen this bit is reset, the transmission process is placed in the\n\nStopped state after completing the transmission of the current\n\nframe. The Next Descriptor position in the Transmit List is saved,\n\nand becomes the current position when transmission is restarted.\n\nThe stop transmission command is effective only the transmission\n\nof the current frame is complete or when the transmission is in\n\nthe Suspended state."]
pub type StR = crate::BitReader;
#[doc = "Field `ST` writer - Start/Stop Transmission Command\n\nWhen this bit is set, transmission is placed in the Running state,\n\nand the DMA checks the Transmit List at the current position for a\n\nframe to be transmitted. Descriptor acquisition is attempted\n\neither from the current position in the list, which is the Transmit\n\nList Base Address set by Register GMAC_TX_DESC_LIST_ADDR,\n\nor from the position retained when transmission was stopped\n\npreviously. If the current descriptor is not owned by the DMA,\n\ntransmission enters the Suspended state and Transmit Buffer\n\nUnavailable (Register GMAC_STATUS\\[2\\]) is set. The Start\n\nTransmission command is effective only when transmission is\n\nstopped. If the command is issued before setting DMA Register\n\nTX_DESC_LIST_ADDR, then the DMA behavior is unpredictable.\n\nWhen this bit is reset, the transmission process is placed in the\n\nStopped state after completing the transmission of the current\n\nframe. The Next Descriptor position in the Transmit List is saved,\n\nand becomes the current position when transmission is restarted.\n\nThe stop transmission command is effective only the transmission\n\nof the current frame is complete or when the transmission is in\n\nthe Suspended state."]
pub type StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Transmit Threshold Control\n\nThese three bits control the threshold level of the MTL Transmit\n\nFIFO. Transmission starts when the frame size within the MTL\n\nTransmit FIFO is larger than the threshold. In addition, full\n\nframes with a length less than the threshold are also transmitted.\n\nThese bits are used only when the TSF bit (Bit 21) is reset.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ttc {
    #[doc = "0: 64"]
    B000 = 0,
    #[doc = "1: 128"]
    B001 = 1,
    #[doc = "2: 192"]
    B010 = 2,
    #[doc = "3: 256"]
    B011 = 3,
    #[doc = "4: 40"]
    B100 = 4,
    #[doc = "5: 32"]
    B101 = 5,
    #[doc = "6: 24"]
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
#[doc = "Field `TTC` reader - Transmit Threshold Control\n\nThese three bits control the threshold level of the MTL Transmit\n\nFIFO. Transmission starts when the frame size within the MTL\n\nTransmit FIFO is larger than the threshold. In addition, full\n\nframes with a length less than the threshold are also transmitted.\n\nThese bits are used only when the TSF bit (Bit 21) is reset."]
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
    #[doc = "64"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Ttc::B000
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Ttc::B001
    }
    #[doc = "192"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Ttc::B010
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Ttc::B011
    }
    #[doc = "40"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Ttc::B100
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == Ttc::B101
    }
    #[doc = "24"]
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
#[doc = "Field `TTC` writer - Transmit Threshold Control\n\nThese three bits control the threshold level of the MTL Transmit\n\nFIFO. Transmission starts when the frame size within the MTL\n\nTransmit FIFO is larger than the threshold. In addition, full\n\nframes with a length less than the threshold are also transmitted.\n\nThese bits are used only when the TSF bit (Bit 21) is reset."]
pub type TtcW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Ttc>;
impl<'a, REG> TtcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "64"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Ttc::B000)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(Ttc::B001)
    }
    #[doc = "192"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(Ttc::B010)
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(Ttc::B011)
    }
    #[doc = "40"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Ttc::B100)
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(Ttc::B101)
    }
    #[doc = "24"]
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
#[doc = "Field `FTF` reader - Flush Transmit FIFO\n\nWhen this bit is set, the transmit FIFO controller logic is reset to\n\nits default values and thus all data in the Tx FIFO is lost/flushed.\n\nThis bit is cleared internally when the flushing operation is\n\ncompleted fully. The Operation Mode register should not be\n\nwritten to until this bit is cleared. The data which is already\n\naccepted by the MAC transmitter will not be flushed. It will be\n\nscheduled for transmission and will result in underflow and runt\n\nframe transmission.\n\nNote: The flush operation completes only after emptying the\n\nTxFIFO of its contents and all the pending Transmit Status of the\n\ntransmitted frames are accepted by the host. In order to\n\ncomplete this flush operation, the PHY transmit clock (clk_tx_i) is\n\nrequired to be active."]
pub type FtfR = crate::BitReader;
#[doc = "Field `FTF` writer - Flush Transmit FIFO\n\nWhen this bit is set, the transmit FIFO controller logic is reset to\n\nits default values and thus all data in the Tx FIFO is lost/flushed.\n\nThis bit is cleared internally when the flushing operation is\n\ncompleted fully. The Operation Mode register should not be\n\nwritten to until this bit is cleared. The data which is already\n\naccepted by the MAC transmitter will not be flushed. It will be\n\nscheduled for transmission and will result in underflow and runt\n\nframe transmission.\n\nNote: The flush operation completes only after emptying the\n\nTxFIFO of its contents and all the pending Transmit Status of the\n\ntransmitted frames are accepted by the host. In order to\n\ncomplete this flush operation, the PHY transmit clock (clk_tx_i) is\n\nrequired to be active."]
pub type FtfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TSF` reader - Transmit Store and Forward\n\nWhen this bit is set, transmission starts when a full frame resides\n\nin the MTL Transmit FIFO. When this bit is set, the TTC values\n\nspecified in Register GMAC_OP_MODE\\[16:14\\]
are ignored. This\n\nbit should be changed only when transmission is stopped."]
pub type TsfR = crate::BitReader;
#[doc = "Field `TSF` writer - Transmit Store and Forward\n\nWhen this bit is set, transmission starts when a full frame resides\n\nin the MTL Transmit FIFO. When this bit is set, the TTC values\n\nspecified in Register GMAC_OP_MODE\\[16:14\\]
are ignored. This\n\nbit should be changed only when transmission is stopped."]
pub type TsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFF` reader - Disable Flushing of Received Frames\n\nWhen this bit is set, the RxDMA does not flush any frames due to\n\nthe unavailability of receive descriptors/buffers as it does\n\nnormally when this bit is reset."]
pub type DffR = crate::BitReader;
#[doc = "Field `DFF` writer - Disable Flushing of Received Frames\n\nWhen this bit is set, the RxDMA does not flush any frames due to\n\nthe unavailability of receive descriptors/buffers as it does\n\nnormally when this bit is reset."]
pub type DffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSF` reader - Receive Store and Forward\n\nWhen this bit is set, the MTL only reads a frame from the Rx FIFO\n\nafter the complete frame has been written to it, ignoring RTC\n\nbits. When this bit is reset, the Rx FIFO operates in Cut-Through\n\nmode, subject to the threshold specified by the RTC bits."]
pub type RsfR = crate::BitReader;
#[doc = "Field `RSF` writer - Receive Store and Forward\n\nWhen this bit is set, the MTL only reads a frame from the Rx FIFO\n\nafter the complete frame has been written to it, ignoring RTC\n\nbits. When this bit is reset, the Rx FIFO operates in Cut-Through\n\nmode, subject to the threshold specified by the RTC bits."]
pub type RsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DT` reader - Disable Dropping of TCP/IP Checksum Error Frames\n\nWhen this bit is set, the core does not drop frames that only have\n\nerrors detected by the Receive Checksum Offload engine. Such\n\nframes do not have any errors (including FCS error) in the\n\nEthernet frame received by the MAC but have errors in the\n\nencapsulated payload only. When this bit is reset, all error frames\n\nare dropped if the FEF bit is reset."]
pub type DtR = crate::BitReader;
#[doc = "Field `DT` writer - Disable Dropping of TCP/IP Checksum Error Frames\n\nWhen this bit is set, the core does not drop frames that only have\n\nerrors detected by the Receive Checksum Offload engine. Such\n\nframes do not have any errors (including FCS error) in the\n\nEthernet frame received by the MAC but have errors in the\n\nencapsulated payload only. When this bit is reset, all error frames\n\nare dropped if the FEF bit is reset."]
pub type DtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Start/Stop Receive\n\nWhen this bit is set, the Receive process is placed in the Running\n\nstate. The DMA attempts to acquire the descriptor from the\n\nReceive list and processes incoming frames. Descriptor\n\nacquisition is attempted from the current position in the list,\n\nwhich is the address set by register GMAC_RX_DESC_LIST_ADDR\n\nor the position retained when the Receive process was previously\n\nstopped. If no descriptor is owned by the DMA, reception is\n\nsuspended and Receive Buffer Unavailable (Register\n\nGMAC_STATUS\\[7\\]) is set. The Start Receive command is effective\n\nonly when reception has stopped. If the command was issued\n\nbefore setting register GMAC_RX_DESC_LIST_ADDR, DMA\n\nbehavior is unpredictable.\n\nWhen this bit is cleared, RxDMA operation is stopped after the\n\ntransfer of the current frame. The next descriptor position in the\n\nReceive list is saved and becomes the current position after the\n\nReceive process is restarted. The Stop Receive command is\n\neffective only when the Receive process is in either the Running\n\n(waiting for receive packet) or in the Suspended state."]
    #[inline(always)]
    pub fn sr(&self) -> SrR {
        SrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Operate on Second Frame\n\nWhen this bit is set, this bit instructs the DMA to process a\n\nsecond frame of Transmit data even before status for first frame\n\nis obtained."]
    #[inline(always)]
    pub fn osf(&self) -> OsfR {
        OsfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Receive Threshold Control\n\nThese two bits control the threshold level of the MTL Receive\n\nFIFO. Transfer (request) to DMA starts when the frame size\n\nwithin the MTL Receive FIFO is larger than the threshold. In\n\naddition, full frames with a length less than the threshold are\n\ntransferred automatically. Note that value of 11 is not applicable\n\nif the configured Receive FIFO size is 128 bytes. These bits are\n\nvalid only when the RSF bit is zero, and are ignored when the\n\nRSF bit is set to 1."]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 6 - Forward Undersized Good Frames\n\nWhen set, the Rx FIFO will forward Undersized frames (frames\n\nwith no Error and length less than 64 bytes) including pad-bytes\n\nand CRC).\n\nWhen reset, the Rx FIFO will drop all frames of less than 64\n\nbytes, unless it is already transferred due to lower value of\n\nReceive Threshold (e.g., RTC = 01)."]
    #[inline(always)]
    pub fn fuf(&self) -> FufR {
        FufR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Forward Error Frames\n\nWhen this bit is reset, the Rx FIFO drops frames with error status\n\n(CRC error, collision error, GMII_ER, giant frame, watchdog\n\ntimeout, overflow). However, if the frame's start byte (write)\n\npointer is already transferred to the read controller side (in\n\nThreshold mode), then the frames are not dropped.\n\nWhen FEF is set, all frames except runt error frames are\n\nforwarded to the DMA. But when RxFIFO overflows when a partial\n\nframe is written, then such frames are dropped even when FEF is\n\nset."]
    #[inline(always)]
    pub fn fef(&self) -> FefR {
        FefR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable HW flow control\n\nWhen this bit is set, the flow control signal operation based on\n\nfill-level of Rx FIFO is enabled. When reset, the flow control\n\noperation is disabled."]
    #[inline(always)]
    pub fn efc(&self) -> EfcR {
        EfcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Threshold for activating flow control (in both HD and FD)\n\nThese bits control the threshold (Fill level of Rx FIFO) at which\n\nflow control is activated."]
    #[inline(always)]
    pub fn rfa(&self) -> RfaR {
        RfaR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - Threshold for deactivating flow control (in both HD and FD)\n\nThese bits control the threshold (Fill-level of Rx FIFO) at which\n\nthe flow-control is de-asserted after activation."]
    #[inline(always)]
    pub fn rfd(&self) -> RfdR {
        RfdR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Start/Stop Transmission Command\n\nWhen this bit is set, transmission is placed in the Running state,\n\nand the DMA checks the Transmit List at the current position for a\n\nframe to be transmitted. Descriptor acquisition is attempted\n\neither from the current position in the list, which is the Transmit\n\nList Base Address set by Register GMAC_TX_DESC_LIST_ADDR,\n\nor from the position retained when transmission was stopped\n\npreviously. If the current descriptor is not owned by the DMA,\n\ntransmission enters the Suspended state and Transmit Buffer\n\nUnavailable (Register GMAC_STATUS\\[2\\]) is set. The Start\n\nTransmission command is effective only when transmission is\n\nstopped. If the command is issued before setting DMA Register\n\nTX_DESC_LIST_ADDR, then the DMA behavior is unpredictable.\n\nWhen this bit is reset, the transmission process is placed in the\n\nStopped state after completing the transmission of the current\n\nframe. The Next Descriptor position in the Transmit List is saved,\n\nand becomes the current position when transmission is restarted.\n\nThe stop transmission command is effective only the transmission\n\nof the current frame is complete or when the transmission is in\n\nthe Suspended state."]
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16 - Transmit Threshold Control\n\nThese three bits control the threshold level of the MTL Transmit\n\nFIFO. Transmission starts when the frame size within the MTL\n\nTransmit FIFO is larger than the threshold. In addition, full\n\nframes with a length less than the threshold are also transmitted.\n\nThese bits are used only when the TSF bit (Bit 21) is reset."]
    #[inline(always)]
    pub fn ttc(&self) -> TtcR {
        TtcR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 20 - Flush Transmit FIFO\n\nWhen this bit is set, the transmit FIFO controller logic is reset to\n\nits default values and thus all data in the Tx FIFO is lost/flushed.\n\nThis bit is cleared internally when the flushing operation is\n\ncompleted fully. The Operation Mode register should not be\n\nwritten to until this bit is cleared. The data which is already\n\naccepted by the MAC transmitter will not be flushed. It will be\n\nscheduled for transmission and will result in underflow and runt\n\nframe transmission.\n\nNote: The flush operation completes only after emptying the\n\nTxFIFO of its contents and all the pending Transmit Status of the\n\ntransmitted frames are accepted by the host. In order to\n\ncomplete this flush operation, the PHY transmit clock (clk_tx_i) is\n\nrequired to be active."]
    #[inline(always)]
    pub fn ftf(&self) -> FtfR {
        FtfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit Store and Forward\n\nWhen this bit is set, transmission starts when a full frame resides\n\nin the MTL Transmit FIFO. When this bit is set, the TTC values\n\nspecified in Register GMAC_OP_MODE\\[16:14\\]
are ignored. This\n\nbit should be changed only when transmission is stopped."]
    #[inline(always)]
    pub fn tsf(&self) -> TsfR {
        TsfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Disable Flushing of Received Frames\n\nWhen this bit is set, the RxDMA does not flush any frames due to\n\nthe unavailability of receive descriptors/buffers as it does\n\nnormally when this bit is reset."]
    #[inline(always)]
    pub fn dff(&self) -> DffR {
        DffR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Receive Store and Forward\n\nWhen this bit is set, the MTL only reads a frame from the Rx FIFO\n\nafter the complete frame has been written to it, ignoring RTC\n\nbits. When this bit is reset, the Rx FIFO operates in Cut-Through\n\nmode, subject to the threshold specified by the RTC bits."]
    #[inline(always)]
    pub fn rsf(&self) -> RsfR {
        RsfR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Disable Dropping of TCP/IP Checksum Error Frames\n\nWhen this bit is set, the core does not drop frames that only have\n\nerrors detected by the Receive Checksum Offload engine. Such\n\nframes do not have any errors (including FCS error) in the\n\nEthernet frame received by the MAC but have errors in the\n\nencapsulated payload only. When this bit is reset, all error frames\n\nare dropped if the FEF bit is reset."]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Start/Stop Receive\n\nWhen this bit is set, the Receive process is placed in the Running\n\nstate. The DMA attempts to acquire the descriptor from the\n\nReceive list and processes incoming frames. Descriptor\n\nacquisition is attempted from the current position in the list,\n\nwhich is the address set by register GMAC_RX_DESC_LIST_ADDR\n\nor the position retained when the Receive process was previously\n\nstopped. If no descriptor is owned by the DMA, reception is\n\nsuspended and Receive Buffer Unavailable (Register\n\nGMAC_STATUS\\[7\\]) is set. The Start Receive command is effective\n\nonly when reception has stopped. If the command was issued\n\nbefore setting register GMAC_RX_DESC_LIST_ADDR, DMA\n\nbehavior is unpredictable.\n\nWhen this bit is cleared, RxDMA operation is stopped after the\n\ntransfer of the current frame. The next descriptor position in the\n\nReceive list is saved and becomes the current position after the\n\nReceive process is restarted. The Stop Receive command is\n\neffective only when the Receive process is in either the Running\n\n(waiting for receive packet) or in the Suspended state."]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SrW<GmacOpModeSpec> {
        SrW::new(self, 1)
    }
    #[doc = "Bit 2 - Operate on Second Frame\n\nWhen this bit is set, this bit instructs the DMA to process a\n\nsecond frame of Transmit data even before status for first frame\n\nis obtained."]
    #[inline(always)]
    #[must_use]
    pub fn osf(&mut self) -> OsfW<GmacOpModeSpec> {
        OsfW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Receive Threshold Control\n\nThese two bits control the threshold level of the MTL Receive\n\nFIFO. Transfer (request) to DMA starts when the frame size\n\nwithin the MTL Receive FIFO is larger than the threshold. In\n\naddition, full frames with a length less than the threshold are\n\ntransferred automatically. Note that value of 11 is not applicable\n\nif the configured Receive FIFO size is 128 bytes. These bits are\n\nvalid only when the RSF bit is zero, and are ignored when the\n\nRSF bit is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RtcW<GmacOpModeSpec> {
        RtcW::new(self, 3)
    }
    #[doc = "Bit 6 - Forward Undersized Good Frames\n\nWhen set, the Rx FIFO will forward Undersized frames (frames\n\nwith no Error and length less than 64 bytes) including pad-bytes\n\nand CRC).\n\nWhen reset, the Rx FIFO will drop all frames of less than 64\n\nbytes, unless it is already transferred due to lower value of\n\nReceive Threshold (e.g., RTC = 01)."]
    #[inline(always)]
    #[must_use]
    pub fn fuf(&mut self) -> FufW<GmacOpModeSpec> {
        FufW::new(self, 6)
    }
    #[doc = "Bit 7 - Forward Error Frames\n\nWhen this bit is reset, the Rx FIFO drops frames with error status\n\n(CRC error, collision error, GMII_ER, giant frame, watchdog\n\ntimeout, overflow). However, if the frame's start byte (write)\n\npointer is already transferred to the read controller side (in\n\nThreshold mode), then the frames are not dropped.\n\nWhen FEF is set, all frames except runt error frames are\n\nforwarded to the DMA. But when RxFIFO overflows when a partial\n\nframe is written, then such frames are dropped even when FEF is\n\nset."]
    #[inline(always)]
    #[must_use]
    pub fn fef(&mut self) -> FefW<GmacOpModeSpec> {
        FefW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable HW flow control\n\nWhen this bit is set, the flow control signal operation based on\n\nfill-level of Rx FIFO is enabled. When reset, the flow control\n\noperation is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn efc(&mut self) -> EfcW<GmacOpModeSpec> {
        EfcW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Threshold for activating flow control (in both HD and FD)\n\nThese bits control the threshold (Fill level of Rx FIFO) at which\n\nflow control is activated."]
    #[inline(always)]
    #[must_use]
    pub fn rfa(&mut self) -> RfaW<GmacOpModeSpec> {
        RfaW::new(self, 9)
    }
    #[doc = "Bits 11:12 - Threshold for deactivating flow control (in both HD and FD)\n\nThese bits control the threshold (Fill-level of Rx FIFO) at which\n\nthe flow-control is de-asserted after activation."]
    #[inline(always)]
    #[must_use]
    pub fn rfd(&mut self) -> RfdW<GmacOpModeSpec> {
        RfdW::new(self, 11)
    }
    #[doc = "Bit 13 - Start/Stop Transmission Command\n\nWhen this bit is set, transmission is placed in the Running state,\n\nand the DMA checks the Transmit List at the current position for a\n\nframe to be transmitted. Descriptor acquisition is attempted\n\neither from the current position in the list, which is the Transmit\n\nList Base Address set by Register GMAC_TX_DESC_LIST_ADDR,\n\nor from the position retained when transmission was stopped\n\npreviously. If the current descriptor is not owned by the DMA,\n\ntransmission enters the Suspended state and Transmit Buffer\n\nUnavailable (Register GMAC_STATUS\\[2\\]) is set. The Start\n\nTransmission command is effective only when transmission is\n\nstopped. If the command is issued before setting DMA Register\n\nTX_DESC_LIST_ADDR, then the DMA behavior is unpredictable.\n\nWhen this bit is reset, the transmission process is placed in the\n\nStopped state after completing the transmission of the current\n\nframe. The Next Descriptor position in the Transmit List is saved,\n\nand becomes the current position when transmission is restarted.\n\nThe stop transmission command is effective only the transmission\n\nof the current frame is complete or when the transmission is in\n\nthe Suspended state."]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> StW<GmacOpModeSpec> {
        StW::new(self, 13)
    }
    #[doc = "Bits 14:16 - Transmit Threshold Control\n\nThese three bits control the threshold level of the MTL Transmit\n\nFIFO. Transmission starts when the frame size within the MTL\n\nTransmit FIFO is larger than the threshold. In addition, full\n\nframes with a length less than the threshold are also transmitted.\n\nThese bits are used only when the TSF bit (Bit 21) is reset."]
    #[inline(always)]
    #[must_use]
    pub fn ttc(&mut self) -> TtcW<GmacOpModeSpec> {
        TtcW::new(self, 14)
    }
    #[doc = "Bit 20 - Flush Transmit FIFO\n\nWhen this bit is set, the transmit FIFO controller logic is reset to\n\nits default values and thus all data in the Tx FIFO is lost/flushed.\n\nThis bit is cleared internally when the flushing operation is\n\ncompleted fully. The Operation Mode register should not be\n\nwritten to until this bit is cleared. The data which is already\n\naccepted by the MAC transmitter will not be flushed. It will be\n\nscheduled for transmission and will result in underflow and runt\n\nframe transmission.\n\nNote: The flush operation completes only after emptying the\n\nTxFIFO of its contents and all the pending Transmit Status of the\n\ntransmitted frames are accepted by the host. In order to\n\ncomplete this flush operation, the PHY transmit clock (clk_tx_i) is\n\nrequired to be active."]
    #[inline(always)]
    #[must_use]
    pub fn ftf(&mut self) -> FtfW<GmacOpModeSpec> {
        FtfW::new(self, 20)
    }
    #[doc = "Bit 21 - Transmit Store and Forward\n\nWhen this bit is set, transmission starts when a full frame resides\n\nin the MTL Transmit FIFO. When this bit is set, the TTC values\n\nspecified in Register GMAC_OP_MODE\\[16:14\\]
are ignored. This\n\nbit should be changed only when transmission is stopped."]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TsfW<GmacOpModeSpec> {
        TsfW::new(self, 21)
    }
    #[doc = "Bit 24 - Disable Flushing of Received Frames\n\nWhen this bit is set, the RxDMA does not flush any frames due to\n\nthe unavailability of receive descriptors/buffers as it does\n\nnormally when this bit is reset."]
    #[inline(always)]
    #[must_use]
    pub fn dff(&mut self) -> DffW<GmacOpModeSpec> {
        DffW::new(self, 24)
    }
    #[doc = "Bit 25 - Receive Store and Forward\n\nWhen this bit is set, the MTL only reads a frame from the Rx FIFO\n\nafter the complete frame has been written to it, ignoring RTC\n\nbits. When this bit is reset, the Rx FIFO operates in Cut-Through\n\nmode, subject to the threshold specified by the RTC bits."]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RsfW<GmacOpModeSpec> {
        RsfW::new(self, 25)
    }
    #[doc = "Bit 26 - Disable Dropping of TCP/IP Checksum Error Frames\n\nWhen this bit is set, the core does not drop frames that only have\n\nerrors detected by the Receive Checksum Offload engine. Such\n\nframes do not have any errors (including FCS error) in the\n\nEthernet frame received by the MAC but have errors in the\n\nencapsulated payload only. When this bit is reset, all error frames\n\nare dropped if the FEF bit is reset."]
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
