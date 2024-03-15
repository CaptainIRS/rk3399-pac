#[doc = "Register `GMAC_STATUS` reader"]
pub type R = crate::R<GmacStatusSpec>;
#[doc = "Register `GMAC_STATUS` writer"]
pub type W = crate::W<GmacStatusSpec>;
#[doc = "Field `TI` reader - Transmit Interrupt This bit indicates that frame transmission is finished and TDES1\\[31\\]
is set in the First Descriptor."]
pub type TiR = crate::BitReader;
#[doc = "Field `TI` writer - Transmit Interrupt This bit indicates that frame transmission is finished and TDES1\\[31\\]
is set in the First Descriptor."]
pub type TiW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TPS` reader - Transmit Process Stopped This bit is set when the transmission is stopped."]
pub type TpsR = crate::BitReader;
#[doc = "Field `TPS` writer - Transmit Process Stopped This bit is set when the transmission is stopped."]
pub type TpsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TU` reader - Transmit Buffer Unavailable This bit indicates that the Next Descriptor in the Transmit List is owned by the host and cannot be acquired by the DMA. Transmission is suspended. Bits\\[22:20\\]
explain the Transmit Process state transitions. To resume processing transmit descriptors, the host should change the ownership of the bit of the descriptor and then issue a Transmit Poll Demand command."]
pub type TuR = crate::BitReader;
#[doc = "Field `TU` writer - Transmit Buffer Unavailable This bit indicates that the Next Descriptor in the Transmit List is owned by the host and cannot be acquired by the DMA. Transmission is suspended. Bits\\[22:20\\]
explain the Transmit Process state transitions. To resume processing transmit descriptors, the host should change the ownership of the bit of the descriptor and then issue a Transmit Poll Demand command."]
pub type TuW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TJT` reader - Transmit Jabber Timeout This bit indicates that the Transmit Jabber Timer expired, meaning that the transmitter had been excessively active. The transmission process is aborted and placed in the Stopped state. This causes the Transmit Jabber Timeout TDES0\\[14\\]
flag to assert."]
pub type TjtR = crate::BitReader;
#[doc = "Field `TJT` writer - Transmit Jabber Timeout This bit indicates that the Transmit Jabber Timer expired, meaning that the transmitter had been excessively active. The transmission process is aborted and placed in the Stopped state. This causes the Transmit Jabber Timeout TDES0\\[14\\]
flag to assert."]
pub type TjtW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OVF` reader - Receive Overflow This bit indicates that the Receive Buffer had an Overflow during frame reception. If the partial frame is transferred to application, the overflow status is set in RDES0\\[11\\]."]
pub type OvfR = crate::BitReader;
#[doc = "Field `OVF` writer - Receive Overflow This bit indicates that the Receive Buffer had an Overflow during frame reception. If the partial frame is transferred to application, the overflow status is set in RDES0\\[11\\]."]
pub type OvfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `UNF` reader - Transmit Underflow This bit indicates that the Transmit Buffer had an Underflow during frame transmission. Transmission is suspended and an Underflow Error TDES0\\[1\\]
is set."]
pub type UnfR = crate::BitReader;
#[doc = "Field `UNF` writer - Transmit Underflow This bit indicates that the Transmit Buffer had an Underflow during frame transmission. Transmission is suspended and an Underflow Error TDES0\\[1\\]
is set."]
pub type UnfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RI` reader - Receive Interrupt This bit indicates the completion of frame reception. Specific frame status information has been posted in the descriptor. Reception remains in the Running state."]
pub type RiR = crate::BitReader;
#[doc = "Field `RI` writer - Receive Interrupt This bit indicates the completion of frame reception. Specific frame status information has been posted in the descriptor. Reception remains in the Running state."]
pub type RiW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RU` reader - Receive Buffer Unavailable This bit indicates that the Next Descriptor in the Receive List is owned by the host and cannot be acquired by the DMA. Receive Process is suspended. To resume processing Receive descriptors, the host should change the ownership of the descriptor and issue a Receive Poll Demand command. If no Receive Poll Demand is issued, Receive Process resumes when the next recognized incoming frame is received. Register GMAC_STATUS\\[7\\]
is set only when the previous Receive Descriptor was owned by the DMA."]
pub type RuR = crate::BitReader;
#[doc = "Field `RU` writer - Receive Buffer Unavailable This bit indicates that the Next Descriptor in the Receive List is owned by the host and cannot be acquired by the DMA. Receive Process is suspended. To resume processing Receive descriptors, the host should change the ownership of the descriptor and issue a Receive Poll Demand command. If no Receive Poll Demand is issued, Receive Process resumes when the next recognized incoming frame is received. Register GMAC_STATUS\\[7\\]
is set only when the previous Receive Descriptor was owned by the DMA."]
pub type RuW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RPS` reader - Receive Process Stopped This bit is asserted when the Receive Process enters the Stopped state."]
pub type RpsR = crate::BitReader;
#[doc = "Field `RPS` writer - Receive Process Stopped This bit is asserted when the Receive Process enters the Stopped state."]
pub type RpsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RWT` reader - Receive Watchdog Timeout This bit is asserted when a frame with a length greater than 2,048 bytes is received."]
pub type RwtR = crate::BitReader;
#[doc = "Field `RWT` writer - Receive Watchdog Timeout This bit is asserted when a frame with a length greater than 2,048 bytes is received."]
pub type RwtW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ETI` reader - Early Transmit Interrupt This bit indicates that the frame to be transmitted was fully transferred to the MTL Transmit FIFO."]
pub type EtiR = crate::BitReader;
#[doc = "Field `ETI` writer - Early Transmit Interrupt This bit indicates that the frame to be transmitted was fully transferred to the MTL Transmit FIFO."]
pub type EtiW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FBI` reader - Fatal Bus Error Interrupt This bit indicates that a bus error occurred, as detailed in \\[25:23\\]. When this bit is set, the corresponding DMA engine disables all its bus accesses."]
pub type FbiR = crate::BitReader;
#[doc = "Field `FBI` writer - Fatal Bus Error Interrupt This bit indicates that a bus error occurred, as detailed in \\[25:23\\]. When this bit is set, the corresponding DMA engine disables all its bus accesses."]
pub type FbiW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ERI` reader - Early Receive Interrupt This bit indicates that the DMA had filled the first data buffer of the packet. Receive Interrupt Register GMAC_STATUS\\[6\\]
automatically clears this bit."]
pub type EriR = crate::BitReader;
#[doc = "Field `ERI` writer - Early Receive Interrupt This bit indicates that the DMA had filled the first data buffer of the packet. Receive Interrupt Register GMAC_STATUS\\[6\\]
automatically clears this bit."]
pub type EriW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in Register OP_MODE: Register GMAC_STATUS\\[1\\]: Transmit Process Stopped Register GMAC_STATUS\\[3\\]: Transmit Jabber Timeout Register GMAC_STATUS\\[4\\]: Receive FIFO Overflow Register GMAC_STATUS\\[5\\]: Transmit Underflow Register GMAC_STATUS\\[7\\]: Receive Buffer Unavailable Register GMAC_STATUS\\[8\\]: Receive Process Stopped Register GMAC_STATUS\\[9\\]: Receive Watchdog Timeout Register GMAC_STATUS\\[10\\]: Early Transmit Interrupt Register GMAC_STATUS\\[13\\]: Fatal Bus Error Only unmasked bits affect the Abnormal Interrupt Summary bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared."]
pub type AisR = crate::BitReader;
#[doc = "Field `AIS` writer - Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in Register OP_MODE: Register GMAC_STATUS\\[1\\]: Transmit Process Stopped Register GMAC_STATUS\\[3\\]: Transmit Jabber Timeout Register GMAC_STATUS\\[4\\]: Receive FIFO Overflow Register GMAC_STATUS\\[5\\]: Transmit Underflow Register GMAC_STATUS\\[7\\]: Receive Buffer Unavailable Register GMAC_STATUS\\[8\\]: Receive Process Stopped Register GMAC_STATUS\\[9\\]: Receive Watchdog Timeout Register GMAC_STATUS\\[10\\]: Early Transmit Interrupt Register GMAC_STATUS\\[13\\]: Fatal Bus Error Only unmasked bits affect the Abnormal Interrupt Summary bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared."]
pub type AisW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `NIS` reader - Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in Register OP_MODE: Register GMAC_STATUS\\[0\\]: Transmit Interrupt Register GMAC_STATUS\\[2\\]: Transmit Buffer Unavailable Register GMAC_STATUS\\[6\\]: Receive Interrupt Register GMAC_STATUS\\[14\\]: Early Receive Interrupt Only unmasked bits affect the Normal Interrupt Summary bit. This is a sticky bit and must be cleared (by writing a 1 to this bit) each time a corresponding bit that causes NIS to be set is cleared."]
pub type NisR = crate::BitReader;
#[doc = "Field `NIS` writer - Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in Register OP_MODE: Register GMAC_STATUS\\[0\\]: Transmit Interrupt Register GMAC_STATUS\\[2\\]: Transmit Buffer Unavailable Register GMAC_STATUS\\[6\\]: Receive Interrupt Register GMAC_STATUS\\[14\\]: Early Receive Interrupt Only unmasked bits affect the Normal Interrupt Summary bit. This is a sticky bit and must be cleared (by writing a 1 to this bit) each time a corresponding bit that causes NIS to be set is cleared."]
pub type NisW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Receive Process State These bits indicate the Receive DMA FSM state. This field does not generate an interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rs {
    #[doc = "0: Running: Transferring the receive packet data from receive buffer to host memory."]
    B000 = 0,
    #[doc = "1: Running: Transferring the receive packet data from receive buffer to host memory."]
    B001 = 1,
    #[doc = "2: Running: Transferring the receive packet data from receive buffer to host memory."]
    B010 = 2,
    #[doc = "3: Running: Transferring the receive packet data from receive buffer to host memory."]
    B011 = 3,
    #[doc = "4: Running: Transferring the receive packet data from receive buffer to host memory."]
    B100 = 4,
    #[doc = "5: Running: Transferring the receive packet data from receive buffer to host memory."]
    B101 = 5,
    #[doc = "6: Running: Transferring the receive packet data from receive buffer to host memory."]
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
#[doc = "Field `RS` reader - Receive Process State These bits indicate the Receive DMA FSM state. This field does not generate an interrupt."]
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
    #[doc = "Running: Transferring the receive packet data from receive buffer to host memory."]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Rs::B000
    }
    #[doc = "Running: Transferring the receive packet data from receive buffer to host memory."]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Rs::B001
    }
    #[doc = "Running: Transferring the receive packet data from receive buffer to host memory."]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Rs::B010
    }
    #[doc = "Running: Transferring the receive packet data from receive buffer to host memory."]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Rs::B011
    }
    #[doc = "Running: Transferring the receive packet data from receive buffer to host memory."]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Rs::B100
    }
    #[doc = "Running: Transferring the receive packet data from receive buffer to host memory."]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == Rs::B101
    }
    #[doc = "Running: Transferring the receive packet data from receive buffer to host memory."]
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
#[doc = "Transmit Process State These bits indicate the Transmit DMA FSM state. This field does not generate an interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ts {
    #[doc = "0: Running; Closing Transmit Descriptor."]
    B000 = 0,
    #[doc = "1: Running; Closing Transmit Descriptor."]
    B001 = 1,
    #[doc = "2: Running; Closing Transmit Descriptor."]
    B010 = 2,
    #[doc = "3: Running; Closing Transmit Descriptor."]
    B011 = 3,
    #[doc = "4: Running; Closing Transmit Descriptor."]
    B100 = 4,
    #[doc = "5: Running; Closing Transmit Descriptor."]
    B101 = 5,
    #[doc = "6: Running; Closing Transmit Descriptor."]
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
#[doc = "Field `TS` reader - Transmit Process State These bits indicate the Transmit DMA FSM state. This field does not generate an interrupt."]
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
    #[doc = "Running; Closing Transmit Descriptor."]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Ts::B000
    }
    #[doc = "Running; Closing Transmit Descriptor."]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Ts::B001
    }
    #[doc = "Running; Closing Transmit Descriptor."]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Ts::B010
    }
    #[doc = "Running; Closing Transmit Descriptor."]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Ts::B011
    }
    #[doc = "Running; Closing Transmit Descriptor."]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Ts::B100
    }
    #[doc = "Running; Closing Transmit Descriptor."]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == Ts::B101
    }
    #[doc = "Running; Closing Transmit Descriptor."]
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
#[doc = "Field `EB` reader - Error Bits These bits indicate the type of error that caused a Bus Error (e.g., error response on the AXI interface). Valid only with Fatal Bus Error bit (Register GMAC_STATUS\\[13\\]) set. This field does not generate an interrupt. Bit 23: 1'b1 Error during data transfer by TxDMA 1'b0 Error during data transfer by RxDMA Bit 24: 1'b1 Error during read transfer 1'b0 Error during write transfer Bit 25: 1'b1 Error during descriptor access 1'b0 Error during data buffer access"]
pub type EbR = crate::FieldReader;
#[doc = "Field `GLI` reader - GMAC Line interface Interrupt This bit reflects an interrupt event in the GMAC Core's PCS or RGMII interface block. The software must read the corresponding registers in the GMAC core to get the exact cause of interrupt and clear the source of interrupt to make this bit as 1'b0. The interrupt signal from the GMAC subsystem (sbd_intr_o) is high when this bit is high."]
pub type GliR = crate::BitReader;
#[doc = "Field `GMI` reader - GMAC MMC Interrupt This bit reflects an interrupt event in the MMC module of the GMAC core. The software must read the corresponding registers in the GMAC core to get the exact cause of interrupt and clear the source of interrupt to make this bit as 1'b0. The interrupt signal from the GMAC subsystem (sbd_intr_o) is high when this bit is high."]
pub type GmiR = crate::BitReader;
#[doc = "Field `GPI` reader - GMAC PMT Interrupt This bit indicates an interrupt event in the GMAC core's PMT module. The software must read the corresponding registers in the GMAC core to get the exact cause of interrupt and clear its source to reset this bit to 1'b0. The interrupt signal from the GMAC subsystem (sbd_intr_o) is high when this bit is high."]
pub type GpiR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt This bit indicates that frame transmission is finished and TDES1\\[31\\]
is set in the First Descriptor."]
    #[inline(always)]
    pub fn ti(&self) -> TiR {
        TiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped This bit is set when the transmission is stopped."]
    #[inline(always)]
    pub fn tps(&self) -> TpsR {
        TpsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable This bit indicates that the Next Descriptor in the Transmit List is owned by the host and cannot be acquired by the DMA. Transmission is suspended. Bits\\[22:20\\]
explain the Transmit Process state transitions. To resume processing transmit descriptors, the host should change the ownership of the bit of the descriptor and then issue a Transmit Poll Demand command."]
    #[inline(always)]
    pub fn tu(&self) -> TuR {
        TuR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout This bit indicates that the Transmit Jabber Timer expired, meaning that the transmitter had been excessively active. The transmission process is aborted and placed in the Stopped state. This causes the Transmit Jabber Timeout TDES0\\[14\\]
flag to assert."]
    #[inline(always)]
    pub fn tjt(&self) -> TjtR {
        TjtR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Overflow This bit indicates that the Receive Buffer had an Overflow during frame reception. If the partial frame is transferred to application, the overflow status is set in RDES0\\[11\\]."]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Underflow This bit indicates that the Transmit Buffer had an Underflow during frame transmission. Transmission is suspended and an Underflow Error TDES0\\[1\\]
is set."]
    #[inline(always)]
    pub fn unf(&self) -> UnfR {
        UnfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt This bit indicates the completion of frame reception. Specific frame status information has been posted in the descriptor. Reception remains in the Running state."]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable This bit indicates that the Next Descriptor in the Receive List is owned by the host and cannot be acquired by the DMA. Receive Process is suspended. To resume processing Receive descriptors, the host should change the ownership of the descriptor and issue a Receive Poll Demand command. If no Receive Poll Demand is issued, Receive Process resumes when the next recognized incoming frame is received. Register GMAC_STATUS\\[7\\]
is set only when the previous Receive Descriptor was owned by the DMA."]
    #[inline(always)]
    pub fn ru(&self) -> RuR {
        RuR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Process Stopped This bit is asserted when the Receive Process enters the Stopped state."]
    #[inline(always)]
    pub fn rps(&self) -> RpsR {
        RpsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout This bit is asserted when a frame with a length greater than 2,048 bytes is received."]
    #[inline(always)]
    pub fn rwt(&self) -> RwtR {
        RwtR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt This bit indicates that the frame to be transmitted was fully transferred to the MTL Transmit FIFO."]
    #[inline(always)]
    pub fn eti(&self) -> EtiR {
        EtiR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt This bit indicates that a bus error occurred, as detailed in \\[25:23\\]. When this bit is set, the corresponding DMA engine disables all its bus accesses."]
    #[inline(always)]
    pub fn fbi(&self) -> FbiR {
        FbiR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early Receive Interrupt This bit indicates that the DMA had filled the first data buffer of the packet. Receive Interrupt Register GMAC_STATUS\\[6\\]
automatically clears this bit."]
    #[inline(always)]
    pub fn eri(&self) -> EriR {
        EriR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in Register OP_MODE: Register GMAC_STATUS\\[1\\]: Transmit Process Stopped Register GMAC_STATUS\\[3\\]: Transmit Jabber Timeout Register GMAC_STATUS\\[4\\]: Receive FIFO Overflow Register GMAC_STATUS\\[5\\]: Transmit Underflow Register GMAC_STATUS\\[7\\]: Receive Buffer Unavailable Register GMAC_STATUS\\[8\\]: Receive Process Stopped Register GMAC_STATUS\\[9\\]: Receive Watchdog Timeout Register GMAC_STATUS\\[10\\]: Early Transmit Interrupt Register GMAC_STATUS\\[13\\]: Fatal Bus Error Only unmasked bits affect the Abnormal Interrupt Summary bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared."]
    #[inline(always)]
    pub fn ais(&self) -> AisR {
        AisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in Register OP_MODE: Register GMAC_STATUS\\[0\\]: Transmit Interrupt Register GMAC_STATUS\\[2\\]: Transmit Buffer Unavailable Register GMAC_STATUS\\[6\\]: Receive Interrupt Register GMAC_STATUS\\[14\\]: Early Receive Interrupt Only unmasked bits affect the Normal Interrupt Summary bit. This is a sticky bit and must be cleared (by writing a 1 to this bit) each time a corresponding bit that causes NIS to be set is cleared."]
    #[inline(always)]
    pub fn nis(&self) -> NisR {
        NisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Receive Process State These bits indicate the Receive DMA FSM state. This field does not generate an interrupt."]
    #[inline(always)]
    pub fn rs(&self) -> RsR {
        RsR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Transmit Process State These bits indicate the Transmit DMA FSM state. This field does not generate an interrupt."]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Error Bits These bits indicate the type of error that caused a Bus Error (e.g., error response on the AXI interface). Valid only with Fatal Bus Error bit (Register GMAC_STATUS\\[13\\]) set. This field does not generate an interrupt. Bit 23: 1'b1 Error during data transfer by TxDMA 1'b0 Error during data transfer by RxDMA Bit 24: 1'b1 Error during read transfer 1'b0 Error during write transfer Bit 25: 1'b1 Error during descriptor access 1'b0 Error during data buffer access"]
    #[inline(always)]
    pub fn eb(&self) -> EbR {
        EbR::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 26 - GMAC Line interface Interrupt This bit reflects an interrupt event in the GMAC Core's PCS or RGMII interface block. The software must read the corresponding registers in the GMAC core to get the exact cause of interrupt and clear the source of interrupt to make this bit as 1'b0. The interrupt signal from the GMAC subsystem (sbd_intr_o) is high when this bit is high."]
    #[inline(always)]
    pub fn gli(&self) -> GliR {
        GliR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GMAC MMC Interrupt This bit reflects an interrupt event in the MMC module of the GMAC core. The software must read the corresponding registers in the GMAC core to get the exact cause of interrupt and clear the source of interrupt to make this bit as 1'b0. The interrupt signal from the GMAC subsystem (sbd_intr_o) is high when this bit is high."]
    #[inline(always)]
    pub fn gmi(&self) -> GmiR {
        GmiR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GMAC PMT Interrupt This bit indicates an interrupt event in the GMAC core's PMT module. The software must read the corresponding registers in the GMAC core to get the exact cause of interrupt and clear its source to reset this bit to 1'b0. The interrupt signal from the GMAC subsystem (sbd_intr_o) is high when this bit is high."]
    #[inline(always)]
    pub fn gpi(&self) -> GpiR {
        GpiR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt This bit indicates that frame transmission is finished and TDES1\\[31\\]
is set in the First Descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn ti(&mut self) -> TiW<GmacStatusSpec> {
        TiW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped This bit is set when the transmission is stopped."]
    #[inline(always)]
    #[must_use]
    pub fn tps(&mut self) -> TpsW<GmacStatusSpec> {
        TpsW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable This bit indicates that the Next Descriptor in the Transmit List is owned by the host and cannot be acquired by the DMA. Transmission is suspended. Bits\\[22:20\\]
explain the Transmit Process state transitions. To resume processing transmit descriptors, the host should change the ownership of the bit of the descriptor and then issue a Transmit Poll Demand command."]
    #[inline(always)]
    #[must_use]
    pub fn tu(&mut self) -> TuW<GmacStatusSpec> {
        TuW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout This bit indicates that the Transmit Jabber Timer expired, meaning that the transmitter had been excessively active. The transmission process is aborted and placed in the Stopped state. This causes the Transmit Jabber Timeout TDES0\\[14\\]
flag to assert."]
    #[inline(always)]
    #[must_use]
    pub fn tjt(&mut self) -> TjtW<GmacStatusSpec> {
        TjtW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive Overflow This bit indicates that the Receive Buffer had an Overflow during frame reception. If the partial frame is transferred to application, the overflow status is set in RDES0\\[11\\]."]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OvfW<GmacStatusSpec> {
        OvfW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit Underflow This bit indicates that the Transmit Buffer had an Underflow during frame transmission. Transmission is suspended and an Underflow Error TDES0\\[1\\]
is set."]
    #[inline(always)]
    #[must_use]
    pub fn unf(&mut self) -> UnfW<GmacStatusSpec> {
        UnfW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive Interrupt This bit indicates the completion of frame reception. Specific frame status information has been posted in the descriptor. Reception remains in the Running state."]
    #[inline(always)]
    #[must_use]
    pub fn ri(&mut self) -> RiW<GmacStatusSpec> {
        RiW::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable This bit indicates that the Next Descriptor in the Receive List is owned by the host and cannot be acquired by the DMA. Receive Process is suspended. To resume processing Receive descriptors, the host should change the ownership of the descriptor and issue a Receive Poll Demand command. If no Receive Poll Demand is issued, Receive Process resumes when the next recognized incoming frame is received. Register GMAC_STATUS\\[7\\]
is set only when the previous Receive Descriptor was owned by the DMA."]
    #[inline(always)]
    #[must_use]
    pub fn ru(&mut self) -> RuW<GmacStatusSpec> {
        RuW::new(self, 7)
    }
    #[doc = "Bit 8 - Receive Process Stopped This bit is asserted when the Receive Process enters the Stopped state."]
    #[inline(always)]
    #[must_use]
    pub fn rps(&mut self) -> RpsW<GmacStatusSpec> {
        RpsW::new(self, 8)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout This bit is asserted when a frame with a length greater than 2,048 bytes is received."]
    #[inline(always)]
    #[must_use]
    pub fn rwt(&mut self) -> RwtW<GmacStatusSpec> {
        RwtW::new(self, 9)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt This bit indicates that the frame to be transmitted was fully transferred to the MTL Transmit FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn eti(&mut self) -> EtiW<GmacStatusSpec> {
        EtiW::new(self, 10)
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt This bit indicates that a bus error occurred, as detailed in \\[25:23\\]. When this bit is set, the corresponding DMA engine disables all its bus accesses."]
    #[inline(always)]
    #[must_use]
    pub fn fbi(&mut self) -> FbiW<GmacStatusSpec> {
        FbiW::new(self, 13)
    }
    #[doc = "Bit 14 - Early Receive Interrupt This bit indicates that the DMA had filled the first data buffer of the packet. Receive Interrupt Register GMAC_STATUS\\[6\\]
automatically clears this bit."]
    #[inline(always)]
    #[must_use]
    pub fn eri(&mut self) -> EriW<GmacStatusSpec> {
        EriW::new(self, 14)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in Register OP_MODE: Register GMAC_STATUS\\[1\\]: Transmit Process Stopped Register GMAC_STATUS\\[3\\]: Transmit Jabber Timeout Register GMAC_STATUS\\[4\\]: Receive FIFO Overflow Register GMAC_STATUS\\[5\\]: Transmit Underflow Register GMAC_STATUS\\[7\\]: Receive Buffer Unavailable Register GMAC_STATUS\\[8\\]: Receive Process Stopped Register GMAC_STATUS\\[9\\]: Receive Watchdog Timeout Register GMAC_STATUS\\[10\\]: Early Transmit Interrupt Register GMAC_STATUS\\[13\\]: Fatal Bus Error Only unmasked bits affect the Abnormal Interrupt Summary bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn ais(&mut self) -> AisW<GmacStatusSpec> {
        AisW::new(self, 15)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in Register OP_MODE: Register GMAC_STATUS\\[0\\]: Transmit Interrupt Register GMAC_STATUS\\[2\\]: Transmit Buffer Unavailable Register GMAC_STATUS\\[6\\]: Receive Interrupt Register GMAC_STATUS\\[14\\]: Early Receive Interrupt Only unmasked bits affect the Normal Interrupt Summary bit. This is a sticky bit and must be cleared (by writing a 1 to this bit) each time a corresponding bit that causes NIS to be set is cleared."]
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
