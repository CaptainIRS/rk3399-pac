#[doc = "Register `CLKSEL_CON11` reader"]
pub type R = crate::R<ClkselCon11Spec>;
#[doc = "Register `CLKSEL_CON11` writer"]
pub type W = crate::W<ClkselCon11Spec>;
#[doc = "Field `ACLK_RGA_DIV_CON` reader - aclk_rga divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AclkRgaDivConR = crate::FieldReader;
#[doc = "Field `ACLK_RGA_DIV_CON` writer - aclk_rga divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AclkRgaDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "aclk_rga clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AclkRgaPllSel {
    #[doc = "0: CPLL"]
    B00 = 0,
    #[doc = "1: GPLL"]
    B01 = 1,
    #[doc = "2: NPLL"]
    B10 = 2,
    #[doc = "3: PPLL"]
    B11 = 3,
}
impl From<AclkRgaPllSel> for u8 {
    #[inline(always)]
    fn from(variant: AclkRgaPllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AclkRgaPllSel {
    type Ux = u8;
}
#[doc = "Field `ACLK_RGA_PLL_SEL` reader - aclk_rga clock source select control register"]
pub type AclkRgaPllSelR = crate::FieldReader<AclkRgaPllSel>;
impl AclkRgaPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AclkRgaPllSel {
        match self.bits {
            0 => AclkRgaPllSel::B00,
            1 => AclkRgaPllSel::B01,
            2 => AclkRgaPllSel::B10,
            3 => AclkRgaPllSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == AclkRgaPllSel::B00
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == AclkRgaPllSel::B01
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == AclkRgaPllSel::B10
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == AclkRgaPllSel::B11
    }
}
#[doc = "Field `ACLK_RGA_PLL_SEL` writer - aclk_rga clock source select control register"]
pub type AclkRgaPllSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, AclkRgaPllSel>;
impl<'a, REG> AclkRgaPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(AclkRgaPllSel::B00)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(AclkRgaPllSel::B01)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(AclkRgaPllSel::B10)
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(AclkRgaPllSel::B11)
    }
}
#[doc = "Field `HCLK_RGA_DIV_CON` reader - hclk_rga divider control register\n\nclk=clk_src/(div_con+1)"]
pub type HclkRgaDivConR = crate::FieldReader;
#[doc = "Field `HCLK_RGA_DIV_CON` writer - hclk_rga divider control register\n\nclk=clk_src/(div_con+1)"]
pub type HclkRgaDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - aclk_rga divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn aclk_rga_div_con(&self) -> AclkRgaDivConR {
        AclkRgaDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - aclk_rga clock source select control register"]
    #[inline(always)]
    pub fn aclk_rga_pll_sel(&self) -> AclkRgaPllSelR {
        AclkRgaPllSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - hclk_rga divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn hclk_rga_div_con(&self) -> HclkRgaDivConR {
        HclkRgaDivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - aclk_rga divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_rga_div_con(&mut self) -> AclkRgaDivConW<ClkselCon11Spec> {
        AclkRgaDivConW::new(self, 0)
    }
    #[doc = "Bits 6:7 - aclk_rga clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_rga_pll_sel(&mut self) -> AclkRgaPllSelW<ClkselCon11Spec> {
        AclkRgaPllSelW::new(self, 6)
    }
    #[doc = "Bits 8:12 - hclk_rga divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_rga_div_con(&mut self) -> HclkRgaDivConW<ClkselCon11Spec> {
        HclkRgaDivConW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkselCon11Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselCon11Spec;
impl crate::RegisterSpec for ClkselCon11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel_con11::R`](R) reader structure"]
impl crate::Readable for ClkselCon11Spec {}
#[doc = "`write(|w| ..)` method takes [`clksel_con11::W`](W) writer structure"]
impl crate::Writable for ClkselCon11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL_CON11 to value 0x0101"]
impl crate::Resettable for ClkselCon11Spec {
    const RESET_VALUE: u32 = 0x0101;
}
