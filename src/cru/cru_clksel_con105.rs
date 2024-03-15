#[doc = "Register `CRU_CLKSEL_CON105` reader"]
pub type R = crate::R<CruClkselCon105Spec>;
#[doc = "Register `CRU_CLKSEL_CON105` writer"]
pub type W = crate::W<CruClkselCon105Spec>;
#[doc = "Field `CLK_TESTFRAC_FRAC_DIV_CON` reader - clk_testfrac frac divider control register Fout = Fsrc*numerator/denominator High 16-bit for numerator Low 16-bit for denominator"]
pub type ClkTestfracFracDivConR = crate::FieldReader<u32>;
#[doc = "Field `CLK_TESTFRAC_FRAC_DIV_CON` writer - clk_testfrac frac divider control register Fout = Fsrc*numerator/denominator High 16-bit for numerator Low 16-bit for denominator"]
pub type ClkTestfracFracDivConW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - clk_testfrac frac divider control register Fout = Fsrc*numerator/denominator High 16-bit for numerator Low 16-bit for denominator"]
    #[inline(always)]
    pub fn clk_testfrac_frac_div_con(&self) -> ClkTestfracFracDivConR {
        ClkTestfracFracDivConR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - clk_testfrac frac divider control register Fout = Fsrc*numerator/denominator High 16-bit for numerator Low 16-bit for denominator"]
    #[inline(always)]
    #[must_use]
    pub fn clk_testfrac_frac_div_con(&mut self) -> ClkTestfracFracDivConW<CruClkselCon105Spec> {
        ClkTestfracFracDivConW::new(self, 0)
    }
}
#[doc = "Internal clock select and divide register89\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con105::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con105::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon105Spec;
impl crate::RegisterSpec for CruClkselCon105Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con105::R`](R) reader structure"]
impl crate::Readable for CruClkselCon105Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con105::W`](W) writer structure"]
impl crate::Writable for CruClkselCon105Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON105 to value 0x0bb8_ea60"]
impl crate::Resettable for CruClkselCon105Spec {
    const RESET_VALUE: u32 = 0x0bb8_ea60;
}
