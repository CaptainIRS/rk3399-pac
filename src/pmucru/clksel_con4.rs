#[doc = "Register `CLKSEL_CON4` reader"]
pub type R = crate::R<ClkselCon4Spec>;
#[doc = "Register `CLKSEL_CON4` writer"]
pub type W = crate::W<ClkselCon4Spec>;
#[doc = "Field `CLK_32K_SUSPEND_DIV_CON` reader - clk_32k_suspend divider control register\n\nclk=clk_src/(div_con+1)"]
pub type Clk32kSuspendDivConR = crate::FieldReader<u16>;
#[doc = "Field `CLK_32K_SUSPEND_DIV_CON` writer - clk_32k_suspend divider control register\n\nclk=clk_src/(div_con+1)"]
pub type Clk32kSuspendDivConW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "clk_32k_suspend source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clk32kSuspendSel {
    #[doc = "0: test clock out"]
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
    #[doc = "test clock out"]
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
    #[doc = "test clock out"]
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
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:9 - clk_32k_suspend divider control register\n\nclk=clk_src/(div_con+1)"]
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
    #[doc = "Bits 0:9 - clk_32k_suspend divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_32k_suspend_div_con(&mut self) -> Clk32kSuspendDivConW<ClkselCon4Spec> {
        Clk32kSuspendDivConW::new(self, 0)
    }
    #[doc = "Bit 15 - clk_32k_suspend source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_32k_suspend_sel(&mut self) -> Clk32kSuspendSelW<ClkselCon4Spec> {
        Clk32kSuspendSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkselCon4Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselCon4Spec;
impl crate::RegisterSpec for ClkselCon4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel_con4::R`](R) reader structure"]
impl crate::Readable for ClkselCon4Spec {}
#[doc = "`write(|w| ..)` method takes [`clksel_con4::W`](W) writer structure"]
impl crate::Writable for ClkselCon4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL_CON4 to value 0x02dc"]
impl crate::Resettable for ClkselCon4Spec {
    const RESET_VALUE: u32 = 0x02dc;
}
