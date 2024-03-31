#[doc = "Register `CLKSEL_CON50` reader"]
pub type R = crate::R<ClkselCon50Spec>;
#[doc = "Register `CLKSEL_CON50` writer"]
pub type W = crate::W<ClkselCon50Spec>;
#[doc = "Field `DCLK_VOP1_DIV_CON` reader - vop1 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type DclkVop1DivConR = crate::FieldReader;
#[doc = "Field `DCLK_VOP1_DIV_CON` writer - vop1 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type DclkVop1DivConW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "dclk_vop1 clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DclkVop1PllSel {
    #[doc = "0: VPLL"]
    B00 = 0,
    #[doc = "1: CPLL"]
    B01 = 1,
    #[doc = "2: GPLL"]
    B1x = 2,
}
impl From<DclkVop1PllSel> for u8 {
    #[inline(always)]
    fn from(variant: DclkVop1PllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DclkVop1PllSel {
    type Ux = u8;
}
#[doc = "Field `DCLK_VOP1_PLL_SEL` reader - dclk_vop1 clock source select control register"]
pub type DclkVop1PllSelR = crate::FieldReader<DclkVop1PllSel>;
impl DclkVop1PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DclkVop1PllSel> {
        match self.bits {
            0 => Some(DclkVop1PllSel::B00),
            1 => Some(DclkVop1PllSel::B01),
            2 => Some(DclkVop1PllSel::B1x),
            _ => None,
        }
    }
    #[doc = "VPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == DclkVop1PllSel::B00
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == DclkVop1PllSel::B01
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1x(&self) -> bool {
        *self == DclkVop1PllSel::B1x
    }
}
#[doc = "Field `DCLK_VOP1_PLL_SEL` writer - dclk_vop1 clock source select control register"]
pub type DclkVop1PllSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, DclkVop1PllSel>;
impl<'a, REG> DclkVop1PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(DclkVop1PllSel::B00)
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(DclkVop1PllSel::B01)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1x(self) -> &'a mut crate::W<REG> {
        self.variant(DclkVop1PllSel::B1x)
    }
}
#[doc = "dclk_vop1 clock select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DclkVop1DclkSel {
    #[doc = "0: dclk_vop_divout"]
    B0 = 0,
    #[doc = "1: dclk_vop_frac"]
    B1 = 1,
}
impl From<DclkVop1DclkSel> for bool {
    #[inline(always)]
    fn from(variant: DclkVop1DclkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCLK_VOP1_DCLK_SEL` reader - dclk_vop1 clock select control register"]
pub type DclkVop1DclkSelR = crate::BitReader<DclkVop1DclkSel>;
impl DclkVop1DclkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DclkVop1DclkSel {
        match self.bits {
            false => DclkVop1DclkSel::B0,
            true => DclkVop1DclkSel::B1,
        }
    }
    #[doc = "dclk_vop_divout"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DclkVop1DclkSel::B0
    }
    #[doc = "dclk_vop_frac"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DclkVop1DclkSel::B1
    }
}
#[doc = "Field `DCLK_VOP1_DCLK_SEL` writer - dclk_vop1 clock select control register"]
pub type DclkVop1DclkSelW<'a, REG> = crate::BitWriter<'a, REG, DclkVop1DclkSel>;
impl<'a, REG> DclkVop1DclkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "dclk_vop_divout"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DclkVop1DclkSel::B0)
    }
    #[doc = "dclk_vop_frac"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DclkVop1DclkSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - vop1 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn dclk_vop1_div_con(&self) -> DclkVop1DivConR {
        DclkVop1DivConR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - dclk_vop1 clock source select control register"]
    #[inline(always)]
    pub fn dclk_vop1_pll_sel(&self) -> DclkVop1PllSelR {
        DclkVop1PllSelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - dclk_vop1 clock select control register"]
    #[inline(always)]
    pub fn dclk_vop1_dclk_sel(&self) -> DclkVop1DclkSelR {
        DclkVop1DclkSelR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - vop1 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn dclk_vop1_div_con(&mut self) -> DclkVop1DivConW<ClkselCon50Spec> {
        DclkVop1DivConW::new(self, 0)
    }
    #[doc = "Bits 8:9 - dclk_vop1 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn dclk_vop1_pll_sel(&mut self) -> DclkVop1PllSelW<ClkselCon50Spec> {
        DclkVop1PllSelW::new(self, 8)
    }
    #[doc = "Bit 11 - dclk_vop1 clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn dclk_vop1_dclk_sel(&mut self) -> DclkVop1DclkSelW<ClkselCon50Spec> {
        DclkVop1DclkSelW::new(self, 11)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkselCon50Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register50\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con50::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con50::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselCon50Spec;
impl crate::RegisterSpec for ClkselCon50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel_con50::R`](R) reader structure"]
impl crate::Readable for ClkselCon50Spec {}
#[doc = "`write(|w| ..)` method takes [`clksel_con50::W`](W) writer structure"]
impl crate::Writable for ClkselCon50Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL_CON50 to value 0x03"]
impl crate::Resettable for ClkselCon50Spec {
    const RESET_VALUE: u32 = 0x03;
}
