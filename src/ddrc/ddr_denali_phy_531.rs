#[doc = "Register `DDR_DENALI_PHY_531` reader"]
pub type R = crate::R<DdrDenaliPhy531Spec>;
#[doc = "Field `PHY_ADR_CALVL_OBS0_0` reader - Observation register for CA training for slice 0. READ-ONLY"]
pub type PhyAdrCalvlObs0_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register for CA training for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_calvl_obs0_0(&self) -> PhyAdrCalvlObs0_0R {
        PhyAdrCalvlObs0_0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_531::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy531Spec;
impl crate::RegisterSpec for DdrDenaliPhy531Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_531::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy531Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_531 to value 0"]
impl crate::Resettable for DdrDenaliPhy531Spec {
    const RESET_VALUE: u32 = 0;
}
