#[doc = "Register `DDR_DENALI_PHY_787` reader"]
pub type R = crate::R<DdrDenaliPhy787Spec>;
#[doc = "Field `PHY_ADR_CALVL_OBS0_2` reader - Observation register for CA training for slice 2. READ-ONLY"]
pub type PhyAdrCalvlObs0_2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register for CA training for slice 2. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_calvl_obs0_2(&self) -> PhyAdrCalvlObs0_2R {
        PhyAdrCalvlObs0_2R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_787::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy787Spec;
impl crate::RegisterSpec for DdrDenaliPhy787Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_787::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy787Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_787 to value 0"]
impl crate::Resettable for DdrDenaliPhy787Spec {
    const RESET_VALUE: u32 = 0;
}
