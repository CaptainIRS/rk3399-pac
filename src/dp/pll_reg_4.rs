#[doc = "Register `PLL_REG_4` reader"]
pub type R = crate::R<PllReg4Spec>;
#[doc = "Register `PLL_REG_4` writer"]
pub type W = crate::W<PllReg4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PllReg4Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "Pll_control_4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_reg_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_reg_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllReg4Spec;
impl crate::RegisterSpec for PllReg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_reg_4::R`](R) reader structure"]
impl crate::Readable for PllReg4Spec {}
#[doc = "`write(|w| ..)` method takes [`pll_reg_4::W`](W) writer structure"]
impl crate::Writable for PllReg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL_REG_4 to value 0x23"]
impl crate::Resettable for PllReg4Spec {
    const RESET_VALUE: u32 = 0x23;
}
