#[doc = "Register `CLKSEL_CON21` reader"]
pub type R = crate::R<ClkselCon21Spec>;
#[doc = "Register `CLKSEL_CON21` writer"]
pub type W = crate::W<ClkselCon21Spec>;
#[doc = "Field `ACLK_EMMC_DIV_CON` reader - aclk_emmc divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AclkEmmcDivConR = crate::FieldReader;
#[doc = "Field `ACLK_EMMC_DIV_CON` writer - aclk_emmc divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AclkEmmcDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "aclk_emmc clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AclkEmmcPllSel {
    #[doc = "0: CPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<AclkEmmcPllSel> for bool {
    #[inline(always)]
    fn from(variant: AclkEmmcPllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACLK_EMMC_PLL_SEL` reader - aclk_emmc clock source select control register"]
pub type AclkEmmcPllSelR = crate::BitReader<AclkEmmcPllSel>;
impl AclkEmmcPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AclkEmmcPllSel {
        match self.bits {
            false => AclkEmmcPllSel::B0,
            true => AclkEmmcPllSel::B1,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AclkEmmcPllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AclkEmmcPllSel::B1
    }
}
#[doc = "Field `ACLK_EMMC_PLL_SEL` writer - aclk_emmc clock source select control register"]
pub type AclkEmmcPllSelW<'a, REG> = crate::BitWriter<'a, REG, AclkEmmcPllSel>;
impl<'a, REG> AclkEmmcPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AclkEmmcPllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AclkEmmcPllSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - aclk_emmc divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn aclk_emmc_div_con(&self) -> AclkEmmcDivConR {
        AclkEmmcDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - aclk_emmc clock source select control register"]
    #[inline(always)]
    pub fn aclk_emmc_pll_sel(&self) -> AclkEmmcPllSelR {
        AclkEmmcPllSelR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - aclk_emmc divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_emmc_div_con(&mut self) -> AclkEmmcDivConW<ClkselCon21Spec> {
        AclkEmmcDivConW::new(self, 0)
    }
    #[doc = "Bit 7 - aclk_emmc clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_emmc_pll_sel(&mut self) -> AclkEmmcPllSelW<ClkselCon21Spec> {
        AclkEmmcPllSelW::new(self, 7)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkselCon21Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselCon21Spec;
impl crate::RegisterSpec for ClkselCon21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel_con21::R`](R) reader structure"]
impl crate::Readable for ClkselCon21Spec {}
#[doc = "`write(|w| ..)` method takes [`clksel_con21::W`](W) writer structure"]
impl crate::Writable for ClkselCon21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL_CON21 to value 0x03"]
impl crate::Resettable for ClkselCon21Spec {
    const RESET_VALUE: u32 = 0x03;
}
