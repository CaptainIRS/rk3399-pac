#[doc = "Register `DENALI_PHY_659` reader"]
pub type R = crate::R<DenaliPhy659Spec>;
#[doc = "Field `PHY_ADR_CALVL_OBS0_1` reader - Observation register for CA training for slice 1. READ-ONLY"]
pub type PhyAdrCalvlObs0_1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register for CA training for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_calvl_obs0_1(&self) -> PhyAdrCalvlObs0_1R {
        PhyAdrCalvlObs0_1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_659::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy659Spec;
impl crate::RegisterSpec for DenaliPhy659Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_659::R`](R) reader structure"]
impl crate::Readable for DenaliPhy659Spec {}
#[doc = "`reset()` method sets DENALI_PHY_659 to value 0"]
impl crate::Resettable for DenaliPhy659Spec {
    const RESET_VALUE: u32 = 0;
}
