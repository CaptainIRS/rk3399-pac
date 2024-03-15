#[doc = "Register `DENALI_PHY_909` reader"]
pub type R = crate::R<DenaliPhy909Spec>;
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_909::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy909Spec;
impl crate::RegisterSpec for DenaliPhy909Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_909::R`](R) reader structure"]
impl crate::Readable for DenaliPhy909Spec {}
#[doc = "`reset()` method sets DENALI_PHY_909 to value 0"]
impl crate::Resettable for DenaliPhy909Spec {
    const RESET_VALUE: u32 = 0;
}
