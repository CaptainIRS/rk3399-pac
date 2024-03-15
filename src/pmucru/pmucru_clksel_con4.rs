#[doc = "Register `PMUCRU_CLKSEL_CON4` reader"]
pub type R = crate::R<PmucruClkselCon4Spec>;
#[doc = "Register `PMUCRU_CLKSEL_CON4` writer"]
pub type W = crate::W<PmucruClkselCon4Spec>;
#[doc = "Field `CLK_32K_SUSPEND_DIV_CON` reader - clk_32k_suspend divider control register clk=clk_src/(div_con+1)"]
pub type Clk32kSuspendDivConR = crate::FieldReader<u16>;
#[doc = "Field `CLK_32K_SUSPEND_DIV_CON` writer - clk_32k_suspend divider control register clk=clk_src/(div_con+1)"]
pub type Clk32kSuspendDivConW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "clk_32k_suspend source select control register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clk32kSuspendSel {
    #[doc = "0: 32k from pmu 24m div"]
    B0 = 0,
    #[doc = "1: 32k from pmu 24m div"]
    B1 = 1,
}
impl From<Clk32kSuspendSel> for bool {
    #[inline(always)]
    fn from(variant: Clk32kSuspendSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_32K_SUSPEND_SEL` reader - clk_32k_suspend source select control register"]
pub type Clk32kSuspendSelR = crate::BitReader<Clk32kSuspendSel>;
impl Clk32kSuspendSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clk32kSuspendSel {
        match self.bits {
            false => Clk32kSuspendSel::B0,
            true => Clk32kSuspendSel::B1,
        }
    }
    #[doc = "32k from pmu 24m div"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Clk32kSuspendSel::B0
    }
    #[doc = "32k from pmu 24m div"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Clk32kSuspendSel::B1
    }
}
#[doc = "Field `CLK_32K_SUSPEND_SEL` writer - clk_32k_suspend source select control register"]
pub type Clk32kSuspendSelW<'a, REG> = crate::BitWriter<'a, REG, Clk32kSuspendSel>;
impl<'a, REG> Clk32kSuspendSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "32k from pmu 24m div"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Clk32kSuspendSel::B0)
    }
    #[doc = "32k from pmu 24m div"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Clk32kSuspendSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:9 - clk_32k_suspend divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_32k_suspend_div_con(&self) -> Clk32kSuspendDivConR {
        Clk32kSuspendDivConR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - clk_32k_suspend source select control register"]
    #[inline(always)]
    pub fn clk_32k_suspend_sel(&self) -> Clk32kSuspendSelR {
        Clk32kSuspendSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - clk_32k_suspend divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_32k_suspend_div_con(&mut self) -> Clk32kSuspendDivConW<PmucruClkselCon4Spec> {
        Clk32kSuspendDivConW::new(self, 0)
    }
    #[doc = "Bit 15 - clk_32k_suspend source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_32k_suspend_sel(&mut self) -> Clk32kSuspendSelW<PmucruClkselCon4Spec> {
        Clk32kSuspendSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<PmucruClkselCon4Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_clksel_con4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_clksel_con4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmucruClkselCon4Spec;
impl crate::RegisterSpec for PmucruClkselCon4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmucru_clksel_con4::R`](R) reader structure"]
impl crate::Readable for PmucruClkselCon4Spec {}
#[doc = "`write(|w| ..)` method takes [`pmucru_clksel_con4::W`](W) writer structure"]
impl crate::Writable for PmucruClkselCon4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUCRU_CLKSEL_CON4 to value 0x02dc"]
impl crate::Resettable for PmucruClkselCon4Spec {
    const RESET_VALUE: u32 = 0x02dc;
}
