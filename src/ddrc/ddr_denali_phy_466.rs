#[doc = "Register `DDR_DENALI_PHY_466` reader"]
pub type R = crate::R<DdrDenaliPhy466Spec>;
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_466::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy466Spec;
impl crate::RegisterSpec for DdrDenaliPhy466Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_466::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy466Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_466 to value 0"]
impl crate::Resettable for DdrDenaliPhy466Spec {
    const RESET_VALUE: u32 = 0;
}
