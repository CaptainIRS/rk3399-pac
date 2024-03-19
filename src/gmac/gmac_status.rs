#[doc = "Register `GMAC_STATUS` reader"]
pub type R = crate::R<GmacStatusSpec>;
#[doc = "Register `GMAC_STATUS` writer"]
pub type W = crate::W<GmacStatusSpec>;
#[doc = "Field `TI` reader - Transmit Interrupt\n\nThis bit indicates that frame transmission is finished and\n\nTDES1\\[31\\]
is set in the First Descriptor."]
pub type TiR = crate::BitReader;
#[doc = "Field `TI` writer - Transmit Interrupt\n\nThis bit indicates that frame transmission is finished and\n\nTDES1\\[31\\]
is set in the First Descriptor."]
pub type TiW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TPS` reader - Transmit Process Stopped\n\nThis bit is set when the transmission is stopped."]
pub type TpsR = crate::BitReader;
#[doc = "Field `TPS` writer - Transmit Process Stopped\n\nThis bit is set when the transmission is stopped."]
pub type TpsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TU` reader - Transmit Buffer Unavailable\n\nThis bit indicates that the Next Descriptor in the Transmit List is\n\nowned by the host and cannot be acquired by the DMA.\n\nTransmission is suspended. Bits\\[22:20\\]
explain the Transmit\n\nProcess state transitions. To resume processing transmit\n\ndescriptors, the host should change the ownership of the bit of\n\nthe descriptor and then issue a Transmit Poll Demand command."]
pub type TuR = crate::BitReader;
#[doc = "Field `TU` writer - Transmit Buffer Unavailable\n\nThis bit indicates that the Next Descriptor in the Transmit List is\n\nowned by the host and cannot be acquired by the DMA.\n\nTransmission is suspended. Bits\\[22:20\\]
explain the Transmit\n\nProcess state transitions. To resume processing transmit\n\ndescriptors, the host should change the ownership of the bit of\n\nthe descriptor and then issue a Transmit Poll Demand command."]
pub type TuW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TJT` reader - Transmit Jabber Timeout\n\nThis bit indicates that the Transmit Jabber Timer expired,\n\nmeaning that the transmitter had been excessively active. The\n\ntransmission process is aborted and placed in the Stopped state.\n\nThis causes the Transmit Jabber Timeout TDES0\\[14\\]
flag to\n\nassert."]
pub type TjtR = crate::BitReader;
#[doc = "Field `TJT` writer - Transmit Jabber Timeout\n\nThis bit indicates that the Transmit Jabber Timer expired,\n\nmeaning that the transmitter had been excessively active. The\n\ntransmission process is aborted and placed in the Stopped state.\n\nThis causes the Transmit Jabber Timeout TDES0\\[14\\]
flag to\n\nassert."]
pub type TjtW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OVF` reader - Receive Overflow\n\nThis bit indicates that the Receive Buffer had an Overflow during\n\nframe reception. If the partial frame is transferred to application,\n\nthe overflow status is set in RDES0\\[11\\]."]
pub type OvfR = crate::BitReader;
#[doc = "Field `OVF` writer - Receive Overflow\n\nThis bit indicates that the Receive Buffer had an Overflow during\n\nframe reception. If the partial frame is transferred to application,\n\nthe overflow status is set in RDES0\\[11\\]."]
pub type OvfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `UNF` reader - Transmit Underflow\n\nThis bit indicates that the Transmit Buffer had an Underflow\n\nduring frame transmission. Transmission is suspended and an\n\nUnderflow Error TDES0\\[1\\]
is set."]
pub type UnfR = crate::BitReader;
#[doc = "Field `UNF` writer - Transmit Underflow\n\nThis bit indicates that the Transmit Buffer had an Underflow\n\nduring frame transmission. Transmission is suspended and an\n\nUnderflow Error TDES0\\[1\\]
is set."]
pub type UnfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RI` reader - Receive Interrupt\n\nThis bit indicates the completion of frame reception. Specific\n\nframe status information has been posted in the descriptor.\n\nReception remains in the Running state."]
pub type RiR = crate::BitReader;
#[doc = "Field `RI` writer - Receive Interrupt\n\nThis bit indicates the completion of frame reception. Specific\n\nframe status information has been posted in the descriptor.\n\nReception remains in the Running state."]
pub type RiW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RU` reader - Receive Buffer Unavailable\n\nThis bit indicates that the Next Descriptor in the Receive List is\n\nowned by the host and cannot be acquired by the DMA. Receive\n\nProcess is suspended. To resume processing Receive descriptors,\n\nthe host should change the ownership of the descriptor and issue\n\na Receive Poll Demand command. If no Receive Poll Demand is\n\nissued, Receive Process resumes when the next recognized\n\nincoming frame is received. Register GMAC_STATUS\\[7\\]
is set\n\nonly when the previous Receive Descriptor was owned by the\n\nDMA."]
pub type RuR = crate::BitReader;
#[doc = "Field `RU` writer - Receive Buffer Unavailable\n\nThis bit indicates that the Next Descriptor in the Receive List is\n\nowned by the host and cannot be acquired by the DMA. Receive\n\nProcess is suspended. To resume processing Receive descriptors,\n\nthe host should change the ownership of the descriptor and issue\n\na Receive Poll Demand command. If no Receive Poll Demand is\n\nissued, Receive Process resumes when the next recognized\n\nincoming frame is received. Register GMAC_STATUS\\[7\\]
is set\n\nonly when the previous Receive Descriptor was owned by the\n\nDMA."]
pub type RuW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RPS` reader - Receive Process Stopped\n\nThis bit is asserted when the Receive Process enters the Stopped\n\nstate."]
pub type RpsR = crate::BitReader;
#[doc = "Field `RPS` writer - Receive Process Stopped\n\nThis bit is asserted when the Receive Process enters the Stopped\n\nstate."]
pub type RpsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RWT` reader - Receive Watchdog Timeout\n\nThis bit is asserted when a frame with a length greater than\n\n2,048 bytes is received."]
pub type RwtR = crate::BitReader;
#[doc = "Field `RWT` writer - Receive Watchdog Timeout\n\nThis bit is asserted when a frame with a length greater than\n\n2,048 bytes is received."]
pub type RwtW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ETI` reader - Early Transmit Interrupt\n\nThis bit indicates that the frame to be transmitted was fully\n\ntransferred to the MTL Transmit FIFO."]
pub type EtiR = crate::BitReader;
#[doc = "Field `ETI` writer - Early Transmit Interrupt\n\nThis bit indicates that the frame to be transmitted was fully\n\ntransferred to the MTL Transmit FIFO."]
pub type EtiW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FBI` reader - Fatal Bus Error Interrupt\n\nThis bit indicates that a bus error occurred, as detailed in\n\n\\[25:23\\]. When this bit is set, the corresponding DMA engine\n\ndisables all its bus accesses."]
pub type FbiR = crate::BitReader;
#[doc = "Field `FBI` writer - Fatal Bus Error Interrupt\n\nThis bit indicates that a bus error occurred, as detailed in\n\n\\[25:23\\]. When this bit is set, the corresponding DMA engine\n\ndisables all its bus accesses."]
pub type FbiW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ERI` reader - Early Receive Interrupt\n\nThis bit indicates that the DMA had filled the first data buffer of\n\nthe packet. Receive Interrupt Register GMAC_STATUS\\[6\\]\n\nautomatically clears this bit."]
pub type EriR = crate::BitReader;
#[doc = "Field `ERI` writer - Early Receive Interrupt\n\nThis bit indicates that the DMA had filled the first data buffer of\n\nthe packet. Receive Interrupt Register GMAC_STATUS\\[6\\]\n\nautomatically clears this bit."]
pub type EriW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary\n\nAbnormal Interrupt Summary bit value is the logical OR of the\n\nfollowing when the corresponding interrupt bits are enabled in\n\nRegister OP_MODE:\n\nRegister GMAC_STATUS\\[1\\]: Transmit Process Stopped\n\nRegister GMAC_STATUS\\[3\\]: Transmit Jabber Timeout\n\nRegister GMAC_STATUS\\[4\\]: Receive FIFO Overflow\n\nRegister GMAC_STATUS\\[5\\]: Transmit Underflow\n\nRegister GMAC_STATUS\\[7\\]: Receive Buffer Unavailable\n\nRegister GMAC_STATUS\\[8\\]: Receive Process Stopped\n\nRegister GMAC_STATUS\\[9\\]: Receive Watchdog Timeout\n\nRegister GMAC_STATUS\\[10\\]: Early Transmit Interrupt\n\nRegister GMAC_STATUS\\[13\\]: Fatal Bus Error\n\nOnly unmasked bits affect the Abnormal Interrupt Summary bit.\n\nThis is a sticky bit and must be cleared each time a\n\ncorresponding bit that causes AIS to be set is cleared."]
pub type AisR = crate::BitReader;
#[doc = "Field `AIS` writer - Abnormal Interrupt Summary\n\nAbnormal Interrupt Summary bit value is the logical OR of the\n\nfollowing when the corresponding interrupt bits are enabled in\n\nRegister OP_MODE:\n\nRegister GMAC_STATUS\\[1\\]: Transmit Process Stopped\n\nRegister GMAC_STATUS\\[3\\]: Transmit Jabber Timeout\n\nRegister GMAC_STATUS\\[4\\]: Receive FIFO Overflow\n\nRegister GMAC_STATUS\\[5\\]: Transmit Underflow\n\nRegister GMAC_STATUS\\[7\\]: Receive Buffer Unavailable\n\nRegister GMAC_STATUS\\[8\\]: Receive Process Stopped\n\nRegister GMAC_STATUS\\[9\\]: Receive Watchdog Timeout\n\nRegister GMAC_STATUS\\[10\\]: Early Transmit Interrupt\n\nRegister GMAC_STATUS\\[13\\]: Fatal Bus Error\n\nOnly unmasked bits affect the Abnormal Interrupt Summary bit.\n\nThis is a sticky bit and must be cleared each time a\n\ncorresponding bit that causes AIS to be set is cleared."]
pub type AisW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `NIS` reader - Normal Interrupt Summary\n\nNormal Interrupt Summary bit value is the logical OR of the\n\nfollowing when the corresponding interrupt bits are enabled in\n\nRegister OP_MODE:\n\nRegister GMAC_STATUS\\[0\\]: Transmit Interrupt\n\nRegister GMAC_STATUS\\[2\\]: Transmit Buffer Unavailable\n\nRegister GMAC_STATUS\\[6\\]: Receive Interrupt\n\nRegister GMAC_STATUS\\[14\\]: Early Receive Interrupt\n\nOnly unmasked bits affect the Normal Interrupt Summary bit.\n\nThis is a sticky bit and must be cleared (by writing a 1 to this bit)\n\neach time a corresponding bit that causes NIS to be set is\n\ncleared."]
pub type NisR = crate::BitReader;
#[doc = "Field `NIS` writer - Normal Interrupt Summary\n\nNormal Interrupt Summary bit value is the logical OR of the\n\nfollowing when the corresponding interrupt bits are enabled in\n\nRegister OP_MODE:\n\nRegister GMAC_STATUS\\[0\\]: Transmit Interrupt\n\nRegister GMAC_STATUS\\[2\\]: Transmit Buffer Unavailable\n\nRegister GMAC_STATUS\\[6\\]: Receive Interrupt\n\nRegister GMAC_STATUS\\[14\\]: Early Receive Interrupt\n\nOnly unmasked bits affect the Normal Interrupt Summary bit.\n\nThis is a sticky bit and must be cleared (by writing a 1 to this bit)\n\neach time a corresponding bit that causes NIS to be set is\n\ncleared."]
pub type NisW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Receive Process State\n\nThese bits indicate the Receive DMA FSM state. This field does\n\nnot generate an interrupt.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rs {
    #[doc = "0: Stopped: Reset or Stop Receive Command issued."]
    B000 = 0,
    #[doc = "1: Running: Fetching Receive Transfer Descriptor."]
    B001 = 1,
    #[doc = "2: Reserved for future use."]
    B010 = 2,
    #[doc = "3: Running: Waiting for receive packet."]
    B011 = 3,
    #[doc = "4: Suspended: Receive Descriptor Unavailable."]
    B100 = 4,
    #[doc = "5: Running: Closing Receive Descriptor."]
    B101 = 5,
    #[doc = "6: TIME_STAMP write state."]
    B110 = 6,
    #[doc = "7: Running: Transferring the receive packet data from receive buffer to host memory."]
    B111 = 7,
}
impl From<Rs> for u8 {
    #[inline(always)]
    fn from(variant: Rs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rs {
    type Ux = u8;
}
#[doc = "Field `RS` reader - Receive Process State\n\nThese bits indicate the Receive DMA FSM state. This field does\n\nnot generate an interrupt."]
pub type RsR = crate::FieldReader<Rs>;
impl RsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rs {
        match self.bits {
            0 => Rs::B000,
            1 => Rs::B001,
            2 => Rs::B010,
            3 => Rs::B011,
            4 => Rs::B100,
            5 => Rs::B101,
            6 => Rs::B110,
            7 => Rs::B111,
            _ => unreachable!(),
        }
    }
    #[doc = "Stopped: Reset or Stop Receive Command issued."]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Rs::B000
    }
    #[doc = "Running: Fetching Receive Transfer Descriptor."]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Rs::B001
    }
    #[doc = "Reserved for future use."]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Rs::B010
    }
    #[doc = "Running: Waiting for receive packet."]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Rs::B011
    }
    #[doc = "Suspended: Receive Descriptor Unavailable."]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Rs::B100
    }
    #[doc = "Running: Closing Receive Descriptor."]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == Rs::B101
    }
    #[doc = "TIME_STAMP write state."]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == Rs::B110
    }
    #[doc = "Running: Transferring the receive packet data from receive buffer to host memory."]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == Rs::B111
    }
}
#[doc = "Transmit Process State\n\nThese bits indicate the Transmit DMA FSM state. This field does\n\nnot generate an interrupt.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ts {
    #[doc = "0: Stopped; Reset or Stop Transmit Command issued."]
    B000 = 0,
    #[doc = "1: Running; Fetching Transmit Transfer Descriptor."]
    B001 = 1,
    #[doc = "2: Running; Waiting for status."]
    B010 = 2,
    #[doc = "3: Running; Reading Data from host memory buffer and queuing it to transmit buffer (Tx FIFO)."]
    B011 = 3,
    #[doc = "4: TIME_STAMP write state."]
    B100 = 4,
    #[doc = "5: Reserved for future use."]
    B101 = 5,
    #[doc = "6: Suspended; Transmit Descriptor Unavailable or Transmit Buffer Underflow."]
    B110 = 6,
    #[doc = "7: Running; Closing Transmit Descriptor."]
    B111 = 7,
}
impl From<Ts> for u8 {
    #[inline(always)]
    fn from(variant: Ts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ts {
    type Ux = u8;
}
#[doc = "Field `TS` reader - Transmit Process State\n\nThese bits indicate the Transmit DMA FSM state. This field does\n\nnot generate an interrupt."]
pub type TsR = crate::FieldReader<Ts>;
impl TsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ts {
        match self.bits {
            0 => Ts::B000,
            1 => Ts::B001,
            2 => Ts::B010,
            3 => Ts::B011,
            4 => Ts::B100,
            5 => Ts::B101,
            6 => Ts::B110,
            7 => Ts::B111,
            _ => unreachable!(),
        }
    }
    #[doc = "Stopped; Reset or Stop Transmit Command issued."]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Ts::B000
    }
    #[doc = "Running; Fetching Transmit Transfer Descriptor."]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Ts::B001
    }
    #[doc = "Running; Waiting for status."]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Ts::B010
    }
    #[doc = "Running; Reading Data from host memory buffer and queuing it to transmit buffer (Tx FIFO)."]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Ts::B011
    }
    #[doc = "TIME_STAMP write state."]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Ts::B100
    }
    #[doc = "Reserved for future use."]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == Ts::B101
    }
    #[doc = "Suspended; Transmit Descriptor Unavailable or Transmit Buffer Underflow."]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == Ts::B110
    }
    #[doc = "Running; Closing Transmit Descriptor."]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == Ts::B111
    }
}
#[doc = "Field `EB` reader - Error Bits\n\nThese bits indicate the type of error that caused a Bus Error\n\n(e.g., error response on the AXI interface). Valid only with Fatal\n\nBus Error bit (Register GMAC_STATUS\\[13\\]) set. This field does\n\nnot generate an interrupt.\n\nBit 23: 1'b1 Error during data transfer by TxDMA\n\n1'b0 Error during data transfer by RxDMA\n\nBit 24: 1'b1 Error during read transfer\n\n1'b0 Error during write transfer\n\nBit 25: 1'b1 Error during descriptor access\n\n1'b0 Error during data buffer access"]
pub type EbR = crate::FieldReader;
#[doc = "Field `GLI` reader - GMAC Line interface Interrupt\n\nThis bit reflects an interrupt event in the GMAC Core's PCS or\n\nRGMII interface block. The software must read the corresponding\n\nregisters in the GMAC core to get the exact cause of interrupt and\n\nclear the source of interrupt to make this bit as 1'b0. The\n\ninterrupt signal from the GMAC subsystem (sbd_intr_o) is high\n\nwhen this bit is high."]
pub type GliR = crate::BitReader;
#[doc = "Field `GMI` reader - GMAC MMC Interrupt\n\nThis bit reflects an interrupt event in the MMC module of the\n\nGMAC core. The software must read the corresponding registers\n\nin the GMAC core to get the exact cause of interrupt and clear the\n\nsource of interrupt to make this bit as 1'b0. The interrupt signal\n\nfrom the GMAC subsystem (sbd_intr_o) is high when this bit is\n\nhigh."]
pub type GmiR = crate::BitReader;
#[doc = "Field `GPI` reader - GMAC PMT Interrupt\n\nThis bit indicates an interrupt event in the GMAC core's PMT\n\nmodule. The software must read the corresponding registers in\n\nthe GMAC core to get the exact cause of interrupt and clear its\n\nsource to reset this bit to 1'b0. The interrupt signal from the\n\nGMAC subsystem (sbd_intr_o) is high when this bit is high."]
pub type GpiR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt\n\nThis bit indicates that frame transmission is finished and\n\nTDES1\\[31\\]
is set in the First Descriptor."]
    #[inline(always)]
    pub fn ti(&self) -> TiR {
        TiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped\n\nThis bit is set when the transmission is stopped."]
    #[inline(always)]
    pub fn tps(&self) -> TpsR {
        TpsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable\n\nThis bit indicates that the Next Descriptor in the Transmit List is\n\nowned by the host and cannot be acquired by the DMA.\n\nTransmission is suspended. Bits\\[22:20\\]
explain the Transmit\n\nProcess state transitions. To resume processing transmit\n\ndescriptors, the host should change the ownership of the bit of\n\nthe descriptor and then issue a Transmit Poll Demand command."]
    #[inline(always)]
    pub fn tu(&self) -> TuR {
        TuR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout\n\nThis bit indicates that the Transmit Jabber Timer expired,\n\nmeaning that the transmitter had been excessively active. The\n\ntransmission process is aborted and placed in the Stopped state.\n\nThis causes the Transmit Jabber Timeout TDES0\\[14\\]
flag to\n\nassert."]
    #[inline(always)]
    pub fn tjt(&self) -> TjtR {
        TjtR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Overflow\n\nThis bit indicates that the Receive Buffer had an Overflow during\n\nframe reception. If the partial frame is transferred to application,\n\nthe overflow status is set in RDES0\\[11\\]."]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Underflow\n\nThis bit indicates that the Transmit Buffer had an Underflow\n\nduring frame transmission. Transmission is suspended and an\n\nUnderflow Error TDES0\\[1\\]
is set."]
    #[inline(always)]
    pub fn unf(&self) -> UnfR {
        UnfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt\n\nThis bit indicates the completion of frame reception. Specific\n\nframe status information has been posted in the descriptor.\n\nReception remains in the Running state."]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable\n\nThis bit indicates that the Next Descriptor in the Receive List is\n\nowned by the host and cannot be acquired by the DMA. Receive\n\nProcess is suspended. To resume processing Receive descriptors,\n\nthe host should change the ownership of the descriptor and issue\n\na Receive Poll Demand command. If no Receive Poll Demand is\n\nissued, Receive Process resumes when the next recognized\n\nincoming frame is received. Register GMAC_STATUS\\[7\\]
is set\n\nonly when the previous Receive Descriptor was owned by the\n\nDMA."]
    #[inline(always)]
    pub fn ru(&self) -> RuR {
        RuR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Process Stopped\n\nThis bit is asserted when the Receive Process enters the Stopped\n\nstate."]
    #[inline(always)]
    pub fn rps(&self) -> RpsR {
        RpsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout\n\nThis bit is asserted when a frame with a length greater than\n\n2,048 bytes is received."]
    #[inline(always)]
    pub fn rwt(&self) -> RwtR {
        RwtR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt\n\nThis bit indicates that the frame to be transmitted was fully\n\ntransferred to the MTL Transmit FIFO."]
    #[inline(always)]
    pub fn eti(&self) -> EtiR {
        EtiR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt\n\nThis bit indicates that a bus error occurred, as detailed in\n\n\\[25:23\\]. When this bit is set, the corresponding DMA engine\n\ndisables all its bus accesses."]
    #[inline(always)]
    pub fn fbi(&self) -> FbiR {
        FbiR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early Receive Interrupt\n\nThis bit indicates that the DMA had filled the first data buffer of\n\nthe packet. Receive Interrupt Register GMAC_STATUS\\[6\\]\n\nautomatically clears this bit."]
    #[inline(always)]
    pub fn eri(&self) -> EriR {
        EriR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary\n\nAbnormal Interrupt Summary bit value is the logical OR of the\n\nfollowing when the corresponding interrupt bits are enabled in\n\nRegister OP_MODE:\n\nRegister GMAC_STATUS\\[1\\]: Transmit Process Stopped\n\nRegister GMAC_STATUS\\[3\\]: Transmit Jabber Timeout\n\nRegister GMAC_STATUS\\[4\\]: Receive FIFO Overflow\n\nRegister GMAC_STATUS\\[5\\]: Transmit Underflow\n\nRegister GMAC_STATUS\\[7\\]: Receive Buffer Unavailable\n\nRegister GMAC_STATUS\\[8\\]: Receive Process Stopped\n\nRegister GMAC_STATUS\\[9\\]: Receive Watchdog Timeout\n\nRegister GMAC_STATUS\\[10\\]: Early Transmit Interrupt\n\nRegister GMAC_STATUS\\[13\\]: Fatal Bus Error\n\nOnly unmasked bits affect the Abnormal Interrupt Summary bit.\n\nThis is a sticky bit and must be cleared each time a\n\ncorresponding bit that causes AIS to be set is cleared."]
    #[inline(always)]
    pub fn ais(&self) -> AisR {
        AisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary\n\nNormal Interrupt Summary bit value is the logical OR of the\n\nfollowing when the corresponding interrupt bits are enabled in\n\nRegister OP_MODE:\n\nRegister GMAC_STATUS\\[0\\]: Transmit Interrupt\n\nRegister GMAC_STATUS\\[2\\]: Transmit Buffer Unavailable\n\nRegister GMAC_STATUS\\[6\\]: Receive Interrupt\n\nRegister GMAC_STATUS\\[14\\]: Early Receive Interrupt\n\nOnly unmasked bits affect the Normal Interrupt Summary bit.\n\nThis is a sticky bit and must be cleared (by writing a 1 to this bit)\n\neach time a corresponding bit that causes NIS to be set is\n\ncleared."]
    #[inline(always)]
    pub fn nis(&self) -> NisR {
        NisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Receive Process State\n\nThese bits indicate the Receive DMA FSM state. This field does\n\nnot generate an interrupt."]
    #[inline(always)]
    pub fn rs(&self) -> RsR {
        RsR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Transmit Process State\n\nThese bits indicate the Transmit DMA FSM state. This field does\n\nnot generate an interrupt."]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Error Bits\n\nThese bits indicate the type of error that caused a Bus Error\n\n(e.g., error response on the AXI interface). Valid only with Fatal\n\nBus Error bit (Register GMAC_STATUS\\[13\\]) set. This field does\n\nnot generate an interrupt.\n\nBit 23: 1'b1 Error during data transfer by TxDMA\n\n1'b0 Error during data transfer by RxDMA\n\nBit 24: 1'b1 Error during read transfer\n\n1'b0 Error during write transfer\n\nBit 25: 1'b1 Error during descriptor access\n\n1'b0 Error during data buffer access"]
    #[inline(always)]
    pub fn eb(&self) -> EbR {
        EbR::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 26 - GMAC Line interface Interrupt\n\nThis bit reflects an interrupt event in the GMAC Core's PCS or\n\nRGMII interface block. The software must read the corresponding\n\nregisters in the GMAC core to get the exact cause of interrupt and\n\nclear the source of interrupt to make this bit as 1'b0. The\n\ninterrupt signal from the GMAC subsystem (sbd_intr_o) is high\n\nwhen this bit is high."]
    #[inline(always)]
    pub fn gli(&self) -> GliR {
        GliR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GMAC MMC Interrupt\n\nThis bit reflects an interrupt event in the MMC module of the\n\nGMAC core. The software must read the corresponding registers\n\nin the GMAC core to get the exact cause of interrupt and clear the\n\nsource of interrupt to make this bit as 1'b0. The interrupt signal\n\nfrom the GMAC subsystem (sbd_intr_o) is high when this bit is\n\nhigh."]
    #[inline(always)]
    pub fn gmi(&self) -> GmiR {
        GmiR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GMAC PMT Interrupt\n\nThis bit indicates an interrupt event in the GMAC core's PMT\n\nmodule. The software must read the corresponding registers in\n\nthe GMAC core to get the exact cause of interrupt and clear its\n\nsource to reset this bit to 1'b0. The interrupt signal from the\n\nGMAC subsystem (sbd_intr_o) is high when this bit is high."]
    #[inline(always)]
    pub fn gpi(&self) -> GpiR {
        GpiR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt\n\nThis bit indicates that frame transmission is finished and\n\nTDES1\\[31\\]
is set in the First Descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn ti(&mut self) -> TiW<GmacStatusSpec> {
        TiW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped\n\nThis bit is set when the transmission is stopped."]
    #[inline(always)]
    #[must_use]
    pub fn tps(&mut self) -> TpsW<GmacStatusSpec> {
        TpsW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable\n\nThis bit indicates that the Next Descriptor in the Transmit List is\n\nowned by the host and cannot be acquired by the DMA.\n\nTransmission is suspended. Bits\\[22:20\\]
explain the Transmit\n\nProcess state transitions. To resume processing transmit\n\ndescriptors, the host should change the ownership of the bit of\n\nthe descriptor and then issue a Transmit Poll Demand command."]
    #[inline(always)]
    #[must_use]
    pub fn tu(&mut self) -> TuW<GmacStatusSpec> {
        TuW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout\n\nThis bit indicates that the Transmit Jabber Timer expired,\n\nmeaning that the transmitter had been excessively active. The\n\ntransmission process is aborted and placed in the Stopped state.\n\nThis causes the Transmit Jabber Timeout TDES0\\[14\\]
flag to\n\nassert."]
    #[inline(always)]
    #[must_use]
    pub fn tjt(&mut self) -> TjtW<GmacStatusSpec> {
        TjtW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive Overflow\n\nThis bit indicates that the Receive Buffer had an Overflow during\n\nframe reception. If the partial frame is transferred to application,\n\nthe overflow status is set in RDES0\\[11\\]."]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OvfW<GmacStatusSpec> {
        OvfW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit Underflow\n\nThis bit indicates that the Transmit Buffer had an Underflow\n\nduring frame transmission. Transmission is suspended and an\n\nUnderflow Error TDES0\\[1\\]
is set."]
    #[inline(always)]
    #[must_use]
    pub fn unf(&mut self) -> UnfW<GmacStatusSpec> {
        UnfW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive Interrupt\n\nThis bit indicates the completion of frame reception. Specific\n\nframe status information has been posted in the descriptor.\n\nReception remains in the Running state."]
    #[inline(always)]
    #[must_use]
    pub fn ri(&mut self) -> RiW<GmacStatusSpec> {
        RiW::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable\n\nThis bit indicates that the Next Descriptor in the Receive List is\n\nowned by the host and cannot be acquired by the DMA. Receive\n\nProcess is suspended. To resume processing Receive descriptors,\n\nthe host should change the ownership of the descriptor and issue\n\na Receive Poll Demand command. If no Receive Poll Demand is\n\nissued, Receive Process resumes when the next recognized\n\nincoming frame is received. Register GMAC_STATUS\\[7\\]
is set\n\nonly when the previous Receive Descriptor was owned by the\n\nDMA."]
    #[inline(always)]
    #[must_use]
    pub fn ru(&mut self) -> RuW<GmacStatusSpec> {
        RuW::new(self, 7)
    }
    #[doc = "Bit 8 - Receive Process Stopped\n\nThis bit is asserted when the Receive Process enters the Stopped\n\nstate."]
    #[inline(always)]
    #[must_use]
    pub fn rps(&mut self) -> RpsW<GmacStatusSpec> {
        RpsW::new(self, 8)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout\n\nThis bit is asserted when a frame with a length greater than\n\n2,048 bytes is received."]
    #[inline(always)]
    #[must_use]
    pub fn rwt(&mut self) -> RwtW<GmacStatusSpec> {
        RwtW::new(self, 9)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt\n\nThis bit indicates that the frame to be transmitted was fully\n\ntransferred to the MTL Transmit FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn eti(&mut self) -> EtiW<GmacStatusSpec> {
        EtiW::new(self, 10)
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt\n\nThis bit indicates that a bus error occurred, as detailed in\n\n\\[25:23\\]. When this bit is set, the corresponding DMA engine\n\ndisables all its bus accesses."]
    #[inline(always)]
    #[must_use]
    pub fn fbi(&mut self) -> FbiW<GmacStatusSpec> {
        FbiW::new(self, 13)
    }
    #[doc = "Bit 14 - Early Receive Interrupt\n\nThis bit indicates that the DMA had filled the first data buffer of\n\nthe packet. Receive Interrupt Register GMAC_STATUS\\[6\\]\n\nautomatically clears this bit."]
    #[inline(always)]
    #[must_use]
    pub fn eri(&mut self) -> EriW<GmacStatusSpec> {
        EriW::new(self, 14)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary\n\nAbnormal Interrupt Summary bit value is the logical OR of the\n\nfollowing when the corresponding interrupt bits are enabled in\n\nRegister OP_MODE:\n\nRegister GMAC_STATUS\\[1\\]: Transmit Process Stopped\n\nRegister GMAC_STATUS\\[3\\]: Transmit Jabber Timeout\n\nRegister GMAC_STATUS\\[4\\]: Receive FIFO Overflow\n\nRegister GMAC_STATUS\\[5\\]: Transmit Underflow\n\nRegister GMAC_STATUS\\[7\\]: Receive Buffer Unavailable\n\nRegister GMAC_STATUS\\[8\\]: Receive Process Stopped\n\nRegister GMAC_STATUS\\[9\\]: Receive Watchdog Timeout\n\nRegister GMAC_STATUS\\[10\\]: Early Transmit Interrupt\n\nRegister GMAC_STATUS\\[13\\]: Fatal Bus Error\n\nOnly unmasked bits affect the Abnormal Interrupt Summary bit.\n\nThis is a sticky bit and must be cleared each time a\n\ncorresponding bit that causes AIS to be set is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn ais(&mut self) -> AisW<GmacStatusSpec> {
        AisW::new(self, 15)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary\n\nNormal Interrupt Summary bit value is the logical OR of the\n\nfollowing when the corresponding interrupt bits are enabled in\n\nRegister OP_MODE:\n\nRegister GMAC_STATUS\\[0\\]: Transmit Interrupt\n\nRegister GMAC_STATUS\\[2\\]: Transmit Buffer Unavailable\n\nRegister GMAC_STATUS\\[6\\]: Receive Interrupt\n\nRegister GMAC_STATUS\\[14\\]: Early Receive Interrupt\n\nOnly unmasked bits affect the Normal Interrupt Summary bit.\n\nThis is a sticky bit and must be cleared (by writing a 1 to this bit)\n\neach time a corresponding bit that causes NIS to be set is\n\ncleared."]
    #[inline(always)]
    #[must_use]
    pub fn nis(&mut self) -> NisW<GmacStatusSpec> {
        NisW::new(self, 16)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacStatusSpec;
impl crate::RegisterSpec for GmacStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_status::R`](R) reader structure"]
impl crate::Readable for GmacStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_status::W`](W) writer structure"]
impl crate::Writable for GmacStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_e7ff;
}
#[doc = "`reset()` method sets GMAC_STATUS to value 0"]
impl crate::Resettable for GmacStatusSpec {
    const RESET_VALUE: u32 = 0;
}
