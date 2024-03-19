#[doc = "Register `CRU_CLKSEL_CON49` reader"]
pub type R = crate::R<CruClkselCon49Spec>;
#[doc = "Register `CRU_CLKSEL_CON49` writer"]
pub type W = crate::W<CruClkselCon49Spec>;
#[doc = "Field `DCLK_VOP0_DIV_CON` reader - dclk_vop0 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type DclkVop0DivConR = crate::FieldReader;
#[doc = "Field `DCLK_VOP0_DIV_CON` writer - dclk_vop0 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type DclkVop0DivConW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "dclk_vop0 clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DclkVop0PllSel {
    #[doc = "0: VPLL"]
    B00 = 0,
    #[doc = "1: CPLL"]
    B01 = 1,
    #[doc = "2: GPLL"]
    B1x = 2,
}
impl From<DclkVop0PllSel> for u8 {
    #[inline(always)]
    fn from(variant: DclkVop0PllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DclkVop0PllSel {
    type Ux = u8;
}
#[doc = "Field `DCLK_VOP0_PLL_SEL` reader - dclk_vop0 clock source select control register"]
pub type DclkVop0PllSelR = crate::FieldReader<DclkVop0PllSel>;
impl DclkVop0PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DclkVop0PllSel> {
        match self.bits {
            0 => Some(DclkVop0PllSel::B00),
            1 => Some(DclkVop0PllSel::B01),
            2 => Some(DclkVop0PllSel::B1x),
            _ => None,
        }
    }
    #[doc = "VPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == DclkVop0PllSel::B00
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == DclkVop0PllSel::B01
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1x(&self) -> bool {
        *self == DclkVop0PllSel::B1x
    }
}
#[doc = "Field `DCLK_VOP0_PLL_SEL` writer - dclk_vop0 clock source select control register"]
pub type DclkVop0PllSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, DclkVop0PllSel>;
impl<'a, REG> DclkVop0PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(DclkVop0PllSel::B00)
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(DclkVop0PllSel::B01)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1x(self) -> &'a mut crate::W<REG> {
        self.variant(DclkVop0PllSel::B1x)
    }
}
#[doc = "dclk_vop0 clock select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DclkVop0DclkSel {
    #[doc = "0: dclk_vop_divout"]
    B0 = 0,
    #[doc = "1: dclk_vop_frac"]
    B1 = 1,
}
impl From<DclkVop0DclkSel> for bool {
    #[inline(always)]
    fn from(variant: DclkVop0DclkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCLK_VOP0_DCLK_SEL` reader - dclk_vop0 clock select control register"]
pub type DclkVop0DclkSelR = crate::BitReader<DclkVop0DclkSel>;
impl DclkVop0DclkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DclkVop0DclkSel {
        match self.bits {
            false => DclkVop0DclkSel::B0,
            true => DclkVop0DclkSel::B1,
        }
    }
    #[doc = "dclk_vop_divout"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DclkVop0DclkSel::B0
    }
    #[doc = "dclk_vop_frac"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DclkVop0DclkSel::B1
    }
}
#[doc = "Field `DCLK_VOP0_DCLK_SEL` writer - dclk_vop0 clock select control register"]
pub type DclkVop0DclkSelW<'a, REG> = crate::BitWriter<'a, REG, DclkVop0DclkSel>;
impl<'a, REG> DclkVop0DclkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "dclk_vop_divout"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DclkVop0DclkSel::B0)
    }
    #[doc = "dclk_vop_frac"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DclkVop0DclkSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - dclk_vop0 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn dclk_vop0_div_con(&self) -> DclkVop0DivConR {
        DclkVop0DivConR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - dclk_vop0 clock source select control register"]
    #[inline(always)]
    pub fn dclk_vop0_pll_sel(&self) -> DclkVop0PllSelR {
        DclkVop0PllSelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - dclk_vop0 clock select control register"]
    #[inline(always)]
    pub fn dclk_vop0_dclk_sel(&self) -> DclkVop0DclkSelR {
        DclkVop0DclkSelR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - dclk_vop0 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn dclk_vop0_div_con(&mut self) -> DclkVop0DivConW<CruClkselCon49Spec> {
        DclkVop0DivConW::new(self, 0)
    }
    #[doc = "Bits 8:9 - dclk_vop0 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn dclk_vop0_pll_sel(&mut self) -> DclkVop0PllSelW<CruClkselCon49Spec> {
        DclkVop0PllSelW::new(self, 8)
    }
    #[doc = "Bit 11 - dclk_vop0 clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn dclk_vop0_dclk_sel(&mut self) -> DclkVop0DclkSelW<CruClkselCon49Spec> {
        DclkVop0DclkSelW::new(self, 11)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon49Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register49\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con49::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con49::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon49Spec;
impl crate::RegisterSpec for CruClkselCon49Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con49::R`](R) reader structure"]
impl crate::Readable for CruClkselCon49Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con49::W`](W) writer structure"]
impl crate::Writable for CruClkselCon49Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON49 to value 0x01"]
impl crate::Resettable for CruClkselCon49Spec {
    const RESET_VALUE: u32 = 0x01;
}
