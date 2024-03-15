#[doc = "Register `DENALI_PHY_210` reader"]
pub type R = crate::R<DenaliPhy210Spec>;
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_210::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy210Spec;
impl crate::RegisterSpec for DenaliPhy210Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_210::R`](R) reader structure"]
impl crate::Readable for DenaliPhy210Spec {}
#[doc = "`reset()` method sets DENALI_PHY_210 to value 0"]
impl crate::Resettable for DenaliPhy210Spec {
    const RESET_VALUE: u32 = 0;
}
