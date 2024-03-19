#[doc = "Register `DDR_DENALI_PHY_660` reader"]
pub type R = crate::R<DdrDenaliPhy660Spec>;
#[doc = "Field `PHY_ADR_CALVL_OBS1_1` reader - Observation register for CA training for slice 1. READ-ONLY"]
pub type PhyAdrCalvlObs1_1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register for CA training for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_calvl_obs1_1(&self) -> PhyAdrCalvlObs1_1R {
        PhyAdrCalvlObs1_1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_660::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy660Spec;
impl crate::RegisterSpec for DdrDenaliPhy660Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_660::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy660Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_660 to value 0"]
impl crate::Resettable for DdrDenaliPhy660Spec {
    const RESET_VALUE: u32 = 0;
}
