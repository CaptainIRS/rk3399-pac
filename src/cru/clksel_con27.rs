#[doc = "Register `CLKSEL_CON27` reader"]
pub type R = crate::R<ClkselCon27Spec>;
#[doc = "Register `CLKSEL_CON27` writer"]
pub type W = crate::W<ClkselCon27Spec>;
#[doc = "Field `CLK_TSADC_DIV_CON` reader - clk tsadc divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkTsadcDivConR = crate::FieldReader<u16>;
#[doc = "Field `CLK_TSADC_DIV_CON` writer - clk tsadc divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkTsadcDivConW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "clk tsadc clock select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkTsadcSel {
    #[doc = "0: xin_24m"]
    B0 = 0,
    #[doc = "1: clk_32k"]
    B1 = 1,
}
impl From<ClkTsadcSel> for bool {
    #[inline(always)]
    fn from(variant: ClkTsadcSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_TSADC_SEL` reader - clk tsadc clock select control register"]
pub type ClkTsadcSelR = crate::BitReader<ClkTsadcSel>;
impl ClkTsadcSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkTsadcSel {
        match self.bits {
            false => ClkTsadcSel::B0,
            true => ClkTsadcSel::B1,
        }
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkTsadcSel::B0
    }
    #[doc = "clk_32k"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkTsadcSel::B1
    }
}
#[doc = "Field `CLK_TSADC_SEL` writer - clk tsadc clock select control register"]
pub type ClkTsadcSelW<'a, REG> = crate::BitWriter<'a, REG, ClkTsadcSel>;
impl<'a, REG> ClkTsadcSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkTsadcSel::B0)
    }
    #[doc = "clk_32k"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkTsadcSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:9 - clk tsadc divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_tsadc_div_con(&self) -> ClkTsadcDivConR {
        ClkTsadcDivConR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - clk tsadc clock select control register"]
    #[inline(always)]
    pub fn clk_tsadc_sel(&self) -> ClkTsadcSelR {
        ClkTsadcSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - clk tsadc divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_tsadc_div_con(&mut self) -> ClkTsadcDivConW<ClkselCon27Spec> {
        ClkTsadcDivConW::new(self, 0)
    }
    #[doc = "Bit 15 - clk tsadc clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_tsadc_sel(&mut self) -> ClkTsadcSelW<ClkselCon27Spec> {
        ClkTsadcSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkselCon27Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con27::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con27::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselCon27Spec;
impl crate::RegisterSpec for ClkselCon27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel_con27::R`](R) reader structure"]
impl crate::Readable for ClkselCon27Spec {}
#[doc = "`write(|w| ..)` method takes [`clksel_con27::W`](W) writer structure"]
impl crate::Writable for ClkselCon27Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL_CON27 to value 0x02dc"]
impl crate::Resettable for ClkselCon27Spec {
    const RESET_VALUE: u32 = 0x02dc;
}
