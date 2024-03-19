#[doc = "Register `CRU_CLKSEL_CON57` reader"]
pub type R = crate::R<CruClkselCon57Spec>;
#[doc = "Register `CRU_CLKSEL_CON57` writer"]
pub type W = crate::W<CruClkselCon57Spec>;
#[doc = "Field `PCLK_ALIVE_DIV_CON` reader - pclk_alive divider control register\n\nclk=clk_src/(div_con+1)"]
pub type PclkAliveDivConR = crate::FieldReader;
#[doc = "Field `PCLK_ALIVE_DIV_CON` writer - pclk_alive divider control register\n\nclk=clk_src/(div_con+1)"]
pub type PclkAliveDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CLKOUT_24M_DIV_CON` reader - clkout_24m divider control register\n\nclk=clk_src/(div_con+1)"]
pub type Clkout24mDivConR = crate::FieldReader<u16>;
#[doc = "Field `CLKOUT_24M_DIV_CON` writer - clkout_24m divider control register\n\nclk=clk_src/(div_con+1)"]
pub type Clkout24mDivConW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - pclk_alive divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn pclk_alive_div_con(&self) -> PclkAliveDivConR {
        PclkAliveDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:15 - clkout_24m divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clkout_24m_div_con(&self) -> Clkout24mDivConR {
        Clkout24mDivConR::new(((self.bits >> 6) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - pclk_alive divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_alive_div_con(&mut self) -> PclkAliveDivConW<CruClkselCon57Spec> {
        PclkAliveDivConW::new(self, 0)
    }
    #[doc = "Bits 6:15 - clkout_24m divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clkout_24m_div_con(&mut self) -> Clkout24mDivConW<CruClkselCon57Spec> {
        Clkout24mDivConW::new(self, 6)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon57Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register57\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con57::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con57::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon57Spec;
impl crate::RegisterSpec for CruClkselCon57Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con57::R`](R) reader structure"]
impl crate::Readable for CruClkselCon57Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con57::W`](W) writer structure"]
impl crate::Writable for CruClkselCon57Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON57 to value 0x05"]
impl crate::Resettable for CruClkselCon57Spec {
    const RESET_VALUE: u32 = 0x05;
}
