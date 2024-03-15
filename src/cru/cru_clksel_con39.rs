#[doc = "Register `CRU_CLKSEL_CON39` reader"]
pub type R = crate::R<CruClkselCon39Spec>;
#[doc = "Register `CRU_CLKSEL_CON39` writer"]
pub type W = crate::W<CruClkselCon39Spec>;
#[doc = "Field `ACLK_USB3_DIV_CON` reader - aclk_usb3 divider control register clk=clk_src/(div_con+1)"]
pub type AclkUsb3DivConR = crate::FieldReader;
#[doc = "Field `ACLK_USB3_DIV_CON` writer - aclk_usb3 divider control register clk=clk_src/(div_con+1)"]
pub type AclkUsb3DivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "aclk_usb3 clock source select control register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AclkUsb3PllSel {
    #[doc = "0: NPLL"]
    B00 = 0,
    #[doc = "1: NPLL"]
    B01 = 1,
    #[doc = "2: NPLL"]
    B1x = 2,
}
impl From<AclkUsb3PllSel> for u8 {
    #[inline(always)]
    fn from(variant: AclkUsb3PllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AclkUsb3PllSel {
    type Ux = u8;
}
#[doc = "Field `ACLK_USB3_PLL_SEL` reader - aclk_usb3 clock source select control register"]
pub type AclkUsb3PllSelR = crate::FieldReader<AclkUsb3PllSel>;
impl AclkUsb3PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AclkUsb3PllSel> {
        match self.bits {
            0 => Some(AclkUsb3PllSel::B00),
            1 => Some(AclkUsb3PllSel::B01),
            2 => Some(AclkUsb3PllSel::B1x),
            _ => None,
        }
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == AclkUsb3PllSel::B00
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == AclkUsb3PllSel::B01
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b1x(&self) -> bool {
        *self == AclkUsb3PllSel::B1x
    }
}
#[doc = "Field `ACLK_USB3_PLL_SEL` writer - aclk_usb3 clock source select control register"]
pub type AclkUsb3PllSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, AclkUsb3PllSel>;
impl<'a, REG> AclkUsb3PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(AclkUsb3PllSel::B00)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(AclkUsb3PllSel::B01)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b1x(self) -> &'a mut crate::W<REG> {
        self.variant(AclkUsb3PllSel::B1x)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - aclk_usb3 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn aclk_usb3_div_con(&self) -> AclkUsb3DivConR {
        AclkUsb3DivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - aclk_usb3 clock source select control register"]
    #[inline(always)]
    pub fn aclk_usb3_pll_sel(&self) -> AclkUsb3PllSelR {
        AclkUsb3PllSelR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - aclk_usb3 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_usb3_div_con(&mut self) -> AclkUsb3DivConW<CruClkselCon39Spec> {
        AclkUsb3DivConW::new(self, 0)
    }
    #[doc = "Bits 6:7 - aclk_usb3 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_usb3_pll_sel(&mut self) -> AclkUsb3PllSelW<CruClkselCon39Spec> {
        AclkUsb3PllSelW::new(self, 6)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon39Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register39\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con39::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con39::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon39Spec;
impl crate::RegisterSpec for CruClkselCon39Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con39::R`](R) reader structure"]
impl crate::Readable for CruClkselCon39Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con39::W`](W) writer structure"]
impl crate::Writable for CruClkselCon39Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON39 to value 0x41"]
impl crate::Resettable for CruClkselCon39Spec {
    const RESET_VALUE: u32 = 0x41;
}
