#[doc = "Register `DDR_DENALI_CTL_74` reader"]
pub type R = crate::R<DdrDenaliCtl74Spec>;
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_74::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl74Spec;
impl crate::RegisterSpec for DdrDenaliCtl74Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_74::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl74Spec {}
#[doc = "`reset()` method sets DDR_DENALI_CTL_74 to value 0"]
impl crate::Resettable for DdrDenaliCtl74Spec {
    const RESET_VALUE: u32 = 0;
}
