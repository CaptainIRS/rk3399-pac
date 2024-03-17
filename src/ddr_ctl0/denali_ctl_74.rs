#[doc = "Register `DENALI_CTL_74` reader"]
pub type R = crate::R<DenaliCtl74Spec>;
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_74::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl74Spec;
impl crate::RegisterSpec for DenaliCtl74Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_74::R`](R) reader structure"]
impl crate::Readable for DenaliCtl74Spec {}
#[doc = "`reset()` method sets DENALI_CTL_74 to value 0"]
impl crate::Resettable for DenaliCtl74Spec {
    const RESET_VALUE: u32 = 0;
}
