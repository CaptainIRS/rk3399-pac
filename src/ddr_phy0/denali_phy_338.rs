#[doc = "Register `DENALI_PHY_338` reader"]
pub type R = crate::R<DenaliPhy338Spec>;
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_338::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy338Spec;
impl crate::RegisterSpec for DenaliPhy338Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_338::R`](R) reader structure"]
impl crate::Readable for DenaliPhy338Spec {}
#[doc = "`reset()` method sets DENALI_PHY_338 to value 0"]
impl crate::Resettable for DenaliPhy338Spec {
    const RESET_VALUE: u32 = 0;
}
