#[doc = "Register `CRU_CLKSEL_CON7` reader"]
pub type R = crate::R<CruClkselCon7Spec>;
#[doc = "Register `CRU_CLKSEL_CON7` writer"]
pub type W = crate::W<CruClkselCon7Spec>;
#[doc = "Field `ACLK_VCODEC_DIV_CON` reader - aclk_vcodec divider control register clk=clk_src/(div_con+1)"]
pub type AclkVcodecDivConR = crate::FieldReader;
#[doc = "Field `ACLK_VCODEC_DIV_CON` writer - aclk_vcodec divider control register clk=clk_src/(div_con+1)"]
pub type AclkVcodecDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "aclk_vcodec clock source select control register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AclkVcodecPllSel {
    #[doc = "0: PPLL"]
    B00 = 0,
    #[doc = "1: PPLL"]
    B01 = 1,
    #[doc = "2: PPLL"]
    B10 = 2,
    #[doc = "3: PPLL"]
    B11 = 3,
}
impl From<AclkVcodecPllSel> for u8 {
    #[inline(always)]
    fn from(variant: AclkVcodecPllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AclkVcodecPllSel {
    type Ux = u8;
}
#[doc = "Field `ACLK_VCODEC_PLL_SEL` reader - aclk_vcodec clock source select control register"]
pub type AclkVcodecPllSelR = crate::FieldReader<AclkVcodecPllSel>;
impl AclkVcodecPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AclkVcodecPllSel {
        match self.bits {
            0 => AclkVcodecPllSel::B00,
            1 => AclkVcodecPllSel::B01,
            2 => AclkVcodecPllSel::B10,
            3 => AclkVcodecPllSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == AclkVcodecPllSel::B00
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == AclkVcodecPllSel::B01
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == AclkVcodecPllSel::B10
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == AclkVcodecPllSel::B11
    }
}
#[doc = "Field `ACLK_VCODEC_PLL_SEL` writer - aclk_vcodec clock source select control register"]
pub type AclkVcodecPllSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, AclkVcodecPllSel>;
impl<'a, REG> AclkVcodecPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(AclkVcodecPllSel::B00)
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(AclkVcodecPllSel::B01)
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(AclkVcodecPllSel::B10)
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(AclkVcodecPllSel::B11)
    }
}
#[doc = "Field `HCLK_VCODEC_DIV_CON` reader - hclk_vcodec divider control register clk=clk_src/(div_con+1)"]
pub type HclkVcodecDivConR = crate::FieldReader;
#[doc = "Field `HCLK_VCODEC_DIV_CON` writer - hclk_vcodec divider control register clk=clk_src/(div_con+1)"]
pub type HclkVcodecDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - aclk_vcodec divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn aclk_vcodec_div_con(&self) -> AclkVcodecDivConR {
        AclkVcodecDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - aclk_vcodec clock source select control register"]
    #[inline(always)]
    pub fn aclk_vcodec_pll_sel(&self) -> AclkVcodecPllSelR {
        AclkVcodecPllSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - hclk_vcodec divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn hclk_vcodec_div_con(&self) -> HclkVcodecDivConR {
        HclkVcodecDivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - aclk_vcodec divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_vcodec_div_con(&mut self) -> AclkVcodecDivConW<CruClkselCon7Spec> {
        AclkVcodecDivConW::new(self, 0)
    }
    #[doc = "Bits 6:7 - aclk_vcodec clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_vcodec_pll_sel(&mut self) -> AclkVcodecPllSelW<CruClkselCon7Spec> {
        AclkVcodecPllSelW::new(self, 6)
    }
    #[doc = "Bits 8:12 - hclk_vcodec divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_vcodec_div_con(&mut self) -> HclkVcodecDivConW<CruClkselCon7Spec> {
        HclkVcodecDivConW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon7Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon7Spec;
impl crate::RegisterSpec for CruClkselCon7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con7::R`](R) reader structure"]
impl crate::Readable for CruClkselCon7Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con7::W`](W) writer structure"]
impl crate::Writable for CruClkselCon7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON7 to value 0x0101"]
impl crate::Resettable for CruClkselCon7Spec {
    const RESET_VALUE: u32 = 0x0101;
}
