#[doc = "Register `PLL_REG_4` reader"]
pub type R = crate::R<PllReg4Spec>;
#[doc = "Pll_control_4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_reg_4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllReg4Spec;
impl crate::RegisterSpec for PllReg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_reg_4::R`](R) reader structure"]
impl crate::Readable for PllReg4Spec {}
#[doc = "`reset()` method sets PLL_REG_4 to value 0x23"]
impl crate::Resettable for PllReg4Spec {
    const RESET_VALUE: u32 = 0x23;
}
