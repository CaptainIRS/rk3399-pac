#[doc = "Register `CLKDIV` reader"]
pub type R = crate::R<ClkdivSpec>;
#[doc = "Register `CLKDIV` writer"]
pub type W = crate::W<ClkdivSpec>;
#[doc = "Field `CLKDIVL` reader - scl low level clock count\n\nT(SCL_LOW) = Tclk_i2c * (CLKDIVL + 1) * 8"]
pub type ClkdivlR = crate::FieldReader<u16>;
#[doc = "Field `CLKDIVL` writer - scl low level clock count\n\nT(SCL_LOW) = Tclk_i2c * (CLKDIVL + 1) * 8"]
pub type ClkdivlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CLKDIVH` reader - scl high level clock count\n\nT(SCL_HIGH) = Tclk_i2c * (CLKDIVH + 1) * 8"]
pub type ClkdivhR = crate::FieldReader<u16>;
#[doc = "Field `CLKDIVH` writer - scl high level clock count\n\nT(SCL_HIGH) = Tclk_i2c * (CLKDIVH + 1) * 8"]
pub type ClkdivhW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - scl low level clock count\n\nT(SCL_LOW) = Tclk_i2c * (CLKDIVL + 1) * 8"]
    #[inline(always)]
    pub fn clkdivl(&self) -> ClkdivlR {
        ClkdivlR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - scl high level clock count\n\nT(SCL_HIGH) = Tclk_i2c * (CLKDIVH + 1) * 8"]
    #[inline(always)]
    pub fn clkdivh(&self) -> ClkdivhR {
        ClkdivhR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - scl low level clock count\n\nT(SCL_LOW) = Tclk_i2c * (CLKDIVL + 1) * 8"]
    #[inline(always)]
    #[must_use]
    pub fn clkdivl(&mut self) -> ClkdivlW<ClkdivSpec> {
        ClkdivlW::new(self, 0)
    }
    #[doc = "Bits 16:31 - scl high level clock count\n\nT(SCL_HIGH) = Tclk_i2c * (CLKDIVH + 1) * 8"]
    #[inline(always)]
    #[must_use]
    pub fn clkdivh(&mut self) -> ClkdivhW<ClkdivSpec> {
        ClkdivhW::new(self, 16)
    }
}
#[doc = "clock divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkdivSpec;
impl crate::RegisterSpec for ClkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv::R`](R) reader structure"]
impl crate::Readable for ClkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`clkdiv::W`](W) writer structure"]
impl crate::Writable for ClkdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKDIV to value 0x01"]
impl crate::Resettable for ClkdivSpec {
    const RESET_VALUE: u32 = 0x01;
}
