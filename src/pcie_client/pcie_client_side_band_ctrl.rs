#[doc = "Register `PCIE_CLIENT_SIDE_BAND_CTRL` reader"]
pub type R = crate::R<PcieClientSideBandCtrlSpec>;
#[doc = "Register `PCIE_CLIENT_SIDE_BAND_CTRL` writer"]
pub type W = crate::W<PcieClientSideBandCtrlSpec>;
#[doc = "PCIe target non posted reject\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NonPostedRej {
    #[doc = "0: normal operation"]
    B0 = 0,
    #[doc = "1: reject non posted request This is a single bit input signal which can be asserted by client logic when it cannot service a non-posted request. The core will not present any non-posted requests that it receives from the PCIe Link. It will hold them in the PNP FIFO RAM till the signal is de-asserted. If a non-posted TLP has already been queued from the PNP FIFO and this signal is asserted, the core will place it on the AXI bridge. The client logic must accept the non-posted TLP. The in-flight non-posted TLPs in the core from the PNP FIFO cannot be stopped. However, non-posted TLPs that are in the PNP FIFO RAM when this signal is asserted or come in after the signal is asserted will not be forwarded to the AXI interface. The client must assert this signal when it still can process two or three non-posted TLPs. This will allow posted TLPs to go past non-posted TLPs at the AXI master write interface due to client not being able to service non- posted TLPs."]
    B1 = 1,
}
impl From<NonPostedRej> for bool {
    #[inline(always)]
    fn from(variant: NonPostedRej) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NON_POSTED_REJ` reader - PCIe target non posted reject"]
pub type NonPostedRejR = crate::BitReader<NonPostedRej>;
impl NonPostedRejR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NonPostedRej {
        match self.bits {
            false => NonPostedRej::B0,
            true => NonPostedRej::B1,
        }
    }
    #[doc = "normal operation"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == NonPostedRej::B0
    }
    #[doc = "reject non posted request This is a single bit input signal which can be asserted by client logic when it cannot service a non-posted request. The core will not present any non-posted requests that it receives from the PCIe Link. It will hold them in the PNP FIFO RAM till the signal is de-asserted. If a non-posted TLP has already been queued from the PNP FIFO and this signal is asserted, the core will place it on the AXI bridge. The client logic must accept the non-posted TLP. The in-flight non-posted TLPs in the core from the PNP FIFO cannot be stopped. However, non-posted TLPs that are in the PNP FIFO RAM when this signal is asserted or come in after the signal is asserted will not be forwarded to the AXI interface. The client must assert this signal when it still can process two or three non-posted TLPs. This will allow posted TLPs to go past non-posted TLPs at the AXI master write interface due to client not being able to service non- posted TLPs."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == NonPostedRej::B1
    }
}
#[doc = "Field `NON_POSTED_REJ` writer - PCIe target non posted reject"]
pub type NonPostedRejW<'a, REG> = crate::BitWriter<'a, REG, NonPostedRej>;
impl<'a, REG> NonPostedRejW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(NonPostedRej::B0)
    }
    #[doc = "reject non posted request This is a single bit input signal which can be asserted by client logic when it cannot service a non-posted request. The core will not present any non-posted requests that it receives from the PCIe Link. It will hold them in the PNP FIFO RAM till the signal is de-asserted. If a non-posted TLP has already been queued from the PNP FIFO and this signal is asserted, the core will place it on the AXI bridge. The client logic must accept the non-posted TLP. The in-flight non-posted TLPs in the core from the PNP FIFO cannot be stopped. However, non-posted TLPs that are in the PNP FIFO RAM when this signal is asserted or come in after the signal is asserted will not be forwarded to the AXI interface. The client must assert this signal when it still can process two or three non-posted TLPs. This will allow posted TLPs to go past non-posted TLPs at the AXI master write interface due to client not being able to service non- posted TLPs."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(NonPostedRej::B1)
    }
}
#[doc = "Power state of the phy\n\nPower up or down the transceiver.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pwdn {
    #[doc = "0: P0, normal operation"]
    B000 = 0,
    #[doc = "1: P0s, power saving state"]
    B001 = 1,
    #[doc = "2: P1, lower power state"]
    B010 = 2,
    #[doc = "3: P2, lowest power state, PLL not powered"]
    B011 = 3,
    #[doc = "7: L1SS.2, common mode off others : L1SS.1, common mode on"]
    B111 = 7,
}
impl From<Pwdn> for u8 {
    #[inline(always)]
    fn from(variant: Pwdn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pwdn {
    type Ux = u8;
}
#[doc = "Field `PWDN` reader - Power state of the phy\n\nPower up or down the transceiver."]
pub type PwdnR = crate::FieldReader<Pwdn>;
impl PwdnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pwdn> {
        match self.bits {
            0 => Some(Pwdn::B000),
            1 => Some(Pwdn::B001),
            2 => Some(Pwdn::B010),
            3 => Some(Pwdn::B011),
            7 => Some(Pwdn::B111),
            _ => None,
        }
    }
    #[doc = "P0, normal operation"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Pwdn::B000
    }
    #[doc = "P0s, power saving state"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Pwdn::B001
    }
    #[doc = "P1, lower power state"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Pwdn::B010
    }
    #[doc = "P2, lowest power state, PLL not powered"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Pwdn::B011
    }
    #[doc = "L1SS.2, common mode off others : L1SS.1, common mode on"]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == Pwdn::B111
    }
}
#[doc = "Field `TX_DEEMPHASIS_EXT` reader - PIPE phy extended de-emphasis configuration, it combine with\n\nthe standard pipe de-emphasis."]
pub type TxDeemphasisExtR = crate::FieldReader;
#[doc = "Field `TX_DEEMPHASIS_EXT` writer - PIPE phy extended de-emphasis configuration, it combine with\n\nthe standard pipe de-emphasis."]
pub type TxDeemphasisExtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "PIPE bypass codec configuration\n\nControls whether the PHY performs 8b/10b encode and decode:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BypassCodec {
    #[doc = "0: 8b/10b encode/decode performed normally"]
    B0 = 0,
    #[doc = "1: 8b/10b encode/decode bypassed Data bus width is 20 bits, TxDataK and RxDataK interfaces are not used, if encode/decode bypassed, and WIDTH_I shall be set high."]
    B1 = 1,
}
impl From<BypassCodec> for bool {
    #[inline(always)]
    fn from(variant: BypassCodec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASS_CODEC` reader - PIPE bypass codec configuration\n\nControls whether the PHY performs 8b/10b encode and decode:"]
pub type BypassCodecR = crate::BitReader<BypassCodec>;
impl BypassCodecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BypassCodec {
        match self.bits {
            false => BypassCodec::B0,
            true => BypassCodec::B1,
        }
    }
    #[doc = "8b/10b encode/decode performed normally"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BypassCodec::B0
    }
    #[doc = "8b/10b encode/decode bypassed Data bus width is 20 bits, TxDataK and RxDataK interfaces are not used, if encode/decode bypassed, and WIDTH_I shall be set high."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BypassCodec::B1
    }
}
#[doc = "Field `BYPASS_CODEC` writer - PIPE bypass codec configuration\n\nControls whether the PHY performs 8b/10b encode and decode:"]
pub type BypassCodecW<'a, REG> = crate::BitWriter<'a, REG, BypassCodec>;
impl<'a, REG> BypassCodecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "8b/10b encode/decode performed normally"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BypassCodec::B0)
    }
    #[doc = "8b/10b encode/decode bypassed Data bus width is 20 bits, TxDataK and RxDataK interfaces are not used, if encode/decode bypassed, and WIDTH_I shall be set high."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BypassCodec::B1)
    }
}
#[doc = "PCIe phy receiver control\n\nControls whether the PHY RX is active when the PHY is in P0 or\n\nP0s states.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RxStandby {
    #[doc = "0: Active"]
    B0 = 0,
    #[doc = "1: Standby In other modes not mentioned above, this signal is ignored. One bit for each lane."]
    B1 = 1,
}
impl From<RxStandby> for u8 {
    #[inline(always)]
    fn from(variant: RxStandby) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RxStandby {
    type Ux = u8;
}
#[doc = "Field `RX_STANDBY` reader - PCIe phy receiver control\n\nControls whether the PHY RX is active when the PHY is in P0 or\n\nP0s states."]
pub type RxStandbyR = crate::FieldReader<RxStandby>;
impl RxStandbyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RxStandby> {
        match self.bits {
            0 => Some(RxStandby::B0),
            1 => Some(RxStandby::B1),
            _ => None,
        }
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RxStandby::B0
    }
    #[doc = "Standby In other modes not mentioned above, this signal is ignored. One bit for each lane."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RxStandby::B1
    }
}
#[doc = "Field `RX_STANDBY` writer - PCIe phy receiver control\n\nControls whether the PHY RX is active when the PHY is in P0 or\n\nP0s states."]
pub type RxStandbyW<'a, REG> = crate::FieldWriter<'a, REG, 4, RxStandby>;
impl<'a, REG> RxStandbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Active"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RxStandby::B0)
    }
    #[doc = "Standby In other modes not mentioned above, this signal is ignored. One bit for each lane."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RxStandby::B1)
    }
}
#[doc = "Write mask\n\nFor each served bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum WriteMask {
    #[doc = "0: write mask"]
    B0 = 0,
    #[doc = "1: write enable"]
    B1 = 1,
}
impl From<WriteMask> for u16 {
    #[inline(always)]
    fn from(variant: WriteMask) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WriteMask {
    type Ux = u16;
}
#[doc = "Field `WRITE_MASK` writer - Write mask\n\nFor each served bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, WriteMask>;
impl<'a, REG> WriteMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "write mask"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WriteMask::B0)
    }
    #[doc = "write enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WriteMask::B1)
    }
}
impl R {
    #[doc = "Bit 0 - PCIe target non posted reject"]
    #[inline(always)]
    pub fn non_posted_rej(&self) -> NonPostedRejR {
        NonPostedRejR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Power state of the phy\n\nPower up or down the transceiver."]
    #[inline(always)]
    pub fn pwdn(&self) -> PwdnR {
        PwdnR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5 - PIPE phy extended de-emphasis configuration, it combine with\n\nthe standard pipe de-emphasis."]
    #[inline(always)]
    pub fn tx_deemphasis_ext(&self) -> TxDeemphasisExtR {
        TxDeemphasisExtR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - PIPE bypass codec configuration\n\nControls whether the PHY performs 8b/10b encode and decode:"]
    #[inline(always)]
    pub fn bypass_codec(&self) -> BypassCodecR {
        BypassCodecR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - PCIe phy receiver control\n\nControls whether the PHY RX is active when the PHY is in P0 or\n\nP0s states."]
    #[inline(always)]
    pub fn rx_standby(&self) -> RxStandbyR {
        RxStandbyR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PCIe target non posted reject"]
    #[inline(always)]
    #[must_use]
    pub fn non_posted_rej(&mut self) -> NonPostedRejW<PcieClientSideBandCtrlSpec> {
        NonPostedRejW::new(self, 0)
    }
    #[doc = "Bits 4:5 - PIPE phy extended de-emphasis configuration, it combine with\n\nthe standard pipe de-emphasis."]
    #[inline(always)]
    #[must_use]
    pub fn tx_deemphasis_ext(&mut self) -> TxDeemphasisExtW<PcieClientSideBandCtrlSpec> {
        TxDeemphasisExtW::new(self, 4)
    }
    #[doc = "Bit 6 - PIPE bypass codec configuration\n\nControls whether the PHY performs 8b/10b encode and decode:"]
    #[inline(always)]
    #[must_use]
    pub fn bypass_codec(&mut self) -> BypassCodecW<PcieClientSideBandCtrlSpec> {
        BypassCodecW::new(self, 6)
    }
    #[doc = "Bits 8:11 - PCIe phy receiver control\n\nControls whether the PHY RX is active when the PHY is in P0 or\n\nP0s states."]
    #[inline(always)]
    #[must_use]
    pub fn rx_standby(&mut self) -> RxStandbyW<PcieClientSideBandCtrlSpec> {
        RxStandbyW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Write mask\n\nFor each served bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<PcieClientSideBandCtrlSpec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Side band control configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_side_band_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_client_side_band_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientSideBandCtrlSpec;
impl crate::RegisterSpec for PcieClientSideBandCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_client_side_band_ctrl::R`](R) reader structure"]
impl crate::Readable for PcieClientSideBandCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_client_side_band_ctrl::W`](W) writer structure"]
impl crate::Writable for PcieClientSideBandCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_CLIENT_SIDE_BAND_CTRL to value 0x04"]
impl crate::Resettable for PcieClientSideBandCtrlSpec {
    const RESET_VALUE: u32 = 0x04;
}
