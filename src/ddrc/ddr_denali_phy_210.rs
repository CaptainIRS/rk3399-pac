#[doc = "Register `DDR_DENALI_PHY_210` reader"]
pub type R = crate::R<DdrDenaliPhy210Spec>;
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_210::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy210Spec;
impl crate::RegisterSpec for DdrDenaliPhy210Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_210::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy210Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_210 to value 0"]
impl crate::Resettable for DdrDenaliPhy210Spec {
    const RESET_VALUE: u32 = 0;
}
