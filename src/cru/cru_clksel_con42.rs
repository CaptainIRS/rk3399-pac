#[doc = "Register `CRU_CLKSEL_CON42` reader"]
pub type R = crate::R<CruClkselCon42Spec>;
#[doc = "Register `CRU_CLKSEL_CON42` writer"]
pub type W = crate::W<CruClkselCon42Spec>;
#[doc = "Field `ACLK_VIO_DIV_CON` reader - aclk_vio divider control register clk=clk_src/(div_con+1)"]
pub type AclkVioDivConR = crate::FieldReader;
#[doc = "Field `ACLK_VIO_DIV_CON` writer - aclk_vio divider control register clk=clk_src/(div_con+1)"]
pub type AclkVioDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "aclk_vio clock source select control register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AclkVioPllSel {
    #[doc = "0: PPLL"]
    B00 = 0,
    #[doc = "1: PPLL"]
    B01 = 1,
    #[doc = "2: PPLL"]
    B10 = 2,
}
impl From<AclkVioPllSel> for u8 {
    #[inline(always)]
    fn from(variant: AclkVioPllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AclkVioPllSel {
    type Ux = u8;
}
#[doc = "Field `ACLK_VIO_PLL_SEL` reader - aclk_vio clock source select control register"]
pub type AclkVioPllSelR = crate::FieldReader<AclkVioPllSel>;
impl AclkVioPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AclkVioPllSel> {
        match self.bits {
            0 => Some(AclkVioPllSel::B00),
            1 => Some(AclkVioPllSel::B01),
            2 => Some(AclkVioPllSel::B10),
            _ => None,
        }
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == AclkVioPllSel::B00
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == AclkVioPllSel::B01
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == AclkVioPllSel::B10
    }
}
#[doc = "Field `ACLK_VIO_PLL_SEL` writer - aclk_vio clock source select control register"]
pub type AclkVioPllSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, AclkVioPllSel>;
impl<'a, REG> AclkVioPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(AclkVioPllSel::B00)
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(AclkVioPllSel::B01)
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(AclkVioPllSel::B10)
    }
}
#[doc = "Field `ACLK_HDCP_DIV_CON` reader - aclk_hdcp divider control register clk=clk_src/(div_con+1)"]
pub type AclkHdcpDivConR = crate::FieldReader;
#[doc = "Field `ACLK_HDCP_DIV_CON` writer - aclk_hdcp divider control register clk=clk_src/(div_con+1)"]
pub type AclkHdcpDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "aclk_hdcp clock source select control register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AclkHdcpPllSel {
    #[doc = "0: PPLL"]
    B00 = 0,
    #[doc = "1: PPLL"]
    B01 = 1,
    #[doc = "2: PPLL"]
    B10 = 2,
}
impl From<AclkHdcpPllSel> for u8 {
    #[inline(always)]
    fn from(variant: AclkHdcpPllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AclkHdcpPllSel {
    type Ux = u8;
}
#[doc = "Field `ACLK_HDCP_PLL_SEL` reader - aclk_hdcp clock source select control register"]
pub type AclkHdcpPllSelR = crate::FieldReader<AclkHdcpPllSel>;
impl AclkHdcpPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AclkHdcpPllSel> {
        match self.bits {
            0 => Some(AclkHdcpPllSel::B00),
            1 => Some(AclkHdcpPllSel::B01),
            2 => Some(AclkHdcpPllSel::B10),
            _ => None,
        }
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == AclkHdcpPllSel::B00
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == AclkHdcpPllSel::B01
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == AclkHdcpPllSel::B10
    }
}
#[doc = "Field `ACLK_HDCP_PLL_SEL` writer - aclk_hdcp clock source select control register"]
pub type AclkHdcpPllSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, AclkHdcpPllSel>;
impl<'a, REG> AclkHdcpPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(AclkHdcpPllSel::B00)
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(AclkHdcpPllSel::B01)
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(AclkHdcpPllSel::B10)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - aclk_vio divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn aclk_vio_div_con(&self) -> AclkVioDivConR {
        AclkVioDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - aclk_vio clock source select control register"]
    #[inline(always)]
    pub fn aclk_vio_pll_sel(&self) -> AclkVioPllSelR {
        AclkVioPllSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - aclk_hdcp divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn aclk_hdcp_div_con(&self) -> AclkHdcpDivConR {
        AclkHdcpDivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 14:15 - aclk_hdcp clock source select control register"]
    #[inline(always)]
    pub fn aclk_hdcp_pll_sel(&self) -> AclkHdcpPllSelR {
        AclkHdcpPllSelR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - aclk_vio divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_vio_div_con(&mut self) -> AclkVioDivConW<CruClkselCon42Spec> {
        AclkVioDivConW::new(self, 0)
    }
    #[doc = "Bits 6:7 - aclk_vio clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_vio_pll_sel(&mut self) -> AclkVioPllSelW<CruClkselCon42Spec> {
        AclkVioPllSelW::new(self, 6)
    }
    #[doc = "Bits 8:12 - aclk_hdcp divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_hdcp_div_con(&mut self) -> AclkHdcpDivConW<CruClkselCon42Spec> {
        AclkHdcpDivConW::new(self, 8)
    }
    #[doc = "Bits 14:15 - aclk_hdcp clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_hdcp_pll_sel(&mut self) -> AclkHdcpPllSelW<CruClkselCon42Spec> {
        AclkHdcpPllSelW::new(self, 14)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon42Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register42\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con42::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con42::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon42Spec;
impl crate::RegisterSpec for CruClkselCon42Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con42::R`](R) reader structure"]
impl crate::Readable for CruClkselCon42Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con42::W`](W) writer structure"]
impl crate::Writable for CruClkselCon42Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON42 to value 0x0101"]
impl crate::Resettable for CruClkselCon42Spec {
    const RESET_VALUE: u32 = 0x0101;
}
