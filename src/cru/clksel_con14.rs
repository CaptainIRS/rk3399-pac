#[doc = "Register `CLKSEL_CON14` reader"]
pub type R = crate::R<ClkselCon14Spec>;
#[doc = "Register `CLKSEL_CON14` writer"]
pub type W = crate::W<ClkselCon14Spec>;
#[doc = "Field `ACLK_PERIHP_DIV_CON` reader - aclk_perihp divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AclkPerihpDivConR = crate::FieldReader;
#[doc = "Field `ACLK_PERIHP_DIV_CON` writer - aclk_perihp divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AclkPerihpDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "clk_usbphy_480m clock channel select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkUsbphy480mChSel {
    #[doc = "0: usb_phy0_480m"]
    B0 = 0,
    #[doc = "1: usb_phy1_480m"]
    B1 = 1,
}
impl From<ClkUsbphy480mChSel> for bool {
    #[inline(always)]
    fn from(variant: ClkUsbphy480mChSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_USBPHY_480M_CH_SEL` reader - clk_usbphy_480m clock channel select control register"]
pub type ClkUsbphy480mChSelR = crate::BitReader<ClkUsbphy480mChSel>;
impl ClkUsbphy480mChSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkUsbphy480mChSel {
        match self.bits {
            false => ClkUsbphy480mChSel::B0,
            true => ClkUsbphy480mChSel::B1,
        }
    }
    #[doc = "usb_phy0_480m"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkUsbphy480mChSel::B0
    }
    #[doc = "usb_phy1_480m"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkUsbphy480mChSel::B1
    }
}
#[doc = "Field `CLK_USBPHY_480M_CH_SEL` writer - clk_usbphy_480m clock channel select control register"]
pub type ClkUsbphy480mChSelW<'a, REG> = crate::BitWriter<'a, REG, ClkUsbphy480mChSel>;
impl<'a, REG> ClkUsbphy480mChSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "usb_phy0_480m"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUsbphy480mChSel::B0)
    }
    #[doc = "usb_phy1_480m"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUsbphy480mChSel::B1)
    }
}
#[doc = "aclk_perihp clock source select control register\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AclkPerihpPllSel {
    #[doc = "0: CPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<AclkPerihpPllSel> for bool {
    #[inline(always)]
    fn from(variant: AclkPerihpPllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACLK_PERIHP_PLL_SEL` reader - aclk_perihp clock source select control register"]
pub type AclkPerihpPllSelR = crate::BitReader<AclkPerihpPllSel>;
impl AclkPerihpPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AclkPerihpPllSel {
        match self.bits {
            false => AclkPerihpPllSel::B0,
            true => AclkPerihpPllSel::B1,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AclkPerihpPllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AclkPerihpPllSel::B1
    }
}
#[doc = "Field `ACLK_PERIHP_PLL_SEL` writer - aclk_perihp clock source select control register"]
pub type AclkPerihpPllSelW<'a, REG> = crate::BitWriter<'a, REG, AclkPerihpPllSel>;
impl<'a, REG> AclkPerihpPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AclkPerihpPllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AclkPerihpPllSel::B1)
    }
}
#[doc = "Field `HCLK_PERIHP_DIV_CON` reader - hclk_perihp divider control register\n\nclk=aclk_perihp/(div_con+1)"]
pub type HclkPerihpDivConR = crate::FieldReader;
#[doc = "Field `HCLK_PERIHP_DIV_CON` writer - hclk_perihp divider control register\n\nclk=aclk_perihp/(div_con+1)"]
pub type HclkPerihpDivConW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCLK_PERIHP_DIV_CON` reader - pclk_perihp divider control register\n\nclk=aclk_perihp/(div_con+1)"]
pub type PclkPerihpDivConR = crate::FieldReader;
#[doc = "Field `PCLK_PERIHP_DIV_CON` writer - pclk_perihp divider control register\n\nclk=aclk_perihp/(div_con+1)"]
pub type PclkPerihpDivConW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "clk_usbpll_480m_sel clock select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkUsbpll480mSel {
    #[doc = "0: xin_24m"]
    B0 = 0,
    #[doc = "1: clk_usbphy_480m"]
    B1 = 1,
}
impl From<ClkUsbpll480mSel> for bool {
    #[inline(always)]
    fn from(variant: ClkUsbpll480mSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_USBPLL_480M_SEL` reader - clk_usbpll_480m_sel clock select control register"]
pub type ClkUsbpll480mSelR = crate::BitReader<ClkUsbpll480mSel>;
impl ClkUsbpll480mSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkUsbpll480mSel {
        match self.bits {
            false => ClkUsbpll480mSel::B0,
            true => ClkUsbpll480mSel::B1,
        }
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkUsbpll480mSel::B0
    }
    #[doc = "clk_usbphy_480m"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkUsbpll480mSel::B1
    }
}
#[doc = "Field `CLK_USBPLL_480M_SEL` writer - clk_usbpll_480m_sel clock select control register"]
pub type ClkUsbpll480mSelW<'a, REG> = crate::BitWriter<'a, REG, ClkUsbpll480mSel>;
impl<'a, REG> ClkUsbpll480mSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUsbpll480mSel::B0)
    }
    #[doc = "clk_usbphy_480m"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUsbpll480mSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - aclk_perihp divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn aclk_perihp_div_con(&self) -> AclkPerihpDivConR {
        AclkPerihpDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - clk_usbphy_480m clock channel select control register"]
    #[inline(always)]
    pub fn clk_usbphy_480m_ch_sel(&self) -> ClkUsbphy480mChSelR {
        ClkUsbphy480mChSelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - aclk_perihp clock source select control register"]
    #[inline(always)]
    pub fn aclk_perihp_pll_sel(&self) -> AclkPerihpPllSelR {
        AclkPerihpPllSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - hclk_perihp divider control register\n\nclk=aclk_perihp/(div_con+1)"]
    #[inline(always)]
    pub fn hclk_perihp_div_con(&self) -> HclkPerihpDivConR {
        HclkPerihpDivConR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:14 - pclk_perihp divider control register\n\nclk=aclk_perihp/(div_con+1)"]
    #[inline(always)]
    pub fn pclk_perihp_div_con(&self) -> PclkPerihpDivConR {
        PclkPerihpDivConR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - clk_usbpll_480m_sel clock select control register"]
    #[inline(always)]
    pub fn clk_usbpll_480m_sel(&self) -> ClkUsbpll480mSelR {
        ClkUsbpll480mSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - aclk_perihp divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_perihp_div_con(&mut self) -> AclkPerihpDivConW<ClkselCon14Spec> {
        AclkPerihpDivConW::new(self, 0)
    }
    #[doc = "Bit 6 - clk_usbphy_480m clock channel select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_usbphy_480m_ch_sel(&mut self) -> ClkUsbphy480mChSelW<ClkselCon14Spec> {
        ClkUsbphy480mChSelW::new(self, 6)
    }
    #[doc = "Bit 7 - aclk_perihp clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_perihp_pll_sel(&mut self) -> AclkPerihpPllSelW<ClkselCon14Spec> {
        AclkPerihpPllSelW::new(self, 7)
    }
    #[doc = "Bits 8:9 - hclk_perihp divider control register\n\nclk=aclk_perihp/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_perihp_div_con(&mut self) -> HclkPerihpDivConW<ClkselCon14Spec> {
        HclkPerihpDivConW::new(self, 8)
    }
    #[doc = "Bits 12:14 - pclk_perihp divider control register\n\nclk=aclk_perihp/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_perihp_div_con(&mut self) -> PclkPerihpDivConW<ClkselCon14Spec> {
        PclkPerihpDivConW::new(self, 12)
    }
    #[doc = "Bit 15 - clk_usbpll_480m_sel clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_usbpll_480m_sel(&mut self) -> ClkUsbpll480mSelW<ClkselCon14Spec> {
        ClkUsbpll480mSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkselCon14Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselCon14Spec;
impl crate::RegisterSpec for ClkselCon14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel_con14::R`](R) reader structure"]
impl crate::Readable for ClkselCon14Spec {}
#[doc = "`write(|w| ..)` method takes [`clksel_con14::W`](W) writer structure"]
impl crate::Writable for ClkselCon14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL_CON14 to value 0x3181"]
impl crate::Resettable for ClkselCon14Spec {
    const RESET_VALUE: u32 = 0x3181;
}
