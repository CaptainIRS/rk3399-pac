#[doc = "Register `PMUCRU_CLKSEL_CON2` reader"]
pub type R = crate::R<PmucruClkselCon2Spec>;
#[doc = "Register `PMUCRU_CLKSEL_CON2` writer"]
pub type W = crate::W<PmucruClkselCon2Spec>;
#[doc = "Field `I2C0_DIV_CON` reader - i2c0 divider control register clk=clk_src/(div_con+1)"]
pub type I2c0DivConR = crate::FieldReader;
#[doc = "Field `I2C0_DIV_CON` writer - i2c0 divider control register clk=clk_src/(div_con+1)"]
pub type I2c0DivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `I2C8_DIV_CON` reader - i2c8 divider control register clk=clk_src/(div_con+1)"]
pub type I2c8DivConR = crate::FieldReader;
#[doc = "Field `I2C8_DIV_CON` writer - i2c8 divider control register clk=clk_src/(div_con+1)"]
pub type I2c8DivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - i2c0 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn i2c0_div_con(&self) -> I2c0DivConR {
        I2c0DivConR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - i2c8 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn i2c8_div_con(&self) -> I2c8DivConR {
        I2c8DivConR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - i2c0 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_div_con(&mut self) -> I2c0DivConW<PmucruClkselCon2Spec> {
        I2c0DivConW::new(self, 0)
    }
    #[doc = "Bits 8:14 - i2c8 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn i2c8_div_con(&mut self) -> I2c8DivConW<PmucruClkselCon2Spec> {
        I2c8DivConW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<PmucruClkselCon2Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_clksel_con2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_clksel_con2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmucruClkselCon2Spec;
impl crate::RegisterSpec for PmucruClkselCon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmucru_clksel_con2::R`](R) reader structure"]
impl crate::Readable for PmucruClkselCon2Spec {}
#[doc = "`write(|w| ..)` method takes [`pmucru_clksel_con2::W`](W) writer structure"]
impl crate::Writable for PmucruClkselCon2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUCRU_CLKSEL_CON2 to value 0x0303"]
impl crate::Resettable for PmucruClkselCon2Spec {
    const RESET_VALUE: u32 = 0x0303;
}
