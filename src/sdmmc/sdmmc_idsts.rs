#[doc = "Register `SDMMC_IDSTS` reader"]
pub type R = crate::R<SdmmcIdstsSpec>;
#[doc = "Register `SDMMC_IDSTS` writer"]
pub type W = crate::W<SdmmcIdstsSpec>;
#[doc = "Field `TI` reader - Transmit Interrupt. Indicates that data transmission is finished for a descriptor. Writing 1 clears this bit."]
pub type TiR = crate::BitReader;
#[doc = "Field `TI` writer - Transmit Interrupt. Indicates that data transmission is finished for a descriptor. Writing 1 clears this bit."]
pub type TiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RI` reader - Receive Interrupt. Indicates the completion of data reception for a descriptor. Writing a 1 clears this bit."]
pub type RiR = crate::BitReader;
#[doc = "Field `RI` writer - Receive Interrupt. Indicates the completion of data reception for a descriptor. Writing a 1 clears this bit."]
pub type RiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBE` reader - Fatal Bus Error Interrupt. Indicates that a Bus Error occurred (IDSTS\\[12:10\\]) (IDSTS64\\[12:10\\], in case of 64-bit address configuration). When this bit is set, the DMA disables all its bus accesses. Writing a 1 clears this bit."]
pub type FbeR = crate::BitReader;
#[doc = "Field `FBE` writer - Fatal Bus Error Interrupt. Indicates that a Bus Error occurred (IDSTS\\[12:10\\]) (IDSTS64\\[12:10\\], in case of 64-bit address configuration). When this bit is set, the DMA disables all its bus accesses. Writing a 1 clears this bit."]
pub type FbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DU` reader - Descriptor Unavailable Interrupt. This bit is set when the descriptor is unavailable due to OWN bit = 0 (DES0\\[31\\]
=0). Writing a 1 clears this bit."]
pub type DuR = crate::BitReader;
#[doc = "Field `DU` writer - Descriptor Unavailable Interrupt. This bit is set when the descriptor is unavailable due to OWN bit = 0 (DES0\\[31\\]
=0). Writing a 1 clears this bit."]
pub type DuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CES` reader - Card Error Summary. Indicates the status of the transaction to/from the card; also present in RINTSTS. Indicates the logical OR of the following bits: EBE: End Bit Error RTO: Response Timeout/Boot Ack Timeout RCRC: Response CRC SBE: Start Bit Error DRTO: Data Read Timeout/BDS timeout DCRC: Data CRC for Receive RE: Response Error Writing a 1 clears this bit. The abort condition of the IDMAC depends on the setting of this CES bit. If the CES bit is enabled, then the IDMAC aborts on a \"response error\"; however, it will not abort if the CES bit is cleared."]
pub type CesR = crate::BitReader;
#[doc = "Field `CES` writer - Card Error Summary. Indicates the status of the transaction to/from the card; also present in RINTSTS. Indicates the logical OR of the following bits: EBE: End Bit Error RTO: Response Timeout/Boot Ack Timeout RCRC: Response CRC SBE: Start Bit Error DRTO: Data Read Timeout/BDS timeout DCRC: Data CRC for Receive RE: Response Error Writing a 1 clears this bit. The abort condition of the IDMAC depends on the setting of this CES bit. If the CES bit is enabled, then the IDMAC aborts on a \"response error\"; however, it will not abort if the CES bit is cleared."]
pub type CesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIS` reader - Normal Interrupt Summary. Logical OR of the following: IDSTS\\[0\\]
Transmit Interrupt IDSTS\\[1\\]
Receive Interrupt Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes NIS to be set is cleared. Writing a 1 clears this bit."]
pub type NisR = crate::BitReader;
#[doc = "Field `NIS` writer - Normal Interrupt Summary. Logical OR of the following: IDSTS\\[0\\]
Transmit Interrupt IDSTS\\[1\\]
Receive Interrupt Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes NIS to be set is cleared. Writing a 1 clears this bit."]
pub type NisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary. Logical OR of the following: IDSTS\\[2\\]
Fatal Bus Interrupt IDSTS\\[4\\]
DU bit Interrupt Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared. Writing a 1 clears this bit."]
pub type AisR = crate::BitReader;
#[doc = "Field `AIS` writer - Abnormal Interrupt Summary. Logical OR of the following: IDSTS\\[2\\]
Fatal Bus Interrupt IDSTS\\[4\\]
DU bit Interrupt Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared. Writing a 1 clears this bit."]
pub type AisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Error Bits. Indicates the type of error that caused a Bus Error. Valid only with atal Bus Error bit—IDSTS\\[2\\]
(IDSTS64\\[2\\], in case of 64-bit address configuration) set. This field does not generate an interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eb {
    #[doc = "1: Host Abort received during reception Others: Reserved"]
    D1 = 1,
    #[doc = "2: Host Abort received during reception Others: Reserved"]
    D2 = 2,
}
impl From<Eb> for u8 {
    #[inline(always)]
    fn from(variant: Eb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eb {
    type Ux = u8;
}
#[doc = "Field `EB` reader - Error Bits. Indicates the type of error that caused a Bus Error. Valid only with atal Bus Error bit—IDSTS\\[2\\]
(IDSTS64\\[2\\], in case of 64-bit address configuration) set. This field does not generate an interrupt."]
pub type EbR = crate::FieldReader<Eb>;
impl EbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Eb> {
        match self.bits {
            1 => Some(Eb::D1),
            2 => Some(Eb::D2),
            _ => None,
        }
    }
    #[doc = "Host Abort received during reception Others: Reserved"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == Eb::D1
    }
    #[doc = "Host Abort received during reception Others: Reserved"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == Eb::D2
    }
}
#[doc = "DMAC FSM present state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsm {
    #[doc = "0: DESC_CLOSE"]
    D0 = 0,
    #[doc = "1: DESC_CLOSE"]
    D1 = 1,
    #[doc = "2: DESC_CLOSE"]
    D2 = 2,
    #[doc = "3: DESC_CLOSE"]
    D3 = 3,
    #[doc = "4: DESC_CLOSE"]
    D4 = 4,
    #[doc = "5: DESC_CLOSE"]
    D5 = 5,
    #[doc = "6: DESC_CLOSE"]
    D6 = 6,
    #[doc = "7: DESC_CLOSE"]
    D7 = 7,
    #[doc = "8: DESC_CLOSE"]
    D8 = 8,
}
impl From<Fsm> for u8 {
    #[inline(always)]
    fn from(variant: Fsm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsm {
    type Ux = u8;
}
#[doc = "Field `FSM` reader - DMAC FSM present state."]
pub type FsmR = crate::FieldReader<Fsm>;
impl FsmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fsm> {
        match self.bits {
            0 => Some(Fsm::D0),
            1 => Some(Fsm::D1),
            2 => Some(Fsm::D2),
            3 => Some(Fsm::D3),
            4 => Some(Fsm::D4),
            5 => Some(Fsm::D5),
            6 => Some(Fsm::D6),
            7 => Some(Fsm::D7),
            8 => Some(Fsm::D8),
            _ => None,
        }
    }
    #[doc = "DESC_CLOSE"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == Fsm::D0
    }
    #[doc = "DESC_CLOSE"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == Fsm::D1
    }
    #[doc = "DESC_CLOSE"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == Fsm::D2
    }
    #[doc = "DESC_CLOSE"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == Fsm::D3
    }
    #[doc = "DESC_CLOSE"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == Fsm::D4
    }
    #[doc = "DESC_CLOSE"]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == Fsm::D5
    }
    #[doc = "DESC_CLOSE"]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == Fsm::D6
    }
    #[doc = "DESC_CLOSE"]
    #[inline(always)]
    pub fn is_d7(&self) -> bool {
        *self == Fsm::D7
    }
    #[doc = "DESC_CLOSE"]
    #[inline(always)]
    pub fn is_d8(&self) -> bool {
        *self == Fsm::D8
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Interrupt. Indicates that data transmission is finished for a descriptor. Writing 1 clears this bit."]
    #[inline(always)]
    pub fn ti(&self) -> TiR {
        TiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Interrupt. Indicates the completion of data reception for a descriptor. Writing a 1 clears this bit."]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fatal Bus Error Interrupt. Indicates that a Bus Error occurred (IDSTS\\[12:10\\]) (IDSTS64\\[12:10\\], in case of 64-bit address configuration). When this bit is set, the DMA disables all its bus accesses. Writing a 1 clears this bit."]
    #[inline(always)]
    pub fn fbe(&self) -> FbeR {
        FbeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt. This bit is set when the descriptor is unavailable due to OWN bit = 0 (DES0\\[31\\]
=0). Writing a 1 clears this bit."]
    #[inline(always)]
    pub fn du(&self) -> DuR {
        DuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Card Error Summary. Indicates the status of the transaction to/from the card; also present in RINTSTS. Indicates the logical OR of the following bits: EBE: End Bit Error RTO: Response Timeout/Boot Ack Timeout RCRC: Response CRC SBE: Start Bit Error DRTO: Data Read Timeout/BDS timeout DCRC: Data CRC for Receive RE: Response Error Writing a 1 clears this bit. The abort condition of the IDMAC depends on the setting of this CES bit. If the CES bit is enabled, then the IDMAC aborts on a \"response error\"; however, it will not abort if the CES bit is cleared."]
    #[inline(always)]
    pub fn ces(&self) -> CesR {
        CesR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary. Logical OR of the following: IDSTS\\[0\\]
Transmit Interrupt IDSTS\\[1\\]
Receive Interrupt Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes NIS to be set is cleared. Writing a 1 clears this bit."]
    #[inline(always)]
    pub fn nis(&self) -> NisR {
        NisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary. Logical OR of the following: IDSTS\\[2\\]
Fatal Bus Interrupt IDSTS\\[4\\]
DU bit Interrupt Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared. Writing a 1 clears this bit."]
    #[inline(always)]
    pub fn ais(&self) -> AisR {
        AisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Error Bits. Indicates the type of error that caused a Bus Error. Valid only with atal Bus Error bit—IDSTS\\[2\\]
(IDSTS64\\[2\\], in case of 64-bit address configuration) set. This field does not generate an interrupt."]
    #[inline(always)]
    pub fn eb(&self) -> EbR {
        EbR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:16 - DMAC FSM present state."]
    #[inline(always)]
    pub fn fsm(&self) -> FsmR {
        FsmR::new(((self.bits >> 13) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt. Indicates that data transmission is finished for a descriptor. Writing 1 clears this bit."]
    #[inline(always)]
    #[must_use]
    pub fn ti(&mut self) -> TiW<SdmmcIdstsSpec> {
        TiW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Interrupt. Indicates the completion of data reception for a descriptor. Writing a 1 clears this bit."]
    #[inline(always)]
    #[must_use]
    pub fn ri(&mut self) -> RiW<SdmmcIdstsSpec> {
        RiW::new(self, 1)
    }
    #[doc = "Bit 2 - Fatal Bus Error Interrupt. Indicates that a Bus Error occurred (IDSTS\\[12:10\\]) (IDSTS64\\[12:10\\], in case of 64-bit address configuration). When this bit is set, the DMA disables all its bus accesses. Writing a 1 clears this bit."]
    #[inline(always)]
    #[must_use]
    pub fn fbe(&mut self) -> FbeW<SdmmcIdstsSpec> {
        FbeW::new(self, 2)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt. This bit is set when the descriptor is unavailable due to OWN bit = 0 (DES0\\[31\\]
=0). Writing a 1 clears this bit."]
    #[inline(always)]
    #[must_use]
    pub fn du(&mut self) -> DuW<SdmmcIdstsSpec> {
        DuW::new(self, 4)
    }
    #[doc = "Bit 5 - Card Error Summary. Indicates the status of the transaction to/from the card; also present in RINTSTS. Indicates the logical OR of the following bits: EBE: End Bit Error RTO: Response Timeout/Boot Ack Timeout RCRC: Response CRC SBE: Start Bit Error DRTO: Data Read Timeout/BDS timeout DCRC: Data CRC for Receive RE: Response Error Writing a 1 clears this bit. The abort condition of the IDMAC depends on the setting of this CES bit. If the CES bit is enabled, then the IDMAC aborts on a \"response error\"; however, it will not abort if the CES bit is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn ces(&mut self) -> CesW<SdmmcIdstsSpec> {
        CesW::new(self, 5)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary. Logical OR of the following: IDSTS\\[0\\]
Transmit Interrupt IDSTS\\[1\\]
Receive Interrupt Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes NIS to be set is cleared. Writing a 1 clears this bit."]
    #[inline(always)]
    #[must_use]
    pub fn nis(&mut self) -> NisW<SdmmcIdstsSpec> {
        NisW::new(self, 8)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary. Logical OR of the following: IDSTS\\[2\\]
Fatal Bus Interrupt IDSTS\\[4\\]
DU bit Interrupt Only unmasked bits affect this bit. This is a sticky bit and must be cleared each time a corresponding bit that causes AIS to be set is cleared. Writing a 1 clears this bit."]
    #[inline(always)]
    #[must_use]
    pub fn ais(&mut self) -> AisW<SdmmcIdstsSpec> {
        AisW::new(self, 9)
    }
}
#[doc = "Internal DMAC status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_idsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_idsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcIdstsSpec;
impl crate::RegisterSpec for SdmmcIdstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_idsts::R`](R) reader structure"]
impl crate::Readable for SdmmcIdstsSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_idsts::W`](W) writer structure"]
impl crate::Writable for SdmmcIdstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_IDSTS to value 0"]
impl crate::Resettable for SdmmcIdstsSpec {
    const RESET_VALUE: u32 = 0;
}
