#[doc = "Register `DENALI_PHY_466` reader"]
pub type R = crate::R<DenaliPhy466Spec>;
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_466::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy466Spec;
impl crate::RegisterSpec for DenaliPhy466Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_466::R`](R) reader structure"]
impl crate::Readable for DenaliPhy466Spec {}
#[doc = "`reset()` method sets DENALI_PHY_466 to value 0"]
impl crate::Resettable for DenaliPhy466Spec {
    const RESET_VALUE: u32 = 0;
}
