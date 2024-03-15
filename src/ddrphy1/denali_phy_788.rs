#[doc = "Register `DENALI_PHY_788` reader"]
pub type R = crate::R<DenaliPhy788Spec>;
#[doc = "Field `PHY_ADR_CALVL_OBS1_2` reader - Observation register for CA training for slice 2. READ-ONLY"]
pub type PhyAdrCalvlObs1_2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register for CA training for slice 2. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_calvl_obs1_2(&self) -> PhyAdrCalvlObs1_2R {
        PhyAdrCalvlObs1_2R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_788::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy788Spec;
impl crate::RegisterSpec for DenaliPhy788Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_788::R`](R) reader structure"]
impl crate::Readable for DenaliPhy788Spec {}
#[doc = "`reset()` method sets DENALI_PHY_788 to value 0"]
impl crate::Resettable for DenaliPhy788Spec {
    const RESET_VALUE: u32 = 0;
}
