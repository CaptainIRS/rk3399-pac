#[doc = "Register `CLKSEL_CON6` reader"]
pub type R = crate::R<ClkselCon6Spec>;
#[doc = "Register `CLKSEL_CON6` writer"]
pub type W = crate::W<ClkselCon6Spec>;
#[doc = "Field `CLK_DDRC_DIV_CON` reader - clk_ddrc divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkDdrcDivConR = crate::FieldReader;
#[doc = "Field `CLK_DDRC_DIV_CON` writer - clk_ddrc divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkDdrcDivConW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "clk_ddrc clock source select control register\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkDdrcPllSel {
    #[doc = "0: LPLL"]
    B00 = 0,
    #[doc = "1: BPLL"]
    B01 = 1,
    #[doc = "2: DPLL"]
    B10 = 2,
    #[doc = "3: GPLL"]
    B11 = 3,
}
impl From<ClkDdrcPllSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkDdrcPllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkDdrcPllSel {
    type Ux = u8;
}
#[doc = "Field `CLK_DDRC_PLL_SEL` reader - clk_ddrc clock source select control register"]
pub type ClkDdrcPllSelR = crate::FieldReader<ClkDdrcPllSel>;
impl ClkDdrcPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkDdrcPllSel {
        match self.bits {
            0 => ClkDdrcPllSel::B00,
            1 => ClkDdrcPllSel::B01,
            2 => ClkDdrcPllSel::B10,
            3 => ClkDdrcPllSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "LPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkDdrcPllSel::B00
    }
    #[doc = "BPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkDdrcPllSel::B01
    }
    #[doc = "DPLL"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ClkDdrcPllSel::B10
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == ClkDdrcPllSel::B11
    }
}
#[doc = "Field `CLK_DDRC_PLL_SEL` writer - clk_ddrc clock source select control register"]
pub type ClkDdrcPllSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ClkDdrcPllSel>;
impl<'a, REG> ClkDdrcPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkDdrcPllSel::B00)
    }
    #[doc = "BPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkDdrcPllSel::B01)
    }
    #[doc = "DPLL"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(ClkDdrcPllSel::B10)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(ClkDdrcPllSel::B11)
    }
}
#[doc = "Field `PCLK_DDR_DIV_CON` reader - pclk_ddr divider control register\n\nclk=clk_src/(div_con+1)"]
pub type PclkDdrDivConR = crate::FieldReader;
#[doc = "Field `PCLK_DDR_DIV_CON` writer - pclk_ddr divider control register\n\nclk=clk_src/(div_con+1)"]
pub type PclkDdrDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "pclk_ddr clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PclkDdrPllSel {
    #[doc = "0: CPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<PclkDdrPllSel> for bool {
    #[inline(always)]
    fn from(variant: PclkDdrPllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCLK_DDR_PLL_SEL` reader - pclk_ddr clock source select control register"]
pub type PclkDdrPllSelR = crate::BitReader<PclkDdrPllSel>;
impl PclkDdrPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkDdrPllSel {
        match self.bits {
            false => PclkDdrPllSel::B0,
            true => PclkDdrPllSel::B1,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PclkDdrPllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PclkDdrPllSel::B1
    }
}
#[doc = "Field `PCLK_DDR_PLL_SEL` writer - pclk_ddr clock source select control register"]
pub type PclkDdrPllSelW<'a, REG> = crate::BitWriter<'a, REG, PclkDdrPllSel>;
impl<'a, REG> PclkDdrPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PclkDdrPllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PclkDdrPllSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:2 - clk_ddrc divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_ddrc_div_con(&self) -> ClkDdrcDivConR {
        ClkDdrcDivConR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - clk_ddrc clock source select control register"]
    #[inline(always)]
    pub fn clk_ddrc_pll_sel(&self) -> ClkDdrcPllSelR {
        ClkDdrcPllSelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:12 - pclk_ddr divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn pclk_ddr_div_con(&self) -> PclkDdrDivConR {
        PclkDdrDivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - pclk_ddr clock source select control register"]
    #[inline(always)]
    pub fn pclk_ddr_pll_sel(&self) -> PclkDdrPllSelR {
        PclkDdrPllSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - clk_ddrc divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddrc_div_con(&mut self) -> ClkDdrcDivConW<ClkselCon6Spec> {
        ClkDdrcDivConW::new(self, 0)
    }
    #[doc = "Bits 4:5 - clk_ddrc clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddrc_pll_sel(&mut self) -> ClkDdrcPllSelW<ClkselCon6Spec> {
        ClkDdrcPllSelW::new(self, 4)
    }
    #[doc = "Bits 8:12 - pclk_ddr divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_ddr_div_con(&mut self) -> PclkDdrDivConW<ClkselCon6Spec> {
        PclkDdrDivConW::new(self, 8)
    }
    #[doc = "Bit 15 - pclk_ddr clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_ddr_pll_sel(&mut self) -> PclkDdrPllSelW<ClkselCon6Spec> {
        PclkDdrPllSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkselCon6Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselCon6Spec;
impl crate::RegisterSpec for ClkselCon6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel_con6::R`](R) reader structure"]
impl crate::Readable for ClkselCon6Spec {}
#[doc = "`write(|w| ..)` method takes [`clksel_con6::W`](W) writer structure"]
impl crate::Writable for ClkselCon6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL_CON6 to value 0x0320"]
impl crate::Resettable for ClkselCon6Spec {
    const RESET_VALUE: u32 = 0x0320;
}
