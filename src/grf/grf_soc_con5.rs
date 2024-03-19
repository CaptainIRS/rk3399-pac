#[doc = "Register `GRF_SOC_CON5` reader"]
pub type R = crate::R<GrfSocCon5Spec>;
#[doc = "Register `GRF_SOC_CON5` writer"]
pub type W = crate::W<GrfSocCon5Spec>;
#[doc = "RMII clock selection\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RmiiClkSel {
    #[doc = "1: 25MHz"]
    B1 = 1,
    #[doc = "0: 2.5MHz"]
    B0 = 0,
}
impl From<RmiiClkSel> for bool {
    #[inline(always)]
    fn from(variant: RmiiClkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMII_CLK_SEL` reader - RMII clock selection"]
pub type RmiiClkSelR = crate::BitReader<RmiiClkSel>;
impl RmiiClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RmiiClkSel {
        match self.bits {
            true => RmiiClkSel::B1,
            false => RmiiClkSel::B0,
        }
    }
    #[doc = "25MHz"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RmiiClkSel::B1
    }
    #[doc = "2.5MHz"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RmiiClkSel::B0
    }
}
#[doc = "Field `RMII_CLK_SEL` writer - RMII clock selection"]
pub type RmiiClkSelW<'a, REG> = crate::BitWriter<'a, REG, RmiiClkSel>;
impl<'a, REG> RmiiClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "25MHz"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RmiiClkSel::B1)
    }
    #[doc = "2.5MHz"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RmiiClkSel::B0)
    }
}
#[doc = "RGMII clock selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GmacClkSel {
    #[doc = "0: 125MHz"]
    B00 = 0,
    #[doc = "3: 25MHz"]
    B11 = 3,
    #[doc = "2: 2.5MHz"]
    B10 = 2,
}
impl From<GmacClkSel> for u8 {
    #[inline(always)]
    fn from(variant: GmacClkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GmacClkSel {
    type Ux = u8;
}
#[doc = "Field `GMAC_CLK_SEL` reader - RGMII clock selection"]
pub type GmacClkSelR = crate::FieldReader<GmacClkSel>;
impl GmacClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GmacClkSel> {
        match self.bits {
            0 => Some(GmacClkSel::B00),
            3 => Some(GmacClkSel::B11),
            2 => Some(GmacClkSel::B10),
            _ => None,
        }
    }
    #[doc = "125MHz"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == GmacClkSel::B00
    }
    #[doc = "25MHz"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == GmacClkSel::B11
    }
    #[doc = "2.5MHz"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == GmacClkSel::B10
    }
}
#[doc = "Field `GMAC_CLK_SEL` writer - RGMII clock selection"]
pub type GmacClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, GmacClkSel>;
impl<'a, REG> GmacClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "125MHz"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(GmacClkSel::B00)
    }
    #[doc = "25MHz"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(GmacClkSel::B11)
    }
    #[doc = "2.5MHz"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(GmacClkSel::B10)
    }
}
#[doc = "RMII mode selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RmiiMode {
    #[doc = "1: RMII mode"]
    B1 = 1,
    #[doc = "0: MII mode"]
    B0 = 0,
}
impl From<RmiiMode> for bool {
    #[inline(always)]
    fn from(variant: RmiiMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMII_MODE` reader - RMII mode selection"]
pub type RmiiModeR = crate::BitReader<RmiiMode>;
impl RmiiModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RmiiMode {
        match self.bits {
            true => RmiiMode::B1,
            false => RmiiMode::B0,
        }
    }
    #[doc = "RMII mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RmiiMode::B1
    }
    #[doc = "MII mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RmiiMode::B0
    }
}
#[doc = "Field `RMII_MODE` writer - RMII mode selection"]
pub type RmiiModeW<'a, REG> = crate::BitWriter<'a, REG, RmiiMode>;
impl<'a, REG> RmiiModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RMII mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RmiiMode::B1)
    }
    #[doc = "MII mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RmiiMode::B0)
    }
}
#[doc = "MAC speed\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GmacSpeed {
    #[doc = "1: 100-Mbps"]
    B1 = 1,
    #[doc = "0: 10-Mbps"]
    B0 = 0,
}
impl From<GmacSpeed> for bool {
    #[inline(always)]
    fn from(variant: GmacSpeed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GMAC_SPEED` reader - MAC speed"]
pub type GmacSpeedR = crate::BitReader<GmacSpeed>;
impl GmacSpeedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GmacSpeed {
        match self.bits {
            true => GmacSpeed::B1,
            false => GmacSpeed::B0,
        }
    }
    #[doc = "100-Mbps"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GmacSpeed::B1
    }
    #[doc = "10-Mbps"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GmacSpeed::B0
    }
}
#[doc = "Field `GMAC_SPEED` writer - MAC speed"]
pub type GmacSpeedW<'a, REG> = crate::BitWriter<'a, REG, GmacSpeed>;
impl<'a, REG> GmacSpeedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "100-Mbps"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GmacSpeed::B1)
    }
    #[doc = "10-Mbps"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GmacSpeed::B0)
    }
}
#[doc = "Field `GMAC_FLOWCTRL` reader - GMAC transmit flow control\n\nWhen set high, instructs the GMAC to transmit\n\nPAUSE Control frames in\n\nFull-duplex mode. In Half-duplex mode, the\n\nGMAC enables the Back-pressure\n\nfunction until this signal is made low again"]
pub type GmacFlowctrlR = crate::BitReader;
#[doc = "Field `GMAC_FLOWCTRL` writer - GMAC transmit flow control\n\nWhen set high, instructs the GMAC to transmit\n\nPAUSE Control frames in\n\nFull-duplex mode. In Half-duplex mode, the\n\nGMAC enables the Back-pressure\n\nfunction until this signal is made low again"]
pub type GmacFlowctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "PHY interface select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GmacPhyIntfSel {
    #[doc = "1: RGMII"]
    B001 = 1,
    #[doc = "4: RMII All others: Reserved"]
    B100 = 4,
}
impl From<GmacPhyIntfSel> for u8 {
    #[inline(always)]
    fn from(variant: GmacPhyIntfSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GmacPhyIntfSel {
    type Ux = u8;
}
#[doc = "Field `GMAC_PHY_INTF_SEL` reader - PHY interface select"]
pub type GmacPhyIntfSelR = crate::FieldReader<GmacPhyIntfSel>;
impl GmacPhyIntfSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GmacPhyIntfSel> {
        match self.bits {
            1 => Some(GmacPhyIntfSel::B001),
            4 => Some(GmacPhyIntfSel::B100),
            _ => None,
        }
    }
    #[doc = "RGMII"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == GmacPhyIntfSel::B001
    }
    #[doc = "RMII All others: Reserved"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == GmacPhyIntfSel::B100
    }
}
#[doc = "Field `GMAC_PHY_INTF_SEL` writer - PHY interface select"]
pub type GmacPhyIntfSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, GmacPhyIntfSel>;
impl<'a, REG> GmacPhyIntfSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RGMII"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(GmacPhyIntfSel::B001)
    }
    #[doc = "RMII All others: Reserved"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(GmacPhyIntfSel::B100)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 3 - RMII clock selection"]
    #[inline(always)]
    pub fn rmii_clk_sel(&self) -> RmiiClkSelR {
        RmiiClkSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - RGMII clock selection"]
    #[inline(always)]
    pub fn gmac_clk_sel(&self) -> GmacClkSelR {
        GmacClkSelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - RMII mode selection"]
    #[inline(always)]
    pub fn rmii_mode(&self) -> RmiiModeR {
        RmiiModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MAC speed"]
    #[inline(always)]
    pub fn gmac_speed(&self) -> GmacSpeedR {
        GmacSpeedR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GMAC transmit flow control\n\nWhen set high, instructs the GMAC to transmit\n\nPAUSE Control frames in\n\nFull-duplex mode. In Half-duplex mode, the\n\nGMAC enables the Back-pressure\n\nfunction until this signal is made low again"]
    #[inline(always)]
    pub fn gmac_flowctrl(&self) -> GmacFlowctrlR {
        GmacFlowctrlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - PHY interface select"]
    #[inline(always)]
    pub fn gmac_phy_intf_sel(&self) -> GmacPhyIntfSelR {
        GmacPhyIntfSelR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 3 - RMII clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn rmii_clk_sel(&mut self) -> RmiiClkSelW<GrfSocCon5Spec> {
        RmiiClkSelW::new(self, 3)
    }
    #[doc = "Bits 4:5 - RGMII clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn gmac_clk_sel(&mut self) -> GmacClkSelW<GrfSocCon5Spec> {
        GmacClkSelW::new(self, 4)
    }
    #[doc = "Bit 6 - RMII mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn rmii_mode(&mut self) -> RmiiModeW<GrfSocCon5Spec> {
        RmiiModeW::new(self, 6)
    }
    #[doc = "Bit 7 - MAC speed"]
    #[inline(always)]
    #[must_use]
    pub fn gmac_speed(&mut self) -> GmacSpeedW<GrfSocCon5Spec> {
        GmacSpeedW::new(self, 7)
    }
    #[doc = "Bit 8 - GMAC transmit flow control\n\nWhen set high, instructs the GMAC to transmit\n\nPAUSE Control frames in\n\nFull-duplex mode. In Half-duplex mode, the\n\nGMAC enables the Back-pressure\n\nfunction until this signal is made low again"]
    #[inline(always)]
    #[must_use]
    pub fn gmac_flowctrl(&mut self) -> GmacFlowctrlW<GrfSocCon5Spec> {
        GmacFlowctrlW::new(self, 8)
    }
    #[doc = "Bits 9:11 - PHY interface select"]
    #[inline(always)]
    #[must_use]
    pub fn gmac_phy_intf_sel(&mut self) -> GmacPhyIntfSelW<GrfSocCon5Spec> {
        GmacPhyIntfSelW::new(self, 9)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfSocCon5Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "SoC control register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfSocCon5Spec;
impl crate::RegisterSpec for GrfSocCon5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_soc_con5::R`](R) reader structure"]
impl crate::Readable for GrfSocCon5Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_soc_con5::W`](W) writer structure"]
impl crate::Writable for GrfSocCon5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_SOC_CON5 to value 0x08"]
impl crate::Resettable for GrfSocCon5Spec {
    const RESET_VALUE: u32 = 0x08;
}
