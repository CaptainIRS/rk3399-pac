#[doc = "Register `DDR_DENALI_PHY_338` reader"]
pub type R = crate::R<DdrDenaliPhy338Spec>;
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_338::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy338Spec;
impl crate::RegisterSpec for DdrDenaliPhy338Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_338::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy338Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_338 to value 0"]
impl crate::Resettable for DdrDenaliPhy338Spec {
    const RESET_VALUE: u32 = 0;
}
