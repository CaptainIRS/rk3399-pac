#[doc = "Register `CRU_CLKSEL_CON1` reader"]
pub type R = crate::R<CruClkselCon1Spec>;
#[doc = "Register `CRU_CLKSEL_CON1` writer"]
pub type W = crate::W<CruClkselCon1Spec>;
#[doc = "Field `ATCLK_CORE_L_DIV_CON` reader - atclk_core_l divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AtclkCoreLDivConR = crate::FieldReader;
#[doc = "Field `ATCLK_CORE_L_DIV_CON` writer - atclk_core_l divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AtclkCoreLDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PCLK_DBG_L_DIV_CON` reader - pclk_dbg_l divider control register\n\nclk=clk_src/(div_con+1)"]
pub type PclkDbgLDivConR = crate::FieldReader;
#[doc = "Field `PCLK_DBG_L_DIV_CON` writer - pclk_dbg_l divider control register\n\nclk=clk_src/(div_con+1)"]
pub type PclkDbgLDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - atclk_core_l divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn atclk_core_l_div_con(&self) -> AtclkCoreLDivConR {
        AtclkCoreLDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - pclk_dbg_l divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn pclk_dbg_l_div_con(&self) -> PclkDbgLDivConR {
        PclkDbgLDivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - atclk_core_l divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn atclk_core_l_div_con(&mut self) -> AtclkCoreLDivConW<CruClkselCon1Spec> {
        AtclkCoreLDivConW::new(self, 0)
    }
    #[doc = "Bits 8:12 - pclk_dbg_l divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_dbg_l_div_con(&mut self) -> PclkDbgLDivConW<CruClkselCon1Spec> {
        PclkDbgLDivConW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon1Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon1Spec;
impl crate::RegisterSpec for CruClkselCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con1::R`](R) reader structure"]
impl crate::Readable for CruClkselCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con1::W`](W) writer structure"]
impl crate::Writable for CruClkselCon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON1 to value 0x0303"]
impl crate::Resettable for CruClkselCon1Spec {
    const RESET_VALUE: u32 = 0x0303;
}
