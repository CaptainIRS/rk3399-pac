#[doc = "Register `PMUCRU_CLKSEL_CON3` reader"]
pub type R = crate::R<PmucruClkselCon3Spec>;
#[doc = "Register `PMUCRU_CLKSEL_CON3` writer"]
pub type W = crate::W<PmucruClkselCon3Spec>;
#[doc = "Field `I2C4_DIV_CON` reader - i2c4 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type I2c4DivConR = crate::FieldReader;
#[doc = "Field `I2C4_DIV_CON` writer - i2c4 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type I2c4DivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - i2c4 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn i2c4_div_con(&self) -> I2c4DivConR {
        I2c4DivConR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - i2c4 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4_div_con(&mut self) -> I2c4DivConW<PmucruClkselCon3Spec> {
        I2c4DivConW::new(self, 0)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<PmucruClkselCon3Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_clksel_con3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_clksel_con3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmucruClkselCon3Spec;
impl crate::RegisterSpec for PmucruClkselCon3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmucru_clksel_con3::R`](R) reader structure"]
impl crate::Readable for PmucruClkselCon3Spec {}
#[doc = "`write(|w| ..)` method takes [`pmucru_clksel_con3::W`](W) writer structure"]
impl crate::Writable for PmucruClkselCon3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUCRU_CLKSEL_CON3 to value 0x03"]
impl crate::Resettable for PmucruClkselCon3Spec {
    const RESET_VALUE: u32 = 0x03;
}
