#[doc = "Register `DENALI_PHY_82` reader"]
pub type R = crate::R<DenaliPhy82Spec>;
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_82::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy82Spec;
impl crate::RegisterSpec for DenaliPhy82Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_82::R`](R) reader structure"]
impl crate::Readable for DenaliPhy82Spec {}
#[doc = "`reset()` method sets DENALI_PHY_82 to value 0"]
impl crate::Resettable for DenaliPhy82Spec {
    const RESET_VALUE: u32 = 0;
}
