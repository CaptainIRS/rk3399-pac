#[doc = "Register `CLKSEL_CON47` reader"]
pub type R = crate::R<ClkselCon47Spec>;
#[doc = "Register `CLKSEL_CON47` writer"]
pub type W = crate::W<ClkselCon47Spec>;
#[doc = "Field `ACLK_VOP0_DIV_CON` reader - aclk_vop0 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AclkVop0DivConR = crate::FieldReader;
#[doc = "Field `ACLK_VOP0_DIV_CON` writer - aclk_vop0 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AclkVop0DivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "aclk_vop0 clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AclkVop0PllSel {
    #[doc = "0: VPLL"]
    B00 = 0,
    #[doc = "1: CPLL"]
    B01 = 1,
    #[doc = "2: GPLL"]
    B10 = 2,
    #[doc = "3: NPLL"]
    B11 = 3,
}
impl From<AclkVop0PllSel> for u8 {
    #[inline(always)]
    fn from(variant: AclkVop0PllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AclkVop0PllSel {
    type Ux = u8;
}
#[doc = "Field `ACLK_VOP0_PLL_SEL` reader - aclk_vop0 clock source select control register"]
pub type AclkVop0PllSelR = crate::FieldReader<AclkVop0PllSel>;
impl AclkVop0PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AclkVop0PllSel {
        match self.bits {
            0 => AclkVop0PllSel::B00,
            1 => AclkVop0PllSel::B01,
            2 => AclkVop0PllSel::B10,
            3 => AclkVop0PllSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "VPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == AclkVop0PllSel::B00
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == AclkVop0PllSel::B01
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == AclkVop0PllSel::B10
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == AclkVop0PllSel::B11
    }
}
#[doc = "Field `ACLK_VOP0_PLL_SEL` writer - aclk_vop0 clock source select control register"]
pub type AclkVop0PllSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, AclkVop0PllSel>;
impl<'a, REG> AclkVop0PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(AclkVop0PllSel::B00)
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(AclkVop0PllSel::B01)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(AclkVop0PllSel::B10)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(AclkVop0PllSel::B11)
    }
}
#[doc = "Field `HCLK_VOP0_DIV_CON` reader - hclk_vop0 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type HclkVop0DivConR = crate::FieldReader;
#[doc = "Field `HCLK_VOP0_DIV_CON` writer - hclk_vop0 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type HclkVop0DivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - aclk_vop0 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn aclk_vop0_div_con(&self) -> AclkVop0DivConR {
        AclkVop0DivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - aclk_vop0 clock source select control register"]
    #[inline(always)]
    pub fn aclk_vop0_pll_sel(&self) -> AclkVop0PllSelR {
        AclkVop0PllSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - hclk_vop0 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn hclk_vop0_div_con(&self) -> HclkVop0DivConR {
        HclkVop0DivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - aclk_vop0 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_vop0_div_con(&mut self) -> AclkVop0DivConW<ClkselCon47Spec> {
        AclkVop0DivConW::new(self, 0)
    }
    #[doc = "Bits 6:7 - aclk_vop0 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_vop0_pll_sel(&mut self) -> AclkVop0PllSelW<ClkselCon47Spec> {
        AclkVop0PllSelW::new(self, 6)
    }
    #[doc = "Bits 8:12 - hclk_vop0 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_vop0_div_con(&mut self) -> HclkVop0DivConW<ClkselCon47Spec> {
        HclkVop0DivConW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkselCon47Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register47\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con47::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con47::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselCon47Spec;
impl crate::RegisterSpec for ClkselCon47Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel_con47::R`](R) reader structure"]
impl crate::Readable for ClkselCon47Spec {}
#[doc = "`write(|w| ..)` method takes [`clksel_con47::W`](W) writer structure"]
impl crate::Writable for ClkselCon47Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL_CON47 to value 0x0102"]
impl crate::Resettable for ClkselCon47Spec {
    const RESET_VALUE: u32 = 0x0102;
}
