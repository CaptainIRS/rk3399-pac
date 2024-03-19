#[doc = "Register `DDR_DENALI_PHY_82` reader"]
pub type R = crate::R<DdrDenaliPhy82Spec>;
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_82::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy82Spec;
impl crate::RegisterSpec for DdrDenaliPhy82Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_82::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy82Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_82 to value 0"]
impl crate::Resettable for DdrDenaliPhy82Spec {
    const RESET_VALUE: u32 = 0;
}
