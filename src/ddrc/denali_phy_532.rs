#[doc = "Register `DENALI_PHY_532` reader"]
pub type R = crate::R<DenaliPhy532Spec>;
#[doc = "Field `PHY_ADR_CALVL_OBS1_0` reader - Observation register for CA training for slice 0."]
pub type PhyAdrCalvlObs1_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register for CA training for slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_obs1_0(&self) -> PhyAdrCalvlObs1_0R {
        PhyAdrCalvlObs1_0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_532::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy532Spec;
impl crate::RegisterSpec for DenaliPhy532Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_532::R`](R) reader structure"]
impl crate::Readable for DenaliPhy532Spec {}
#[doc = "`reset()` method sets DENALI_PHY_532 to value 0"]
impl crate::Resettable for DenaliPhy532Spec {
    const RESET_VALUE: u32 = 0;
}
