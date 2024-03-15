#[doc = "Register `CRU_CLKSEL_CON3` reader"]
pub type R = crate::R<CruClkselCon3Spec>;
#[doc = "Register `CRU_CLKSEL_CON3` writer"]
pub type W = crate::W<CruClkselCon3Spec>;
#[doc = "Field `ATCLK_CORE_B_DIV_CON` reader - atclk_core_b divider control register clk=clk_src/(div_con+1)"]
pub type AtclkCoreBDivConR = crate::FieldReader;
#[doc = "Field `ATCLK_CORE_B_DIV_CON` writer - atclk_core_b divider control register clk=clk_src/(div_con+1)"]
pub type AtclkCoreBDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PCLK_DBG_B_DIV_CON` reader - pclk_dbg_b divider control register clk=clk_src/(div_con+1)"]
pub type PclkDbgBDivConR = crate::FieldReader;
#[doc = "Field `PCLK_DBG_B_DIV_CON` writer - pclk_dbg_b divider control register clk=clk_src/(div_con+1)"]
pub type PclkDbgBDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PCLKEN_DBG_B_DIV_CON` reader - pclken_dbg_b divider control register clk=clk_src/(div_con+1)"]
pub type PclkenDbgBDivConR = crate::FieldReader;
#[doc = "Field `PCLKEN_DBG_B_DIV_CON` writer - pclken_dbg_b divider control register clk=clk_src/(div_con+1)"]
pub type PclkenDbgBDivConW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - atclk_core_b divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn atclk_core_b_div_con(&self) -> AtclkCoreBDivConR {
        AtclkCoreBDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - pclk_dbg_b divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn pclk_dbg_b_div_con(&self) -> PclkDbgBDivConR {
        PclkDbgBDivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - pclken_dbg_b divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn pclken_dbg_b_div_con(&self) -> PclkenDbgBDivConR {
        PclkenDbgBDivConR::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - atclk_core_b divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn atclk_core_b_div_con(&mut self) -> AtclkCoreBDivConW<CruClkselCon3Spec> {
        AtclkCoreBDivConW::new(self, 0)
    }
    #[doc = "Bits 8:12 - pclk_dbg_b divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_dbg_b_div_con(&mut self) -> PclkDbgBDivConW<CruClkselCon3Spec> {
        PclkDbgBDivConW::new(self, 8)
    }
    #[doc = "Bits 13:14 - pclken_dbg_b divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn pclken_dbg_b_div_con(&mut self) -> PclkenDbgBDivConW<CruClkselCon3Spec> {
        PclkenDbgBDivConW::new(self, 13)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon3Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon3Spec;
impl crate::RegisterSpec for CruClkselCon3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con3::R`](R) reader structure"]
impl crate::Readable for CruClkselCon3Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con3::W`](W) writer structure"]
impl crate::Writable for CruClkselCon3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON3 to value 0x6303"]
impl crate::Resettable for CruClkselCon3Spec {
    const RESET_VALUE: u32 = 0x6303;
}
