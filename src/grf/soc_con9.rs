#[doc = "Register `SOC_CON9` reader"]
pub type R = crate::R<SocCon9Spec>;
#[doc = "Register `SOC_CON9` writer"]
pub type W = crate::W<SocCon9Spec>;
#[doc = "Field `DPHY_RX0_TURNREQUEST` reader - dphy rx0 runrequest port control"]
pub type DphyRx0TurnrequestR = crate::FieldReader;
#[doc = "Field `DPHY_RX0_TURNREQUEST` writer - dphy rx0 runrequest port control"]
pub type DphyRx0TurnrequestW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DISABLE_ISP0` reader - isp0 disable control"]
pub type DisableIsp0R = crate::BitReader;
#[doc = "Field `DISABLE_ISP0` writer - isp0 disable control"]
pub type DisableIsp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLE_ISP1` reader - isp1 disable control"]
pub type DisableIsp1R = crate::BitReader;
#[doc = "Field `DISABLE_ISP1` writer - isp1 disable control"]
pub type DisableIsp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPHY_RX0_CLK_INV_SEL` reader - dphy rx0 clock inveter select bit"]
pub type DphyRx0ClkInvSelR = crate::BitReader;
#[doc = "Field `DPHY_RX0_CLK_INV_SEL` writer - dphy rx0 clock inveter select bit"]
pub type DphyRx0ClkInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPHY_RX1_CLK_INV_SEL` reader - dphy rx1 clock inveter select bit"]
pub type DphyRx1ClkInvSelR = crate::BitReader;
#[doc = "Field `DPHY_RX1_CLK_INV_SEL` writer - dphy rx1 clock inveter select bit"]
pub type DphyRx1ClkInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "dp lcdc select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DpLcdcSel {
    #[doc = "0: vop big"]
    B0 = 0,
    #[doc = "1: vop little"]
    B1 = 1,
}
impl From<DpLcdcSel> for bool {
    #[inline(always)]
    fn from(variant: DpLcdcSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DP_LCDC_SEL` reader - dp lcdc select"]
pub type DpLcdcSelR = crate::BitReader<DpLcdcSel>;
impl DpLcdcSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DpLcdcSel {
        match self.bits {
            false => DpLcdcSel::B0,
            true => DpLcdcSel::B1,
        }
    }
    #[doc = "vop big"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DpLcdcSel::B0
    }
    #[doc = "vop little"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DpLcdcSel::B1
    }
}
#[doc = "Field `DP_LCDC_SEL` writer - dp lcdc select"]
pub type DpLcdcSelW<'a, REG> = crate::BitWriter<'a, REG, DpLcdcSel>;
impl<'a, REG> DpLcdcSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "vop big"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DpLcdcSel::B0)
    }
    #[doc = "vop little"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DpLcdcSel::B1)
    }
}
#[doc = "Field `DSI0_DPICOLORM` reader - DSI0 dpicolorm bit control"]
pub type Dsi0DpicolormR = crate::BitReader;
#[doc = "Field `DSI0_DPICOLORM` writer - DSI0 dpicolorm bit control"]
pub type Dsi0DpicolormW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI0_DPISHUTDN` reader - DSI0 dpishutdn bit control"]
pub type Dsi0DpishutdnR = crate::BitReader;
#[doc = "Field `DSI0_DPISHUTDN` writer - DSI0 dpishutdn bit control"]
pub type Dsi0DpishutdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI0_DPIUPDATECFG` reader - DSI host0 dpiupdatecfg bit control"]
pub type Dsi0DpiupdatecfgR = crate::BitReader;
#[doc = "Field `DSI0_DPIUPDATECFG` writer - DSI host0 dpiupdatecfg bit control"]
pub type Dsi0DpiupdatecfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - dphy rx0 runrequest port control"]
    #[inline(always)]
    pub fn dphy_rx0_turnrequest(&self) -> DphyRx0TurnrequestR {
        DphyRx0TurnrequestR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - isp0 disable control"]
    #[inline(always)]
    pub fn disable_isp0(&self) -> DisableIsp0R {
        DisableIsp0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - isp1 disable control"]
    #[inline(always)]
    pub fn disable_isp1(&self) -> DisableIsp1R {
        DisableIsp1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - dphy rx0 clock inveter select bit"]
    #[inline(always)]
    pub fn dphy_rx0_clk_inv_sel(&self) -> DphyRx0ClkInvSelR {
        DphyRx0ClkInvSelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - dphy rx1 clock inveter select bit"]
    #[inline(always)]
    pub fn dphy_rx1_clk_inv_sel(&self) -> DphyRx1ClkInvSelR {
        DphyRx1ClkInvSelR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - dp lcdc select"]
    #[inline(always)]
    pub fn dp_lcdc_sel(&self) -> DpLcdcSelR {
        DpLcdcSelR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DSI0 dpicolorm bit control"]
    #[inline(always)]
    pub fn dsi0_dpicolorm(&self) -> Dsi0DpicolormR {
        Dsi0DpicolormR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DSI0 dpishutdn bit control"]
    #[inline(always)]
    pub fn dsi0_dpishutdn(&self) -> Dsi0DpishutdnR {
        Dsi0DpishutdnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DSI host0 dpiupdatecfg bit control"]
    #[inline(always)]
    pub fn dsi0_dpiupdatecfg(&self) -> Dsi0DpiupdatecfgR {
        Dsi0DpiupdatecfgR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - dphy rx0 runrequest port control"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_rx0_turnrequest(&mut self) -> DphyRx0TurnrequestW<SocCon9Spec> {
        DphyRx0TurnrequestW::new(self, 0)
    }
    #[doc = "Bit 8 - isp0 disable control"]
    #[inline(always)]
    #[must_use]
    pub fn disable_isp0(&mut self) -> DisableIsp0W<SocCon9Spec> {
        DisableIsp0W::new(self, 8)
    }
    #[doc = "Bit 9 - isp1 disable control"]
    #[inline(always)]
    #[must_use]
    pub fn disable_isp1(&mut self) -> DisableIsp1W<SocCon9Spec> {
        DisableIsp1W::new(self, 9)
    }
    #[doc = "Bit 10 - dphy rx0 clock inveter select bit"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_rx0_clk_inv_sel(&mut self) -> DphyRx0ClkInvSelW<SocCon9Spec> {
        DphyRx0ClkInvSelW::new(self, 10)
    }
    #[doc = "Bit 11 - dphy rx1 clock inveter select bit"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_rx1_clk_inv_sel(&mut self) -> DphyRx1ClkInvSelW<SocCon9Spec> {
        DphyRx1ClkInvSelW::new(self, 11)
    }
    #[doc = "Bit 12 - dp lcdc select"]
    #[inline(always)]
    #[must_use]
    pub fn dp_lcdc_sel(&mut self) -> DpLcdcSelW<SocCon9Spec> {
        DpLcdcSelW::new(self, 12)
    }
    #[doc = "Bit 13 - DSI0 dpicolorm bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dsi0_dpicolorm(&mut self) -> Dsi0DpicolormW<SocCon9Spec> {
        Dsi0DpicolormW::new(self, 13)
    }
    #[doc = "Bit 14 - DSI0 dpishutdn bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dsi0_dpishutdn(&mut self) -> Dsi0DpishutdnW<SocCon9Spec> {
        Dsi0DpishutdnW::new(self, 14)
    }
    #[doc = "Bit 15 - DSI host0 dpiupdatecfg bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dsi0_dpiupdatecfg(&mut self) -> Dsi0DpiupdatecfgW<SocCon9Spec> {
        Dsi0DpiupdatecfgW::new(self, 15)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<SocCon9Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "SoC control register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SocCon9Spec;
impl crate::RegisterSpec for SocCon9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soc_con9::R`](R) reader structure"]
impl crate::Readable for SocCon9Spec {}
#[doc = "`write(|w| ..)` method takes [`soc_con9::W`](W) writer structure"]
impl crate::Writable for SocCon9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOC_CON9 to value 0"]
impl crate::Resettable for SocCon9Spec {
    const RESET_VALUE: u32 = 0;
}
